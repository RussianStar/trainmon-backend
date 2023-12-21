use anyhow::Error as AnyhowError;
use anyhow::Result as AnyhowResult;

use crate::ports::data_management_port::TrainingDataManagement;
use crate::domain::model::results::general_result::GeneralResult;

pub struct WebAdapter{
}

impl WebAdapter{
}

impl TrainingDataManagement for WebAdapter {
    fn create(&self, _item: GeneralResult) -> AnyhowResult<GeneralResult, AnyhowError> {
        todo!()
    }

    fn read(&self) -> AnyhowResult<Vec<GeneralResult>, AnyhowError> {
        todo!()
    }

    fn update(&self, _id: u32, _updated_quote: GeneralResult) -> AnyhowResult<(), AnyhowError> {
        todo!()
    }

    fn delete(&self, _id: u32) -> AnyhowResult<(), AnyhowError> {
        todo!()
    }
}