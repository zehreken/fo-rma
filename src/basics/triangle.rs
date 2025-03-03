use super::{
    core::Vertex,
    material::Material,
    primitive::{Primitive, PrimitiveState},
};
use crate::{renderer::Renderer, utils};
use wgpu::RenderPass;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5,  0.0], color: utils::CCP.palette[1], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0] }, // Top
    Vertex { position: [-0.5, -0.5,  0.0], color: utils::CCP.palette[2], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5,  0.0], color: utils::CCP.palette[3], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] }, // Bottom-right
];

// Double faced triangle
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 1];

pub struct Triangle {
    state: PrimitiveState,
}

impl Triangle {
    pub fn new(renderer: &Renderer, material: Material) -> Self {
        Self {
            state: PrimitiveState::new(renderer, VERTICES, INDICES, material),
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
        self.state.model_matrix.to_cols_array_2d()
    }

    fn normal_matrix(&self) -> glam::Mat3 {
        self.state.normal_matrix
    }

    fn transform(&mut self) -> &mut super::core::Transform {
        &mut self.state.transform
    }

    fn material(&self) -> &super::material::Material {
        &self.state.material
    }

    fn material_mut(&mut self) -> &mut super::material::Material {
        &mut self.state.material
    }
}
