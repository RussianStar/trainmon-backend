use askama::Template;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono::Utc;
use core::f64;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::types::PgInterval;
use sqlx::types::BigDecimal;
use std::fmt::Display;

#[derive(Template)]
#[template(path = "workout.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutHtml {
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub duration: chrono::NaiveTime,
    pub sport: String,
    pub distance: f64,
    pub tss: f64,
}

impl Into<WorkoutHtml> for WorkoutDb {
    fn into(self) -> WorkoutHtml {
        let total_seconds = self.duration.microseconds / 1_000_000; 
        let hours = (total_seconds / 3600) as u32;
        let minutes = ((total_seconds % 3600) / 60) as u32;
        let seconds = (total_seconds % 60) as u32;

        return WorkoutHtml {
            start: self.start,
            end: self.end,
            duration: match NaiveTime::from_hms_micro_opt(hours, minutes, seconds, 0) {
                Some(time) => time,
                None => {
                    eprintln!(
                        "Failed to create NaiveTime from duration microseconds: {}",
                        self.duration.microseconds
                    );
                    NaiveTime::from_hms_micro_opt(0, 0, 0, 0).unwrap()
                }
            },
            sport: match &self.sport[..] {
                "cycling::gravel_cycling" => "Gravel".to_string(),
                "cycling::mountain" => "Gravel".to_string(),
                "cycling::road" => "Rennrad".to_string(),
                "cycling::generic" => "Rennrad".to_string(),
                "cycling::indoor" => "Indoor".to_string(),
                "cycling::virtual_activity" => "Indoor".to_string(),
                "training::strength_training" => "Kraft".to_string(),
                "training::cardio_training" => "Fußball".to_string(),
                "running::generic" => "Laufen".to_string(),
                _ => "Wurscht".to_string(),
            },
            distance: self.distance.to_string().parse::<f64>().expect(&format!(
                "Failed to parse distance '{}' to f64",
                self.distance
            )),
            tss: self
                .tss
                .to_string()
                .parse::<f64>()
                .expect(&format!("Failed to parse tss '{}' to usize", self.tss)),
        };
    }
}
