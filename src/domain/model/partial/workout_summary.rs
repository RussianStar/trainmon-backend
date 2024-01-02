use chrono::TimeZone;
use chrono::Utc;
use std::fmt;

use serde::Serialize;
use serde::Deserialize;
#[derive(Debug,Serialize, Deserialize)]
pub struct WorkoutSummary{
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String,
    pub distance: f64,
    pub tss: f64,
    pub file_hash: std::string::String
}

impl WorkoutSummary {
    pub fn new(hash: String) -> Self{
        Self { 
            start:Utc.with_ymd_and_hms(2020, 1, 1,0, 0, 0).unwrap(),
            end:Utc.with_ymd_and_hms(2020, 1, 1,0, 0, 0).unwrap(),
            duration: 0, sport: "".to_string(), distance: 0.0, tss: 0.0 , file_hash: hash }
    }
}

impl fmt::Display for WorkoutSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Duration: {}, Distance {}m,  Sport: {}, Start: {}, End: {}, ID : {}", self.duration, self.distance, self.sport , self.start, self.end, self.file_hash)
    }
}
