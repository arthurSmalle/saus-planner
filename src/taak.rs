use chrono::{DateTime, TimeDelta, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{fs::File, io::BufReader, path::Path};

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
    pub fn new(
        start: DateTime<Utc>,
        deadline: DateTime<Utc>,
        beschrijving: String,
        moeilijkheid: u8,
        tijdschatting: u8,
    ) -> Self {
        Taak {
            start,
            deadline,
            beschrijving,
            moeilijkheid,
            tijdschatting: TimeDelta::hours(tijdschatting as i64),
        }
    }

    pub fn get_taken(project: &str) -> Vec<Taak> {
        let mut taken: Vec<Taak> = Vec::new();
        if let Ok(entries) = fs::read_dir(project) {
            for entry in entries.flatten() {
                // haal naam van files op
                let os_filename = entry.file_name();
                let filename: &str = os_filename.to_str().unwrap();
                let file_len = filename.len();
                if file_len <= 9 {continue;}
                // open en push taak
                if &filename[..4] == ("taak") && &filename[file_len - 5 ..] == ".json"{
                    let file = File::open(filename).unwrap();
                    let reader = BufReader::new(file);
                    let deserd_taak: Taak = serde_json::from_reader(reader).unwrap();
                    taken.push(deserd_taak);
                }
            }
        }
        taken
    }
}
