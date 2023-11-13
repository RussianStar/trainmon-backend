use chrono::Utc;
use std::fmt;

#[derive(Debug)]
pub struct WorkoutSummary{
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String
}

impl fmt::Display for WorkoutSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start: {}, End: {}, Duration: {}, Sport: {}", self.start, self.end, self.duration, self.sport)
    }
}
