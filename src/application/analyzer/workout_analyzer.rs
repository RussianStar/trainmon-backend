use fitparser;

use crate::domain::model::partial::workout_summary::WorkoutSummary;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;
use crate::ports::analyzer::Analyzer;

use chrono::TimeZone;
use chrono::Utc;

pub struct WorkoutAnalyzer;
impl Analyzer for WorkoutAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, _profile: &UserModel) -> Option<PartialResult> {
        // Logic for WorkoutAnalyzer
        if let fitparser::profile::MesgNum::Session = timeslice.kind() {
            let start_date = timeslice.fields().iter().find(|f| f.name() == "start_time").and_then(|f| match f.value() {
                fitparser::Value::Timestamp(val) => Some(val.timestamp()),
                _ => None,
            });
            
            let end_date = timeslice.fields().iter().find(|f| f.name() == "timestamp").and_then(|f| match f.value() {
                fitparser::Value::Timestamp(val) => Some(val.timestamp()),
                _ => None,
            });
            
            let duration = timeslice.fields().iter().find(|f| f.name() == "total_elapsed_time").and_then(|f| match f.value() {
                fitparser::Value::Float64(v) => Some(*v),
                _ => None,
            });
            
            let sport = timeslice.fields().iter().find(|f| f.name() == "sport").and_then(|f| match f.value() {
                fitparser::Value::String(v) => Some(v.clone()),
                _ => None,
            });
            
            let sub_sport = timeslice.fields().iter().find(|f| f.name() == "sub_sport").and_then(|f| match f.value() {
                fitparser::Value::String(v) => Some(v.clone()),
                _ => None,
            });
            
            let start_datetime = Utc.timestamp_opt(start_date.unwrap(), 0).unwrap();
            let end_datetime = Utc.timestamp_opt(end_date.unwrap(), 0).unwrap();
            let duration_as_seconds = duration.unwrap() as u64;
            
            return Some(PartialResult::WorkoutData(WorkoutSummary { 
                start: start_datetime, 
                end: end_datetime,
                sport: format!("{}::{}", sport.unwrap_or_else(|| "".to_string()), sub_sport.unwrap_or_else(|| "".to_string())),
                duration: duration_as_seconds}));
        }
    None        
    }
}
