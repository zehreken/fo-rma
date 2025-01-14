use kopek::oscillator::WaveType;

pub trait OscillatorType {
    fn new(sample_rate: f32) -> Self;

    fn get_frequency(&self) -> f32;

    fn set_frequency(&mut self, frequency: f32);

    fn get_wave_type(&self) -> WaveType;

    fn set_wave_type(&mut self, wave_type: WaveType);

    fn run(&mut self) -> f32;
}
