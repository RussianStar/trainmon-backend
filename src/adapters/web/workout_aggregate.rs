use askama::Template;
use core::f64;
use serde::{Deserialize, Serialize };
use std::{u8, u32};

use crate::adapters::workout_handler::WorkoutAggregate;

#[derive(Template)]
#[template(path = "weekly_summary.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklySummary {
    pub kw: u8,
    pub year: u32,
    pub total_duration: String,
    pub total_distance: f64,
    pub total_tss: f64,
}

impl Into<WeeklySummary> for WorkoutAggregate {
    fn into(self) -> WeeklySummary {
        let total_seconds = match self.total_duration {
            Some(time) => time.microseconds / 1_000_000,
            None => 0,
        };
        let hours = (total_seconds / 3600) as u32;
        let minutes = ((total_seconds % 3600) / 60) as u32;
        let seconds = (total_seconds % 60) as u32;

        return WeeklySummary {
            kw: match self.aggregation_unit {
                Some(unit) => unit.round() as u8,
                None => 0,
            },
            year: 0,
            total_duration: format!("{:02}:{:02}", hours, minutes),
            total_tss: match self.total_tss {
                Some(tss) => tss
                    .to_string()
                    .parse::<f64>()
                    .expect("Failed to parse tss '{}' to usize"),
                None => {
                    eprintln!("Couldnt get tss, so there must be none");
                    0.0
                }
            },
            total_distance: match self.total_distance {
                Some(distance) => distance
                    .to_string()
                    .parse::<f64>()
                    .expect("Failed to parse tss '{}' to usize"),
                None => {
                    eprintln!("Failed to parse tss");
                    0.0
                }
            },
        };
    }
}
