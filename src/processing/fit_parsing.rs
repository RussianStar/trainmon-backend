use fitparser;
use std::fs::File;

pub fn parse_fit_file(file: &str) -> Result<Vec<fitparser::FitDataRecord>, Box<fitparser::ErrorKind>> {
    println!("Parsing FIT files using Profile version: {:?}", fitparser::profile::VERSION);
    let mut fp = File::open(file)?;
    let mut data_vec = Vec::new();
    for data in fitparser::from_reader(&mut fp)? {
        // print the data in FIT file
        //println!("{:#?}", data);
        data_vec.push(data);
    }
    Ok(data_vec)
}

