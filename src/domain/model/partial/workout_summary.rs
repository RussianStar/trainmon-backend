use chrono::Utc;
use std::fmt;

use serde::Serialize;
#[derive(Debug)]
#[derive(Serialize)]
pub struct WorkoutSummary{
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String
}

impl fmt::Display for WorkoutSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Duration: {}, Sport: {}, Start: {}, End: {}", self.duration, self.sport , self.start, self.end )
    }
}
