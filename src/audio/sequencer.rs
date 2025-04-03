use super::modulated_oscillator::ModulatedOscillator;
use crate::{audio::envelope::Envelope, basics::core::clamp};
use kopek::oscillator::WaveType;

pub struct Sequencer {
    pub is_running: bool,
    oscillator: ModulatedOscillator,
    // vco: VCO,
    // lfo: LFO,
    beat_index: u32,
    prev_beat_index: u32,
    length: u8,
    freq: f32,
    sequence: Vec<f32>,
    tick_period: f32,
    beat_duration: f32,
    is_beat: bool,
    volume: f32,
    ramp: f32,
    envelope: Envelope,
}

impl Sequencer {
    pub fn new(
        bpm: u16,
        sample_rate: u32,
        channel_count: u32,
        sequence: Vec<f32>, // song
    ) -> Self {
        let tick_period = (sample_rate * 60) as f32 / bpm as f32;
        let beat_duration = tick_period / 3.0;
        println!("Sequencer: {bpm}, {sample_rate}, {channel_count}, {tick_period}");
        // let mut vco = VCO::new(sample_rate as f32);
        // vco.set_wave_type(kopek::oscillator::WaveType::Sine);
        // let mut lfo = LFO::new(sample_rate as f32);
        // lfo.set_frequency(10.0);

        Self {
            is_running: false,
            oscillator: ModulatedOscillator::new(sample_rate),
            // vco,
            // lfo,
            beat_index: 0,
            prev_beat_index: 0,
            length: sequence.len() as u8,
            freq: sequence[0],
            sequence,
            tick_period,
            beat_duration,
            is_beat: false,
            volume: 0.1,
            ramp: 0.0,
            envelope: Envelope::new(0.1, 0.1, 0.2, 0.1),
        }
    }

    pub fn update(&mut self, elapsed_samples: u32) -> f32 {
        let remainder = elapsed_samples % self.tick_period as u32;
        self.is_beat = remainder > 0 && remainder < self.beat_duration as u32;
        self.beat_index = elapsed_samples / self.tick_period as u32;
        let step_index = (self.beat_index % self.length as u32) as usize;
        const TEMP_OCTAVE: u8 = 2u8.pow(4);
        let freq_diff: f32 = if step_index == 0 {
            0.0
        } else {
            // Reach next freq in 50 samples
            self.sequence[step_index] - self.sequence[step_index - 1] / 50.0
        };

        // Ramp between steps
        // if self.freq != self.freqs[step_index] {
        //     self.freq += freq_diff;
        // }

        self.freq = self.sequence[step_index];
        // self.oscillator
        //     .set_frequency(self.freq * TEMP_OCTAVE as f32);
        let mut value = self.oscillator.run();

        // Ramp between volumes
        // if self.is_beat && self.ramp < 1.0 {
        //     self.ramp = clamp(self.ramp + 0.001, 0.0, 1.0);
        // } else if !self.is_beat && self.ramp > 0.0 {
        //     self.ramp = clamp(self.ramp - 0.001, 0.0, 1.0);
        // }
        if self.prev_beat_index != self.beat_index {
            self.prev_beat_index = self.beat_index;
            self.envelope.reset();
        }
        const DELTA_TIME: f32 = 1 as f32 / 44100 as f32;
        let envelope = self.envelope.update(DELTA_TIME);
        // if envelope > 0.0 {
        //     println!("{:?}", envelope);
        // }
        value *= envelope;

        // value *= self.ramp;
        value *= self.volume;

        value
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> bool {
        self.is_beat
    }

    pub fn get_frequency(&self) -> f32 {
        self.oscillator.get_frequency()
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    pub fn set_vco_wave_type(&mut self, wave_type: WaveType) {
        self.oscillator.set_vco_wave_type(wave_type);
    }

    pub fn get_vco_wave_type(&mut self) -> WaveType {
        self.oscillator.get_vco_wave_type()
    }

    pub fn get_lfo_frequency(&self) -> f32 {
        self.oscillator.get_lfo_frequency()
    }

    pub fn set_lfo_frequency(&mut self, frequency: f32) {
        self.oscillator.set_lfo_frequency(frequency);
    }

    pub fn set_lfo_wave_type(&mut self, wave_type: WaveType) {
        self.oscillator.set_lfo_wave_type(wave_type);
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}
