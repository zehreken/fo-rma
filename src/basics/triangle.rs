use super::core::{Transform, Vertex};
use glam::{EulerRot, Mat4, Quat};
use wgpu::{util::DeviceExt, Device, RenderPass};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5,  0.0], color: [ 0.0,  1.0,  1.0] },
    Vertex { position: [ -0.5, -0.5,  0.0], color: [ 0.0,  1.0,  1.0] },
    Vertex { position: [ 0.5,  -0.5,  0.0], color: [ 0.0,  1.0,  1.0] },
];

// Double faced triangle
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 1];

pub struct State {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    transform: Transform,
    pub model_matrix: [[f32; 4]; 4],
}

impl State {
    pub fn new(device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            transform: Transform::new(),
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn update(&mut self, delta_time: f32) {
        let mut rotation = self.transform.rotation.to_euler(glam::EulerRot::XYZ);
        rotation.0 += delta_time * 2.0;
        self.transform.rotation =
            Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);

        self.model_matrix = Mat4::from_scale_rotation_translation(
            self.transform.scale,
            self.transform.rotation,
            self.transform.position,
        )
        .to_cols_array_2d();
    }

    pub fn rotate(&mut self) {}

    pub fn resize(&mut self, size: PhysicalSize<u32>) {}
}
