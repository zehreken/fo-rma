use super::sequencer::Sequencer;
use kopek::utils::{Key, Octave};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Read, Write},
};

pub fn save_song(sequencers: &Vec<Sequencer>) -> Result<(), Error> {
    let mut sequences: [[NoteData; 16]; 3] = [[NoteData::default(); 16]; 3];
    for sequencer_index in 0..3 {
        for note_index in 0..16 {
            sequences[sequencer_index][note_index] = NoteData {
                octave: sequencers[sequencer_index].sequence[note_index].octave as i32,
                key: sequencers[sequencer_index].sequence[note_index].key as i32,
            }
        }
    }

    let song = Song { sequences };

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
    pub sequences: [[NoteData; 16]; 3],
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
