use std::fmt;
use crate::domain::model::results::analysis_result::AnalysisResult;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct GeneralResult{
    pub results: Vec<AnalysisResult>,
    pub workout_id: String
}


impl GeneralResult {
    pub fn new(results: Vec<AnalysisResult>) -> Self {
        Self { results , workout_id: "".to_string()}
    }
}

impl fmt::Display for GeneralResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for result in &self.results {
            match result {
                AnalysisResult::HeartRate(hr_data) => writeln!(f, "HEART: {}", hr_data),
                AnalysisResult::Overview(workout_summary) => writeln!(f, "TOTAL: {}", workout_summary),
                AnalysisResult::Power(power) => writeln!(f, "POWER: {}", power)
            }?;
        }
        Ok(())
    }
}
