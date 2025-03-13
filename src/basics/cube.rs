use super::{
    core::Vertex,
    material::Material,
    primitive::{Primitive, PrimitiveState},
};
use glam::{vec3, Mat3, Mat4, Quat};
use wgpu::{Device, RenderPass};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // Front face (z = 0.5)
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.3,  0.7,  0.1], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.3,  0.7,  0.3], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.3,  0.7,  0.2], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.3,  0.7,  0.7], normal: [ 0.0,  0.0,  1.0],
    uv: [0.0, 1.0] },
    // Back face (z = -0.5)
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 1.0] },
    // Right face (x = 0.5)
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    // Left face (x = -0.5)
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [-1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [-1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [-1.0,  0.0,  0.0], uv: [0.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [-1.0,  0.0,  0.0], uv: [0.0, 0.0] },
    // Top face (y = 0.5)
    Vertex { position: [-0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 0.0] },
    // Bottom face (y = -0.5)
    Vertex { position: [-0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [ 0.3,  0.3,  0.3], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 1.0] },
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
    pub fn new(device: &Device, material: Material) -> Self {
        Self {
            state: PrimitiveState::new(device, VERTICES, INDICES, material),
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
        self.state.set_position(vec3(0.5, 0.1, -1.9));
        let rotation_x = Quat::from_rotation_x(delta_time * 0.3);
        let rotation_y = Quat::from_rotation_y(delta_time * 0.2);
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

    fn normal_matrix(&self) -> Mat3 {
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
