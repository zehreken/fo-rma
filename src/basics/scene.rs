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
    basics::{circle::Circle, cylinder::Cylinder},
    color_utils::{self, ColorPalette, ToVec4},
    material::{
        diffuse_color_material::{DiffuseColorMaterial, DiffuseColorUniforms},
        equalizer_material::{EqualizerMaterial, EqualizerUniforms},
        texture_material::{TextureMaterial, TextureUniforms},
        unlit_color_material::UnlitColorMaterial,
        wave_material::{WaveMaterial, WaveUniforms},
        Material, MaterialTrait,
    },
};
use glam::vec3;
use std::{collections::HashMap, sync::Arc};
use wgpu::{Device, Queue, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct Scene {
    pub camera: Camera,
    pub material_object_map: HashMap<Material, Vec<Box<dyn Primitive>>>,
    pub lights: Vec<Light>,
    elapsed: f32,
}

impl Scene {
    pub fn new(
        device: &Device,
        queue: &Queue,
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

        let mut material_object_map: HashMap<Material, Vec<Box<dyn Primitive>>> = HashMap::new();
        for object_data in &scene_data.objects {
            let material_type: Material;
            let material: Box<dyn MaterialTrait> = if object_data.material == "DiffuseColorMaterial"
            {
                material_type = Material::DiffuseColor;
                Box::new(DiffuseColorMaterial::new(device, surface_config))
            } else if object_data.material == "EqualizerMaterial" {
                material_type = Material::Equalizer;
                Box::new(EqualizerMaterial::new(device, surface_config))
            } else if object_data.material == "UnlitColorMaterial" {
                material_type = Material::UnlitColor;
                Box::new(UnlitColorMaterial::new(device, surface_config))
            } else if object_data.material == "WaveMaterial" {
                material_type = Material::Wave;
                Box::new(WaveMaterial::new(device, surface_config))
            } else if object_data.material == "Texture" {
                material_type = Material::Texture;
                Box::new(TextureMaterial::new(device, queue, surface_config))
            } else if object_data.material == "DiffuseTexture" {
                todo!()
            } else {
                material_type = Material::DiffuseColor;
                Box::new(DiffuseColorMaterial::new(device, surface_config))
            };
            let mut object: Box<dyn Primitive> = if object_data.mesh == "cube" {
                Box::new(Cube::new(device, material))
            } else if object_data.mesh == "sphere" {
                Box::new(Sphere::new(device, material))
            } else if object_data.mesh == "triangle" {
                Box::new(Triangle::new(device, material))
            } else if object_data.mesh == "circle" {
                Box::new(Circle::new(device, material))
            } else if object_data.mesh == "cylinder" {
                Box::new(Cylinder::new(device, material))
            } else {
                Box::new(Quad::new(device, material))
            };
            object.transform().set_position(object_data.position.into());
            object.transform().set_rotation(object_data.rotation.into());
            object.transform().set_scale(object_data.scale.into());

            // objects.push(object);
            if material_object_map.contains_key(&material_type) {
                material_object_map
                    .get_mut(&material_type)
                    .unwrap()
                    .push(object);
            } else {
                material_object_map.insert(material_type, vec![object]);
            }
        }
        // material_object_map.insert(MaterialType::WaveMaterial, objects);

        let mut light = Light::new(color_utils::CP0.palette[1]);
        light.set_position(vec3(0.0, 10.0, 0.0));
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
        on_beat: bool,
        wave: Arc<Vec<f32>>,
        color_palette: &ColorPalette<f32, 4>,
    ) {
        self.elapsed += delta_time;
        let el = self.elapsed * 0.5;
        self.lights[0].set_position(vec3(20.0 * el.cos(), 5.0, 20.0 * el.sin()));

        // self.camera.set_position(vec3(
        //     8.0 * self.elapsed.cos(),
        //     5.0,
        //     8.0 * self.elapsed.sin(),
        // ));
        for (material_id, objects) in &mut self.material_object_map {
            if *material_id == Material::Equalizer {
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
                        color1: color_palette.palette[0].to_vec4(1.0),
                        color2: color_palette.palette[1].to_vec4(1.0),
                        color3: color_palette.palette[2].to_vec4(1.0),
                        signal: signal * 5.0,
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
            } else if *material_id == Material::DiffuseColor {
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
                        color: color_palette.palette[0].to_vec4(1.0),
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
            } else if *material_id == Material::Wave {
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
                        color: color_palette.palette[1].to_vec4(1.0),
                    };
                    let color2 = ColorUniform {
                        color: color_palette.palette[2].to_vec4(1.0),
                    };
                    let data = WaveUniforms {
                        object,
                        color1,
                        color2,
                        wave: Arc::clone(&wave),
                    };
                    primitive.material().update(queue, &data);
                }
            } else if *material_id == Material::Texture {
                for primitive in objects {
                    primitive.update(delta_time);
                    let object = ObjectUniform {
                        view_proj: self.camera.build_view_projection_matrix(),
                        model: primitive.model_matrix(),
                        normal1: primitive.normal_matrix().x_axis.extend(0.0).to_array(),
                        normal2: primitive.normal_matrix().y_axis.extend(0.0).to_array(),
                        normal3: primitive.normal_matrix().z_axis.extend(0.0).to_array(),
                    };
                    let data = TextureUniforms { object };
                    primitive.material().update(queue, &data);
                }
            }
        }
    }
}
