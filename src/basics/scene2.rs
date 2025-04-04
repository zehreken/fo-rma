use std::num::NonZeroU64;

use glam::{vec3, Vec3};
use wgpu::{Device, Extent3d, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use super::{
    camera::{self, Camera},
    core::GenericUniformData,
    cube::Cube,
    light::{self, Light},
    material::Material,
    primitive::Primitive,
    quad::Quad,
    sphere::Sphere,
    uniforms::{ColorUniform, EqualizerUniform, UniformTrait, WaveUniform, WaveWorldUniform},
};
use crate::{
    rendering::screen_renderer::DynamicTexture,
    rendering_utils::create_wave_texture,
    utils::{self, ToVec4},
};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Primitive>>,
    pub lights: Vec<Light>,
    elapsed: f32,
}

impl Scene {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        generic_uniform_data: &GenericUniformData,
        light_uniform_data: &GenericUniformData,
        size: PhysicalSize<u32>,
    ) -> Self {
        let camera = camera::Camera::new(
            vec3(0.0, 0.0, 2.4),
            vec3(0.0, 0.0, 0.0),
            size.width as f32 / size.height as f32,
            45.0,
            0.1,
            100.0,
        );

        let mut objects: Vec<Box<dyn Primitive>> = vec![];

        let mut quad = Quad::new(
            device,
            create_equalizer_material(
                device,
                surface_config,
                generic_uniform_data,
                light_uniform_data,
            ),
        );
        quad.state.set_position(Vec3 {
            x: -0.5,
            y: 0.0,
            z: 0.0,
        });
        quad.state.scale(Vec3 {
            x: 1.0,
            y: 2.0,
            z: 1.0,
        });
        objects.push(Box::new(quad));

        let mut quad = Quad::new(
            device,
            create_equalizer_material(
                device,
                surface_config,
                generic_uniform_data,
                light_uniform_data,
            ),
        );
        quad.state.set_position(Vec3 {
            x: 0.5,
            y: 0.0,
            z: 0.0,
        });
        quad.state.scale(Vec3 {
            x: 1.0,
            y: 2.0,
            z: 1.0,
        });
        objects.push(Box::new(quad));

        let mut light = Light::new([1.0, 1.0, 1.0]);
        light.update_position(vec3(0.0, 1.0, 0.0));
        let lights = vec![light];

        Self {
            camera,
            objects,
            lights,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, signal: f32, show_beat: bool) {
        self.elapsed += delta_time;
        let el = self.elapsed * 0.5;
        // self.lights[0].update_position(vec3(5.0 * el.cos(), 0.0, 5.0 * el.sin()));

        // self.camera
        //     .update_position(vec3(5.0 * elapsed.cos(), 0.0, 5.0 * elapsed.sin()));

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

fn create_color_material(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
    light_uniform_data: &GenericUniformData,
) -> Material {
    let shader_main = include_str!("../shaders/basic_light.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(ColorUniform {
        color: utils::CCP.palette[3].to_vec4(1.0),
    });

    create_material(
        device,
        surface_config,
        generic_uniform_data,
        light_uniform_data,
        shader_main,
        uniform,
        "color",
    )
}

fn create_equalizer_material(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
    light_uniform_data: &GenericUniformData,
) -> Material {
    let shader_main = include_str!("../shaders/equalizer.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(EqualizerUniform {
        color1: utils::CP6.palette[0].to_vec4(1.0),
        color2: utils::CP6.palette[1].to_vec4(1.0),
        color3: utils::CP6.palette[2].to_vec4(1.0),
        signal: 0.7,
        _padding: [0.0, 0.0, 0.0],
    });

    create_material(
        device,
        surface_config,
        generic_uniform_data,
        light_uniform_data,
        shader_main,
        uniform,
        "equalizer",
    )
}

fn create_wave_world_material(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
    light_uniform_data: &GenericUniformData,
) -> Material {
    let shader_main = include_str!("../shaders/wave_world.wgsl");
    let uniform: Box<dyn UniformTrait> = Box::new(WaveWorldUniform {
        color1: utils::CP2.palette[0].to_vec4(1.0),
        color2: utils::CP2.palette[1].to_vec4(1.0),
        color3: utils::CP2.palette[2].to_vec4(1.0),
        signal: 0.5,
        _padding: [0.0, 0.0, 0.0],
    });

    create_material(
        device,
        surface_config,
        generic_uniform_data,
        light_uniform_data,
        shader_main,
        uniform,
        "wave_world",
    )
}

fn create_material(
    device: &Device,
    surface_config: &SurfaceConfiguration,
    generic_uniform_data: &GenericUniformData,
    light_uniform_data: &GenericUniformData,
    shader_main: &str,
    uniform: Box<dyn UniformTrait>,
    name: &str,
) -> Material {
    let material = Material::new(
        &device,
        &surface_config,
        &generic_uniform_data.uniform_bind_group_layout,
        &light_uniform_data.uniform_bind_group_layout,
        shader_main,
        name,
        uniform,
    );

    material
}
