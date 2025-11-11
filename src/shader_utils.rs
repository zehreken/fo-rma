use std::sync::LazyLock;
use wgpu::{naga::FastIndexMap, ShaderSource};

pub static EFFECTS: LazyLock<FastIndexMap<Effect, ShaderSource>> = LazyLock::new(|| {
    let mut map = FastIndexMap::default();
    map.insert(
        Effect::None,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/none.comp.wgsl").into()),
    );
    map.insert(
        Effect::Noise,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/noise.comp.wgsl").into()),
    );
    map.insert(
        Effect::Pixelate,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/pixelate.comp.wgsl").into()),
    );
    map.insert(
        Effect::InvertColor,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/invert_color.comp.wgsl").into()),
    );
    map.insert(
        Effect::Wave,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/wave.comp.wgsl").into()),
    );
    map.insert(
        Effect::Interlace,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/interlace.comp.wgsl").into()),
    );
    map.insert(
        Effect::FlipAxis,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/flip_axis.comp.wgsl").into()),
    );
    map.insert(
        Effect::Grayscale,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/grayscale.comp.wgsl").into()),
    );
    map.insert(
        Effect::Step,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/step.comp.wgsl").into()),
    );
    map.insert(
        Effect::Watercolor,
        wgpu::ShaderSource::Wgsl(include_str!("shaders/compute/watercolor.comp.wgsl").into()),
    );
    map
});

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    None,
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
        Effect::None => "none",
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
