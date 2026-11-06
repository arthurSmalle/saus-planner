use chrono::{DateTime, Days, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub trait SaveToFile: Serialize {
    fn to_file(&self, path: &Path) {
        let json = serde_json::to_string(&self).unwrap();
        fs::write(path, json).unwrap();
    }
}

struct ProjectLijst {
    projecten: Vec<Project>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Project {
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    taken: Vec<Taak>,
}
impl Project {
    fn add_task(&mut self, taak: Taak) -> Result<(), &'static str> {
        if self.deadline.timestamp() - taak.deadline.timestamp() >= 0 {
            self.taken.push(taak);
            Ok(())
        } else {
            Err("De deadline van deze taak komt na de deadline van het project")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Taak {
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    moeilijkheid: u8,
    status: TaakStatus,
}
impl SaveToFile for Taak{
}

#[derive(Debug, Serialize, Deserialize)]
enum TaakStatus {
    Onbegonnen,
    Begonnen,
    Halfweg,
    Eindfase,
    Af,
}
fn main(){
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let test_taak = Taak {
        start: huidige_tijd,
        deadline: binnen_dag,
        beschrijving: String::from("saus taakje"),
        moeilijkheid: 5,
        status: TaakStatus::Begonnen,
    };
    test_taak.to_file(Path::new("test.json"));
}
