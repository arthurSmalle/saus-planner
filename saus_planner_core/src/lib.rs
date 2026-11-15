use serde::Serialize;
use std::fs;
use std::path::Path;

pub mod project;
pub mod sausid;
pub mod taak;

pub trait SaveToFile: Serialize {
    fn to_file(&self, path: &Path) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&self)?;
        fs::write(path, json).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{project::*, sausid::*};
    use ::chrono::Days;
    #[test]
    fn test_id_collision() {
        let mut id_gen = SausidGen::new();
        let first_id = id_gen.get_saudid();
        let second_id = id_gen.get_saudid();
        assert_ne!(first_id, second_id);
    }
    #[test]
    fn test_create_and_remove_project() {
        // verwijder alle mogelijke projecten
        let projecten = Project::get_projects(".");
        projecten.into_iter().for_each(|i| i.remove().unwrap());

        // basis die nodig is
        let mut id_gen = SausidGen::new();
        let huidige_tijd = chrono::Utc::now();
        let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
        // project maken
        let test_project = Project::new(
            &mut id_gen,
            huidige_tijd,
            binnen_dag,
            String::from("sausproject"),
            String::from("een project vol saus"),
        );
        // naar file schrijven
        test_project
            .to_project_file()
            .expect("fout bij maken van projectfile");
        let projecten = Project::get_projects(".");
        // test als na deserialize nog steeds zelfde project
        if let Some(project) = projecten.first() {
            assert_eq!(project.start, huidige_tijd);
            assert_eq!(project.deadline, binnen_dag);
            assert_eq!(project.naam, "sausproject");
            assert_eq!(project.beschrijving, "een project vol saus");
        }
        // verwijder test projecten
        projecten.into_iter().for_each(|i| i.remove().unwrap());
    }
}
