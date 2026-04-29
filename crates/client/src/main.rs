use eframe::egui;
use poopoo_types::Post;

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
    posts: Vec<Post>,
    buffer: Buffer,
}

#[derive(Default)]
struct Buffer {
    inputs: String,
}

impl PoopooApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            posts: vec![
                Post::new("haruki7049".into(), "Hoge".into()),
                Post::new("haruki7049".into(), "Fuga".into()),
            ],
            ..Default::default()
        }
    }
}

impl eframe::App for PoopooApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Panel::left("Hoge").show_inside(ui, |ui| {
                ui.text_edit_multiline(&mut self.buffer.inputs);
                ui.separator();

                ui.label(format!("self.buffer.inputs: {}", self.buffer.inputs));
            });

            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(20, 10))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for post in &self.posts {
                            ui.label(format!("{}: {}", post.username, post.content));
                        }
                    });
                });
        });
    }
}
