use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use std::collections::{HashMap, HashSet};

use crate::color_utils::{self, ColorPalette};

#[derive(Debug)]
pub struct Cell {
    id: u32,
    column: u32,
    row: u32,
    neighbors: Vec<u32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    pub a_id: u32,
    pub b_id: u32,
}

impl Edge {
    pub fn new(a: u32, b: u32) -> Self {
        let (a_id, b_id) = if a < b { (a, b) } else { (b, a) };

        Self { a_id, b_id }
    }
}

pub struct SaveTexture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

const BYTES_PER_PIXEL: u32 = 4;

const WIDTH: u32 = 3;
const HEIGHT: u32 = 300;
const PIXEL_PER_CELL: u32 = 30;
const OFFSET: u32 = 10;
const CELL_SIZE: u32 = 10;

#[test]
fn test() {
    let texture = generate_texture(WIDTH, HEIGHT, color_utils::CP0.into());

    let buffer: image::ImageBuffer<image::Rgba<u8>, _> =
        image::ImageBuffer::from_raw(texture.width, texture.height, texture.data).unwrap();
    let image_path = format!("out/basic-maze.png");
    buffer.save(&image_path).unwrap();
}

/// Generates a texture from a maze.
///
/// # Arguments
///
/// * `maze_width` - The width of the maze
/// * `maze_height` - The height of the maze
pub fn generate_texture(
    maze_width: u32,
    maze_height: u32,
    color_palette: ColorPalette<u8, 4>,
) -> SaveTexture {
    let (id_to_cell, edge_to_connected) = generate_maze(maze_width, maze_height);

    let width = maze_width * PIXEL_PER_CELL;
    let height = maze_height * PIXEL_PER_CELL;

    // data lengthis 4 * width * height, it is flat
    let mut data: Vec<u8> = Vec::new();
    for row in 0..height {
        for column in 0..width {
            let v = ((row * column) as f32 / (width * height) as f32) * 255.0;
            let v = (v + 100.0) as u8;
            data.push(v);
            data.push(0);
            data.push(255 - v);
            data.push(255); // alpha
        }
    }
    // Paint cells
    for (_id, cell) in &id_to_cell {
        let c_start = cell.column as u32 * PIXEL_PER_CELL + OFFSET;
        let c_end = c_start as u32 + CELL_SIZE;
        let r_start = cell.row as u32 * PIXEL_PER_CELL + OFFSET;
        let r_end = r_start as u32 + CELL_SIZE;
        let stride = width * BYTES_PER_PIXEL;
        for row in r_start..r_end {
            let start = row * stride;
            for column in c_start..c_end {
                let index = (start + column * BYTES_PER_PIXEL) as usize;
                data[index] = color_palette.palette[0][0];
                data[index + 1] = color_palette.palette[0][1];
                data[index + 2] = color_palette.palette[0][2];
                // data[index + 3] = 255; // skip alpha
            }
        }
    }
    // Paint edges
    for (edge, is_connected) in edge_to_connected {
        let a_cell = id_to_cell.get(&edge.a_id).unwrap();
        let b_cell = id_to_cell.get(&edge.b_id).unwrap();
        let (mut c_start, mut c_end, mut r_start, mut r_end) = (0, 0, 0, 0);
        if is_connected {
            if a_cell.column == b_cell.column {
                c_start = a_cell.column as u32 * PIXEL_PER_CELL + OFFSET;
                c_end = c_start + CELL_SIZE;
                r_start = a_cell.row as u32 * PIXEL_PER_CELL + OFFSET;
                r_end = b_cell.row as u32 * PIXEL_PER_CELL + OFFSET;
            } else if a_cell.row == b_cell.row {
                c_start = a_cell.column as u32 * PIXEL_PER_CELL + OFFSET;
                c_end = b_cell.column as u32 * PIXEL_PER_CELL + OFFSET;
                r_start = a_cell.row as u32 * PIXEL_PER_CELL + OFFSET;
                r_end = r_start + CELL_SIZE;
            }
            let stride = width * BYTES_PER_PIXEL;
            for row in r_start..r_end {
                let start = row * stride;
                for column in c_start..c_end {
                    let index = (start + column * BYTES_PER_PIXEL) as usize;
                    data[index] = color_palette.palette[2][0];
                    data[index + 1] = color_palette.palette[2][1];
                    data[index + 2] = color_palette.palette[2][2];
                    // data[index + 3] = 255; // skip alpha
                }
            }
        }
    }

    SaveTexture {
        data,
        width,
        height,
    }
}

pub fn generate_maze(
    maze_width: u32,
    maze_height: u32,
) -> (HashMap<u32, Cell>, HashMap<Edge, bool>) {
    let mut grid: Vec<Vec<u32>> = vec![];
    let mut id_to_cell: HashMap<u32, Cell> = HashMap::new();
    let mut edges: HashSet<Edge> = HashSet::new();
    let mut edge_to_connected: HashMap<Edge, bool> = HashMap::new();
    let mut rng: ThreadRng = rng();

    // left, top, right, bottom
    let neighbor: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for column in 0..maze_width {
        grid.push(vec![]);
        for row in 0..maze_height {
            let u_id = unique_id(column, row, maze_width);
            let mut neighbors = Vec::new();
            for coord in neighbor {
                let n_column = column as i32 + coord.0 as i32;
                let n_row = row as i32 + coord.1 as i32;
                if n_column >= 0
                    && n_column < maze_width as i32
                    && n_row >= 0
                    && n_row < maze_height as i32
                {
                    let neighbor_id = unique_id(n_column as u32, n_row as u32, maze_width as u32);
                    neighbors.push(neighbor_id);
                    let edge = Edge::new(u_id, neighbor_id);
                    edges.insert(edge);
                    edge_to_connected.insert(edge, false);
                }
            }
            neighbors.shuffle(&mut rng);
            let cell = Cell {
                id: u_id,
                column: column as u32,
                row: row as u32,
                neighbors,
            };
            grid[column as usize].push(u_id);
            id_to_cell.insert(u_id, cell);
        }
    }

    let start_id: u32 = 0;
    let mut visited: HashSet<u32> = HashSet::new();
    let mut frontier: HashSet<u32> = HashSet::new();

    // dbg!(&grid);
    // dbg!(&id_to_cell);
    // dbg!(&edges);

    dig(
        start_id,
        &id_to_cell,
        &edges,
        &mut edge_to_connected,
        &mut visited,
        &mut frontier,
    );

    // dbg!(&edge_to_connected);

    (id_to_cell, edge_to_connected)
}

pub fn dig(
    start_id: u32,
    id_to_cell: &HashMap<u32, Cell>,
    edges: &HashSet<Edge>,
    edge_to_connected: &mut HashMap<Edge, bool>,
    visited: &mut HashSet<u32>,
    frontier: &mut HashSet<u32>,
) {
    visited.insert(start_id);

    for neighbor in &id_to_cell.get(&start_id).unwrap().neighbors {
        if !visited.contains(neighbor) {
            let edge = Edge::new(start_id, *neighbor);
            if edge_to_connected.contains_key(&edge) {
                // dbg!("connected {} {}", start_id, *neighbor);
                *edge_to_connected.get_mut(&edge).unwrap() = true;
            }
            dig(
                *neighbor,
                id_to_cell,
                edges,
                edge_to_connected,
                visited,
                frontier,
            );
        }
    }
}

fn unique_id(column: u32, row: u32, width: u32) -> u32 {
    row * width + column
}
