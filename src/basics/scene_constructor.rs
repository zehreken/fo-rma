use serde::{Deserialize, Serialize};

pub fn construct_scene_from_json(json: &str) -> Scene {
    let deserialized: Scene = serde_json::from_str(json).unwrap();

    deserialized
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    camera: Camera,
    lights: Vec<Light>,
    objects: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Camera {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
    fov: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Light {
    color: Vec3,
    intensity: f32,
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

#[derive(Debug, Serialize, Deserialize)]
struct Object {
    mesh: String,
    material: String,
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
