use super::{lfo::LFO, oscillator_type::OscillatorType, vco::VCO};

pub struct ModulatedOscillator {
    vco: VCO,
    lfo: LFO,
}

impl ModulatedOscillator {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            vco: VCO::new(sample_rate as f32),
            lfo: LFO::new(sample_rate as f32),
        }
    }
}
