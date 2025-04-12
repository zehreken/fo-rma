use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use crate::{color_utils, material::MaterialTrait};
use wgpu::{Device, RenderPass};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5,  0.0], color: color_utils::CCP.palette[1], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0] }, // Top
    Vertex { position: [-0.5, -0.5,  0.0], color: color_utils::CCP.palette[2], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5,  0.0], color: color_utils::CCP.palette[3], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] }, // Bottom-right
];

// Double faced triangle
const INDICES: &[u16] = &[0, 1, 2, 0, 2, 1];

pub struct Triangle {
    state: PrimitiveState,
}

impl Triangle {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES, material),
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

    fn material(&self) -> &dyn MaterialTrait {
        self.state.material.as_ref()
    }

    fn material_mut(&mut self) -> &mut dyn MaterialTrait {
        self.state.material.as_mut()
    }
}
