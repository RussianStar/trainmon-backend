use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub struct PowerResult{
    pub average: u16,
    pub weighted_average: u16,
    pub normalized: u16,
    pub time_in_zone: Vec<f32>,
    pub time_in_zone_effective: Vec<f32>
}

impl fmt::Display for PowerResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Average: {}, ", self.average)?;
        for (i, zone) in self.time_in_zone.iter().enumerate() {
            write!(f, "Zone {}: {:.2}%, ", i+1, zone)?;
        }
        Ok(())
    }
}