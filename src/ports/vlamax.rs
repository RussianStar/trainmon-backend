pub trait Vlamax {
    /// Estimates the VLamax from a power data sample.
    ///
    /// # Arguments
    ///
    /// * `power_data_sample` - A vector of power data samples.
    ///
    /// # Returns
    ///
    /// * A result containing a tuple of two f32 values if successful, or an error if not.
    /// * The first value is the actual vlamax and the other value is the symmetric error.
    fn estimate_from_sample(&self, power_data_sample: Vec<u16>) -> Result<(f64,f64), &str>;
}