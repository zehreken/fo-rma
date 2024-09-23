use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use glam::Mat4;
use wgpu::{Device, RenderPass};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  0.0,  1.0] },
    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 1.0,  0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 1.0,  0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 1.0,  0.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 1.0,  0.0, 1.0] },
    // Right face
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 1.0,  0.0,  0.0] },
    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], color: [1.0,  1.0,  0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [1.0,  1.0,  0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [1.0,  1.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [1.0,  1.0,  0.0] },
    // Top face
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.0,  1.0,  1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.0,  1.0,  1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.0,  1.0,  1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.0,  1.0,  1.0] },
    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.0, 1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.0, 1.0,  0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.0, 1.0,  0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.0, 1.0,  0.0] },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // front
    4, 6, 5, 6, 4, 7, // back
    8, 9, 10, 10, 11, 8, // right
    12, 14, 13, 14, 12, 15, // left
    16, 18, 17, 18, 16, 19, // top
    20, 21, 22, 22, 23, 20, // bottom
];

pub struct Cube {
    state: PrimitiveState,
}

impl Cube {
    pub fn new(device: &Device) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES),
        }
    }
}

impl Primitive for Cube {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
        // self.state.update(delta_time);
        let mut position = self.state.transform.position;
        position.x += delta_time * 0.1;
        self.state.transform.position = position;
        self.state.model_matrix = Mat4::from_scale_rotation_translation(
            self.state.transform.scale,
            self.state.transform.rotation,
            self.state.transform.position,
        )
        .to_cols_array_2d();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {}

    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.state.model_matrix
    }
}
