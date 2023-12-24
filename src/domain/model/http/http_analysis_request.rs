
use serde::{Deserialize, Serialize};

use askama::Template;
use axum::response::Html;
use chrono::NaiveDate;

use sqlx::FromRow;

use crate::domain::core::data_bytes::DateBytes;

#[derive(Template)]
#[template(path = "import.html")]
#[derive(FromRow, Serialize, Deserialize,Debug)]
pub struct HttpAnalysisRequest {
    pub user_id: uuid::Uuid,
    pub path: String,
    #[serde(default)]
    pub modes: Vec<String>,
    pub hr_zones: DateBytes,
    pub pwr_zones: DateBytes
}

impl HttpAnalysisRequest {
}

pub async fn import_form() -> Html<String> {
    let template = HttpAnalysisRequest {
        user_id: uuid::Uuid::parse_str("0edade91-3ffc-523c-be43-c649b6412a35").unwrap(),
        path: "".to_string(),
        modes: vec!["".to_string()],
        hr_zones: DateBytes::new(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(), vec![]),
        pwr_zones: DateBytes::new(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(), vec![]),
    };
    Html(template.render().unwrap())
}

