use super::heart_rate_data::HrData;
use std::fmt;
use chrono::Utc;
use fitparser::profile::field_types::DateTime;

#[derive(Debug)]
pub enum PartialResult{
    PowerData(PowerData),
    HeartRateData(HrData),
    WorkoutData(WorkoutSummary)
}

#[derive(Debug)]
pub struct PowerData{
    pub current_power: u16
}

#[derive(Debug)]
pub struct WorkoutSummary{
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String
}

impl fmt::Display for WorkoutSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start: {}, End: {}, Duration: {}, Sport: {}", self.start, self.end, self.duration, self.sport)
    }
}