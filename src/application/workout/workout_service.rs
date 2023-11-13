use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::model::partial::workout_summary::WorkoutSummary;
use chrono::{Utc, NaiveDateTime};

pub fn process_workout_summary(results: &[PartialResult]) -> WorkoutSummary {
    let results_a: Vec<_> = results.iter()
        .filter_map(|res| {
            if let PartialResult::WorkoutData(res_a) = res {
                Some(res_a)
            } else {
                None
            }
        })
        .collect();

    if results_a.is_empty() {
        WorkoutSummary {
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
        WorkoutSummary {
            start: start,
            end: end,
            duration: duration,
            sport: sport,
        }
    }
}