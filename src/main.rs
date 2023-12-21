mod adapters;
mod domain;
mod ports;
mod application;

use axum::routing::{post, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use adapters::database_adapter::{analyze, create_records};

#[tokio::main]
async fn main() {

    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/analyze", post(analyze))
        .route("/analyze/create", post(create_records))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
