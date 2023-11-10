pub struct HeartRateAnalyzer;
impl Analyzer for HeartRateAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {
        // Logic for HeartRateAnalyzer
        Some(PartialResult::HeartRateData(HrData { /* ... */ }))
    }
}
