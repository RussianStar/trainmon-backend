use std::env;
use num_cpus;
use tokio::sync::Semaphore;
use std::sync::Arc;

use fitparser::profile::field_types::Sport;


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
    
    let semaphore = Arc::new(Semaphore::new(num_cpus::get())); // Limiting concurrent processing to the number of cores
    let process = |file: String| async move {
        let data = processing::fit_parsing::parse_fit_file(&file).unwrap();
        if processing::fit_parsing::check_sport_in_data(&data, &[Sport::Cycling]) {
            let zones: Vec<u8> = vec![0, 120, 145, 160, 172, 180, 255];
            Some(processing::heart_rate::map_hr_zones(data, zones))
        } else {
            None
        }
    };
    let heart_rate_data = processing::process::process_entries(semaphore, &paths, process).await;
    let heart_rate_data = heart_rate_data.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    
    println!("Heart rate data for all files: {:?}", heart_rate_data);
    println!("Processing completed");
}