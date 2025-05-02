use crate::audio::sequencer::Sequencer;
use egui::Color32;
use kopek::utils;

pub fn draw(
    ctx: &egui::Context,
    sequencers: &mut Vec<Sequencer>,
    selected: &mut usize,
    is_open: &mut bool,
) {
    egui::Window::new("Sequencers")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();
            ui.horizontal(|ui| {
                for i in 0..sequencers.len() {
                    if ui
                        .add(egui::RadioButton::new(
                            i == *selected,
                            format!("{}", (i + 1)),
                        ))
                        .clicked()
                    {
                        *selected = i;
                    }
                }
            });
            ui.colored_label(Color32::RED, format!("Sequencer {}", (*selected + 1)));
            let selected = *selected;
            let count = sequencers[selected].sequence.len();
            ui.horizontal(|ui| {
                for i in 0..count {
                    ui.vertical(|ui| {
                        ui.menu_button(sequencers[selected].sequence[i].octave.to_string(), |ui| {
                            for (label, value) in utils::OCTAVES {
                                if ui.button(label).clicked() {
                                    sequencers[selected].sequence[i].octave = value;
                                    ui.close_menu();
                                }
                            }
                        });
                        ui.menu_button(sequencers[selected].sequence[i].key.to_string(), |ui| {
                            for (label, value) in utils::KEYS {
                                if ui.button(label).clicked() {
                                    sequencers[selected].sequence[i].key = value;
                                    ui.close_menu();
                                }
                            }
                        });
                    });
                }
            });
        });
}
