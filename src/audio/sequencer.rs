use kopek::{
    oscillator::{self, Oscillator},
    utils,
};
use ringbuf::HeapProducer;

use crate::basics::core::clamp;

pub struct Sequencer {
    pub is_running: bool,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    beat_index: u32,
    length: u8,
    freq: f32,
    freqs: Vec<f32>,
    tick_period: f32,
    beat_period: f32,
    is_beat: bool,
    ramp: f32,
    tick: u32,
    signal: f32,
}

impl Sequencer {
    pub fn new(
        bpm: u16,
        sample_rate: u32,
        channel_count: u32,
        producer: HeapProducer<f32>,
    ) -> Self {
        let tick_period = (sample_rate * channel_count * 60) as f32 / bpm as f32;
        let beat_period = tick_period / 1.0;
        println!("Sequencer: {bpm}, {sample_rate}, {channel_count}, {tick_period}");
        let oscillator = Oscillator::new(sample_rate as f32);
        let freqs = vec![
            // utils::C_FREQ,
            // utils::G_FREQ,
            utils::A_FREQ,
            // utils::G_FREQ,
            // utils::F_FREQ,
            // utils::E_FREQ,
            // utils::D_FREQ,
            // utils::C_FREQ,
        ];
        Self {
            is_running: false,
            oscillator,
            producer,
            beat_index: 0,
            length: freqs.len() as u8,
            freq: freqs[0],
            freqs,
            tick_period,
            beat_period,
            is_beat: false,
            ramp: 0.0,
            tick: 0,
            signal: 0.0,
        }
    }

    pub fn update(&mut self, elapsed_samples: u32) {
        let remainder = elapsed_samples % self.tick_period as u32;
        self.is_beat = remainder > 0 && remainder < self.beat_period as u32;
        self.beat_index = elapsed_samples / self.tick_period as u32;
        let step_index = (self.beat_index % self.length as u32) as usize;
        const TEMP_OCTAVE: u8 = 2u8.pow(3);
        let freq_diff: f32 = if step_index == 0 {
            0.0
        } else {
            // Reach next freq in 50 samples
            self.freqs[step_index] - self.freqs[step_index - 1] / 50.0
        };

        let mut value = 0.0;
        for _ in 0..4096 * 2 {
            if !self.producer.is_full() {
                // Ramp between steps
                // if self.freq != self.freqs[step_index] {
                //     self.freq += freq_diff;
                // }
                self.freq = self.freqs[step_index];
                value = self
                    .oscillator
                    .sine(self.freq * TEMP_OCTAVE as f32, self.tick);

                // Ramp between volumes
                if self.is_beat && self.ramp < 1.0 {
                    self.ramp = clamp(self.ramp + 0.001, 0.0, 1.0);
                } else if !self.is_beat && self.ramp > 0.0 {
                    self.ramp = clamp(self.ramp - 0.001, 0.0, 1.0);
                }
                value *= self.ramp;
                // value *= if self.is_beat { 1.0 } else { 0.0 };
                self.producer.push(value).unwrap();
                self.tick += 1;
            }
        }
        self.signal = value; // Assign the last value to signal
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> bool {
        self.is_beat
    }

    pub fn get_signal(&self) -> f32 {
        self.signal
    }
}
