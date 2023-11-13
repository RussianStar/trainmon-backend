use crate::ports::analyzer::Analyzer;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;

pub struct PowerAnalyzer;
impl Analyzer for PowerAnalyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, profile: &UserModel) -> Option<PartialResult> {
        None
    }
}
