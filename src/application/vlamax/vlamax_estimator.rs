use nalgebra::DVector;

use crate::ports::vlamax::Vlamax;
use crate::application::math::fit_exponential::fit_with_error;

pub struct VLaMaxEstimator{
    max_duration: u8
}
impl VLaMaxEstimator {
    pub fn new() -> Self{
        Self { max_duration: u8::MAX }
    }
}

impl Vlamax for VLaMaxEstimator {
    fn estimate_from_sample(&self, sample: Vec<u16>) -> Result<(f64, f64), &str> {
        
        // No data is invalid.
        if sample.is_empty() {
            return Err("Empty sample.");
        }

        // Estimating longer duration degrades data quality and is not wise.
        if sample.len() > self.max_duration as usize {
            return Err("Sample too long.");
        }

        let x = DVector::from_vec((1..=sample.len()).map(|i| i as f64).collect());
        let y = DVector::from_vec(sample.iter().map(|i| *i as f64).collect());
        
        let (tau, error) = fit_with_error(x, y);

        Ok((tau, error))
    }
}



