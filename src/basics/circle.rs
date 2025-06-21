use crate::{
    basics::{core::Vertex, primitive::PrimitiveState},
    material::MaterialTrait,
};
use wgpu::Device;

const RADIUS: f32 = 0.5;
const VERTEX_COUNT: usize = 10;

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

fn calculate_vertices_and_indices() -> ([Vertex; VERTEX_COUNT], [u16; VERTEX_COUNT]) {
    let mut vertices = Vec::new();
    vertices.push(Vertex::default());
    let mut indices = Vec::new();
    indices.push(0);

    let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    let mut indices_array: [u16; VERTEX_COUNT] = [0; VERTEX_COUNT];
    (vertices_array, indices_array)
}
