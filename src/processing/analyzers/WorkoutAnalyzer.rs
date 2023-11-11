use fitparser;

use crate::model::enums::PartialResult;
use crate::model::enums::WorkoutSummary;
use crate::model::traits::Analyzer;


pub struct WorkoutAnalyzer;
impl Analyzer for WorkoutAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {
        // Logic for WorkoutAnalyzer
        if let MesgNum::Session = timeslice.kind() {
            let start_time = timeslice.get_field("timestamp");
            Some(PartialResult::WorkoutData(WorkoutSummary { date: start_time}))
        }


    }
}
