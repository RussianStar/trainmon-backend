use crate::application::helper::data_frame::create_dataframe_from_csv;
use axum::{
    extract::Multipart,
    response::Html
};


use serde::Deserialize;
use serde::Serialize;
use askama::Template;
use futures::StreamExt;

use axum::http;


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

pub async fn oura_csv_upload(mut multipart: Multipart) -> Html<String> {

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
        let df = create_dataframe_from_csv(&data);

        println!("{:?}", df);
        break;
    }

    let template = OuraUpload {path: "".to_string()};
    Html(template.render().unwrap())
}

