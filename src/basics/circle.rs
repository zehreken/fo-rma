use crate::{
    basics::{
        core::Vertex,
        primitive::{Primitive, PrimitiveState},
    },
    material::MaterialTrait,
};
use glam::{Mat3, Mat4};
use std::f32::consts::PI;
use wgpu::Device;

const RADIUS: f32 = 0.5;
const SECTOR_COUNT: usize = 36; // Sector is like a slice of pizza
const VERTEX_COUNT: usize = SECTOR_COUNT + 1; // Plus is the center vertex

pub struct Circle {
    pub state: PrimitiveState,
}

impl Circle {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>) -> Self {
        let (vertices, indices) = calculate_vertices_and_indices();
        Self {
            state: PrimitiveState::new(device, &vertices, &indices, material),
        }
    }
}

impl Primitive for Circle {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
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

fn calculate_vertices_and_indices() -> ([Vertex; VERTEX_COUNT], [u16; SECTOR_COUNT * 3]) {
    let mut vertices = Vec::new();
    for i in 0..SECTOR_COUNT {
        let sector_angle = i as f32 * 2.0 * PI / SECTOR_COUNT as f32;
        let x = RADIUS * sector_angle.cos();
        let y = RADIUS * sector_angle.sin();

        let s = x + 0.5;
        let t = y + 0.5;

        vertices.push(Vertex {
            position: [x, y, 0.0],
            color: [0.1, 0.1, 0.1],
            normal: [0.0, 0.0, -1.0],
            uv: [s, t],
        });
    }
    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        color: [0.1, 0.1, 0.1],
        normal: [0.0, 0.0, -1.0],
        uv: [0.5, 0.5],
    }); // Add center vertex last, index = SECTOR_COUNT

    let mut indices = Vec::new();
    for i in 0..SECTOR_COUNT {
        indices.push(i);
        indices.push((i + 1) % SECTOR_COUNT);
        indices.push(SECTOR_COUNT);
    }

    let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    for (i, vertex) in vertices.iter().enumerate() {
        vertices_array[i] = *vertex;
    }

    let mut indices_array: [u16; SECTOR_COUNT * 3] = [0; SECTOR_COUNT * 3];
    for (i, index) in indices.iter().enumerate() {
        indices_array[i] = *index as u16;
    }

    (vertices_array, indices_array)
}
