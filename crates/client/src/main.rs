use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Poopoo",
        native_options,
        Box::new(|cc| Ok(Box::new(PoopooApp::new(cc)))),
    )?;

    Ok(())
}

#[derive(Default)]
struct PoopooApp {
    counter: i32,
}

impl PoopooApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for PoopooApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Poopoo");

            ui.horizontal(|ui| {
                if ui.button("Counter +1").clicked() {
                    self.counter += 1;
                }
                if ui.button("Counter -1").clicked() {
                    self.counter -= 1;
                }
            });

            ui.label(format!("self.counter = {}", self.counter));
        });
    }
}
