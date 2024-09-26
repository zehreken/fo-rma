use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use wgpu::{Device, RenderPass};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5,  0.0], color: [ 1.0, 0.0, 0.0] },
    Vertex { position: [ -0.5, -0.5,  0.0], color: [ 1.0, 0.0, 0.0] },
    Vertex { position: [ 0.5,  -0.5,  0.0], color: [ 1.0, 0.0, 0.0] },
];

// Double faced triangle
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 1];

pub struct Triangle {
    state: PrimitiveState,
}

impl Triangle {
    pub fn new(device: &Device) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES),
        }
    }
}

impl Primitive for Triangle {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
        self.state.update(delta_time);
    }

    fn model_matrix(&self) -> [[f32; 4]; 4] {
        self.state.model_matrix
    }
}
