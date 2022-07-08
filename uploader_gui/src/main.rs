use eframe::{
    egui,
    epi::{self, App},
};

fn main() {
    let app = MyApp::default();
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}

#[derive(Default)]
struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag and drop your files here!");

            if ui.button("Click for some fun").clicked() {
                println!("Clicked!");
            }

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            info += &format!(" ({} bytes)", bytes.len());
                        }
                        ui.label(info);
                    }
                });
            }
        });

        preview_files_being_dropped(ctx);
        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }

    fn name(&self) -> &str {
        "Drag-and-drop files"
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();

        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::GRAY);
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
