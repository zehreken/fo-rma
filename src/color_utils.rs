use num_traits::float::Float;
use std::ops::Div;

pub const GAMMA: f32 = 2.2;

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

pub fn force_srgb_to_linear<T: Float>(color: [T; 3], gamma: T) -> [T; 3] {
    [
        color[0].powf(gamma),
        color[1].powf(gamma),
        color[2].powf(gamma),
    ]
}

pub fn srgb_encode(mut color: Vec<u8>) -> Vec<u8> {
    let threshold = 0.0031308;
    let linear_scale = 12.92;
    let gamma = 1.0 / 2.4;
    let a = 1.055;
    let b = 0.055;
    for i in 0..3 {
        let linear = (color[i] as f32 / 255.0).clamp(0.0, 1.0);
        let srgb: f32 = if linear <= threshold {
            linear * linear_scale
        } else {
            a * linear.powf(gamma) - b
        };
        color[i] = (srgb * 255.0).round() as u8;
    }

    color
}

pub trait ToVec4 {
    fn to_vec4(self, fill: f32) -> [f32; 4];
}

impl ToVec4 for [f32; 3] {
    fn to_vec4(self, fill: f32) -> [f32; 4] {
        [self[0], self[1], self[2], fill]
    }
}

#[derive(Clone, Copy)]
pub struct ColorPalette<T, const N: usize> {
    pub name: &'static str,
    pub palette: [[T; 3]; N],
}

impl<const N: usize> From<ColorPalette<f32, N>> for ColorPalette<u8, N> {
    fn from(f32_palette: ColorPalette<f32, N>) -> Self {
        let mut new_palette = [[0u8; 3]; N];
        for i in 0..N {
            new_palette[i] = [
                (f32_palette.palette[i][0].clamp(0.0, 1.0) * 255.0).round() as u8,
                (f32_palette.palette[i][1].clamp(0.0, 1.0) * 255.0).round() as u8,
                (f32_palette.palette[i][2].clamp(0.0, 1.0) * 255.0).round() as u8,
            ];
        }
        ColorPalette {
            name: f32_palette.name,
            palette: new_palette,
        }
    }
}

pub const CP0: ColorPalette<f32, 4> = ColorPalette {
    name: "cp0",
    palette: [
        [0.263, 0.208, 0.655],
        [1.000, 0.498, 0.243],
        [1.000, 0.965, 0.914],
        [0.502, 0.769, 0.914],
    ],
};

pub const CP1: ColorPalette<f32, 4> = ColorPalette {
    name: "cp1",
    palette: [
        [0.956, 0.964, 1.000],
        [0.952, 0.776, 0.137],
        [0.062, 0.215, 0.361],
        [0.921, 0.513, 0.091],
    ],
};

pub const CP2: ColorPalette<f32, 4> = ColorPalette {
    name: "cp2",
    palette: [
        [0.690, 0.188, 0.322],
        [0.843, 0.424, 0.510],
        [0.494, 0.831, 0.678],
        [0.239, 0.012, 0.004],
    ],
};

pub const CP3: ColorPalette<f32, 4> = ColorPalette {
    name: "cp3",
    palette: [
        [0.263, 0.475, 0.949],
        [0.431, 0.761, 0.027],
        [1.000, 0.922, 0.000],
        [0.067, 0.459, 0.329],
    ],
};

pub const CP4: ColorPalette<f32, 4> = ColorPalette {
    name: "cp4",
    palette: [
        [0.451, 0.467, 0.482],
        [0.925, 0.6, 0.294],
        [0.945, 0.933, 0.914],
        [0.182, 0.175, 0.435],
    ],
};

pub const CP5: ColorPalette<f32, 4> = ColorPalette {
    name: "cp5",
    palette: [
        [0.298, 0.322, 0.439],
        [0.212, 0.933, 0.878],
        [0.965, 0.322, 0.627],
        [0.737, 0.925, 0.878],
    ],
};

pub const CP6: ColorPalette<f32, 4> = ColorPalette {
    name: "cp6",
    palette: [
        [0.20, 0.21, 0.32],
        [0.46, 0.73, 0.28],
        [0.98, 0.82, 0.17],
        [0.86, 0.12, 0.28],
    ],
};

pub const CP7: ColorPalette<f32, 4> = ColorPalette {
    name: "cp7",
    palette: [
        [0.275, 0.208, 0.694],
        [1.000, 0.984, 0.792],
        [0.718, 0.443, 0.898],
        [0.682, 0.918, 0.580],
    ],
};

pub const CP8: ColorPalette<f32, 4> = ColorPalette {
    name: "cp8",
    palette: [
        [0.1, 0.1, 0.1],
        [0.2, 0.2, 0.2],
        [0.3, 0.3, 0.3],
        [0.4, 0.4, 0.4],
    ],
};

pub const COLORS: [ColorPalette<f32, 4>; 9] = [CP0, CP1, CP2, CP3, CP4, CP5, CP6, CP7, CP8];
