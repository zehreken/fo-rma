use serde::{Deserialize, Serialize};

pub fn construct_scene_from_json(json: &str) -> SceneData {
    let deserialized: SceneData = serde_json::from_str(json).unwrap();

    deserialized
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneData {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub color: Vec3,
    pub intensity: f32,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub mesh: String,
    pub material: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for glam::Vec3 {
    fn from(v: Vec3) -> Self {
        glam::Vec3::new(v.x, v.y, v.z)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Quat> for glam::Quat {
    fn from(v: Quat) -> Self {
        glam::quat(v.x, v.y, v.z, v.w)
    }
}
