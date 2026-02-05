use eframe::egui;
use chrono::Local;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0]), 
        ..Default::default()
    };

    eframe::run_native(
        "GBS - Growth Behind the Scenes",
        options,
        Box::new(|_cc| Ok(Box::new(GBSApp::new()))),
    )
}

enum View {
    Write,
    Archive,
}

struct GBSApp {
    view: View,
    title: String,
    scene: String,
    status: String,
    history: String,
}

impl GBSApp {
    fn new() -> Self {
        Self {
            view: View::Write,
            title: String::new(),
            scene: String::new(),
            status: String::new(),
            history: load_scenes().unwrap_or_else(|_| "No scenes yet.".to_string()),
        }
    }
}

impl eframe::App for GBSApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // ================= SIDEBAR =================

        let mouse_x = ctx.input(|i| i.pointer.hover_pos().map(|p| p.x).unwrap_or(f32::MAX));

       let expanded = mouse_x < 110.0; // smoother hover
        let sidebar_width = if expanded { 170.0 } else { 50.0 };

        egui::SidePanel::left("sidebar")
            .exact_width(sidebar_width)
            .show(ctx, |ui| {

                ui.vertical_centered(|ui| {
                    ui.heading("🌱");
                });

                ui.separator();

                if ui.button(if expanded { "📝 Write Scene" } else { "📝" }).clicked() {
                    self.view = View::Write;
                }

                if ui.button(if expanded { "📚 Past Scenes" } else { "📚" }).clicked() {
                    self.view = View::Archive;
                }
            });

        // ================= MAIN AREA =================

        egui::CentralPanel::default().show(ctx, |ui| {

            // ⭐ HEADER IDENTITY
            ui.horizontal(|ui| {
                ui.heading("🌱 GBS — Growth Behind the Scenes");

                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.label("Reflect. Learn. Grow.");
                    },
                );
            });

            ui.separator();
            ui.add_space(10.0);

            match self.view {

                // =================================================
                // WRITE VIEW
                // =================================================

                View::Write => {

                    ui.label("Behind the scenes is where growth begins,");
                    ui.label("After the noise, the loss, the wins.");
                    ui.label("You sit, you feel, you start to see,");
                    ui.label("The lesson forming from within.");

                    ui.add_space(15.0);

                    ui.label("Your keyboard’s ready, take your time,");
                    ui.label("Put into words what’s on your mind.");
                    ui.label("Remember — this is all about you.");

                    ui.separator();
                    ui.add_space(15.0);

                   ui.heading("Write a Scene");

egui::ScrollArea::vertical()
    .auto_shrink([false; 2])
    .show(ui, |ui| {

        // FULL WIDTH title
        ui.add(
            egui::TextEdit::singleline(&mut self.title)
                .hint_text("Title — what would you like to name this scene?")
                .desired_width(f32::INFINITY),
        );

        ui.add_space(10.0);

        // BIG SCROLLABLE WRITING BOX
        ui.add(
            egui::TextEdit::multiline(&mut self.scene)
                .hint_text("Tell the scene...")
                .desired_width(f32::INFINITY)
                .desired_rows(20),
        );

        ui.add_space(10.0);

        let can_save =
            !self.title.trim().is_empty() &&
            !self.scene.trim().is_empty();

        if ui
            .add_enabled(can_save, egui::Button::new("Save Scene 💾"))
            .clicked()
        {
            match save_scene(&self.title, &self.scene) {
                Ok(_) => {
                    self.status = "Scene saved successfully ✓".to_string();
                    self.title.clear();
                    self.scene.clear();
                    self.history = load_scenes().unwrap_or_default();
                }
                Err(_) => {
                    self.status = "Failed to save scene 😢".to_string();
                }
            }
        }

        ui.add_space(10.0);
       ui.label(&self.status);

});
} 


                // =================================================
                // ARCHIVE VIEW
                // =================================================

                View::Archive => {

                    ui.heading("📚 Past Scenes");
                    ui.label("Read gently. This is you, becoming.");
                    ui.separator();
                    ui.add_space(10.0);

                    // ⭐ SCROLLABLE ARCHIVE
                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {

                            ui.add(
                                egui::TextEdit::multiline(&mut self.history)
                                    .desired_width(f32::INFINITY)
                                    .interactive(false),
                            );

                        });
                }
            }
        });
    }
}


// =================================================
// SAVE
// =================================================

fn save_scene(title: &str, scene: &str) -> std::io::Result<()> {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("gbs_scenes.txt")?;

    let timestamp = Local::now().format("%d/%m/%Y %H:%M");

    writeln!(file, "════════════════════════════")?;
    writeln!(file, "🕰️ {}", timestamp)?;
    writeln!(file, "📌 {}", title.to_uppercase())?;
    writeln!(file)?;
    writeln!(file, "{}", scene)?;
    writeln!(file)?;

    Ok(())
}


// =================================================
// LOAD
// =================================================

fn load_scenes() -> std::io::Result<String> {
    read_to_string("gbs_scenes.txt")
}
