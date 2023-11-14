use std::fmt;
use crate::domain::model::results::analysis_result::AnalysisResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeneralResult{
    results: Vec<AnalysisResult>
}

impl GeneralResult {
    pub fn new(results: Vec<AnalysisResult>) -> Self {
        Self { results }
    }
}

impl fmt::Display for GeneralResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for result in &self.results {
            match result {
                AnalysisResult::HeartRate(hr_data) => writeln!(f, "HEART: {}", hr_data),
                AnalysisResult::Overview(workout_summary) => writeln!(f, "TOTAL: {}", workout_summary),
                AnalysisResult::Power(power) => writeln!(f, "POWER: {}", power),
                _ => write!(f, "Invalid")
            }?;
        }
        Ok(())
    }
}