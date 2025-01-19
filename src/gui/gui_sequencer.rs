use crate::audio::sequencer::Sequencer;
use egui::Color32;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer) {
    egui::Window::new("oscillator").show(ctx, |ui| {
        ctx.request_repaint();
        // ui.label(format!("Running: {}", sequencer.is_running));
        // volume
        let mut volume = sequencer.get_volume();
        ui.horizontal(|ui| {
            ui.label("vol: ");
            ui.add(egui::Slider::new(&mut volume, 0.0..=1.0));
        });
        sequencer.set_volume(volume);
        // vco freq
        let mut vco_frequency = sequencer.get_frequency();
        ui.horizontal(|ui| {
            ui.label("vco: ");
            ui.add(egui::widgets::Slider::new(
                &mut vco_frequency,
                200.0..=8_000.0,
            ));
        });
        sequencer.set_frequency(vco_frequency);
        // lfo freq
        let mut lfo_frequency = sequencer.get_lfo_frequency();
        ui.horizontal(|ui| {
            ui.label("lfo: ");
            ui.add(egui::widgets::Slider::new(&mut lfo_frequency, 1.0..=20.0));
        });
        sequencer.set_lfo_frequency(lfo_frequency);
    });
}
