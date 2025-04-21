use super::{
    core::Vertex,
    primitive::{Primitive, PrimitiveState},
};
use crate::material::MaterialTrait;
use glam::{EulerRot, Mat3, Mat4, Quat};
use std::f32::consts::PI;
use wgpu::Device;

const RADIUS: f32 = 0.5;
const STACK_COUNT: usize = 8;
const STACK_STEP: f32 = PI / STACK_COUNT as f32;
const SECTOR_COUNT: usize = 12;
const SECTOR_STEP: f32 = 2_f32 * PI / SECTOR_COUNT as f32;
const VERTEX_COUNT: usize = (STACK_COUNT + 1) * (SECTOR_COUNT + 1);

pub struct Sphere {
    pub state: PrimitiveState,
}

impl Sphere {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>) -> Self {
        let (vertices, indices) = calculate_vertices_and_indices();
        Self {
            state: PrimitiveState::new(device, &vertices, &indices, material),
        }
    }
}

impl Primitive for Sphere {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.state.num_indices, 0, 0..1);
    }

    fn update(&mut self, delta_time: f32) {
        let mut rotation = self.state.transform.rotation.to_euler(glam::EulerRot::XYZ);
        rotation.0 = PI / 2.0;
        rotation.2 += delta_time * 0.1;
        self.state.transform.rotation =
            Quat::from_euler(EulerRot::XYZ, rotation.0, rotation.1, rotation.2);

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

fn calculate_vertices_and_indices() -> ([Vertex; VERTEX_COUNT], [u16; VERTEX_COUNT * 6]) {
    let mut vertices = Vec::new();

    for i in 0..=STACK_COUNT {
        let stack_angle = PI / 2_f32 - i as f32 * STACK_STEP; // From PI/2 to -PI/2
        let xy = RADIUS * stack_angle.cos();
        let z = RADIUS * stack_angle.sin();

        for j in 0..=SECTOR_COUNT {
            let sector_angle = j as f32 * SECTOR_STEP; // From 0 to 2PI

            // Vertex position (x, y, z)
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            let s = j as f32 / SECTOR_COUNT as f32;
            let t = i as f32 / STACK_COUNT as f32;
            vertices.push(Vertex {
                position: [x, y, z],
                color: [0.1, 0.1, 0.1],
                normal: [x / RADIUS, y / RADIUS, z / RADIUS],
                uv: [s, t],
            });
        }
    }

    let mut indices = Vec::new();
    for i in 0..STACK_COUNT {
        let mut k1: u16 = i as u16 * (SECTOR_COUNT as u16 + 1);
        let mut k2: u16 = k1 + SECTOR_COUNT as u16 + 1;

        for _ in 0..SECTOR_COUNT {
            if i != 0 {
                indices.push(k1);
                indices.push(k1 + 1);
                indices.push(k2);
            }

            if i != (STACK_COUNT - 1) {
                indices.push(k1 + 1);
                indices.push(k2 + 1);
                indices.push(k2);
            }

            k1 += 1;
            k2 += 1;
        }
    }

    let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    for (i, vertex) in vertices.iter().enumerate() {
        vertices_array[i] = *vertex;
    }

    let mut indices_array: [u16; VERTEX_COUNT * 6] = [0; VERTEX_COUNT * 6];
    for (i, index) in indices.iter().enumerate() {
        indices_array[i] = *index;
    }

    (vertices_array, indices_array)
}
