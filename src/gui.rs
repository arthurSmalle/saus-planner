use eframe::{App, egui};
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
impl eframe::App for ProjectLijst {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("test in scroll area");
        });
    }
}
