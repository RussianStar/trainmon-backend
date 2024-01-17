use polars::frame::DataFrame;
use sqlx::PgPool;
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseRetrieval {

async fn get_column_from_table(pool: &PgPool,user_id: &String ,column_name: &String, table_name: &String) -> Result<DataFrame, anyhow::Error>; 
}
