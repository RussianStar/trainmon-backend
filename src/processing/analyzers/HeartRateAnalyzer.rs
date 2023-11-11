use crate::model::traits::Analyzer;
use crate::model::enums::PartialResult;
use crate::model::heart_rate_data::HrData;

pub struct HeartRateAnalyzer;

impl Analyzer for HeartRateAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {
        // Logic for HeartRateAnalyzer
        Some(PartialResult::HeartRateData(HrData { current: 0, average: 0, zone_percentages : vec![0.0;5]}))
    }
}
