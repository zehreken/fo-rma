use crate::audio::sequencer::Sequencer;
use egui::Color32;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer) {
    egui::Window::new("sequencer").show(ctx, |ui| {
        ctx.request_repaint();
        ui.colored_label(Color32::RED, "MAIN TAPE ⏺");
        ui.label(format!("Running: {}", sequencer.is_running));
        ui.add(egui::Slider::new(&mut sequencer.volume, 0.0..=1.0));
    });
}
