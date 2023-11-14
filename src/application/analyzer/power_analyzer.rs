use crate::domain::model::partial::power_data::PowerData;
use crate::ports::analyzer::Analyzer;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;

pub struct PowerAnalyzer;
impl Analyzer for PowerAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, _profile: &UserModel) -> Option<PartialResult> {
        if let fitparser::profile::MesgNum::Record = timeslice.kind() {
            let current_power = timeslice.fields().iter().find(|f| f.name() == "power").and_then(|f| match f.value() {
                fitparser::Value::UInt16(v) => Some(*v),
                _ => None,
            });        
    
            if let Some(pwr) = current_power {
                return Some(PartialResult::PowerData(PowerData{ current_power: pwr}));
            }
        }
        None
    }
}
