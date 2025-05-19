use crate::audio::{
    noise_generator::NoiseType,
    sequencer::{Sequencer, SequencerMode},
};
use kopek::{noise::Noise, oscillator::WaveType};

pub fn draw(ctx: &egui::Context, sequencer: &mut Sequencer, is_open: &mut bool) {
    egui::Window::new("oscillator")
        .open(is_open)
        .show(ctx, |ui| {
            ctx.request_repaint();
            // ui.label(format!("Running: {}", sequencer.is_running));
            // mode
            let mut selected_mode: SequencerMode = sequencer.mode;
            ui.horizontal(|ui| {
                ui.label("mode: ");
                egui::ComboBox::from_label("_")
                    .selected_text(format!("{:?}", selected_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected_mode, SequencerMode::Wave, "Wave");
                        ui.selectable_value(&mut selected_mode, SequencerMode::Noise, "Noise");
                    });
            });
            if selected_mode != sequencer.mode {
                sequencer.mode = selected_mode;
            }
            ui.separator();
            let mut volume = sequencer.volume();
            ui.horizontal(|ui| {
                ui.label("vol: ");
                ui.add(egui::Slider::new(&mut volume, 0.0..=1.0));
            });
            sequencer.set_volume(volume);
            // volume
            match sequencer.mode {
                SequencerMode::Wave => {
                    // vco freq
                    let mut vco_frequency = sequencer.frequency();
                    ui.horizontal(|ui| {
                        ui.label("vco: ");
                        ui.add(egui::widgets::Slider::new(&mut vco_frequency, 0.0..=800.0));
                    });
                    sequencer.set_frequency(vco_frequency);
                    // lfo freq
                    let mut lfo_frequency = sequencer.lfo_frequency();
                    ui.horizontal(|ui| {
                        ui.label("lfo: ");
                        ui.add(egui::widgets::Slider::new(&mut lfo_frequency, 0.0..=20.0));
                    });
                    sequencer.set_lfo_frequency(lfo_frequency);

                    let mut selected_wave: WaveType = sequencer.vco_wave_type();
                    egui::ComboBox::from_label("wave")
                        .selected_text(format!("{:?}", selected_wave))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected_wave, WaveType::Sine, "sine");
                            ui.selectable_value(&mut selected_wave, WaveType::Triangle, "triangle");
                            ui.selectable_value(
                                &mut selected_wave,
                                WaveType::Square { duty: 0.5 },
                                "square",
                            );
                            ui.selectable_value(&mut selected_wave, WaveType::Sawtooth, "sawtooth")
                        });
                    if selected_wave != sequencer.vco_wave_type() {
                        sequencer.set_vco_wave_type(selected_wave);
                    }
                }

                SequencerMode::Noise => {
                    let mut selected_noise = *sequencer.noise_generator.noise_type_mut();
                    egui::ComboBox::from_label("noise")
                        .selected_text(format!("{:?}", selected_noise))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected_noise, NoiseType::None, "none");
                            ui.selectable_value(&mut selected_noise, NoiseType::Random, "random");
                            ui.selectable_value(&mut selected_noise, NoiseType::White, "white");
                        });
                    if selected_noise != *sequencer.noise_generator.noise_type_mut() {
                        *sequencer.noise_generator.noise_type_mut() = selected_noise;
                    }
                }
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("attack: ");
                ui.add(egui::Slider::new(&mut sequencer.envelope.attack, 0.0..=0.5));
            });
            ui.horizontal(|ui| {
                ui.label("decay: ");
                ui.add(egui::Slider::new(&mut sequencer.envelope.decay, 0.0..=0.5));
            });
            ui.horizontal(|ui| {
                ui.label("sustain: ");
                ui.add(egui::Slider::new(
                    &mut sequencer.envelope.sustain,
                    0.0..=0.5,
                ));
            });
            ui.horizontal(|ui| {
                ui.label("release: ");
                ui.add(egui::Slider::new(
                    &mut sequencer.envelope.release,
                    0.0..=0.5,
                ));
            });
            if ui.button("play").clicked() {
                println!("try envelope");
            }
        });
}
