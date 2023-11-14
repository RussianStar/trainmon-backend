use crate::domain::model::partial::heart_rate_data::HrData;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;
use crate::ports::analyzer::Analyzer;

pub struct HeartRateAnalyzer;

impl Analyzer for HeartRateAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, _profile: &UserModel) -> Option<PartialResult> {

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
