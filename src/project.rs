use super::SaveToFile;
use super::{sausid::*, taak::*};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::fs::{self, DirBuilder, File, create_dir_all};
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i64,
    pub start: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub naam: String,
    pub beschrijving: String,
}

impl SaveToFile for Project {}

impl Project {
    pub fn new(
        id_gen: &mut SausidGen,
        start: DateTime<Utc>,
        deadline: DateTime<Utc>,
        naam: String,
        beschrijving: String,
    ) -> Self {
        Project {
            id: id_gen.get_saudid(),
            start,
            deadline,
            naam,
            beschrijving,
        }
    }
    pub fn to_project_file(self) -> Result<(), serde_json::Error> {
        let project_naam = format!("project_{}", self.get_id());
        create_dir_all(&project_naam).expect("fout bij maken van dir");
        let path = Path::new(&project_naam).join(format!("{}.json", project_naam));
        self.to_file(path.as_path())
    }
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_projects(loc: &str) -> Vec<Project> {
        let mut projecten: Vec<Project> = vec![];
        if let Ok(entries) = fs::read_dir(loc) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata()
                    && metadata.is_dir()
                {
                    // haal naam van files op
                    let os_filename = entry.file_name();
                    let filename: &str = os_filename.to_str().unwrap();
                    // open en push taak
                    if filename.len() > 7 && &filename[..7] == ("project") {
                        let file = File::open(format!("{}/{}.json", filename, filename)).unwrap();
                        let reader = BufReader::new(file);
                        let deserd_project: Project = serde_json::from_reader(reader).unwrap();
                        projecten.push(deserd_project);
                    }
                }
            }
        }
        projecten
    }

    pub fn remove(self) {
        let project_naam = format!("project_{}", self.get_id());
        fs::remove_dir_all(project_naam);
    }
}
