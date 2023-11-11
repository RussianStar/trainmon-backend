use super::heart_rate_data::HrData;
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
    pub duration: u64
}