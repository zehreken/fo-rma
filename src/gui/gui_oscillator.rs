use crate::audio::sequencer::Sequencer;
use kopek::oscillator::WaveType;

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer, is_open: &mut bool) {
    egui::Window::new("oscillator")
        .open(is_open)
        .show(ctx, |ui| {
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
                ui.add(egui::widgets::Slider::new(&mut vco_frequency, 10.0..=400.0));
            });
            sequencer.set_frequency(vco_frequency);
            // lfo freq
            let mut lfo_frequency = sequencer.get_lfo_frequency();
            ui.horizontal(|ui| {
                ui.label("lfo: ");
                ui.add(egui::widgets::Slider::new(&mut lfo_frequency, 0.0..=20.0));
            });
            sequencer.set_lfo_frequency(lfo_frequency);

            let mut selected: WaveType = sequencer.get_vco_wave_type();
            ui.horizontal(|ui| {
                ui.label("wave type: ");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected, WaveType::Sine, "Sine");
                        ui.selectable_value(&mut selected, WaveType::Triangle, "Triangle");
                        ui.selectable_value(
                            &mut selected,
                            WaveType::Square { duty: 0.5 },
                            "Square",
                        );
                        ui.selectable_value(&mut selected, WaveType::Sawtooth, "Sawtooth")
                    });
            });
            if selected != sequencer.get_vco_wave_type() {
                sequencer.set_vco_wave_type(selected);
            }
        });
}
