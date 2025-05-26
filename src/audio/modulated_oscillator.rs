use super::{lfo::LFO, oscillator_type::OscillatorType, vco::VCO};
use kopek::oscillator::WaveType;

pub struct ModulatedOscillator {
    vco: VCO,
    lfo: LFO,
    frequency: f32,
}

impl ModulatedOscillator {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            vco: VCO::new(sample_rate as f32),
            lfo: LFO::new(sample_rate as f32),
            frequency: 440.0,
        }
    }

    pub fn run(&mut self) -> f32 {
        let modulation = self.lfo.run() * 10.0;
        let vco_frequency = self.frequency + modulation;
        self.vco.set_frequency(vco_frequency);
        let signal = self.vco.run();

        signal
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn vco_wave_type(&self) -> WaveType {
        self.vco.wave_type()
    }

    pub fn set_wave_type_mut(&mut self, wave_type: WaveType) {
        self.vco.set_wave_type(wave_type);
    }

    pub fn lfo_frequency(&self) -> f32 {
        self.lfo.frequency()
    }

    pub fn set_lfo_frequency(&mut self, frequency: f32) {
        self.lfo.set_frequency(frequency);
    }

    pub fn lfo_wave_type(&self) -> WaveType {
        self.lfo.wave_type()
    }

    pub fn set_lfo_wave_type(&mut self, wave_type: WaveType) {
        self.lfo.set_wave_type(wave_type);
    }
}
