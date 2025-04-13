use super::{
    camera::{self, Camera},
    cube::Cube,
    light::Light,
    primitive::Primitive,
    quad::Quad,
    sphere::Sphere,
    uniforms::{ColorUniform, EqualizerUniform, LightUniform, ObjectUniform},
};
use crate::{
    color_utils::{self, ToVec4},
    material::{
        diffuse_color_material::{DiffuseColorMaterial, DiffuseColorUniforms},
        equalizer_material::{EqualizerMaterial, EqualizerUniforms},
        unlit_color_material::{UnlitColorMaterial, UnlitColorUniforms},
        wave_material::{WaveMaterial, WaveUniforms},
        MaterialType,
    },
};
use glam::{vec3, Vec3};
use std::{collections::HashMap, sync::Arc};
use wgpu::{Device, Queue, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct Scene {
    pub camera: Camera,
    pub material_object_map: HashMap<MaterialType, Vec<Box<dyn Primitive>>>,
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
        let mut material_object_map = HashMap::new();
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
        material_object_map.insert(MaterialType::EqualizerMaterial, objects);

        objects = vec![];
        for i in 0..4 {
            let material = WaveMaterial::new(device, surface_config);
            let mut sphere = Sphere::new(&device, Box::new(material));
            let x = 10.0 + i as f32 * 3.0;
            let z = i as f32 * 3.0;
            sphere.state.set_position(Vec3 { x, y: 0.0, z });
            objects.push(Box::new(sphere));
        }
        material_object_map.insert(MaterialType::WaveMaterial, objects);

        objects = vec![];
        let material = DiffuseColorMaterial::new(device, surface_config);
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

        let material = DiffuseColorMaterial::new(device, surface_config);
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
        material_object_map.insert(MaterialType::DiffuseColorMaterial, objects);

        let mut light = Light::new([1.0, 1.0, 1.0]);
        light.update_position(vec3(0.0, 1.0, 0.0));
        let lights = vec![light];

        Self {
            camera,
            material_object_map,
            lights,
            elapsed: 0.0,
        }
    }

    pub fn update(
        &mut self,
        queue: &Queue,
        delta_time: f32,
        signal: f32,
        show_beat: bool,
        wave: Arc<Vec<f32>>,
    ) {
        for (material_id, objects) in &mut self.material_object_map {
            if *material_id == MaterialType::EqualizerMaterial {
                for primitive in objects {
                    primitive.update(delta_time);
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
                        color: self.lights[0].color.to_vec4(signal),
                    };
                    let data = EqualizerUniforms {
                        object,
                        equalizer,
                        light,
                    };
                    primitive.material().update(queue, &data);
                }
            } else if *material_id == MaterialType::UnlitColorMaterial {
                for primitive in objects {
                    primitive.update(delta_time);
                    let object = ObjectUniform {
                        view_proj: self.camera.build_view_projection_matrix(),
                        model: primitive.model_matrix(),
                        normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                        normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                        normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
                    };
                    let color = ColorUniform {
                        color: [signal, signal, signal, 1.0],
                    };
                    let data = UnlitColorUniforms { object, color };
                    primitive.material().update(queue, &data);
                }
            } else if *material_id == MaterialType::DiffuseColorMaterial {
                for primitive in objects {
                    primitive.update(delta_time);
                    let object = ObjectUniform {
                        view_proj: self.camera.build_view_projection_matrix(),
                        model: primitive.model_matrix(),
                        normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                        normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                        normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
                    };
                    let color = ColorUniform {
                        color: color_utils::CCP.palette[0].to_vec4(1.0),
                    };
                    let light = LightUniform {
                        position: self.lights[0].transform.position.extend(0.0).to_array(),
                        color: self.lights[0].color.to_vec4(signal),
                    };
                    let data = DiffuseColorUniforms {
                        object,
                        color,
                        light,
                    };
                    primitive.material().update(queue, &data);
                }
            } else if *material_id == MaterialType::WaveMaterial {
                for primitive in objects {
                    primitive.update(delta_time);
                    let object = ObjectUniform {
                        view_proj: self.camera.build_view_projection_matrix(),
                        model: primitive.model_matrix(),
                        normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                        normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                        normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
                    };
                    let data = WaveUniforms {
                        object,
                        wave: Arc::clone(&wave),
                    };
                    primitive.material().update(queue, &data);
                }
            }
        }
    }
}
