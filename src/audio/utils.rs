use kopek::utils::{get_freq, Key, Octave};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub octave: Octave,
    pub key: Key,
}

impl Note {
    fn get_octave(&self) -> f32 {
        let shift = match self.octave {
            Octave::First => 0,
            Octave::Second => 1,
            Octave::Third => 2,
            Octave::Fourth => 3,
            Octave::Fifth => 4,
        };
        2.0_f32.powi(shift)
    }

    fn get_freq(&self) -> f32 {
        get_freq(self.key)
    }

    pub fn get(&self) -> f32 {
        self.get_octave() * self.get_freq()
    }
}
