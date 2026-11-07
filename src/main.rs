use SausPlanner::taak::*;
use chrono::{DateTime, Days, TimeDelta, prelude::*};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

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

fn main() {
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let test_taak = Taak::new( huidige_tijd, binnen_dag, String::from("saus taak"), 5, 5);
    let test_taak_lokatie :&str = "taak_test.json";
    test_taak.to_file(Path::new(&test_taak_lokatie));

    let file = File::open(&test_taak_lokatie).unwrap();
    let reader = BufReader::new(file);
    let deserialized_json: Taak = serde_json::from_reader(reader).unwrap();
    println!("{:?}",deserialized_json)
}
