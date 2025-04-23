use crate::audio::{modulated_oscillator::ModulatedOscillator, sequencer::Sequencer};
use egui::Color32;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer, is_open: &mut bool) {
    egui::Window::new("Sequencer")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();
            ui.colored_label(Color32::RED, "MAIN TAPE ⏺");

            ui.horizontal(|ui| {
                for i in 0..16 {
                    ui.menu_button("", |ui| {
                        if ui.button("C").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("D").clicked() {
                            ui.close_menu();
                        }
                    });
                }
            })
        });
}
