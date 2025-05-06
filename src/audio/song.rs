use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub sequences: [[NoteData; 16]; 3],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteData {
    pub octave: i32,
    pub key: i32,
}
