use polars::frame::DataFrame;

pub trait TrainingEffect {

    // Calculates the trainings effect by trying to get rid of the acute fitness changes.
    fn calculate_effect(performance_indicator: String, load_indicator: String, form_indicator: String) -> Result<DataFrame, anyhow::Error>;
}
