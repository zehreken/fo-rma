use super::core::Vertex;
use glam::{Mat4, Vec3};
use wgpu::{util::DeviceExt, Device, RenderPass};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] }, // Bottom-right
    Vertex { position: [ 0.5,  0.5, 0.0], color: [0.0, 0.0, 1.0] }, // Top-right
    Vertex { position: [-0.5,  0.5, 0.0], color: [1.0, 1.0, 1.0] }, // Top-left
];

const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

pub struct State {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    position: Vec3,
    model_matrix: [[f32; 4]; 4],
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
            position: Vec3::ZERO,
            model_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn update(&mut self) {
        self.position.z += 0.01;
        self.model_matrix = Mat4::from_translation(self.position).to_cols_array_2d();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {}
}
