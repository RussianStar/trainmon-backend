mod adapters;
mod domain;
mod ports;
mod application;

use  std::collections::HashMap;

use crate::domain::core::user_model::UserModel;
use crate::domain::model::results::general_result::GeneralResult;

use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

use crate::adapters::fit_parser_adapter::FitParserAdapter;
use crate::adapters::fit_file_processor::FitFileProcessor;

use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Starting main function");
    let analyze = warp::path!("analyze")
    .and(warp::post())
    .and(warp::body::json::<HashMap<String, Vec<String>>>())
    .and_then(|body: HashMap<String, Vec<String>>| {
        let paths = body.get("paths").unwrap_or(&Vec::new()).clone();
        let modes = body.get("modes").unwrap_or(&Vec::new()).clone();

            let paths_clone = paths.clone();
            let modes_clone = modes.clone();
            
            async move {
                let profile: UserModel  = UserModel{
                    name: String::from("test"), 
                    hr_zones: vec![0,120,145,160,170,185,255],
                    pwr_zones: vec![0,120,165,210,250,300,350,3000]
                };

                let parser = FitParserAdapter::new().into();
                let processor = FitFileProcessor::new(modes_clone.clone(),parser).unwrap();

                let all: Vec<GeneralResult> =  processor.execute(&paths_clone, modes_clone, profile).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&all))
            }
        });

    warp::serve(analyze).run(([127, 0, 0, 1], 3030)).await;
}