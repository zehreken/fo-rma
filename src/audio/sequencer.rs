#[derive(Debug, Clone, Copy)]
pub struct Sequencer {
    pub is_running: bool,
    beat_index: u32,
    sample_count: u32,
    tick_period: f32,
    is_beat: bool,
}

impl Sequencer {
    pub fn new(bpm: u16, sample_rate: u32, channel_count: u32) -> Self {
        let tick_period = (sample_rate * channel_count * 60) as f32 / bpm as f32;
        Self {
            is_running: false,
            beat_index: 0,
            sample_count: 0,
            tick_period,
            is_beat: false,
        }
    }

    // update should be called from the audio thread and while processing the samples
    pub fn update(&mut self) {
        self.sample_count += 1;

        let remainder = self.sample_count % self.tick_period as u32;
        self.is_beat = remainder > 0 && remainder < 8192;
        self.beat_index = self.sample_count / self.tick_period as u32;
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> bool {
        self.is_beat
    }

    // This is to sync the metronome with the app, if the metronome is created
    // after the app has started
    pub fn sync(&mut self, elapsed_samples: u32) {
        self.sample_count = elapsed_samples;
    }
}
