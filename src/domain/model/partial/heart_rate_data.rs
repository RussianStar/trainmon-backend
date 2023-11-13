use std::fmt;

#[derive(Debug)]
pub struct HrData {
    pub current: u8,
    pub average: u8,
    pub zone_percentages: Vec<f32>
}

impl fmt::Display for HrData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Average: {}, ", self.average)?;
        for (i, zone) in self.zone_percentages.iter().enumerate() {
            write!(f, "Zone {}: {:.2}%, ", i+1, zone)?;
        }
        Ok(())
    }
}