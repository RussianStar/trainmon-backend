use anyhow::Error as AnyhowError;
use crate::domain::model::results::general_result::GeneralResult;
use anyhow::Result as AnyhowResult;

pub trait TrainingDataManagement {
    fn create(&self, item: GeneralResult) -> AnyhowResult<GeneralResult, AnyhowError>;
    fn read(&self) -> AnyhowResult<Vec<GeneralResult>,AnyhowError>;
    fn update(&self, id: u32, updated_item: GeneralResult) -> AnyhowResult<(),AnyhowError>;
    fn delete(&self, id: u32) -> AnyhowResult<(), AnyhowError>;
}