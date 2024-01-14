use std::fmt::Display;
use std::ops::Div;
use std::str::FromStr;

use chrono::TimeZone;
use chrono::Utc;
use askama::Template;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::types::PgInterval;
use sqlx::types::BigDecimal;

#[derive(Template)]
#[template(path = "workout.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,
    pub duration: u64,
    pub sport: std::string::String,
    pub distance: f64,
    pub tss: f64,
    pub file_hash: std::string::String,
}

impl WorkoutSummary {
    pub fn new(hash: String) -> Self {
        Self {
            start: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            duration: 0,
            sport: "".to_string(),
            distance: 0.0,
            tss: 0.0,
            file_hash: hash,
        }
    }
}


#[derive(Debug)]
pub struct FitPgInterval(PgInterval);
impl Display for FitPgInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_seconds = self.0.microseconds / 1_000_000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl From<PgInterval> for FitPgInterval {
    fn from(value: PgInterval) -> Self {
        Self(value)
    }
}

impl Serialize for FitPgInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_i64(self.0.microseconds)
    }
}
impl<'de> Deserialize<'de> for FitPgInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(FitPgInterval(PgInterval {microseconds: i64::from_str_radix(&s, 10).unwrap()
            , months: 0, days:0}))
    }
}
#[derive(Debug)]
pub struct FitBigDecimal(BigDecimal);
impl Display for FitBigDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:03}", self.0.to_string());
    }
}
impl From<BigDecimal> for FitBigDecimal {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}
impl<'de> Deserialize<'de> for FitBigDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(FitBigDecimal(BigDecimal::from_str(&s).unwrap()))
    }
}
impl Serialize for FitBigDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(Template)]
#[template(path = "workout.html")]
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutResponse {
    pub start: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub duration: FitPgInterval,
    pub sport: std::string::String,
    pub distance: FitBigDecimal,
    pub tss: FitBigDecimal,
}
