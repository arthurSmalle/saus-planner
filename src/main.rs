use chrono::Days;
use saus_planner::{project::*, sausid::*, taak::*};

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
    let test_project = Project::new(
        &mut id_gen,
        huidige_tijd,
        binnen_dag,
        String::from("sausproject"),
        String::from("een project vol saus"),
    );
    test_taak
        .to_taak_file()
        .expect("fout bij maken van taakfile");
    test_project
        .to_project_file()
        .expect("fout bij maken van projectfile");

    let taken = Taak::get_taken(".");
    let projecten = Project::get_projects(".");
    println!("Taken: \n{:?}\n", taken);
    println!("Projecten:\n{:?}\n", projecten);
    
    for proj in projecten{
        proj.to_project_file().unwrap();
    }
}
