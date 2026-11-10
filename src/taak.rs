use super::{SaveToFile, project::*, sausid::*};
use chrono::{DateTime, TimeDelta, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{fs::File, io::BufReader, path::Path, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Taak {
    id: i64,
    pub start: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub beschrijving: String,
    pub moeilijkheid: u8,
    pub tijdschatting: TimeDelta,
}
impl SaveToFile for Taak {}

impl Taak {
    pub fn new(
        id_gen: &mut SausidGen,
        start: DateTime<Utc>,
        deadline: DateTime<Utc>,
        beschrijving: String,
        moeilijkheid: u8,
        tijdschatting: u8,
    ) -> Self {
        Taak {
            id: id_gen.get_saudid(),
            start,
            deadline,
            beschrijving,
            moeilijkheid,
            tijdschatting: TimeDelta::hours(tijdschatting as i64),
        }
    }

    pub fn get_taken(loc: &str) -> Vec<Taak> {
        let mut taken: Vec<Taak> = Vec::new();
        // if let Ok(entries) = fs::read_dir(proj.get_id().to_string()) {
        if let Ok(entries) = fs::read_dir(loc) {
            for entry in entries.flatten() {
                // haal naam van files op
                let os_filename = entry.file_name();
                let filename: &str = os_filename.to_str().unwrap();
                let file_len = filename.len();
                if file_len <= 9 {
                    continue;
                }
                // open en push taak
                if &filename[..4] == ("taak") && &filename[file_len - 5..] == ".json" {
                    let file = File::open(filename).unwrap();
                    let reader = BufReader::new(file);
                    let deserd_taak: Taak = serde_json::from_reader(reader).unwrap();
                    taken.push(deserd_taak);
                }
            }
        }
        taken
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn to_taak_file(self) -> Result<(), serde_json::Error> {
        let path = &self.taak_path();
        self.to_file(path)
    }
    pub fn remove(self){
        let path = &self.taak_path();
        fs::remove_file(path);
    }

    fn taak_path(&self) -> PathBuf {
        let filepath = &(format!("taak_{}.json", self.get_id()));
        Path::new(filepath).to_owned()
    }
}
