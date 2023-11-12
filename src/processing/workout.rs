use crate::model::{self, enums};
use chrono::{Utc, NaiveDateTime};

pub fn process_workout_summary(results: &[model::enums::PartialResult]) -> model::enums::WorkoutSummary {
    let results_a: Vec<_> = results.iter()
        .filter_map(|res| {
            if let model::enums::PartialResult::WorkoutData(res_a) = res {
                Some(res_a)
            } else {
                None
            }
        })
        .collect();

    if results_a.is_empty() {
        model::enums::WorkoutSummary {
            start: chrono::DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            end: chrono::DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            duration: 0,
            sport: String::from(""),
        }
    } else {
        let start = results_a.first().unwrap().start;
        let end = results_a.last().unwrap().end;
        let duration = (end - start).num_seconds() as u64;
        let sport = results_a.first().unwrap().sport.clone();
        model::enums::WorkoutSummary {
            start: start,
            end: end,
            duration: duration,
            sport: sport,
        }
    }
}