use crate::model::traits::Analyzer;
use crate::model::enums::PartialResult;
use crate::model::heart_rate_data::HrData;

pub struct HeartRateAnalyzer;

impl Analyzer for HeartRateAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult> {

        if let fitparser::profile::MesgNum::Record = timeslice.kind() {
            let heart_rate = timeslice.fields().iter().find(|f| f.name() == "heart_rate").and_then(|f| match f.value() {
                fitparser::Value::UInt8(v) => Some(*v),
                _ => None,
            });        
        
            println!("HeartRate : {}", heart_rate.unwrap() as u8); 
        // Logic for HeartRateAnalyzer
            return Some(PartialResult::HeartRateData(HrData { current: 0, average: 0, zone_percentages : vec![0.0;5]}));
        }
        None
    }
}
