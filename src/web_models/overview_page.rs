use serde::Deserialize;
use serde::Serialize;
use askama_axum::{Template, IntoResponse};

#[derive(Template)]
#[template(path = "pages/overview.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewHtml {

}

pub async fn overview_handler() -> impl IntoResponse {
    OverviewHtml::into_response(OverviewHtml {  })
}
