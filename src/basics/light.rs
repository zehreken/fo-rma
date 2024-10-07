use glam::Vec3;
use wgpu::Device;

use super::{core::Transform, cube::Cube, primitive::Primitive};

pub struct Light {
    pub transform: Transform,
    pub color: [f32; 3],
    pub intensity: f32,
}

impl Light {
    pub fn new(device: &Device, color: [f32; 3]) -> Self {
        Self {
            transform: Transform::new(),
            color,
            intensity: 1.0,
        }
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.transform.position = position;
    }
}
