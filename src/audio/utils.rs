pub enum Octave {
    First,
    Second,
    Third,
    Fourth,
}

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

pub struct Note {
    pub octave: Octave,
    pub pitch: Pitch,
}
