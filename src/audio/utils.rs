pub enum Octave {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}
pub const OCTAVES: [&str; 5] = ["First", "Second", "Third", "Fourth", "Fifth"];

pub enum Pitch {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}
pub const PITCHES: [&str; 12] = [
    "C", "Cs", "D", "Ds", "E", "F", "Fs", "G", "Gs", "A", "As", "B",
];

pub struct Note {
    pub octave: Octave,
    pub pitch: Pitch,
}
