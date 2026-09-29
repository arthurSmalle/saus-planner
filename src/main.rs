use chrono::{DateTime, Days, prelude::*};
use serde::{Deserialize, Serialize};
use eframe::{App, egui};

struct ProjectLijst{
    projecten: Vec<Project>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Project{
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    taken: Vec<Taak>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Taak{
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    moeilijkheid: u8,
    status: TaakStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaakStatus{Onbegonnen, Begonnen, Halfweg, Eindfase ,Af}

struct AppTest{
    tekst: String,
    waarde: u8,
}

fn main() -> eframe::Result{
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let taak_ib3 = Taak {
        start: huidige_tijd,
        deadline: binnen_dag,
        beschrijving: String::from("saus taakje"),
        moeilijkheid: 5,
        status: TaakStatus::Begonnen,
    };
    print!("{:?}", taak_ib3);

    let mut project = Project {
        start: huidige_tijd,
        deadline: binnen_dag,
        beschrijving: String::from("saus projectje"),
        taken: vec![],
    };
    print!("{:?}", project);
    project.add_task(taak_ib3).unwrap();
    print!("{:?}", project);
    let serialized = serde_json::to_string(&project).unwrap();
    print!("\n==================\n{serialized}");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("saus planner", options,Box::new(|cc| Ok(Box::new(AppTest::default()))) )
}

impl Project {
    fn add_task (&mut self, taak: Taak) -> Result<(),&'static str>{
        if self.deadline.timestamp() - taak.deadline.timestamp() >= 0 {
            self.taken.push(taak);
            Ok(())
        } else {
            Err("De deadline van deze taak komt na de deadline van het project")
        }
    }
}

impl eframe::App for AppTest {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui|{
            ui.heading("Saus planner test");
            ui.horizontal(|ui|{
                ui.label(&self.tekst);
            });
            ui.add(egui::Slider::new(&mut self.waarde, 0..=10).text("getal"));
        });
    }
}

impl eframe::App for ProjectLijst {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui|{
            ui.label("test in scroll area");
        });
    }
}

impl Default for AppTest{
    fn default() -> Self {
        Self {
            tekst: String::from("tekst"),
            waarde: 5,
        }
    }
}
