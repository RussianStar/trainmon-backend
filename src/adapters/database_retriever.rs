use crate::ports::database_retrieval::DatabaseRetrieval;
pub struct DatabaseRetriever{}
use async_trait::async_trait;
use polars::frame::DataFrame;
use polars::series::Series;

#[async_trait]
impl DatabaseRetrieval for DatabaseRetriever {
    async fn get_column_from_table(pool: &sqlx::PgPool,user_id: &String,column_name: &String, table_name: &String) -> Result<polars::prelude::DataFrame, anyhow::Error> {
    let mut transaction = pool.begin().await.map_err(|err| {
        eprintln!("Database error: {}", err);
        anyhow::Error::new(err)
        })?;

        let _query_result = sqlx::query(
            r#"
            SELECT 
            Timestamp as Timestamp,
            $1
            FROM $2
            WHERE 
            user_id == $3
            ORDER BY
            Timestamp
            "#)
            .bind(&column_name)
            .bind(&table_name)
            .bind(&user_id)
            .execute(&mut *transaction)
            .await
            .map_err(|err| {
                eprintln!("Database error: {}", err);
                anyhow::Error::new(err)
                    })?;
        let empty_df = DataFrame::new(Vec::<Series>::new())?;
        return Ok(empty_df);
    }
}   

