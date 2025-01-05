use kopek::{noise::Noise, oscillator::*, utils::C_FREQ};
use ringbuf::{HeapConsumer, HeapProducer};

use crate::basics::core::clamp;

pub enum WaveType {
    Sine,
    Sawtooth,
    Square,
    Triangle,
}

pub enum NoiseType {
    None,
    Random,
    White,
}

pub struct Generator {
    is_running: bool,
    tick: u32,
    freq: f32,
    oscillator: Oscillator,
    oscillator_type: WaveType,
    noise: Noise,
    noise_type: NoiseType,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<f32>,
    ramp: f32,
}

impl Generator {
    pub fn new(
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<f32>,
        sample_rate: f32,
    ) -> Result<Generator, ()> {
        const TEMP_OCTAVE: u8 = 2u8.pow(1);
        Ok(Generator {
            is_running: true,
            tick: 0,
            freq: C_FREQ * TEMP_OCTAVE as f32,
            oscillator: Oscillator::new(sample_rate),
            oscillator_type: WaveType::Sawtooth,
            noise: Noise::new(),
            noise_type: NoiseType::None,
            producer,
            input_consumer,
            view_producer,
            ramp: 0.0,
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = match self.oscillator_type {
                    WaveType::Sine => self.oscillator.sine(),
                    WaveType::Sawtooth => self.oscillator.sawtooth(),
                    WaveType::Square => self.oscillator.square(0.5),
                    WaveType::Triangle => self.oscillator.triangle(),
                };
                value += 0.5
                    * match self.noise_type {
                        NoiseType::None => 0.0,
                        NoiseType::Random => self.noise.rand_noise(),
                        NoiseType::White => self.noise.white_noise(),
                    };
                // let value = kopek::wave::white_noise();
                // let value = kopek::wave::rand_noise();
                if self.is_running && self.ramp < 1.0 {
                    self.ramp = clamp(self.ramp + 0.001, 0.0, 1.0);
                } else if !self.is_running && self.ramp > 0.0 {
                    self.ramp = clamp(self.ramp - 0.001, 0.0, 1.0);
                }
                value *= self.ramp;
                self.producer.push(value).unwrap();
                if !self.view_producer.is_full() {
                    self.view_producer.push(value).unwrap();
                }
                self.tick += 1;
            }
        }
        // Input
        if let Some(input) = self.input_consumer.pop() {
            match input {
                Input::Start => self.is_running = true,
                Input::Stop => self.is_running = false,
                Input::ChangeFreq(freq) => self.freq = freq,
                Input::ChangeOscillator(osc) => {
                    if osc == 0 {
                        self.oscillator_type = WaveType::Sine;
                    } else if osc == 1 {
                        self.oscillator_type = WaveType::Sawtooth;
                    } else if osc == 2 {
                        self.oscillator_type = WaveType::Square;
                    } else if osc == 3 {
                        self.oscillator_type = WaveType::Triangle;
                    }
                }
                Input::ChangeNoise(noise) => {
                    if noise == 0 {
                        self.noise_type = NoiseType::None;
                    } else if noise == 1 {
                        self.noise_type = NoiseType::Random;
                    } else if noise == 2 {
                        self.noise_type = NoiseType::White;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Input {
    Start,
    Stop,
    ChangeFreq(f32),
    ChangeOscillator(u8),
    ChangeNoise(u8),
}
