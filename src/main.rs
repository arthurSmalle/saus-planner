use chrono::{DateTime, Days, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Project{
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    taken: Vec<Taak>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Taak{
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    moeilijkheid: u8,
    status: TaakStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaakStatus{Onbegonnen, Begonnen, Halfweg, Eindfase ,Af}

fn main() {
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let taak_ib3 = Taak {
        start: huidige_tijd,
        deadline: binnen_dag,
        moeilijkheid: 5,
        status: TaakStatus::Begonnen,
    };
    print!("{:?}", taak_ib3);

    let mut project = Project {
        start: huidige_tijd,
        deadline: binnen_dag,
        beschrijving: String::from("saus projectje"),
        taken: vec![],
    };
    print!("{:?}", project);
    project.add_task(taak_ib3).unwrap();
    print!("{:?}", project);
}

impl Project {
    fn add_task (&mut self, taak: Taak) -> Result<(),&'static str>{
        if self.deadline.timestamp() - taak.deadline.timestamp() >= 0 {
            Ok(())
        } else {
            Err("De deadline van deze taak komt na de deadline van het project")
        }
    }
}
