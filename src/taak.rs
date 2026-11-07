use chrono::{DateTime, TimeDelta, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Taak {
    pub start: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub beschrijving: String,
    pub moeilijkheid: u8,
    pub tijdschatting: TimeDelta,
}
impl SaveToFile for Taak {}

pub trait SaveToFile: Serialize {
    fn to_file(&self, path: &Path) {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(path, json).unwrap();
    }
}

impl Taak {
    pub fn new(start: DateTime<Utc>, deadline: DateTime<Utc>, beschrijving:String, moeilijkheid: u8, tijdschatting: u8) -> Self{
        Taak {
            start,
            deadline,
            beschrijving, 
            moeilijkheid, 
            tijdschatting: TimeDelta::hours(tijdschatting as i64)
        }
    }
}
