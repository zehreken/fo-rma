use kopek::utils::{get_freq, Key, Octave};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub octave: Octave,
    pub key: Key,
}

impl Note {
    fn get_octave(&self) -> f32 {
        match self.octave {
            Octave::First => 1.0,
            Octave::Second => 2.0,
            Octave::Third => 4.0,
            Octave::Fourth => 8.0,
            Octave::Fifth => 16.0,
        }
    }
    fn get_freq(&self) -> f32 {
        get_freq(self.key)
    }

    pub fn get(&self) -> f32 {
        self.get_octave() * self.get_freq()
    }
}
