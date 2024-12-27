use kopek::oscillator::{Oscillator, WaveType};

use super::oscillator_type::OscillatorType;

pub struct LFO {
    oscillator: Oscillator,
}

impl OscillatorType for LFO {
    fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
        }
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    fn set_wave_type(&mut self, wave_type: WaveType) {
        self.oscillator.set_wave_type(wave_type);
    }

    fn run(&mut self, tick: u32) -> f32 {
        self.oscillator.run(tick)
    }
}
