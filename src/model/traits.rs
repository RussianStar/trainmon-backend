use fitparser;
use crate::model::enums::PartialResult;

pub trait Analyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<PartialResult>;
}