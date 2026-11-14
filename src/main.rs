use saus_planner::{project::*,sausid::*, taak::*};
use chrono::Days;

fn main() {
    let mut id_gen = SausidGen::new();
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let test_taak = Taak::new(
        &mut id_gen,
        huidige_tijd,
        binnen_dag,
        String::from("saus taak"),
        5,
        5,
    );
    let test_taak2 = Taak::new(
        &mut id_gen,
        huidige_tijd,
        binnen_dag,
        String::from("saus taak"),
        5,
        5,
    );
    let mut test_project = Project::new(
        &mut id_gen,
        huidige_tijd,
        binnen_dag,
        String::from("sausproject"),
        String::from("een project vol saus"),
    );
    
    let taken = vec![test_taak, test_taak2];
    test_project.add_taken(taken);
    test_project.to_project_file();
    let projecten = Project::get_projects(".");
    for mut project in projecten {
        project.fetch_taken();
        let taken =project.get_taken();
        println!("==============================\nproject: {}", project.naam);
        taken.iter().for_each(|i| println!("\n{}\n",i.pretty_string()))
    }
}
