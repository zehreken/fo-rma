use super::oscillator_type::OscillatorType;
use kopek::oscillator::{Oscillator, WaveType};

pub struct VCO {
    oscillator: Oscillator,
}

impl OscillatorType for VCO {
    fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
        }
    }

    fn get_frequency(&self) -> f32 {
        self.oscillator.get_frequency()
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    fn get_wave_type(&self) -> WaveType {
        self.oscillator.get_wave_type()
    }

    fn set_wave_type(&mut self, wave_type: WaveType) {
        self.oscillator.set_wave_type(wave_type);
    }

    fn run(&mut self) -> f32 {
        self.oscillator.run()
    }
}
