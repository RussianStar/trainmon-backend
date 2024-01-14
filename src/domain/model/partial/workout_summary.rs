use std::fmt::Display;
use std::ops::Div;
use std::str::FromStr;

use chrono::DateTime;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono::Utc;
use askama::Template;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::types::PgInterval;
use sqlx::types::BigDecimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String,
    pub distance: f64,
    pub tss: f64,
    pub file_hash: std::string::String,
}

impl WorkoutSummary {
    pub fn new(hash: String) -> Self {
        Self {
            start: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            duration: 0,
            sport: "".to_string(),
            distance: 0.0,
            tss: 0.0,
            file_hash: hash,
        }
    }
}
impl Display for WorkoutSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<WorkoutSummary> for WorkoutDb {
    fn from(value: WorkoutSummary) -> Self {
        todo!()
    }
}

impl From<&WorkoutSummary> for &WorkoutDb {
    fn from(value: &WorkoutSummary) -> Self {
        todo!()
    }
}
pub struct WorkoutDb {
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub duration: PgInterval,
    pub sport: std::string::String,
    pub distance: BigDecimal,
    pub tss: BigDecimal,
}



#[derive(Template)]
#[template(path = "workout.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutHtml {
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub duration: chrono::NaiveTime,
    pub sport:  String,
    pub distance: usize,
    pub tss: usize,
}

impl Into<WorkoutHtml> for WorkoutDb {
    fn into(self) -> WorkoutHtml {
        WorkoutHtml {
        start: self.start,
            end: self.end,
            duration: NaiveTime::from_hms_micro_opt(0, 0, 0, self.duration.microseconds as u32).unwrap(),
            sport: match self.sport { 
                    "cycling::gravel_cycling" => "Gravel".to_string(),
                    "cycling::mountain" => "Gravel".to_string(),
                    "cycling::road" => "Rennrad".to_string(),
                    "cycling::generic" => "Rennrad".to_string(),
                    "training::strength_training" => "Kraft".to_string(),
                    "training::cardio_training" => "Fußball".to_string(),
                    "running::generic" => "Laufen".to_string(),
                    _ => sport.to_string(),
                    },
            distance: self.distance as usize,
            tss: usize::try_from(self.tss).ok()
            };
    }
}
