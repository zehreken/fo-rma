use kopek::noise::Noise;

pub struct NoiseGenerator {
    noise: Noise,
    noise_type: NoiseType,
}

impl NoiseGenerator {
    pub fn new() -> Self {
        NoiseGenerator {
            noise: Noise::new(),
            noise_type: NoiseType::None,
        }
    }

    pub fn run(&mut self) -> f32 {
        match self.noise_type {
            NoiseType::None => 0.0,
            NoiseType::Random => self.noise.rand_noise(),
            NoiseType::White => self.noise.white_noise(),
        }
    }

    pub fn noise_type_mut(&mut self) -> &mut NoiseType {
        &mut self.noise_type
    }
}

pub enum NoiseType {
    None,
    Random,
    White,
}
