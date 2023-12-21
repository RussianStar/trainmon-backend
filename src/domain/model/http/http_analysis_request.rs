
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

use crate::domain::core::data_bytes::DateBytes;

#[derive(FromRow, Serialize, Deserialize)]
pub struct HttpAnalysisRequest {
    pub paths: Vec<String>,
    pub modes: Vec<String>,
    pub hr_zones: DateBytes,
    pub pwr_zones: DateBytes
}

impl HttpAnalysisRequest {
}