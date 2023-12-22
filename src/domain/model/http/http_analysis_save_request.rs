use serde::{Deserialize, Serialize};
use crate::domain::model::results::general_result::GeneralResult;
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct HttpAnalysisSaveRequest{
    pub user_id: Uuid,
    pub data: Vec<GeneralResult>
}