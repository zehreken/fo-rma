use crate::{
    basics::{
        core::Vertex,
        primitive::{Primitive, PrimitiveState},
    },
    material::MaterialTrait,
};
use glam::Mat4;
use std::f32::consts::PI;
use wgpu::Device;

const RADIUS: f32 = 0.5;
const SECTOR_COUNT: usize = 36;
const VERTEX_COUNT: usize = (SECTOR_COUNT + 1) * 2 + SECTOR_COUNT * 2;

pub struct Cylinder {
    pub state: PrimitiveState,
}

impl Cylinder {
    pub fn new(device: &Device, material: Box<dyn MaterialTrait>, sector_count: usize) -> Self {
        // let (vertices, indices) = calculate_vertices_and_indices();
        let (vertices, indices) = calculate_vertices_and_indices_dynamic(sector_count);
        Self {
            state: PrimitiveState::new(device, &vertices, &indices, material),
        }
    }
}

impl Primitive for Cylinder {
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

        self.state.normal_matrix =
            glam::Mat3::from_mat4(self.state.model_matrix.inverse().transpose());
        // self.state.update(delta_time);
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

fn calculate_vertices_and_indices_dynamic(sector_count: usize) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    // Top face
    for i in 0..sector_count {
        let sector_angle = i as f32 * 2.0 * PI / sector_count as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let s = x + 0.5;
        let t = z + 0.5;

        vertices.push(Vertex {
            position: [x, 0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: [0.0, 1.0, 0.0],
            uv: [s, t],
        });
    }
    vertices.push(Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.1, 0.1, 0.1],
        normal: [0.0, 1.0, 0.0],
        uv: [0.5, 0.5],
    }); // Add center vertex last, index = SECTOR_COUNT

    // Bottom face
    for i in 0..sector_count {
        let sector_angle = i as f32 * 2.0 * PI / sector_count as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let s = x + 0.5;
        let t = z + 0.5;

        vertices.push(Vertex {
            position: [x, -0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: [0.0, -1.0, 0.0],
            uv: [s, t],
        });
    }
    vertices.push(Vertex {
        position: [0.0, -0.5, 0.0],
        color: [0.1, 0.1, 0.1],
        normal: [0.0, -1.0, 0.0],
        uv: [0.5, 0.5],
    }); // Add center vertex last, index = SECTOR_COUNT

    // Side face vertices
    for i in 0..sector_count {
        let sector_angle = i as f32 * 2.0 * PI / sector_count as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let side_face_normal = [sector_angle.cos(), 0.0, sector_angle.sin()];
        vertices.push(Vertex {
            position: [x, 0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: side_face_normal,
            uv: [i as f32 / sector_count as f32, 1.0],
        });
    }
    for i in 0..sector_count {
        let sector_angle = i as f32 * 2.0 * PI / sector_count as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let side_face_normal = [sector_angle.cos(), 0.0, sector_angle.sin()];
        vertices.push(Vertex {
            position: [x, -0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: side_face_normal,
            uv: [i as f32 / sector_count as f32, 0.0],
        });
    }

    // Top face
    let mut indices = Vec::new();
    for i in 0..sector_count {
        indices.push(i);
        indices.push((i + 1) % sector_count);
        indices.push(sector_count);
    }

    // Bottom face
    for i in 0..sector_count {
        let offset = sector_count + 1;
        indices.push((i + 1) % sector_count + offset);
        indices.push(i + offset);
        indices.push(sector_count + offset);
    }

    // Side faces
    for i in 0..sector_count {
        let offset = 2 * (sector_count + 1);
        let k1 = (i + 1) % sector_count + offset;
        let k2 = i + offset;
        let k3 = (i + 1) % sector_count + offset + sector_count;
        indices.push(k1);
        indices.push(k2);
        indices.push(k3);

        let k1 = i + offset + sector_count;
        let k2 = (i + 1) % sector_count + offset + sector_count;
        let k3 = i + offset;
        indices.push(k1);
        indices.push(k2);
        indices.push(k3);
    }

    // let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    // for (i, vertex) in vertices.iter().enumerate() {
    //     vertices_array[i] = *vertex;
    // }

    // let mut indices_array: [u16; SECTOR_COUNT * 4 * 3] = [0; SECTOR_COUNT * 4 * 3];
    // for (i, index) in indices.iter().enumerate() {
    //     indices_array[i] = *index as u16;
    // }

    let indices = indices.iter().map(|u| *u as u16).collect();
    (vertices, indices)
}

fn calculate_vertices_and_indices() -> ([Vertex; VERTEX_COUNT], [u16; SECTOR_COUNT * 4 * 3]) {
    let mut vertices = Vec::new();
    // Top face
    for i in 0..SECTOR_COUNT {
        let sector_angle = i as f32 * 2.0 * PI / SECTOR_COUNT as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let s = x + 0.5;
        let t = z + 0.5;

        vertices.push(Vertex {
            position: [x, 0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: [0.0, 1.0, 0.0],
            uv: [s, t],
        });
    }
    vertices.push(Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.1, 0.1, 0.1],
        normal: [0.0, 1.0, 0.0],
        uv: [0.5, 0.5],
    }); // Add center vertex last, index = SECTOR_COUNT

    // Bottom face
    for i in 0..SECTOR_COUNT {
        let sector_angle = i as f32 * 2.0 * PI / SECTOR_COUNT as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let s = x + 0.5;
        let t = z + 0.5;

        vertices.push(Vertex {
            position: [x, -0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: [0.0, -1.0, 0.0],
            uv: [s, t],
        });
    }
    vertices.push(Vertex {
        position: [0.0, -0.5, 0.0],
        color: [0.1, 0.1, 0.1],
        normal: [0.0, -1.0, 0.0],
        uv: [0.5, 0.5],
    }); // Add center vertex last, index = SECTOR_COUNT

    // Side face vertices
    for i in 0..SECTOR_COUNT {
        let sector_angle = i as f32 * 2.0 * PI / SECTOR_COUNT as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let side_face_normal = [sector_angle.cos(), 0.0, sector_angle.sin()];
        vertices.push(Vertex {
            position: [x, 0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: side_face_normal,
            uv: [i as f32 / SECTOR_COUNT as f32, 1.0],
        });
    }
    for i in 0..SECTOR_COUNT {
        let sector_angle = i as f32 * 2.0 * PI / SECTOR_COUNT as f32;
        let x = RADIUS * sector_angle.cos();
        let z = RADIUS * sector_angle.sin();

        let side_face_normal = [sector_angle.cos(), 0.0, sector_angle.sin()];
        vertices.push(Vertex {
            position: [x, -0.5, z],
            color: [0.1, 0.1, 0.1],
            normal: side_face_normal,
            uv: [i as f32 / SECTOR_COUNT as f32, 0.0],
        });
    }

    // Top face
    let mut indices = Vec::new();
    for i in 0..SECTOR_COUNT {
        indices.push(i);
        indices.push((i + 1) % SECTOR_COUNT);
        indices.push(SECTOR_COUNT);
    }

    // Bottom face
    for i in 0..SECTOR_COUNT {
        let offset = SECTOR_COUNT + 1;
        indices.push((i + 1) % SECTOR_COUNT + offset);
        indices.push(i + offset);
        indices.push(SECTOR_COUNT + offset);
    }

    // Side faces
    for i in 0..SECTOR_COUNT {
        let offset = 2 * (SECTOR_COUNT + 1);
        let k1 = (i + 1) % SECTOR_COUNT + offset;
        let k2 = i + offset;
        let k3 = (i + 1) % SECTOR_COUNT + offset + SECTOR_COUNT;
        indices.push(k1);
        indices.push(k2);
        indices.push(k3);

        let k1 = i + offset + SECTOR_COUNT;
        let k2 = (i + 1) % SECTOR_COUNT + offset + SECTOR_COUNT;
        let k3 = i + offset;
        indices.push(k1);
        indices.push(k2);
        indices.push(k3);
    }

    let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    for (i, vertex) in vertices.iter().enumerate() {
        vertices_array[i] = *vertex;
    }

    let mut indices_array: [u16; SECTOR_COUNT * 4 * 3] = [0; SECTOR_COUNT * 4 * 3];
    for (i, index) in indices.iter().enumerate() {
        indices_array[i] = *index as u16;
    }

    (vertices_array, indices_array)
}
