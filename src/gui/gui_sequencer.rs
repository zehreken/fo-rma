use crate::audio::{sequencer::Sequencer, utils};
use egui::Color32;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer, is_open: &mut bool) {
    egui::Window::new("Sequencer")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();
            ui.colored_label(Color32::RED, "MAIN TAPE ⏺");
            let count = sequencer.sequence.len();
            ui.horizontal(|ui| {
                for _ in 0..count {
                    ui.menu_button("", |ui| {
                        for text in utils::OCTAVES {
                            if ui.button(text).clicked() {
                                println!("{} selected", text);
                                ui.close_menu();
                            }
                        }
                    });
                }
            });
            ui.horizontal(|ui| {
                for _ in 0..count {
                    ui.menu_button("", |ui| {
                        for text in utils::PITCHES {
                            if ui.button(text).clicked() {
                                println!("{} selected", text);
                                ui.close_menu();
                            }
                        }
                    });
                }
            });
        });
}
