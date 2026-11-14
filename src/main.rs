use saus_planner::{project::*, project_met_taken::ProjectMetTaken, sausid::*, taak::*};
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
    let test_project = Project::new(
        &mut id_gen,
        huidige_tijd,
        binnen_dag,
        String::from("sausproject"),
        String::from("een project vol saus"),
    );
    let taken = vec![test_taak, test_taak2];
    let test_project_taken = ProjectMetTaken::new(test_project, taken);
    test_project_taken.to_file().unwrap();
    let projecten = Project::get_projects(".");
    let taken = Taak::get_taken(projecten.first().unwrap());
    taken.iter().for_each(|i| println!("\n{}\n",i.pretty_string()))
    // projecten.into_iter().for_each(|i| i.remove().unwrap());
}
