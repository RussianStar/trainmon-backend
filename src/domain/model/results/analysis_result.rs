use crate::domain::model::results::heart_rate_result::HeartRateResult;
use crate::domain::model::results::power_result::PowerResult;
use crate::domain::model::partial::workout_summary::WorkoutSummary;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub enum AnalysisResult {
    Overview(WorkoutSummary),
    HeartRate(HeartRateResult),
    Power(PowerResult)
}
