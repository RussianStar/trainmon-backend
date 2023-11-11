use super::heart_rate_data::HrData;

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
//    date: chrono::NaiveDateTime,
    pub date: u8
}