use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use glam::{EulerRot, Mat4, Quat};
use wgpu::{Device, RenderPass};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0] }, // Bottom-right
    Vertex { position: [ 0.5,  0.5, 0.0], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0] }, // Top-right
    Vertex { position: [-0.5,  0.5, 0.0], color: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0] }, // Top-left
];

const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

pub struct Quad {
    state: PrimitiveState,
}

impl Quad {
    pub fn new(device: &Device) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES),
        }
    }
}

impl Primitive for Quad {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
        // self.state.update(delta_time);
        // let mut rotation = self.state.transform.rotation.to_euler(glam::EulerRot::XYZ);
        // rotation.2 += delta_time * 0.1;
        // self.state.transform.rotation =
        //     Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);

        // self.state.model_matrix = Mat4::from_scale_rotation_translation(
        //     self.state.transform.scale,
        //     self.state.transform.rotation,
        //     self.state.transform.position,
        // )
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
}
