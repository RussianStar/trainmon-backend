use crate::domain::model::results::general_result::GeneralResult;

pub trait Database{
    fn save_results(&self, results:Vec<GeneralResult>) -> Result<(), Box<dyn std::error::Error>>;
}