
use chrono::NaiveDate;
use serde::{Deserializer, Serializer, Deserialize, Serialize};


#[derive(Debug)]
pub struct DateBytes {
    pub date: NaiveDate,
    pub data: Vec<u8>
}

impl DateBytes {
    pub fn new(date: NaiveDate, data: Vec<u8>) -> Self {
        Self { date: date, data: data }
    }
}

impl<'de> Deserialize<'de> for DateBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date_bytes: Vec<&str> = s.split('|').collect();
        let date = NaiveDate::parse_from_str(date_bytes[0], "%Y-%m-%d").map_err(serde::de::Error::custom)?;
        let bytes = base64::decode(date_bytes[1]).map_err(serde::de::Error::custom)?;
        Ok(DateBytes::new(date, bytes))
    }
}

impl Serialize for DateBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}|{}", self.date.format("%Y-%m-%d"), base64::encode(&self.data));
        serializer.serialize_str(&s)
    }
}