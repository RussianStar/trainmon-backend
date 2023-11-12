use std::env;
use num_cpus;
use tokio::sync::Semaphore;
use std::sync::Arc;

use fitparser::profile::field_types::Sport;

use crate::processing::analyzers::heart_rate_analyzer::HeartRateAnalyzer;
use crate::processing::analyzers::workout_analyzer::WorkoutAnalyzer;
use  crate::processing::heart_rate::process_heart_rate_data;
use crate::processing::workout::process_workout_summary;

mod processing;
mod model;

#[tokio::main]
async fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    println!("Starting main function");
    let file = "C:\\Users\\TilmanRuß\\Downloads\\10429100379_ACTIVITY.fit";

    let paths = std::fs::read_dir("C:\\Users\\TilmanRuß\\Garmin")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(std::ffi::OsStr::to_str) == Some("fit"))
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    
    let analyzers: Vec<Arc<dyn model::traits::Analyzer + Send + Sync>> = vec![Arc::new(HeartRateAnalyzer), Arc::new(WorkoutAnalyzer)];
    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));
    // Limiting concurrent processing to the number of cores
    let process = move |file: String| {
        let analyzers = analyzers.clone();
        async move {
            let data = processing::fit_parsing::parse_fit_file(&file).unwrap();
            let mut results: Vec<model::enums::PartialResult> = Vec::new();
            if processing::fit_parsing::check_sport_in_data(&data, &[Sport::Cycling]) {
                for dataslice in data {
                    for analyzer in &analyzers {
                        if let Some(result) = analyzer.analyze(&dataslice) {
                            results.push(result);
                        }
                    }
                }
            }
            results
        }
    };

    let all_results = processing::process::process_entries(semaphore, &paths, process).await;

    for results in &all_results {        
        let zones: [u8; 7] = [0, 120, 145, 160, 172, 180, 255];
        let final_result_hr = process_heart_rate_data(&results, &zones);
        let overview = process_workout_summary(&results);
        println!("{}",final_result_hr);
        println!("{}", overview);
    }

}