use super::{self::*, sausid::*, taak::*};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, create_dir_all};
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
/// struct die project omvat
pub struct Project {
    id: i64,
    pub start: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub naam: String,
    pub beschrijving: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub taken: Vec<Taak>,
}

impl SaveToFile for Project {}

impl Project {
    /// new project aanmaken
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
            taken: Vec::new(),
        }
    }
    /// schrijf het projct weg naar een file, er zal ook een dir worden gemaakt met de projectnaam
    pub fn to_project_file(self) -> Result<(), serde_json::Error> {
        let project_naam = format!("project_{}", self.get_id());
        create_dir_all(&project_naam).expect("fout bij maken van dir");
        let path = Path::new(&project_naam).join(format!("{}.json", project_naam));
        self.to_file(path.as_path())
    }
    /// haal de interne id van de taak
    pub fn get_id(&self) -> i64 {
        self.id
    }

    /// haal alle projecten in een locatie op
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

    /// verwijder een project (en alle bijhorende  bestanden
    pub fn remove(self) -> Result<(), std::io::Error> {
        fs::remove_dir_all(self.get_project_path())
    }

    pub fn get_project_path(&self) -> PathBuf {
        let filepath = &(format!("project_{}", self.get_id()));
        Path::new(filepath).to_owned()
    }
}
