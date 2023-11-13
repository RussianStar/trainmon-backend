use std::fmt;

#[derive(Debug)]
pub struct PowerData{
    pub current_power: u16
}

impl fmt::Display for PowerData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Power: {}", self.current_power)
    }
}