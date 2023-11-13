use crate::domain::model::partial::workout_summary::WorkoutSummary;
use crate::domain::model::partial::heart_rate_data::HrData;
use crate::domain::model::partial::power_data::PowerData;
#[derive(Debug)]
pub enum PartialResult{
    PowerData(PowerData),
    HeartRateData(HrData),
    WorkoutData(WorkoutSummary)
}