//! Assumes that the input and output devices can use the same stream configuration and that they
//! support the f32 sample format.
//!
//! Uses a delay of `LATENCY_MS` milliseconds in case the default input and output streams are not
//! precisely synchronised.

extern crate cpal;
extern crate ringbuf;

use super::{audio_clock::AudioClock, generator::Input};
use crate::audio::{sequencer::Sequencer, songs};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};
use kopek::{metronome::Metronome, utils};
use ringbuf::{HeapProducer, HeapRb};
use std::sync::Arc;

const LATENCY_MS: f32 = 10.0;

pub struct AudioModel {
    output_stream: Stream,
    audio_clock: Arc<AudioClock>,
    metronome: Metronome,
    sequencers: Vec<Sequencer>,
    input_producer: HeapProducer<Input>,
    producer: HeapProducer<f32>,
    signal: f32, // for visuals
                 // view_consumer: HeapConsumer<f32>,
}

impl AudioModel {
    pub fn new() -> Result<AudioModel, ()> {
        let host = cpal::default_host();

        // Default devices.
        let input_device = host
            .default_input_device()
            .expect("failed to get default input device");
        let output_device = host
            .default_output_device()
            .expect("failed to get default output device");
        println!(
            "Using default input device: \"{}\"",
            input_device.name().unwrap()
        );
        println!(
            "Using default output device: \"{}\"",
            output_device.name().unwrap()
        );

        // We'll try and use the same configuration between streams to keep it simple.
        let output_config: cpal::StreamConfig =
            output_device.default_output_config().unwrap().into();

        let ring = HeapRb::new(1024 * 2);
        let (mut producer, mut consumer) = ring.split();
        if !producer.is_full() {
            producer.push(0.0).unwrap();
        }
        let input_ring = HeapRb::new(10);
        let (input_producer, input_consumer) = input_ring.split();
        // let view_ring = HeapRb::new(100000);
        // let (view_producer, view_consumer) = view_ring.split();

        let sample_rate = output_config.sample_rate.0;
        let audio_clock = Arc::new(AudioClock::new());

        let metronome = Metronome::new(60, sample_rate, output_config.channels as u32);

        let clock_for_audio = Arc::clone(&audio_clock);
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(output_config.channels as usize) {
                if let Some(input) = consumer.pop() {
                    for sample in frame {
                        *sample = input;
                    }
                } else {
                    // eprintln!("Ringbuffer underrun {:?}", std::time::SystemTime::now())
                }
                clock_for_audio.update();
            }
        };

        // Build streams.
        println!(
            "Attempting to build both streams with f32 samples and `{:?}`.",
            output_config
        );
        let output_stream = output_device
            .build_output_stream(&output_config, output_data_fn, err_fn, None)
            .unwrap();
        println!("Successfully built streams.");

        // Play the streams, This is not correct
        println!(
            "Starting the output stream with `{}` milliseconds of latency.",
            LATENCY_MS
        );
        output_stream.play().expect("Can't play output stream");

        // let mut generator =
        //     Generator::new(producer, input_consumer, view_producer, sample_rate as f32).unwrap();
        // std::thread::spawn(move || loop {
        //     generator.update();
        // });

        let sequencer = Sequencer::new(
            120,
            sample_rate,
            output_config.channels.into(),
            vec![utils::A_FREQ],
        );

        let sequencer_2 = Sequencer::new(
            240,
            sample_rate,
            output_config.channels.into(),
            songs::BILLIE_JEAN_2.to_vec(),
        );
        let mut sequencers = Vec::new();
        sequencers.push(sequencer);
        // sequencers.push(sequencer_2);
        // std::thread::spawn(move || loop {
        //     let elapsed_samples = audio_clock.get_elapsed_samples();
        //     sequencer.update(elapsed_samples);
        // });

        Ok(AudioModel {
            output_stream,
            audio_clock,
            metronome,
            sequencers,
            input_producer,
            producer,
            signal: 0.0,
            // view_consumer,
        })
    }

    pub fn get_signal(&mut self) -> f32 {
        self.signal
    }

    pub fn show_beat(&self) -> bool {
        self.metronome.show_beat()
    }

    pub fn update(&mut self) {
        let elapsed_samples = self.audio_clock.get_elapsed_samples();
        // self.metronome.update(sample_count);
        while !self.producer.is_full() {
            let mut value = 0.0;
            for s in &mut self.sequencers {
                value += s.update(elapsed_samples);
            }
            value = value / self.sequencers.len() as f32;
            self.producer.push(value).unwrap();
            self.signal = value;
        }
        // if self.metronome.show_beat() {
        //     self.input_producer.push(Input::Start).unwrap();
        // } else {
        //     self.input_producer.push(Input::Stop).unwrap();
        // }
    }

    pub fn get_sequencers(&mut self) -> &mut Vec<Sequencer> {
        &mut self.sequencers
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
