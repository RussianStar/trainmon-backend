
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

use crate::domain::core::data_bytes::DateBytes;
use crate::domain::model::http::file_provider_option::FileProviderOption;

#[derive(FromRow, Serialize, Deserialize)]
pub struct HttpAnalysisRequest {
    pub user_id: uuid::Uuid,
    pub file_provider_option: FileProviderOption,
    pub modes: Vec<String>,
    pub hr_zones: DateBytes,
    pub pwr_zones: DateBytes
}

impl HttpAnalysisRequest {
}

