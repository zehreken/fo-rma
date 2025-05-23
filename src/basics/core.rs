use glam::{Quat, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn default() -> Self {
        Self {
            position: [0.0; 3],
            color: [0.0; 3],
            normal: [0.0; 3],
            uv: [0.0; 2],
        }
    }
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    // Angles in radians
    pub fn rotate(&mut self, euler_angles: Vec3) {
        let rotation = Quat::from_euler(
            glam::EulerRot::XYZ,
            euler_angles.x.to_radians(),
            euler_angles.y.to_radians(),
            euler_angles.z.to_radians(),
        );

        self.rotation *= rotation;
    }

    pub fn set_rotation(&mut self, v: Quat) {
        self.rotation = v;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
