use super::core::Transform;
use glam::Vec3;

pub struct Light {
    pub transform: Transform,
    pub color: [f32; 3],
    pub intensity: f32,
    // pub debug_mesh: Sphere,
}

impl Light {
    pub fn new(color: [f32; 3]) -> Self {
        Self {
            transform: Transform::new(),
            color,
            intensity: 1.0,
            // debug_mesh: Sphere::new(renderer),
        }
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.transform.position = position;
        // self.debug_mesh.transform().position = position;
        // self.debug_mesh.update(0.1);
    }
}
