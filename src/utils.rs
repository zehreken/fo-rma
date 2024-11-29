pub const GAMMA: f32 = 2.2;

/// Applies gamma correction to a color (linear -> sRGB)
pub fn gamma_correction(color: [f32; 3], gamma: f32) -> [f32; 3] {
    [
        color[0].powf(1.0 / gamma),
        color[1].powf(1.0 / gamma),
        color[2].powf(1.0 / gamma),
    ]
}

/// Applies inverse gamma correction to a color (sRGB -> linear)
pub fn inverse_gamma_correction(color: [f32; 3], gamma: f32) -> [f32; 3] {
    [
        color[0].powf(gamma),
        color[1].powf(gamma),
        color[2].powf(gamma),
    ]
}
