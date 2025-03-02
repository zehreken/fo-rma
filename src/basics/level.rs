use super::{
    material::Material,
    primitive::Primitive,
    quad::Quad,
    sphere::Sphere,
    uniforms::{ColorUniform, EqualizerUniform, UniformTrait},
};
use crate::{
    renderer::Renderer,
    utils::{self, ToVec4},
};

pub struct Level {
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Level {
    pub fn new(renderer: &Renderer) -> Self {
        let color_material = create_color_material(renderer);
        let equalizer_material = create_equalizer_material(renderer);

        let primitives: Vec<Box<dyn Primitive>> = vec![
            Box::new(Sphere::new(renderer, color_material)),
            Box::new(Quad::new(renderer, equalizer_material)),
        ];

        Self { primitives }
    }

    pub fn update(&mut self, delta_time: f32) {
        for primitive in &mut self.primitives {
            // primitive.update(if app.audio_model.show_beat() {
            //     delta_time * 20.0
            // } else {
            //     delta_time
            // });
            primitive.update(delta_time);
        }
    }
}

fn create_color_material(renderer: &Renderer) -> Material {
    let shader_main = include_str!("../shaders/basic_light.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(ColorUniform {
        color: utils::CCP.palette[1].to_vec4(0.5),
    });

    create_material(renderer, shader_main, uniform)
}

fn create_equalizer_material(renderer: &Renderer) -> Material {
    let shader_main = include_str!("../shaders/equalizer.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(EqualizerUniform {
        color1: utils::CP4.palette[0].to_vec4(1.0),
        color2: utils::CP4.palette[1].to_vec4(1.0),
        color3: utils::CP4.palette[2].to_vec4(1.0),
        signal: 0.7,
        _padding: [0.0, 0.0, 0.0],
    });

    create_material(renderer, shader_main, uniform)
}

fn create_material(
    renderer: &Renderer,
    shader_main: &str,
    uniform: Box<dyn UniformTrait>,
) -> Material {
    let material = Material::new(
        &renderer.device,
        &renderer.surface_config,
        &renderer.generic_uniform_data.uniform_bind_group_layout,
        &renderer.light_uniform_data.uniform_bind_group_layout,
        shader_main,
        "material",
        uniform,
    );

    material
}
