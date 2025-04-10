use glam::{vec3, Vec3};
use wgpu::{Device, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

use super::{
    camera::{self, Camera},
    core::GenericUniformData,
    cube::Cube,
    light::{self, Light},
    material::Material,
    primitive::Primitive,
    quad::Quad,
    uniforms::{ColorUniform, EqualizerUniform, UniformTrait, WaveWorldUniform},
};
use crate::{
    color_utils::{self, ToVec4},
    material::{
        diffuse_color_material::DiffuseColorMaterial, unlit_color_material::UnlitColorMaterial,
    },
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

        // let color_material = create_color_material(
        //     device,
        //     surface_config,
        //     generic_uniform_data,
        //     light_uniform_data,
        // );

        let mut objects: Vec<Box<dyn Primitive>> = vec![];
        for i in 0..25 {
            // let material = UnlitColorMaterial::new(device, surface_config);
            let material = DiffuseColorMaterial::new(device, surface_config);
            let mut cube = Cube::new(&device, Box::new(material));
            let x = (-8 + i % 5 * 4) as f32;
            let z = (-8 + i / 5 * 4) as f32;
            cube.state.set_position(Vec3 { x, y: 0.0, z });
            cube.state.scale(Vec3 {
                x: 1.0,
                y: 6.0,
                z: 1.0,
            });
            objects.push(Box::new(cube));
        }

        // let material = UnlitColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: 0.0,
        //     y: -0.5,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 2.0,
        //     y: 1.0,
        //     z: 1.0,
        // });
        // objects.push(Box::new(quad));

        // let material = UnlitColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: 0.0,
        //     y: 0.5,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 2.0,
        //     y: 1.0,
        //     z: 1.0,
        // });
        // objects.push(Box::new(quad));

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
            // primitive.material_mut().uniform.set_signal(signal);
            primitive.update(if show_beat {
                delta_time * 20.0
            } else {
                delta_time
            });
        }
    }
}

// fn create_color_material(
//     device: &Device,
//     surface_config: &SurfaceConfiguration,
//     generic_uniform_data: &GenericUniformData,
//     light_uniform_data: &GenericUniformData,
// ) -> Material {
//     let shader_main = include_str!("../shaders/basic_light.wgsl");
//     let uniform: Box<dyn UniformTrait> = Box::new(ColorUniform {
//         color: color_utils::CCP.palette[3].to_vec4(1.0),
//     });

//     create_material(
//         device,
//         surface_config,
//         generic_uniform_data,
//         light_uniform_data,
//         shader_main,
//         uniform,
//         "color",
//     )
// }

// fn create_material(
//     device: &Device,
//     surface_config: &SurfaceConfiguration,
//     generic_uniform_data: &GenericUniformData,
//     light_uniform_data: &GenericUniformData,
//     shader_main: &str,
//     uniform: Box<dyn UniformTrait>,
//     name: &str,
// ) -> Material {
//     let material = Material::new(
//         &device,
//         &surface_config,
//         &generic_uniform_data.uniform_bind_group_layout,
//         &light_uniform_data.uniform_bind_group_layout,
//         shader_main,
//         name,
//         uniform,
//     );

//     material
// }
