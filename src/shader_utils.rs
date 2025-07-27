use std::{collections::HashMap, sync::LazyLock};
use wgpu::ShaderSource;

pub static EFFECTS: LazyLock<HashMap<String, ShaderSource>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "noise".to_owned(),
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/noise.comp.wgsl").into()),
    );
    map.insert(
        "pixelate".to_owned(),
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/pixelate.comp.wgsl").into()),
    );
    // map.insert(
    //     "invert_color".to_owned(),
    //     wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/invert_color.comp.wgsl").into()),
    // );
    // map.insert(
    //     "wave".to_owned(),
    //     wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/wave.comp.wgsl").into()),
    // );
    map.insert(
        "interlace".to_owned(),
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/interlace.comp.wgsl").into()),
    );
    // map.insert(
    //     "flip_axis".to_owned(),
    //     wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/flip_axis.comp.wgsl").into()),
    // );
    // map.insert(
    //     "grayscale".to_owned(),
    //     wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/grayscale.comp.wgsl").into()),
    // );
    // map.insert(
    //     "step".to_owned(),
    //     wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/step.comp.wgsl").into()),
    // );
    map.insert(
        "watercolor".to_owned(),
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/watercolor.comp.wgsl").into()),
    );
    map
});

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    // None,
    Noise,
    Pixelate,
    InvertColor,
    Wave,
    Interlace,
    FlipAxis,
    Grayscale,
    Step,
    Watercolor,
}

pub fn effect_to_name(effect: Effect) -> &'static str {
    match effect {
        Effect::Noise => "noise",
        Effect::Pixelate => "pixelate",
        Effect::InvertColor => "invert_color",
        Effect::Wave => "wave",
        Effect::Interlace => "interlace",
        Effect::FlipAxis => "flipaxis",
        Effect::Grayscale => "grayscale",
        Effect::Step => "step",
        Effect::Watercolor => "watercolor",
    }
}
