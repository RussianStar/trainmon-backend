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
    
            
            if let Some(hr) = heart_rate {
                return Some(PartialResult::HeartRateData(HrData { current: hr, average: 0, zone_percentages : vec![0.0;5]}));
            }
        }
        None
    }
}
