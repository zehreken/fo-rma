use crate::audio::sequencer::Sequencer;

pub fn draw(ctx: &egui::Context, is_open: &mut bool) {
    egui::Window::new("Sequencers")
        .open(is_open)
        .show(ctx, |ui| {});
}
