mod adapters;
mod domain;
mod ports;
mod application;

use axum::routing::{post, get,Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use adapters::database_adapter::{analyze, create_records,full,htmx};
use adapters::oura_adapter::*;
use uuid::Uuid;

use domain::model::http::http_analysis_request::import_form;

use adapters::askama::index;

#[tokio::main]
async fn main() {

    println!("Tilman : {}",Uuid::new_v5(&Uuid::NAMESPACE_DNS, "tilman".as_bytes()));
    println!("Sonja : {}",Uuid::new_v5(&Uuid::NAMESPACE_DNS, "sonja".as_bytes()));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/test/form", get(import_form))
        .route("/test/htmx", post(htmx))
        .route("/analyze", post(analyze))
        .route("/analyze/create", post(create_records))
        .route("/analyze/full", post(full))
        .route("/", get(index))
        .route("/oura/upload", get(import_oura))
        .route("/oura/upload", post(upload))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
