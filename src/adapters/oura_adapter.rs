use std::io::Read;

use axum::extract::Form;
use axum::response::Html;
use sqlx::PgPool;
use tokio::fs::File;
use tokio_util::{codec::{BytesCodec, FramedRead}, bytes::Bytes};
use futures::TryStreamExt;
use csv_async::AsyncDeserializer;

use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct SleepData {
    date: String,
    sleep_score: Option<i32>,
    total_sleep_score: Option<i32>,
    rem_sleep_score: Option<i32>,
    deep_sleep_score: Option<i32>,
    sleep_efficiency_score: Option<i32>,
    restfulness_score: Option<i32>,
    sleep_latency_score: Option<i32>,
    sleep_timing_score: Option<i32>,
    total_sleep_duration: Option<i32>,
    total_bedtime: Option<i32>,
    awake_time: Option<i32>,
    rem_sleep_duration: Option<i32>,
    light_sleep_duration: Option<i32>,
    deep_sleep_duration: Option<i32>,
    restless_sleep: Option<i32>,
    sleep_efficiency: Option<i32>,
    sleep_latency: Option<i32>,
    sleep_timing: Option<i32>,
    bedtime_start: Option<String>,
    bedtime_end: Option<String>,
    average_resting_heart_rate: Option<f32>,
    lowest_resting_heart_rate: Option<i32>,
    average_hrv: Option<i32>,
    temperature_deviation: Option<f32>,
    temperature_trend_deviation: Option<Option<f32>>,
    respiratory_rate: Option<f32>,
    activity_score: Option<i32>,
    stay_active_score: Option<i32>,
    move_every_hour_score: Option<i32>,
    meet_daily_targets_score: Option<i32>,
    training_frequency_score: Option<i32>,
    training_volume_score: Option<i32>,
    activity_burn: Option<i32>,
    total_burn: Option<i32>,
    steps: Option<i32>,
    equivalent_walking_distance: Option<i32>,
    inactive_time: Option<i32>,
    rest_time: Option<i32>,
    low_activity_time: Option<i32>,
    medium_activity_time: Option<i32>,
    high_activity_time: Option<i32>,
    non_wear_time: Option<i32>,
    average_met: Option<f32>,
    long_periods_of_inactivity: Option<i32>,
    readiness_score: Option<i32>,
    previous_night_score: Option<i32>,
    sleep_balance_score: Option<i32>,
    previous_day_activity_score: Option<i32>,
    activity_balance_score: Option<i32>,
    temperature_score: Option<i32>,
    resting_heart_rate_score: Option<i32>,
    hrv_balance_score: Option<i32>,
    recovery_index_score: Option<Option<i32>>,
}

use axum::extract::Multipart;
use serde::Serialize;
use askama::Template;
use futures::StreamExt;

use axum::http;


pub async fn upload(mut multipart: Multipart) -> Result<(http::StatusCode, axum::Json<i32>), http::StatusCode> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data: Bytes = field.bytes().await.unwrap();
        if name == "file" {

            let mut rdr = AsyncDeserializer::from_reader(data.as_ref());
            let mut records = rdr.deserialize::<SleepData>();
            while let Some(result) = records.next().await {
                match result {
                    Ok(res) => {
                        println!("{:?}", res.sleep_score);
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                } 
            }
        }

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    Ok((http::StatusCode::OK, axum::Json(1 as i32)))
}

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

