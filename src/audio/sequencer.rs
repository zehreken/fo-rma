use super::{modulated_oscillator::ModulatedOscillator, noise_generator::NoiseGenerator};
use crate::audio::envelope::Envelope;
use kopek::{
    oscillator::WaveType,
    utils::{Key, Note},
};

pub struct Sequencer {
    pub is_running: bool,
    pub modulated_oscillator: ModulatedOscillator,
    pub noise_generator: NoiseGenerator,
    beat_index: u32,
    prev_beat_index: u32,
    length: u8,
    freq: f32,
    pub sequence: Vec<Note>,
    tick_period: f32,
    beat_duration: f32,
    on_beat: bool,
    wave_volume: f32,
    pub noise_volume: f32,
    pub envelope: Envelope,
}

impl Sequencer {
    pub fn new(
        bpm: u16,
        sample_rate: u32,
        channel_count: u32,
        sequence: Vec<Note>, // song
    ) -> Self {
        let tick_period = (sample_rate * 60) as f32 / bpm as f32;
        let beat_duration = tick_period / 3.0;
        println!("Sequencer: {bpm}, {sample_rate}, {channel_count}, {tick_period}");

        let noise_generator = NoiseGenerator::new();

        const FACTOR: f32 = 0.2;
        Self {
            is_running: false,
            modulated_oscillator: ModulatedOscillator::new(sample_rate),
            noise_generator,
            beat_index: 0,
            prev_beat_index: 0,
            length: sequence.len() as u8,
            freq: sequence[0].get(),
            sequence,
            tick_period,
            beat_duration,
            on_beat: false,
            wave_volume: 0.9,
            noise_volume: 0.1,
            envelope: Envelope::new(0.1 * FACTOR, 0.1 * FACTOR, 0.2 * FACTOR, 0.1 * FACTOR),
        }
    }

    pub fn update(&mut self, elapsed_samples: u32) -> f32 {
        let remainder = elapsed_samples % self.tick_period as u32;
        self.on_beat = remainder > 0 && remainder < self.beat_duration as u32;
        self.beat_index = elapsed_samples / self.tick_period as u32;
        let step_index = (self.beat_index % self.length as u32) as usize;

        self.freq = self.sequence[step_index].get();
        self.modulated_oscillator.set_frequency(self.freq);
        let mut wave_value = self.modulated_oscillator.run();
        wave_value = wave_value * self.wave_volume;

        let mut noise_value = if self.sequence[step_index].key != Key::Rest {
            self.noise_generator.run()
        } else {
            0.0
        };
        noise_value = noise_value * self.noise_volume;
        // self.freq = self.sequence[step_index].get();
        // self.modulated_oscillator.frequency_mut(self.freq);
        // let mut value = self.modulated_oscillator.run();
        // value = self.noise_generator.run();

        if self.prev_beat_index != self.beat_index {
            self.prev_beat_index = self.beat_index;
            self.envelope.reset(); // envelope should never reset actually
        }
        const DELTA_TIME: f32 = 1 as f32 / 44100 as f32;
        let envelope = self.envelope.update(DELTA_TIME);
        // if envelope > 0.0 {
        //     println!("{:?}", envelope);
        // }
        let mut value = wave_value + noise_value;
        value *= envelope;

        value
    }

    // Current number of beats played, similar to elapsed time
    pub fn beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn on_beat(&self) -> bool {
        self.on_beat
    }

    pub fn frequency(&self) -> f32 {
        self.modulated_oscillator.frequency()
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.modulated_oscillator.set_frequency(frequency);
    }

    pub fn vco_wave_type(&mut self) -> WaveType {
        self.modulated_oscillator.vco_wave_type()
    }

    pub fn set_vco_wave_type(&mut self, wave_type: WaveType) {
        self.modulated_oscillator.set_wave_type_mut(wave_type);
    }

    pub fn lfo_frequency(&self) -> f32 {
        self.modulated_oscillator.lfo_frequency()
    }

    pub fn set_lfo_frequency(&mut self, frequency: f32) {
        self.modulated_oscillator.set_lfo_frequency(frequency);
    }

    pub fn lfo_wave_type(&self) -> WaveType {
        self.modulated_oscillator.lfo_wave_type()
    }

    pub fn set_lfo_wave_type(&mut self, wave_type: WaveType) {
        self.modulated_oscillator.set_lfo_wave_type(wave_type);
    }

    pub fn volume(&self) -> f32 {
        self.wave_volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.wave_volume = volume.clamp(0.0, 1.0);
    }
}
