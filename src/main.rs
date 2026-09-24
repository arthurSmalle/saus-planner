use chrono::{Days, prelude::*};
use serde::Serialize;

#[derive(Debug)]
struct Project{
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    taken: Vec<Taak>,
}

#[derive(Debug)]
struct Taak {
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    moeilijkheid: u8,
    status: TaakStatus,
}

#[derive(Debug, Serialize)]
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
}
