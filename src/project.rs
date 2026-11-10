struct ProjectLijst {
    projecten: Vec<Project>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Project {
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    taken: Vec<Taak>,
}
impl Project {
    fn add_task(&mut self, taak: Taak) -> Result<(), &'static str> {
        if self.deadline.timestamp() - taak.deadline.timestamp() >= 0 {
            self.taken.push(taak);
            Ok(())
        } else {
            Err("De deadline van deze taak komt na de deadline van het project")
        }
    }
}
