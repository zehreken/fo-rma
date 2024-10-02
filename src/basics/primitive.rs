use glam::{EulerRot, Mat3, Mat4, Quat};
use wgpu::{util::DeviceExt, Device, RenderPass};

use super::core::{Transform, Vertex};

pub trait Primitive {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>);
    fn update(&mut self, delta_time: f32);
    fn model_matrix(&self) -> [[f32; 4]; 4];
    fn normal_matrix(&self) -> Mat3;
}

pub struct PrimitiveState {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub transform: Transform,
    pub model_matrix: Mat4,
    pub normal_matrix: Mat3,
}

impl PrimitiveState {
    pub fn new(device: &Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            transform: Transform::new(),
            model_matrix: Mat4::IDENTITY,
            normal_matrix: Mat3::IDENTITY,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let mut rotation = self.transform.rotation.to_euler(glam::EulerRot::XYZ);
        rotation.0 += delta_time * 0.001;
        self.transform.rotation =
            Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);

        self.model_matrix = Mat4::from_scale_rotation_translation(
            self.transform.scale,
            self.transform.rotation,
            self.transform.position,
        );

        self.normal_matrix = Mat3::from_mat4(self.model_matrix.inverse().transpose());
    }

    pub fn set_position(&mut self) {
        todo!()
    }

    pub fn set_rotation(&mut self) {
        todo!()
    }

    pub fn set_scale(&mut self) {
        todo!()
    }
}
