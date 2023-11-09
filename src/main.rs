use std::env;

mod processing;

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Starting main function");
    let file = "C:\\Users\\TilmanRuß\\Downloads\\10429100379_ACTIVITY.fit";
    let data = processing::fit_parsing::parse_fit_file(&file).unwrap();
   
    let zones: Vec<u8> = vec![0, 120, 145, 160, 172, 180, 255];
    processing::heart_rate::map_hr_zones(data, zones);
    println!("Processing completed");
}