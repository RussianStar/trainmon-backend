use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize)]
pub struct HeartRateResult{
    pub average: u8,
    pub time_in_zone: Vec<f32>,
    pub average_effective: u8,
    pub time_in_zone_effective: Vec<f32>,
}

impl HeartRateResult {
    pub fn new(average: u8, time_in_zone: Vec<f32>) -> Self {
        Self { average: (average), 
            time_in_zone: (time_in_zone),
             average_effective: (0),
              time_in_zone_effective: (vec![0.0]) }
    }
}

impl std::fmt::Display for HeartRateResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Average: {}, Time in Zone: {:?}", 
            self.average, self.time_in_zone)
    }
}