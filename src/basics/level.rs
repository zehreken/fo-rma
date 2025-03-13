use glam::Vec3;

use super::{
    cube::Cube,
    material::Material,
    primitive::Primitive,
    quad::Quad,
    sphere::Sphere,
    uniforms::{ColorUniform, EqualizerUniform, UniformTrait, WaveWorldUniform},
};
use crate::{
    renderer::Renderer,
    utils::{self, ToVec4},
};

pub struct Level {
    pub objects: Vec<Box<dyn Primitive>>,
}

impl Level {
    pub fn new(renderer: &Renderer) -> Self {
        let color_material = create_color_material(renderer);
        let equalizer_material = create_equalizer_material(renderer);
        let wave_world_material = create_wave_world_material(renderer);

        let mut objects: Vec<Box<dyn Primitive>> = vec![];
        for i in 0..25 {
            let mut sphere = Sphere::new(&renderer.device, create_color_material(renderer));
            let x = (-4 + i % 5 * 2) as f32;
            let y = (-4 + i / 5 * 2) as f32;
            sphere.state.set_position(Vec3 { x, y, z: 0.0 });
            objects.push(Box::new(sphere));
        }

        // let objects: Vec<Box<dyn Primitive>> = vec![
        //     Box::new(Sphere::new(renderer, color_material)),
        //     Box::new(Quad::new(renderer, equalizer_material)),
        //     Box::new(Cube::new(renderer, wave_world_material)),
        // ];

        Self { objects }
    }

    pub fn update(&mut self, delta_time: f32, signal: f32, show_beat: bool) {
        for primitive in &mut self.objects {
            primitive.material_mut().uniform.set_signal(signal);
            primitive.update(if show_beat {
                delta_time * 20.0
            } else {
                delta_time
            });
        }
    }
}

fn create_color_material(renderer: &Renderer) -> Material {
    let shader_main = include_str!("../shaders/basic_light.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(ColorUniform {
        color: utils::CP1.palette[3].to_vec4(1.0),
    });

    create_material(renderer, shader_main, uniform, "color")
}

fn create_equalizer_material(renderer: &Renderer) -> Material {
    let shader_main = include_str!("../shaders/equalizer.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(EqualizerUniform {
        color1: utils::CP6.palette[0].to_vec4(1.0),
        color2: utils::CP6.palette[1].to_vec4(1.0),
        color3: utils::CP6.palette[2].to_vec4(1.0),
        signal: 0.7,
        _padding: [0.0, 0.0, 0.0],
    });

    create_material(renderer, shader_main, uniform, "equalizer")
}

fn create_wave_world_material(renderer: &Renderer) -> Material {
    let shader_main = include_str!("../shaders/wave_world.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(WaveWorldUniform {
        color1: utils::CP2.palette[0].to_vec4(1.0),
        color2: utils::CP2.palette[1].to_vec4(1.0),
        color3: utils::CP2.palette[2].to_vec4(1.0),
        signal: 0.5,
        _padding: [0.0, 0.0, 0.0],
    });

    create_material(renderer, shader_main, uniform, "wave_world")
}
fn create_material(
    renderer: &Renderer,
    shader_main: &str,
    uniform: Box<dyn UniformTrait>,
    name: &str,
) -> Material {
    let material = Material::new(
        &renderer.device,
        &renderer.surface_config,
        &renderer.generic_uniform_data.uniform_bind_group_layout,
        &renderer.light_uniform_data.uniform_bind_group_layout,
        shader_main,
        name,
        uniform,
    );

    material
}
