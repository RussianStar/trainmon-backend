use fitparser;

pub trait Analyzer {
    fn analyze(&self, timeslice: &fitparser::FitDataRecord) -> Option<enums::PartialResult>;
}