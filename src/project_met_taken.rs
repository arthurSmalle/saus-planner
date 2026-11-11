use super::{project::*, taak::*};
use crate::SaveToFile;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectMetTaken {
    project: Project,
    taken: Vec<Taak>,
}

/*
impl SaveToFile for ProjectMetTaken {
    fn to_file(self, path: &std::path::Path) -> Result<(), serde_json::Error> {
        self.project.to_project_file()?;
        self.taken.into_iter().try_for_each(|i| i.to_file(path))
    }
}
*/

impl ProjectMetTaken {
    pub fn new(project: Project, taken: Vec<Taak>) -> Self {
        ProjectMetTaken { project, taken }
    }

    pub fn to_file(self) -> Result<(), serde_json::Error> {
        let project_path = self.project.get_project_path();
        self.project.to_project_file()?;
        self.taken.into_iter().try_for_each(|i| {
            let taak_path = i.get_taak_path();
            i.to_file(&project_path.join(taak_path))
        })
    }
}
