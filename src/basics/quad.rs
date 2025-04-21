use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use crate::material::MaterialTrait;
use glam::{Mat3, Mat4};
use wgpu::{Device, RenderPass};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], uv: [0.0, 0.0] }, // Bottom-left
    Vertex { position: [ 0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], uv: [1.0, 0.0] }, // Bottom-right
    Vertex { position: [ 0.5,  0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], uv: [1.0, 1.0] }, // Top-right
    Vertex { position: [-0.5,  0.5, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], uv: [0.0, 1.0] }, // Top-left
];

const INDICES: &[u16] = &[
    0, 1, 2, // First triangle
    2, 3, 0, // Second triangle
];

pub struct Quad {
    pub state: PrimitiveState,
}

impl Quad {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES, material),
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
        // rotation.1 = 0.0;
        // self.state.transform.rotation =
        //     Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);

        // self.state.transform.scale = vec3(5.0, 5.0, 5.0);
        // self.state.transform.position = vec3(0.0, 0.0, -3.5);

        self.state.model_matrix = Mat4::from_scale_rotation_translation(
            self.state.transform.scale,
            self.state.transform.rotation,
            self.state.transform.position,
        );

        self.state.normal_matrix = Mat3::from_mat4(self.state.model_matrix.inverse().transpose());
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
