mod adapters;
mod domain;
mod ports;
mod application;

use crate::adapters::fit_file_processor::FitFileProcessor;
use crate::domain::core::user_model::UserModel;
use crate::adapters::fit_parser_adapter::FitParserAdapter;
use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

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

        
    let modes: Vec<String> = vec!["heart_rate".to_string(), "workout".to_string()];
    let profile: UserModel  = UserModel{name: String::from("test"), hr_zones: vec![0,120,145,160,170,185,255]};

    let parser = FitParserAdapter::new();
    let processor = FitFileProcessor::new(modes.clone(),parser);
    
    // Stop time.
    let start_time_all = std::time::Instant::now();
    let all =  processor.unwrap().execute(&paths, modes, profile).await;

    let elapsed_time_all = start_time_all.elapsed();
    println!("Total time for all paths: {:?}", elapsed_time_all);
    println!("Total .fit files {}", paths.len());
    println!("Average time per path: {:?}", elapsed_time_all / count);

}