use chrono::{DateTime, Days, prelude::*};
use eframe::{App, egui};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub trait SaveToFile: Serialize {
    fn to_file(&self, path: &Path) {
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

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
impl SaveToFile for Taak{
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
    let huidige_tijd = chrono::Utc::now();
    let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    let test_taak = Taak {
        start: huidige_tijd,
        deadline: binnen_dag,
        beschrijving: String::from("saus taakje"),
        moeilijkheid: 5,
        status: TaakStatus::Begonnen,
    };
    test_taak.to_file(Path::new("test.json"));

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
