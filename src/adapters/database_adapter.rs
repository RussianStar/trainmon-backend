use axum::{extract, http};
use sqlx::PgPool;

use crate::application::helper::uuid::{create_uuid, create_test_user_id};

use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

use crate::adapters::fit_parser_adapter::FitParserAdapter;
use crate::adapters::fit_file_processor::FitFileProcessor;

use crate::domain::core::user_model::UserModel;
use crate::domain::model::results::general_result::GeneralResult;
use crate::domain::model::http::http_analysis_request::HttpAnalysisRequest;
use crate::domain::model::http::http_analysis_result::HttpAnalysisResult;
use crate::domain::model::results::analysis_result::AnalysisResult;

pub async fn analyze(
    axum::Json(payload): axum::Json<HttpAnalysisRequest>,
) -> Result<(http::StatusCode, axum::Json<HttpAnalysisResult>), http::StatusCode> {

        let paths = payload.paths.clone();
        let modes = payload.modes.clone();

        println!("Received paths: {:?}", paths);
        println!("Received modes: {:?}", modes);

        let paths_clone = paths.clone();
        let modes_clone = modes.clone();
            
        let analyis = async move {
            let profile: UserModel  = UserModel{
                name: String::from("test"), 
                hr_zones: vec![0,120,145,160,170,185,255],
                pwr_zones: vec![0,120,165,210,250,300,350,3000]
            };  

            let parser = FitParserAdapter::new().into();
            let processor = FitFileProcessor::new(parser).unwrap();

            let all: Vec<GeneralResult> =  processor.execute(&paths_clone, modes_clone, profile).await;
            println!("Processor execution finished.");
            all
        };

        let res = analyis.await;
        let ret = HttpAnalysisResult::new(res);
        Ok((http::StatusCode::CREATED, axum::Json(ret)))
}



pub async fn create_records(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<HttpAnalysisResult>,
) -> Result<(http::StatusCode, axum::Json<i32>), http::StatusCode> {

    for result in &payload.data {
        let unique_id = create_uuid(&result);
        let user_id = create_test_user_id();

        for analysis_result in &result.results {
            let mut transaction = pool.begin().await.map_err(|err| {
                eprintln!("Database error: {}", err);
                http::StatusCode::INTERNAL_SERVER_ERROR
            })?;

            match analysis_result {
                AnalysisResult::Overview(workout_summary) => {
                    let query_result = sqlx::query(
                        r#"
                        INSERT INTO workouts (id, user_id, start_time, end_time, duration, sport, distance, tss)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        "#)
                        .bind(&unique_id)
                        .bind(&user_id)
                        .bind(&workout_summary.start)
                        .bind(&workout_summary.end)
                        .bind(&sqlx::postgres::types::PgInterval { months: 0, days: 0, microseconds: workout_summary.duration as i64 * 1_000_000 })
                        .bind(&workout_summary.sport)
                        .bind(&(workout_summary.distance as f64))
                        .bind(&(workout_summary.tss as f64))
                        .execute(&mut *transaction)
                        .await;

                    if let Err(err) = query_result {
                        eprintln!("Database error: {}", err);
                        continue;
                    }
                },
                AnalysisResult::HeartRate(heart_rate_result) => {
                    let query_result = sqlx::query(
                        r#"
                        INSERT INTO heart_rate_data (workout_id, average,  average_effective,time_in_zone, time_in_zone_effective)
                        VALUES ($1, $2, $3, $4, $5)
                        "#)
                        .bind(&unique_id)
                        .bind(&(heart_rate_result.average as i32))
                        .bind(&(heart_rate_result.average_effective as i32))
                        .bind(&heart_rate_result.time_in_zone.iter().map(|f| *f as i32).collect::<Vec<i32>>())
                        .bind(&heart_rate_result.time_in_zone_effective.iter().map(|f| *f as i32).collect::<Vec<i32>>())
                        .execute(&mut *transaction)
                        .await;

                    if let Err(err) = query_result {
                        eprintln!("Database error: {}", err);
                        continue;
                    }

                },
                AnalysisResult::Power(power_result) => {
                    let query_result = sqlx::query(
                        r#"
                        INSERT INTO power_data (workout_id, average, weighted_average, normalized, time_in_zone, time_in_zone_effective)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        "#)
                        .bind(&unique_id)
                        .bind(&(power_result.average as i32))
                        .bind(&(power_result.weighted_average as i32))
                        .bind(&(power_result.normalized as i32))
                        .bind(&power_result.time_in_zone)
                        .bind(&power_result.time_in_zone_effective)
                        .execute(&mut *transaction)
                        .await;

                    if let Err(err) = query_result {
                        eprintln!("Database error: {}", err);
                        continue;
                    }
                },
            }

            transaction.commit().await.map_err(|err| {
                eprintln!("Database error: {}", err);
                http::StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
    }

    Ok((http::StatusCode::CREATED, axum::Json(1)))
}