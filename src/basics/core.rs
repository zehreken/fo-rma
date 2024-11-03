use glam::{Mat4, Quat, Vec3};
use wgpu::{BindGroup, Buffer, RenderPipeline};

pub struct PipelineData {
    pub render_pipeline: RenderPipeline,
    pub uniform_buffer: Buffer,
    pub uniform_bind_group: BindGroup,
    pub light_data: Option<LightData>,
}

pub struct LightData {
    pub uniform: LightUniform,
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn default() -> Self {
        Self {
            position: [0.0; 3],
            color: [0.0; 3],
            normal: [0.0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
    pub model: [[f32; 4]; 4],
    _padding: [f32; 3],
    pub signal: f32,
    pub normal1: [f32; 4],
    pub normal2: [f32; 4],
    pub normal3: [f32; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            model: Mat4::IDENTITY.to_cols_array_2d(),
            signal: 0.0,
            _padding: [0.0; 3],
            normal1: [0.0; 4],
            normal2: [0.0; 4],
            normal3: [0.0; 4],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 3],
    pub _padding2: f32,
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
            euler_angles.x,
            euler_angles.y,
            euler_angles.z,
        );

        self.rotation *= rotation;
    }

    pub fn set_rotation(&mut self, euler_angles: Vec3) {
        self.rotation = Quat::from_euler(
            glam::EulerRot::XYZ,
            euler_angles.x,
            euler_angles.y,
            euler_angles.z,
        );
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
