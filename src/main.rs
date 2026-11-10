use chrono::Days;
use std::path::Path;
use saus_planner::{sausid::*, taak::*};

fn main() {
    let mut id_gen = SausidGen::new();
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let test_taak = Taak::new(&mut id_gen,huidige_tijd, binnen_dag, String::from("saus taak"), 5, 5);
    let test_taak_lokatie: &str = &(format!("taak_{}.json", test_taak.get_id()));
    test_taak.to_file(Path::new(&test_taak_lokatie));
    println!("{:?}", Taak::get_taken("."));
}
