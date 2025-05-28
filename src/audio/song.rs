use super::sequencer::Sequencer;
use kopek::{
    noise_generator::NoiseType,
    oscillator::WaveType,
    utils::{Key, Octave},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Read, Write},
};

pub fn save_song(sequencers: &Vec<Sequencer>) -> Result<(), Error> {
    let mut seri_sequencers: [SequencerData; 3] = [SequencerData::default(); 3];
    let mut envelopes: [EnvelopeData; 3] = [EnvelopeData::default(); 3];
    let mut sequences: [[NoteData; 16]; 3] = [[NoteData::default(); 16]; 3];
    for sequencer_index in 0..3 {
        seri_sequencers[sequencer_index].vco_wave_type =
            sequencers[sequencer_index].vco_wave_type().to_u8();
        seri_sequencers[sequencer_index].lfo_wave_type =
            sequencers[sequencer_index].lfo_wave_type().to_u8();
        seri_sequencers[sequencer_index].noise_type =
            sequencers[sequencer_index].noise_type() as u8;

        envelopes[sequencer_index].attack = sequencers[sequencer_index].envelope.attack;
        envelopes[sequencer_index].decay = sequencers[sequencer_index].envelope.decay;
        envelopes[sequencer_index].sustain = sequencers[sequencer_index].envelope.sustain;
        envelopes[sequencer_index].release = sequencers[sequencer_index].envelope.release;

        for note_index in 0..16 {
            sequences[sequencer_index][note_index] = NoteData {
                octave: sequencers[sequencer_index].sequence[note_index].octave as i32,
                key: sequencers[sequencer_index].sequence[note_index].key as i32,
            }
        }
    }

    let song = Song {
        sequencers: seri_sequencers,
        envelopes,
        sequences,
    };

    let serialized = serde_json::to_string_pretty(&song).unwrap();

    let mut file = File::create("song.json").unwrap();
    file.write_all(serialized.as_bytes())
}

pub fn load_song(sequencers: &mut Vec<Sequencer>) {
    let mut file = File::open("song.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let song: Song = serde_json::from_str(&contents).unwrap();

    for sequencer_index in 0..3 {
        sequencers[sequencer_index].set_vco_wave_type(
            WaveType::from_u8(song.sequencers[sequencer_index].vco_wave_type).unwrap(),
        );
        sequencers[sequencer_index].set_lfo_wave_type(
            WaveType::from_u8(song.sequencers[sequencer_index].lfo_wave_type).unwrap(),
        );
        sequencers[sequencer_index].set_noise_type(
            NoiseType::from_u8(song.sequencers[sequencer_index].noise_type).unwrap(),
        );

        sequencers[sequencer_index].envelope.attack = song.envelopes[sequencer_index].attack;
        sequencers[sequencer_index].envelope.decay = song.envelopes[sequencer_index].decay;
        sequencers[sequencer_index].envelope.sustain = song.envelopes[sequencer_index].sustain;
        sequencers[sequencer_index].envelope.release = song.envelopes[sequencer_index].release;

        for note_index in 0..16 {
            let note = &mut sequencers[sequencer_index].sequence[note_index];
            let note_data = song.sequences[sequencer_index][note_index];
            note.octave = int_to_octave(note_data.octave);
            note.key = int_to_key(note_data.key);
        }
    }
}

pub fn clear_song(sequencers: &mut Vec<Sequencer>) {
    for sequencer in sequencers {
        for note in &mut sequencer.sequence {
            note.octave = Octave::Third;
            note.key = Key::Rest;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub sequencers: [SequencerData; 3],
    pub envelopes: [EnvelopeData; 3],
    pub sequences: [[NoteData; 16]; 3],
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SequencerData {
    pub vco_wave_type: u8,
    pub lfo_wave_type: u8,
    pub noise_type: u8,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct EnvelopeData {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct NoteData {
    pub octave: i32,
    pub key: i32,
}

pub fn int_to_key(i: i32) -> Key {
    match i {
        0 => Key::C,
        1 => Key::Cs,
        2 => Key::D,
        3 => Key::Ds,
        4 => Key::E,
        5 => Key::F,
        6 => Key::Fs,
        7 => Key::G,
        8 => Key::Gs,
        9 => Key::A,
        10 => Key::As,
        11 => Key::B,
        12 => Key::Rest,
        _ => Key::Rest,
    }
}

pub fn int_to_octave(i: i32) -> Octave {
    match i {
        0 => Octave::First,
        1 => Octave::Second,
        2 => Octave::Third,
        3 => Octave::Fourth,
        4 => Octave::Fifth,
        _ => Octave::Third,
    }
}
