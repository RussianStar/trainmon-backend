use fitparser;
use std::fs::File;

use fitparser::profile::field_types::Sport;
use fitparser::profile::MesgNum;

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

pub fn check_sport_in_data(data: &[fitparser::FitDataRecord], sports: &[Sport]) -> bool {
    data.iter().any(|record| {
        if let fitparser::FitDataRecord { .. } = record {
            if record.kind() == MesgNum::Sport {
                let sport_field = record.fields().iter()
                    .find(|field| field.name() == "sport");
                match sport_field {
                    Some(field) => {
                        match field.value() {
                            fitparser::Value::String(value) => {
                                return sports.iter().any(|sport| sport.to_string() == value.as_str());
                            },
                            _ => {
                                println!("The value is not a string");
                                return false;
                            }
                        }
                    },
                    None => {
                        println!("Sport field not found");
                        return false;
                    }
                }
            }
        }
        false
    })
}