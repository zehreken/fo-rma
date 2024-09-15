use super::core::Vertex;
use wgpu::{util::DeviceExt, Device, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.0,  0.0, -1.0] },
    // Right face
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [-1.0,  0.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [-1.0,  0.0,  0.0] },
    // Top face
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  1.0,  0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  1.0,  0.0] },
    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0, -1.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0, -1.0,  0.0] },
];

const INDICES: &[u16] = &[
    0, 2, 1, 2, 0, 3, // front
    4, 6, 5, 6, 4, 7, // back
    8, 10, 9, 10, 8, 11, // right
    12, 14, 13, 14, 12, 15, // left
    16, 18, 17, 18, 16, 19, // top
    20, 22, 21, 22, 20, 23, // bottom
];

pub struct State {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
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
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {}
}
