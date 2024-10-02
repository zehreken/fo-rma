use glam::Vec3;
use wgpu::Device;

use super::{core::Transform, cube::Cube, primitive::Primitive};

pub struct Light {
    pub transform: Transform,
    pub color: [f32; 3],
    pub intensity: f32,
    pub debug_mesh: Cube,
}

impl Light {
    pub fn new(device: &Device, color: [f32; 3]) -> Self {
        Self {
            transform: Transform::new(),
            color,
            intensity: 1.0,
            debug_mesh: Cube::new(device),
        }
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.transform.position = position;
    }
}
