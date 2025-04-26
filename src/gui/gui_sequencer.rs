use crate::audio::sequencer::Sequencer;
use egui::Color32;
use kopek::utils;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer, is_open: &mut bool) {
    egui::Window::new("Sequencer")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();
            ui.colored_label(Color32::RED, "MAIN TAPE ⏺");
            let count = sequencer.sequence.len();
            ui.horizontal(|ui| {
                for i in 0..count {
                    ui.menu_button(sequencer.sequence[i].octave.to_string(), |ui| {
                        for (label, value) in utils::OCTAVES {
                            if ui.button(label).clicked() {
                                println!("{} selected", label);
                                sequencer.sequence[i].octave = value;
                                ui.close_menu();
                            }
                        }
                    });
                }
            });
            ui.horizontal(|ui| {
                for i in 0..count {
                    ui.menu_button(sequencer.sequence[i].key.to_string(), |ui| {
                        for (label, value) in utils::KEYS {
                            if ui.button(label).clicked() {
                                println!("{} selected", label);
                                sequencer.sequence[i].key = value;
                                ui.close_menu();
                            }
                        }
                    });
                }
            });
        });
}
