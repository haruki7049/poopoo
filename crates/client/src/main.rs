use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([640.0, 360.0]),
        ..Default::default()
    };
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
    input_buffer: String,
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
            egui::Panel::left("Hoge").show_inside(ui, |ui| {
                ui.text_edit_multiline(&mut self.input_buffer);
                ui.separator();

                ui.label(format!("self.input_buffer: {}", self.input_buffer));
            });

            ui.horizontal(|ui| {
                if ui.button("Counter +1").clicked() {
                    self.counter += 1;
                }

                ui.separator();

                if ui.button("Counter -1").clicked() {
                    self.counter -= 1;
                }
            });

            ui.label(format!("self.counter = {}", self.counter));
        });
    }
}
