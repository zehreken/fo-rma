use kopek::oscillator::WaveType;

pub trait OscillatorType {
    fn new(sample_rate: f32) -> Self;

    fn set_frequency(&mut self, frequencty: f32);

    fn set_wave_type(&mut self, wave_type: WaveType);
}
