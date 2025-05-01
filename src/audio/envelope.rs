pub struct Envelope {
    time: f32,
    state: EnvelopeState,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            time: 0.0,
            state: EnvelopeState::Attack,
            attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> f32 {
        self.time += delta_time;

        match self.state {
            EnvelopeState::Attack => {
                if self.time >= self.attack {
                    self.state = EnvelopeState::Decay;
                    self.time = 0.0;
                    1.0
                } else {
                    lerp(self.time / self.attack, 0.0, 1.0)
                }
            }
            EnvelopeState::Decay => {
                if self.time >= self.decay {
                    self.state = EnvelopeState::Sustain;
                    self.time = 0.0;
                    0.6
                } else {
                    lerp(self.time / self.decay, 1.0, 0.6)
                }
            }
            EnvelopeState::Sustain => {
                if self.time >= self.sustain {
                    self.state = EnvelopeState::Release;
                    self.time = 0.0;
                    0.5
                } else {
                    lerp(self.time / self.sustain, 0.6, 0.5)
                }
            }
            EnvelopeState::Release => {
                if self.time >= self.release {
                    self.state = EnvelopeState::None;
                    self.time = 0.0;
                    0.0
                } else {
                    lerp(self.time / self.release, 0.5, 0.0)
                }
            }
            EnvelopeState::None => 0.0,
        }
    }

    // Obsolete
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.state = EnvelopeState::Attack;
    }
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    return a + t * (b - a);
}

enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
    None,
}
