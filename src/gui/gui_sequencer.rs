use crate::audio::sequencer::Sequencer;
use egui::Color32;

pub fn draw(ctx: &egui::Context) {
    egui::Window::new("sequencer").show(ctx, |ui| {
        ctx.request_repaint();
        ui.colored_label(Color32::RED, "MAIN TAPE ⏺");
    });
}
