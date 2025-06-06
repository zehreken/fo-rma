use super::core::Vertex;
use wgpu::{util::DeviceExt, Device, RenderPass};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] }, // Bottom-right
    Vertex { position: [ 0.5,  0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0] }, // Top-right
    Vertex { position: [-0.5,  0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0] }, // Top-left
];

const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

pub struct ScreenQuad {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl ScreenQuad {
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
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
