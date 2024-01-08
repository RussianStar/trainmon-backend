use axum::response::Html;

use serde::Deserialize;
use serde::Serialize;
use askama::Template;



#[derive(Template)]
#[template(path = "oura_upload.html")]
#[derive(Serialize, Deserialize,Debug)]
pub struct OuraUpload {
    pub path: String,
}

pub async fn import_oura() -> Html<String> {
    let template = OuraUpload { path: "".to_string()};
    Html(template.render().unwrap())
}

