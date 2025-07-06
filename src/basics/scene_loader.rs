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

pub fn construct_scene_from_object_data(objects: Vec<Object>) -> SceneData {
    let camera = Camera {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        fov: 60.0,
    };
    let light = Light {
        color: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        intensity: 1.0,
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        scale: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    SceneData {
        camera,
        lights: vec![light],
        objects,
    }
}
