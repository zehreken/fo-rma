use super::core::{Transform, Vertex};
use crate::material::MaterialTrait;
use glam::{Mat3, Mat4, Quat, Vec3};
use wgpu::{util::DeviceExt, Device, RenderPass};

pub trait Primitive {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>);
    fn update(&mut self, delta_time: f32);
    fn model_matrix(&self) -> [[f32; 4]; 4];
    fn normal_matrix(&self) -> Mat3;
    fn transform(&mut self) -> &mut Transform;
    fn material(&self) -> &dyn MaterialTrait;
    fn material_mut(&mut self) -> &mut dyn MaterialTrait;
}

pub struct PrimitiveState {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub transform: Transform,
    pub model_matrix: Mat4,
    pub normal_matrix: Mat3,
    pub material: Box<dyn MaterialTrait>,
}

impl PrimitiveState {
    pub fn new(
        device: &Device,
        vertices: &[Vertex],
        indices: &[u16],
        material: Box<dyn MaterialTrait>,
    ) -> Self {
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
            material,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let rotation_x = Quat::from_rotation_x(delta_time * 0.3);
        let rotation_y = Quat::from_rotation_y(delta_time * 0.2);
        let rotation_z = Quat::from_rotation_z(delta_time * 0.1);

        self.transform.rotation = self.transform.rotation * rotation_x * rotation_y * rotation_z;

        self.model_matrix = Mat4::from_scale_rotation_translation(
            self.transform.scale,
            self.transform.rotation,
            self.transform.position,
        );

        self.normal_matrix = Mat3::from_mat4(self.model_matrix.inverse().transpose());
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.transform.set_position(position);
    }

    pub fn rotate(&mut self, euler_angles: Vec3) {
        self.transform.rotate(euler_angles);
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.transform.scale(scale);
    }
}
