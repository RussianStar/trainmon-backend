use axum::{extract, http};
use sqlx::PgPool;
use uuid::Uuid;

use crate::application::helper::uuid::create_uuid;

use crate::ports::fit_file_processing_command::FitFileProcessingCommand;

use crate::adapters::fit_parser_adapter::FitParserAdapter;
use crate::adapters::fit_file_processor::FitFileProcessor;

use crate::domain::core::user_model::UserModel;
use crate::domain::model::results::general_result::GeneralResult;
use crate::domain::model::http::http_analysis_request::HttpAnalysisRequest;
use crate::domain::model::http::http_analysis_result::HttpAnalysisResult;
use crate::domain::model::http::http_analysis_save_request::HttpAnalysisSaveRequest;
use crate::domain::model::results::analysis_result::AnalysisResult;
use crate::domain::model::http::file_provider_option::FileProviderOption;

async fn analyze_paths(file_options: FileProviderOption, modes: Vec<String>, hr_zones: Vec<u8>, pwr_zones: Vec<u16>) -> Vec<GeneralResult> {
    let file_options_clone = file_options.clone();
    let modes_clone = modes.clone();
        
    let analyis = async move {
        let profile: UserModel  = UserModel{
            name: String::from("test"), 
            hr_zones: hr_zones,
            pwr_zones: pwr_zones
        };  

        let parser = FitParserAdapter::new().into();
        let processor = FitFileProcessor::new(parser).unwrap();

        let all: Vec<GeneralResult> =  processor.execute(&file_options_clone, modes_clone, profile).await;
        println!("Processor execution finished.");
        all
    };
    analyis.await
}

pub async fn analyze(
    axum::Json(payload): axum::Json<HttpAnalysisRequest>,
) -> Result<(http::StatusCode, axum::Json<HttpAnalysisResult>), http::StatusCode> {

        let file_options = payload.file_provider_option.clone();
        let modes = payload.modes.clone();
        let hr_zones: Vec<u8> = vec![0,120,145,160,170,185,255];
        let pwr_zones: Vec<u16> = vec![0,120,165,210,250,300,350,3000];

        println!("Received paths: {:?}", file_options);
        println!("Received modes: {:?}", modes);

        let res = analyze_paths(file_options, modes, hr_zones, pwr_zones).await;
        let ret = HttpAnalysisResult::new(res);
        Ok((http::StatusCode::CREATED, axum::Json(ret)))
}


pub async fn full(    
    extract::State(pool): extract::State<PgPool>,
axum::Json(payload): axum::Json<HttpAnalysisRequest>
) -> Result<(http::StatusCode, axum::Json<i32>), http::StatusCode> {
    let file_options = payload.file_provider_option.clone();
    let modes = payload.modes.clone();
    let hr_zones: Vec<u8> = vec![0,120,145,160,170,185,255];
    let pwr_zones: Vec<u16> = vec![0,120,165,210,250,300,350,3000];

    println!("Received paths: {:?}", file_options);
    println!("Received modes: {:?}", modes);

    let user_id = &payload.user_id;

    let mut counter =0;
    let analysis_results = analyze_paths(file_options, modes, hr_zones, pwr_zones).await;
    for result in analysis_results {
        let _ = save_result_to_db(&pool,result,&user_id).await;
        counter += 1;
    }

    Ok((http::StatusCode::OK, axum::Json(counter)))

}

async fn save_result_to_db(pool: &PgPool,result: GeneralResult ,user_id: &Uuid) -> Result<(),anyhow::Error> {

    let mut transaction = pool.begin().await.map_err(|err| {
        eprintln!("Database error: {}", err);
        anyhow::Error::new(err)
    })?;
    let unique_id = create_uuid(&result);
    for analysis_result in &result.results {
        match analysis_result {
            AnalysisResult::Overview(workout_summary) => {
                let _query_result = sqlx::query(
                    r#"
                    INSERT INTO workouts (id, user_id, start_time, end_time, duration, sport, distance, tss)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    ON CONFLICT (id) DO NOTHING
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
                    .await
                    .map_err(|err| {
                        eprintln!("Database error: {}", err);
                        anyhow::Error::new(err)
                    })?;
            },
            AnalysisResult::HeartRate(heart_rate_result) => {
                let _query_result = sqlx::query(
                    r#"
                    INSERT INTO heart_rate_data (workout_id, average,  average_effective,time_in_zone, time_in_zone_effective)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (workout_id) DO NOTHING
                    "#)
                    .bind(&unique_id)
                    .bind(&(heart_rate_result.average as i32))
                    .bind(&(heart_rate_result.average_effective as i32))
                    .bind(&heart_rate_result.time_in_zone.iter().map(|f| *f as i32).collect::<Vec<i32>>())
                    .bind(&heart_rate_result.time_in_zone_effective.iter().map(|f| *f as i32).collect::<Vec<i32>>())
                    .execute(&mut *transaction)
                    .await
                    .map_err(|err| {
                        eprintln!("Database error: {}", err);
                        anyhow::Error::new(err)
                    })?;

            },
            AnalysisResult::Power(power_result) => {
                let _query_result = sqlx::query(
                    r#"
                    INSERT INTO power_data (workout_id, average, weighted_average, normalized, time_in_zone, time_in_zone_effective)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (workout_id) DO NOTHING
                    "#)
                    .bind(&unique_id)
                    .bind(&(power_result.average as i32))
                    .bind(&(power_result.weighted_average as i32))
                    .bind(&(power_result.normalized as i32))
                    .bind(&power_result.time_in_zone)
                    .bind(&power_result.time_in_zone_effective)
                    .execute(&mut *transaction)
                    .await
                    .map_err(|err| {
                        eprintln!("Database error: {}", err);
                        anyhow::Error::new(err)
                    })?;
            },
        }
    }
    let _ = transaction.commit().await.map_err(|err| {
        eprintln!("Database error: {}", err);
        anyhow::Error::new(err)
    })?;

    anyhow::Result::Ok(())
}

pub async fn create_records(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<HttpAnalysisSaveRequest>,
) -> Result<(http::StatusCode, axum::Json<i32>), http::StatusCode> {

    let user_id = &payload.user_id;
    let len = payload.data.len() as i32;

    for result in payload.data {
        let _ = save_result_to_db(&pool,result,user_id).await;
    }

    Ok((http::StatusCode::OK, axum::Json(len)))
}