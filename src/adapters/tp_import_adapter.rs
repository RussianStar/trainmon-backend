use uuid::Uuid;
use sqlx::PgPool;
use crate::application::helper::data_frame::create_dataframe_from_csv;
use axum::{
    extract::Multipart,
    response::Html
};

use crate::application::helper::uuid::create_uuid;
use serde::Deserialize;
use serde::Serialize;
use askama::Template;

use crate::ports::tp_import::TrainingPeaksImport;

use anyhow::Ok;
use anyhow::Result;

use polars::prelude::*;

#[derive(Template)]
#[template(path = "oura_upload.html")]
#[derive(Serialize, Deserialize,Debug)]
pub struct DataImport {
    pub message: String
}

pub async fn import_tp_metrics() -> Html<String> {
    let template = DataImport{ message: "".to_string()};
    Html(template.render().unwrap())
}

impl TrainingPeaksImport for DataImport {
    fn transform(data: DataFrame, metrics: Vec<&str>) -> Result<DataFrame,anyhow::Error> {
        let mut dfs: Vec<DataFrame> = vec![];
        for &metric in metrics.iter() {
            let mut df = data.filter(&data.column("Type")?.equal(metric)?)?;
            let _ = DataFrame::drop_in_place(&mut df, "Type")?;
            let rw = df.rename("Value", metric)?;

            dfs.push(rw.clone());
        }

        let mut result: DataFrame = dfs.first().unwrap().clone();
        for metric_df in dfs.iter().skip(1) {
            result = result.join(&metric_df, ["Timestamp"], ["Timestamp"], JoinArgs::new(JoinType::Outer { coalesce: true }))?;
        }

        let laz = result.clone()
            .lazy()
            .select([
                col("timestamp")
                    .cast(DataType::Date)
            ]
            )
                .collect()?;


        println!("{:?}", laz);
        return Ok(laz);
    }
}

pub async fn tp_metrics_upload(mut multipart: Multipart) -> Html<String> {
    let mut template = DataImport{ message: "".to_string()};

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
        let df = create_dataframe_from_csv(&data);
        println!("Created df");
        println!("{:?}",df);
        let merged = match DataImport::transform(df, vec!["Sleep Hours", "Pulse", "Weight Kilograms"]) {
            std::result::Result::Ok(merged) => merged,
            Err(error) => 
                {
                    println!("{:?}", error);
                    template.message = error.to_string();
                    return Html(template.render().unwrap())
                }
        };    

        println!("This is it : ");
        println!("{:?}", merged);
        break;
    }

    Html(template.render().unwrap())
}

#[derive(Serialize,Debug)]
struct HealthMetric {
    pub weight: f32,
    pub timestamp: f32,
    pub sleep_duration: f32,
    pub resting_heart_rate: i32,
    pub hrv: f32
}

async fn save_result_to_db(pool: &PgPool,result: Vec<HealthMetric> ,user_id: &Uuid) -> Result<(),anyhow::Error> {

    let mut transaction = pool.begin().await.map_err(|err| {
        eprintln!("Database error: {}", err);
        anyhow::Error::new(err)
    })?;
    let unique_id = create_uuid(&result);
    for metric_entry in result {
        let _query_result = sqlx::query(
            r#"
            INSERT INTO tp_metrics (id,time, provider, user_id, weight, sleep_duration, resting_heart_rate, hrv)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO NOTHING
            "#)
            .bind(&unique_id)
            .bind(&metric_entry.timestamp)
            .bind("tp")
            .bind(&user_id)
            .bind(&metric_entry.weight)
            .bind(&metric_entry.sleep_duration)
            .bind(&metric_entry.resting_heart_rate)
            .bind(&metric_entry.hrv)
            .execute(&mut *transaction)
            .await
            .map_err(|err| {
                eprintln!("Database error: {}", err);
                anyhow::Error::new(err)
            })?;
    }
    let _ = transaction.commit().await.map_err(|err| {
        eprintln!("Database error: {}", err);
        anyhow::Error::new(err)
    })?;

    anyhow::Result::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_and_print_first_line() -> Result<(), anyhow::Error> {
        let metrics = "C:\\work\\privat\\training-monitor\\users\\1\\metrics\\tp\\metrics.csv";

        // Read the CSV file into a DataFrame
        let df = CsvReader::from_path(metrics)?
            .infer_schema(None)
            .finish()?;
       
        println!("{:?}",df.head(Some(10)));

        let merged = DataImport::transform(df, vec!["Sleep Hours", "Pulse", "Weight Kilograms"]);
        println!("This is it : ");
        println!("{:?}", merged.unwrap());
        Ok(())
    }
}

