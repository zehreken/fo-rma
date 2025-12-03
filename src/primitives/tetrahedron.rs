use crate::{
    basics::core::Vertex,
    color_utils,
    material::MaterialTrait,
    primitives::primitive::{Primitive, PrimitiveState},
};
use glam::{Mat3, Mat4, Quat};
use wgpu::{Device, RenderPass};

const X: f32 = 1.0;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // Bottom
    Vertex { position: [ X,  X, -X], color: color_utils::CP0.palette[1], normal: [0.577, 0.577, 0.577], uv: [0.5, 1.0] },
    Vertex { position: [ X, -X,  X], color: color_utils::CP0.palette[3], normal: [0.577, 0.577, 0.577], uv: [0.0, 0.0] },
    Vertex { position: [-X,  X,  X], color: color_utils::CP0.palette[2], normal: [0.577, 0.577, 0.577], uv: [1.0, 0.0] },
    // Front
    Vertex { position: [-X,  X,  X], color: color_utils::CP0.palette[1], normal: [0.577, -0.577, 0.577], uv: [0.5, 1.0] },
    Vertex { position: [-X, -X, -X], color: color_utils::CP0.palette[2], normal: [0.577, -0.577, 0.577], uv: [0.0, 0.0] },
    Vertex { position: [ X,  X, -X], color: color_utils::CP0.palette[3], normal: [0.577, -0.577, 0.577], uv: [1.0, 0.0] },
    // Left
    Vertex { position: [-X,  X,  X], color: color_utils::CP0.palette[1], normal: [-0.577, -0.577, 0.577], uv: [0.5, 1.0] },
    Vertex { position: [ X, -X,  X], color: color_utils::CP0.palette[1], normal: [-0.577, -0.577, 0.577], uv: [0.0, 0.0] },
    Vertex { position: [-X, -X, -X], color: color_utils::CP0.palette[2], normal: [-0.577, -0.577, 0.577], uv: [1.0, 0.0] },
    // Right
    Vertex { position: [ X,  X, -X], color: color_utils::CP0.palette[1], normal: [0.577, -0.577, -0.577], uv: [0.5, 1.0] },
    Vertex { position: [-X, -X, -X], color: color_utils::CP0.palette[3], normal: [0.577, -0.577, -0.577], uv: [0.0, 0.0] },
    Vertex { position: [ X, -X,  X], color: color_utils::CP0.palette[1], normal: [0.577, -0.577, -0.577], uv: [1.0, 0.0] },
];

const INDICES: &[u16] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

pub struct Tetrahedron {
    pub state: PrimitiveState,
}

impl Tetrahedron {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES, material),
        }
    }
}

impl Primitive for Tetrahedron {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
        self.state.update(delta_time);
        let rotation_x = Quat::from_rotation_x(delta_time * 0.6);
        let rotation_y = Quat::from_rotation_y(delta_time * -0.2);
        let rotation_z = Quat::from_rotation_z(delta_time * 0.1);

        self.state.transform.rotation =
            self.state.transform.rotation * rotation_x * rotation_y * rotation_z;

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
