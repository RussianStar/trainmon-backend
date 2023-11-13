use fitparser;
use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::core::user_model::UserModel;

pub trait Analyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord, profile: &UserModel) -> Option<PartialResult>;
}