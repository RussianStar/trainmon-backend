
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::pin::Pin;
use std::collections::HashMap;

use tokio::sync::Semaphore;
use num_cpus;

use fitparser::profile::field_types::Sport;

use crate::adapters::fit_parser_adapter::FitParserAdapter;

use crate::ports::fit_file_parser::FitFileParser;
use crate::ports::analyzer::Analyzer;
use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

use crate::application::analyzer::workout_analyzer::WorkoutAnalyzer;
use crate::application::analyzer::heart_rate_analyzer::HeartRateAnalyzer;
use crate::application::analyzer::power_analyzer::PowerAnalyzer;
use crate::application::processing::process::process_entries;
use crate::application::heart_rate::hr_service::process_heart_rate_data;
use crate::application::workout::workout_service::process_workout_summary;
use crate::application::power::pwr_service::process_power_data;

use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::model::results::general_result::GeneralResult;
use crate::domain::core::user_model::UserModel;
use crate::domain::model::results::analysis_result::AnalysisResult;


#[derive(Clone)]
pub struct FitFileProcessor{
    parser: Arc<FitParserAdapter>
}

impl FitFileProcessor {
    pub fn new(parser: Arc<FitParserAdapter>) -> Result<Self, Box<dyn Error>> {
        Ok(Self { parser })
    }
}

impl FitFileProcessingCommand for FitFileProcessor {
    fn execute(self, file_paths: &Vec<String>, analysis_modes: Vec<String>, user_profile: UserModel) -> Pin<Box<dyn Future<Output = Vec<GeneralResult>> + Send + '_>> {

        let user_profile_arc = Arc::new(user_profile.clone());
        let requested_analyzers = map_analysis_modes_to_analyzers(&analysis_modes).unwrap();

        Box::pin(async move {
            let num_threads = num_cpus::get() * 8;
            println!("Number of threads used : {}", num_threads);
            // Limiting concurrent processing to the number of cores
            let semaphore = Arc::new(Semaphore::new(num_threads));

            let process = move |file: String| {
                // Clone data for use inside this async block
                let requested_analyzers = requested_analyzers.clone();

                let parser = self.parser.clone();
                let user_profile = user_profile_arc.clone();

                async move {
                    let data = parser.parse_fit_file(&file).unwrap();
                    let mut results: Vec<PartialResult> = Vec::new();
                    if parser.check_sport_in_data(&data, &[Sport::Cycling]) {
                        for dataslice in data {
                            for analyzer in &requested_analyzers {
                                if let Some(result) = analyzer.analyze(&dataslice, &user_profile) {
                                    results.push(result);
                                }
                            }
                        }
                    }
                    results
                }
            };

            let all_partial_results = process_entries(semaphore, &file_paths, process).await;
            let all_analysis_results: Vec<GeneralResult> = all_partial_results.into_iter().map(|partial_results| {
                let workout_summary = process_workout_summary(&partial_results);
                let hr_data = process_heart_rate_data(&partial_results, &user_profile.hr_zones);
                let pwr_data = process_power_data(&partial_results, &user_profile.pwr_zones);

                let mut results: Vec<AnalysisResult> = Vec::new();
                if let Some(summary) = workout_summary {
                    results.push(AnalysisResult::Overview(summary));
                }
                if let Some(hr) = hr_data {
                    results.push(AnalysisResult::HeartRate(hr));
                }
                if let Some(pwr) = pwr_data {
                    results.push(AnalysisResult::Power(pwr));
                }
                GeneralResult::new(results)
            }).collect();

            all_analysis_results
        })
    }
}

fn map_analysis_modes_to_analyzers(analysis_modes: &Vec<String>) -> Result<Vec<Arc<dyn Analyzer + Send + Sync>>, Box<dyn Error>> {
    let workout_analyzer = Arc::new(WorkoutAnalyzer);
    let heart_rate_analyzer = Arc::new(HeartRateAnalyzer);
    let power_analyzer = Arc::new(PowerAnalyzer);
    let mut analyzers: HashMap<String, Arc<dyn Analyzer + Send + Sync>> = HashMap::new();
    analyzers.insert(String::from("workout"), workout_analyzer);
    analyzers.insert(String::from("heart_rate"), heart_rate_analyzer);
    analyzers.insert(String::from("power"), power_analyzer);
    
    let mut result = Vec::new();
    for mode in analysis_modes.iter() {
        // Use `mode` directly since it's already a &str
        match analyzers.get(mode) {
            Some(analyzer) => result.push(analyzer.clone()),
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid analysis mode"))),
        }
    }
    Ok(result)
    
}
