use chrono::{DateTime, Days, prelude::*};
use eframe::{App, egui};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct Taak {
    start: DateTime<Utc>,
    deadline: DateTime<Utc>,
    beschrijving: String,
    moeilijkheid: u8,
    status: TaakStatus,
}

#[derive(Debug, Serialize, Deserialize)]
enum TaakStatus {
    Onbegonnen,
    Begonnen,
    Halfweg,
    Eindfase,
    Af,
}

struct AppTest {
    tekst: String,
    waarde: u8,
}

impl Default for AppTest {
    fn default() -> Self {
        Self {
            tekst: String::from("tekst"),
            waarde: 5,
        }
    }
}

impl eframe::App for AppTest {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Saus planner test");
            ui.horizontal(|ui| {
                ui.label(&self.tekst);
            });
            ui.add(egui::Slider::new(&mut self.waarde, 0..=10).text("getal"));
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "saus planner",
        options,
        Box::new(|cc| Ok(Box::new(AppTest::default()))),
    )
}


impl eframe::App for ProjectLijst {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("test in scroll area");
        });
    }
}

