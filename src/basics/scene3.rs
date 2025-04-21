use super::{
    camera::{self, Camera},
    cube::Cube,
    light::Light,
    primitive::Primitive,
    quad::Quad,
    scene_loader::SceneData,
    sphere::Sphere,
    triangle::Triangle,
    uniforms::{ColorUniform, EqualizerUniform, LightUniform, ObjectUniform},
};
use crate::{
    color_utils::{self, ToVec4},
    material::{
        diffuse_color_material::{DiffuseColorMaterial, DiffuseColorUniforms},
        equalizer_material::EqualizerUniforms,
        wave_material::{WaveMaterial, WaveUniforms},
        MaterialType,
    },
    // rendering::temp_renderer::create_wave_material,
};
use glam::vec3;
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
        scene_data: &SceneData,
    ) -> Self {
        let camera = camera::Camera::new(
            vec3(0.0, 1.0, 10.0),
            vec3(0.0, 0.0, 0.0),
            size.width as f32 / size.height as f32,
            scene_data.camera.fov,
            0.1,
            1000.0,
        );

        let mut material_object_map = HashMap::new();
        let mut objects: Vec<Box<dyn Primitive>> = vec![];
        for object_data in &scene_data.objects {
            let material = DiffuseColorMaterial::new(device, surface_config);
            let mut object: Box<dyn Primitive> = if object_data.mesh == "cube" {
                Box::new(Cube::new(device, Box::new(material)))
            } else if object_data.mesh == "sphere" {
                Box::new(Sphere::new(device, Box::new(material)))
            } else {
                Box::new(Quad::new(device, Box::new(material)))
            };
            object.transform().set_position(object_data.position.into());
            object.transform().set_rotation(object_data.rotation.into());
            object.transform().set_scale(object_data.scale.into());

            objects.push(object);
        }
        material_object_map.insert(MaterialType::DiffuseColorMaterial, objects);
        // for i in 0..5 {
        //     let material = WaveMaterial::new(device, surface_config);
        //     let mut quad = Quad::new(device, Box::new(material));
        //     quad.state.set_position(Vec3 {
        //         x: -4.0 + i as f32 * 2.0,
        //         y: 2.5,
        //         z: 0.0,
        //     });
        //     quad.state.scale(Vec3 {
        //         x: 2.0,
        //         y: 5.0,
        //         z: 1.0,
        //     });
        //     objects.push(Box::new(quad));
        // }
        // material_object_map.insert(MaterialType::WaveMaterial, objects);

        // floor and ceiling
        // objects = vec![];
        // let material = DiffuseColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: 0.0,
        //     y: 0.0,
        //     z: 0.0,
        // });
        // quad.state.rotate(Vec3 {
        //     x: -90.0,
        //     y: 0.0,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 100.0,
        //     y: 100.0,
        //     z: 100.0,
        // });
        // objects.push(Box::new(quad));

        // let material = DiffuseColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: 0.0,
        //     y: 5.0,
        //     z: 0.0,
        // });
        // quad.state.rotate(Vec3 {
        //     x: 90.0,
        //     y: 0.0,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 100.0,
        //     y: 100.0,
        //     z: 100.0,
        // });
        // objects.push(Box::new(quad));

        // let material = DiffuseColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: 5.0,
        //     y: 2.5,
        //     z: 0.0,
        // });
        // quad.state.rotate(Vec3 {
        //     x: 0.0,
        //     y: -90.0,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 100.0,
        //     y: 5.0,
        //     z: 100.0,
        // });
        // objects.push(Box::new(quad));

        // let material = DiffuseColorMaterial::new(device, surface_config);
        // let mut quad = Quad::new(device, Box::new(material));
        // quad.state.set_position(Vec3 {
        //     x: -5.0,
        //     y: 2.5,
        //     z: 0.0,
        // });
        // quad.state.rotate(Vec3 {
        //     x: 0.0,
        //     y: 90.0,
        //     z: 0.0,
        // });
        // quad.state.scale(Vec3 {
        //     x: 100.0,
        //     y: 5.0,
        //     z: 100.0,
        // });
        // objects.push(Box::new(quad));

        // material_object_map.insert(MaterialType::DiffuseColorMaterial, objects);

        let mut light = Light::new(color_utils::CCP.palette[1]);
        light.update_position(vec3(0.0, 5.0, 0.0));
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
        self.elapsed += delta_time;
        let el = self.elapsed * 0.5;
        // self.lights[0].update_position(vec3(5.0 * el.cos(), 0.0, 5.0 * el.sin()));

        // self.camera
        //     .update_position(vec3(5.0 * elapsed.cos(), 0.0, 5.0 * elapsed.sin()));
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
                        color: self.lights[0].color.to_vec4(0.5 + signal * 0.5),
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
                    let color1 = ColorUniform {
                        color: color_utils::CCP.palette[1].to_vec4(1.0),
                    };
                    let color2 = ColorUniform {
                        color: color_utils::CCP.palette[2].to_vec4(1.0),
                    };
                    let data = WaveUniforms {
                        object,
                        color1,
                        color2,
                        wave: Arc::clone(&wave),
                    };
                    primitive.material().update(queue, &data);
                }
            }
        }
    }
}
