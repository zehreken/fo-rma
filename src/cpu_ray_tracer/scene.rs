use super::super::shapes::hitable::Hitable;
use super::camera::*;
use super::primitives::vec3::*;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hitable>>,
    pub width: u32,
    pub height: u32,
    pub colors: Vec<Vec3>, // remove, this does not belong to the scene
    pub pixels: Vec<u8>,   // remove, this does not belong to the scene
}
