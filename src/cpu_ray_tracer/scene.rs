use super::super::shapes::hitable::Hitable;
use super::{camera::*, scenes};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hitable>>,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            camera: Camera::new(width, height),
            objects: scenes::get_simple_scene(),
        }
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
