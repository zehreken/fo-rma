use crate::{
    basics::core::Vertex,
    material::MaterialTrait,
    primitives::primitive::{Primitive, PrimitiveState},
};
use wgpu::Device;

pub struct Mesh {
    state: PrimitiveState,
}

impl Mesh {
    pub fn new(
        device: &Device,
        material: Box<dyn MaterialTrait>,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        Self {
            state: PrimitiveState::new(device, vertices, indices, material),
        }
    }
}

impl Primitive for Mesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
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

    fn transform(&mut self) -> &mut crate::basics::core::Transform {
        &mut self.state.transform
    }

    fn material(&self) -> &dyn MaterialTrait {
        self.state.material.as_ref()
    }

    fn material_mut(&mut self) -> &mut dyn MaterialTrait {
        self.state.material.as_mut()
    }
}
