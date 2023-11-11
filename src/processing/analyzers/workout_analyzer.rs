use fitparser;

use crate::model::enums::PartialResult;
use crate::model::enums::WorkoutSummary;
use crate::model::traits::Analyzer;

use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;

pub struct WorkoutAnalyzer;
impl Analyzer for WorkoutAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {
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
            
           
            let start_datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(start_date.unwrap(), 0));
            let formatted_start_datetime = start_datetime.format("%d:%m:%y - %H:%M").to_string();
            println!("Start date: {}", formatted_start_datetime);
            
            let end_datetime = Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(end_date.unwrap(), 0));
            let formatted_end_datetime = end_datetime.format("%d:%m:%y - %H:%M").to_string();
            println!("End date: {}", formatted_end_datetime);
            
            let duration_as_seconds = duration.unwrap() as u64;
            let hours = duration_as_seconds / 3600;
            let minutes = (duration_as_seconds % 3600) / 60;
            let seconds = duration_as_seconds % 60;
            println!("Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);

            println!("SPORT : {} : {}", sport.unwrap(),sub_sport.unwrap());
            return Some(PartialResult::WorkoutData(WorkoutSummary { start: start_datetime, end: end_datetime, duration: duration_as_seconds}));
        }
None        
    }
}
