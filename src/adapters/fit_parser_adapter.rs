use fitparser;
use std::fs::File;

use fitparser::profile::field_types::Sport;
use fitparser::profile::MesgNum;

use crate::ports::fit_file_parser::FitFileParser;

#[derive(Clone)]
pub struct FitParserAdapter;

impl FitParserAdapter {
    pub fn new() -> Self {
        FitParserAdapter {
        }
    }
}

impl FitFileParser for FitParserAdapter {
    fn parse_fit_file(&self ,file: &str) -> Result<Vec<fitparser::FitDataRecord>, Box<fitparser::ErrorKind>> {
        let mut fp = File::open(file)?;
        let mut data_vec = Vec::new();
        for data in fitparser::from_reader(&mut fp)? {
            data_vec.push(data);
        }
        Ok(data_vec)
    }
    fn check_sport_in_data(&self, data: &[fitparser::FitDataRecord], sports: &[Sport]) -> bool {
        data.iter().any(|record| {
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
            false
        })
    }
}


