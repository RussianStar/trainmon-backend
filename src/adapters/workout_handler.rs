use crate::domain::model::partial::workout_summary::{WorkoutDb, WorkoutHtml};
use askama::Template;
use axum::extract;
use axum::response::Html;
use serde::Deserialize;
use sqlx::postgres::types::PgInterval;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use super::web::workout_aggregate::WeeklySummary;

#[derive(Deserialize)]
pub struct WorkoutRequestForm {
    user_name: String,
    count: usize,
}

pub async fn get_workouts(
    extract::State(pool): extract::State<PgPool>,
    form: axum::extract::Form<WorkoutRequestForm>,
) -> Html<String> {
    let user_id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, form.user_name.as_bytes());
    let workouts = sqlx::query_as!(
        WorkoutDb,
        r#"
            SELECT start_time as start, end_time as end, duration, sport, distance, tss 
            from workouts
            where user_id = $1
            ORDER BY start_time DESC
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let trimmed: Vec<WorkoutHtml> = workouts
        .into_iter()
        .take(form.count)
        .map(|workout| workout.into())
        .collect();
    let rendered: Vec<String> = trimmed
        .iter()
        .map(|workout| workout.render().unwrap())
        .collect();

    Html(rendered.join("\n"))
}

/// All types are taken from the sqlx database representation
#[derive(Debug)]
pub struct WorkoutAggregate {
    pub aggregation_unit: Option<f64>,
    pub total_duration: Option<PgInterval>,
    pub total_distance: Option<BigDecimal>,
    pub total_tss: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct WorkoutSummaryRequest {
    user_name: String,
    aggregation_interval: f64
}

pub async fn get_workout_summary(
    extract::State(pool): extract::State<PgPool>,
    form: axum::extract::Form<WorkoutSummaryRequest>,
) -> Html<String> {
    println!("Starting");
    let user_id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, form.user_name.as_bytes());
    let workouts = sqlx::query_as!(
        WorkoutAggregate,
        r#"
            SELECT
                EXTRACT(WEEK FROM start_time) AS aggregation_unit,
                SUM(duration) AS total_duration,
                SUM(distance) AS total_distance,
                SUM(tss) AS total_tss
            FROM
                workouts
            WHERE
                user_id = $1
                AND sport IN ('cycling::road', 'cycling::mountain', 'running::generic')
                AND EXTRACT(YEAR FROM start_time) = $2
            GROUP BY
                aggregation_unit
            ORDER BY
                aggregation_unit
        "#,
        user_id,
        form.aggregation_interval
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    println!("{:?}", workouts);
    let mut trimmed: Vec<WeeklySummary> = workouts
        .into_iter()
        .map(|workout| workout.into())
        .collect();

    let rendered: Vec<String> = trimmed
        .iter()
        .map(|workout| workout.render().unwrap())
        .collect();

    Html(rendered.join("\n"))
}
