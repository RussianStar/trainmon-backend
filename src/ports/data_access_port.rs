use std::error::Error;
use crate::domain::model::results::general_result::GeneralResult;

pub trait DataAccessPort {
    fn add(&self, item: GeneralResult) -> Result<GeneralResult, Box<dyn Error>>;
    fn get_all(&self) -> Result<Vec<GeneralResult>, Box<dyn Error>>;
    fn get_by_id(&self, id: u32) -> Result<GeneralResult, Box<dyn Error>>;
    fn update_by_id(&self, id: u32, item: GeneralResult) -> Result<(), Box<dyn Error>>;
    fn delete_by_id(&self, id: u32) -> Result<(), Box<dyn Error>>;
}
