use super::{
    camera::{self, Camera},
    cube::Cube,
    light::Light,
    primitive::Primitive,
    quad::Quad,
    uniforms::{EqualizerUniform, LightUniform, ObjectUniform, UniformTrait, WaveWorldUniform},
};
use crate::{
    color_utils::{self, ToVec4},
    material::{
        diffuse_color_material::DiffuseColorMaterial,
        equalizer_material::{EqualizerMaterial, EqualizerUniforms},
    },
};
use glam::{vec3, Vec3};
use wgpu::{Device, Queue, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

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
        for i in 0..25 {
            // let material = UnlitColorMaterial::new(device, surface_config);
            // let material = DiffuseColorMaterial::new(device, surface_config);
            let material = EqualizerMaterial::new(device, surface_config);
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

        let material = EqualizerMaterial::new(device, surface_config);
        let mut quad = Quad::new(device, Box::new(material));
        quad.state.set_position(Vec3 {
            x: 0.0,
            y: -3.0,
            z: 0.0,
        });
        quad.state.rotate(Vec3 {
            x: -90.0,
            y: 0.0,
            z: 0.0,
        });
        quad.state.scale(Vec3 {
            x: 100.0,
            y: 100.0,
            z: 100.0,
        });
        objects.push(Box::new(quad));

        let material = EqualizerMaterial::new(device, surface_config);
        let mut quad = Quad::new(device, Box::new(material));
        quad.state.set_position(Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        });
        quad.state.rotate(Vec3 {
            x: 90.0,
            y: 0.0,
            z: 0.0,
        });
        quad.state.scale(Vec3 {
            x: 100.0,
            y: 100.0,
            z: 100.0,
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

    pub fn update(&mut self, queue: &Queue, delta_time: f32, signal: f32, show_beat: bool) {
        self.elapsed += delta_time;
        let el = self.elapsed * 0.5;
        // self.lights[0].update_position(vec3(5.0 * el.cos(), 0.0, 5.0 * el.sin()));

        // self.camera
        //     .update_position(vec3(5.0 * elapsed.cos(), 0.0, 5.0 * elapsed.sin()));

        for primitive in &mut self.objects {
            let object = ObjectUniform {
                view_proj: self.camera.build_view_projection_matrix(),
                model: primitive.model_matrix(),
                normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
            };
            let equalizer = EqualizerUniform {
                color1: color_utils::CCP.palette[0].to_vec4(1.0),
                color2: color_utils::CCP.palette[1].to_vec4(1.0),
                color3: color_utils::CCP.palette[2].to_vec4(1.0),
                signal,
                _padding: [0.0, 0.0, 0.0],
            };
            let light = LightUniform {
                position: self.lights[0].transform.position.extend(0.0).to_array(),
                color: self.lights[0].color.to_vec4(1.0),
            };
            let data = EqualizerUniforms {
                object,
                equalizer,
                light,
            };
            primitive.material().update(queue, &data);
            // primitive.material_mut().uniform.set_signal(signal);
            primitive.update(if show_beat {
                delta_time * 20.0
            } else {
                delta_time
            });
        }
    }
}
