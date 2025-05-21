use super::oscillator_type::OscillatorType;
use kopek::oscillator::{Oscillator, WaveType};

pub struct LFO {
    oscillator: Oscillator,
}

impl OscillatorType for LFO {
    fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
        }
    }

    fn frequency(&self) -> f32 {
        self.oscillator.frequency()
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    fn wave_type(&self) -> WaveType {
        self.oscillator.wave_type()
    }

    fn set_wave_type(&mut self, wave_type: WaveType) {
        self.oscillator.set_wave_type(wave_type);
    }

    fn run(&mut self) -> f32 {
        self.oscillator.run()
    }
}
