use crate::model::enums::PartialResult;
use crate::model::enums::WorkoutSummary;
use crate::model::traits::Analyzer;

pub struct WorkoutAnalyzer;
impl Analyzer for WorkoutAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {
        // Logic for WorkoutAnalyzer
        Some(PartialResult::WorkoutData(WorkoutSummary { date: 0}))
    }
}
