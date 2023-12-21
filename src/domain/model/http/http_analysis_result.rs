use serde::{Deserialize, Serialize};
use crate::domain::model::results::general_result::GeneralResult;
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct HttpAnalysisResult{
    pub data: Vec<GeneralResult>
}

impl HttpAnalysisResult {
    pub fn new(data: Vec<GeneralResult>) -> Self {
        Self { data }
    }
}