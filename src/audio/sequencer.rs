use kopek::{
    oscillator::{self, Oscillator},
    utils,
};

pub struct Sequencer {
    pub is_running: bool,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    beat_index: u32,
    length: u8,
    freqs: Vec<f32>,
    sample_count: u32,
    tick_period: f32,
    is_beat: bool,
}

impl Sequencer {
    pub fn new(bpm: u16, sample_rate: u32, channel_count: u32) -> Self {
        let tick_period = (sample_rate * channel_count * 60) as f32 / bpm as f32;
        let oscillator = Oscillator::new(sample_rate as f32);
        Self {
            is_running: false,
            oscillator,
            beat_index: 0,
            length: 8,
            freqs: vec![
                utils::C_FREQ,
                utils::D_FREQ,
                utils::E_FREQ,
                utils::F_FREQ,
                utils::D_FREQ,
                utils::C_FREQ,
                utils::F_FREQ,
                utils::E_FREQ,
            ],
            sample_count: 0,
            tick_period,
            is_beat: false,
        }
    }

    pub fn update(&mut self, elapsed_samples: u32) {
        self.sample_count = elapsed_samples;

        let remainder = self.sample_count % self.tick_period as u32;
        self.is_beat = remainder > 0 && remainder < 8192;
        self.beat_index = self.sample_count / self.tick_period as u32;
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> (bool, f32) {
        let i = (self.beat_index % self.length as u32) as usize;
        (
            self.is_beat,
            self.oscillator.sine(self.freqs[i], self.sample_count),
        )
    }
}
