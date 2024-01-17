use polars::frame::DataFrame;

pub trait TrainingEffect {

    // Calculates the trainings effect by trying to get rid of the acute fitness changes.
    fn calculate_effect(performance_indicator: &str, load_indicator: &str, form_indicator: &str) -> Result<DataFrame, anyhow::Error>;
}
