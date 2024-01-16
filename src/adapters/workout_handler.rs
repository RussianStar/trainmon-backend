use crate::domain::model::partial::workout_summary::{WorkoutDb, WorkoutHtml};
use askama::Template;
use axum::extract;
use axum::response::Html;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

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
