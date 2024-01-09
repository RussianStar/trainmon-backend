use polars::frame::DataFrame;

pub trait TrainingPeaksImport {
    /// Rearanges the training peaks export by creating a data frame for various metrics
    /// Fails when ANY metrics are not present.
    fn transform(data: DataFrame, metrics: Vec<&str>) -> Result<DataFrame,anyhow::Error>;
}
