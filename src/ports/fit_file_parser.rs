use fitparser::FitDataRecord;
use fitparser::ErrorKind;
use fitparser::profile::field_types::Sport;

pub trait FitFileParser : Clone {
    fn parse_fit_file(&self, file: &str) -> Result<Vec<FitDataRecord>, Box<ErrorKind>>;
    fn check_sport_in_data(&self, data: &[FitDataRecord], sports: &[Sport]) -> bool;
}