use crate::domain::model::partial::heart_rate_data::HrData;
use crate::domain::model::results::heart_rate_result::HeartRateResult;
use crate::domain::model::results::power_result::PowerResult;
use crate::domain::model::partial::workout_summary::WorkoutSummary;

pub enum AnalysisResult {
    Overview(WorkoutSummary),
    HeartRate(HrData),
    Power(PowerResult)
}
