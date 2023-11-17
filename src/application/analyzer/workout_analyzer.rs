use fitparser;

use crate::domain::model::partial::workout_summary::WorkoutSummary;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;
use crate::ports::analyzer::Analyzer;

use chrono::TimeZone;
use chrono::Utc;

enum ValueType {
    Timestamp,
    Float64,
    String,
}

fn get_field_value(fields: &Vec<fitparser::FitDataField>, field_name: &str, value_type: ValueType) -> Option<fitparser::Value> {
    fields.iter().find(|f| f.name() == field_name).and_then(|f| match (f.value(), value_type) {
        (fitparser::Value::Timestamp(val), ValueType::Timestamp) => Some(fitparser::Value::Timestamp(Utc.timestamp(val, 0))),
        (fitparser::Value::Float64(val), ValueType::Float64) => Some(fitparser::Value::Float64(val)),
        (fitparser::Value::String(val), ValueType::String) => Some(fitparser::Value::String(val.clone())),
        _ => None,
    })
}

pub struct WorkoutAnalyzer;
impl Analyzer for WorkoutAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, _profile: &UserModel) -> Option<PartialResult> {
        // Logic for WorkoutAnalyzer
        if let fitparser::profile::MesgNum::Session = timeslice.kind() {

            let start_date = get_field_value(timeslice.fields(), "start_time", fitparser::Value::Timestamp(0));
            let end_date = get_field_value(timeslice.fields(), "timestamp", fitparser::Value::Timestamp(0));
            let duration = get_field_value(timeslice.fields(), "total_elapsed_time", fitparser::Value::Float64(0.0));
            let sport = get_field_value(timeslice.fields(), "sport", fitparser::Value::String(String::new()));
            let distance = get_field_value(timeslice.fields(), "total_distance", fitparser::Value::Float64(0.0));
            let tss = get_field_value(timeslice.fields(), "training_stress_score", fitparser::Value::Float64(0.0)).unwrap_or(0.0);
            let sub_sport = get_field_value(timeslice.fields(), "sub_sport", fitparser::Value::String(String::new()));

            return Some(PartialResult::WorkoutData(WorkoutSummary { 
                distance: distance.unwrap_or(0.),
                tss: tss,
                start: start_datetime, 
                end: end_datetime,
                sport: format!("{}::{}", sport.unwrap_or_else(|| "".to_string()), sub_sport.unwrap_or_else(|| "".to_string())),
                duration: duration_as_seconds}));
        }
    None        
    }
}
