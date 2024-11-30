use num_traits::float::Float;
use std::ops::Div;

pub const GAMMA: f64 = 2.2;

/// Applies gamma correction to a color (linear -> sRGB)
pub fn linear_to_srgb<T: Float + Div<T>>(color: [T; 3], gamma: T) -> [T; 3] {
    #[cfg(target_os = "macos")]
    {
        let inv_gamma = T::from(1.0).unwrap() / gamma;
        [
            color[0].powf(inv_gamma),
            color[1].powf(inv_gamma),
            color[2].powf(inv_gamma),
        ]
    }

    #[cfg(not(target_os = "macos"))]
    {
        color
    }
}

/// Applies inverse gamma correction to a color (sRGB -> linear)
pub fn srgb_to_linear<T: Float>(color: [T; 3], gamma: T) -> [T; 3] {
    #[cfg(target_os = "macos")]
    {
        [
            color[0].powf(gamma),
            color[1].powf(gamma),
            color[2].powf(gamma),
        ]
    }

    #[cfg(not(target_os = "macos"))]
    {
        color
    }
}
