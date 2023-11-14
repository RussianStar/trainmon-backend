mod adapters;
mod domain;
mod ports;
mod application;

use crate::domain::core::user_model::UserModel;
use crate::domain::model::results::analysis_result::AnalysisResult;

use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

use crate::adapters::fit_parser_adapter::FitParserAdapter;
use crate::adapters::fit_file_processor::FitFileProcessor;

#[tokio::main]
async fn main() {
    println!("Starting main function");

    let paths = std::fs::read_dir("C:\\Users\\TilmanRuß\\Garmin")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(std::ffi::OsStr::to_str) == Some("fit"))
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    let count = paths.len() as u32;

        
    let modes: Vec<String> = vec!["heart_rate".to_string(), "workout".to_string(), "power".to_string()];
    let profile: UserModel  = UserModel{
        name: String::from("test"), 
        hr_zones: vec![0,120,145,160,170,185,255],
        pwr_zones: vec![0,120,165,210,250,300,350,3000]
    };

    let parser = FitParserAdapter::new();
    let processor = FitFileProcessor::new(modes.clone(),parser);
    
    // Stop time.
    let start_time_all = std::time::Instant::now();
    let all =  processor.unwrap().execute(&paths, modes, profile).await;

    for result in all {
        match result {
            AnalysisResult::HeartRate(hr_data) => println!("HrData: {:?}", hr_data),
            AnalysisResult::Overview(workout_summary) => println!("WorkoutSummary: {}", workout_summary),
            AnalysisResult::Power(power) => println!("Power: {}", power),
            _ => println!("Invalid")
        }
    }

    let elapsed_time_all = start_time_all.elapsed();
    println!("Total time for all paths: {:?}", elapsed_time_all);
    println!("Total .fit files {}", paths.len());
    println!("Average time per path: {:?}", elapsed_time_all / count);
}