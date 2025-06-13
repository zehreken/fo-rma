use rand::{rng, rngs::ThreadRng, seq::SliceRandom};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Cell {
    id: u8,
    column: u8,
    row: u8,
    neighbors: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    pub a_id: u8,
    pub b_id: u8,
}

impl Edge {
    pub fn new(a: u8, b: u8) -> Self {
        let (a_id, b_id) = if a < b { (a, b) } else { (b, a) };

        Self { a_id, b_id }
    }
}

const WIDTH: usize = 3;
const HEIGHT: usize = 3;
const BYTES_PER_PIXEL: u32 = 4;
const PIXEL_PER_CELL: u32 = 300;
const OFFSET: u32 = 100;

#[test]
pub fn generate_texture() {
    let (id_to_cell, edge_to_connected) = generate_maze();

    let width = WIDTH as u32 * PIXEL_PER_CELL;
    let height = HEIGHT as u32 * PIXEL_PER_CELL;

    // tightly_packed_data is 4 * width * height
    let mut tightly_packed_data: Vec<u8> = Vec::new();
    for row in 0..height {
        for column in 0..width {
            let v = (((row * column) as f32 / (width * height) as f32) * 255.0) as u8;
            // println!("{} {} {} {}", v, row, column, row * WIDTH + column);
            // I push 4 bytes per cell
            // if row % 2 == 0 {
            //     tightly_packed_data.push(255);
            // } else {
            //     tightly_packed_data.push(0);
            // }
            tightly_packed_data.push(0);
            tightly_packed_data.push(0);
            tightly_packed_data.push(0);
            tightly_packed_data.push(255); // alpha
        }
    }
    for (_id, cell) in &id_to_cell {
        let c_start = cell.column as u32 * PIXEL_PER_CELL + OFFSET;
        let c_end = c_start as u32 + 50;
        let r_start = cell.row as u32 * PIXEL_PER_CELL + OFFSET;
        let r_end = r_start as u32 + 50;
        let stride = width * BYTES_PER_PIXEL;
        for row in r_start..r_end {
            let start = row * stride;
            for column in c_start..c_end {
                let index = (start + column * BYTES_PER_PIXEL) as usize;
                tightly_packed_data[index] += 255;
                tightly_packed_data[index + 1] += 255;
                tightly_packed_data[index + 2] += 255;
                // tightly_packed_data[index + 3] = 255; // skip alpha
            }
        }
    }
    for (edge, is_connected) in edge_to_connected {
        let a_cell = id_to_cell.get(&edge.a_id);
        let b_cell = id_to_cell.get(&edge.b_id);
        if is_connected {
            if a_cell.unwrap().column == b_cell.unwrap().column {
            } else if a_cell.unwrap().row == b_cell.unwrap().row {
            }
        }
    }
    let buffer: image::ImageBuffer<image::Rgba<u8>, _> =
        image::ImageBuffer::from_raw(width, height, tightly_packed_data).unwrap();
    let image_path = format!("out/basic-maze.png");
    buffer.save(&image_path).unwrap();
}

pub fn generate_maze() -> (HashMap<u8, Cell>, HashMap<Edge, bool>) {
    let mut grid: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut id_to_cell: HashMap<u8, Cell> = HashMap::new();
    let mut edges: HashSet<Edge> = HashSet::new();
    let mut edge_to_connected: HashMap<Edge, bool> = HashMap::new();
    let mut rng: ThreadRng = rng();

    // left, top, right, bottom
    let neighbor: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    for column in 0..WIDTH {
        for row in 0..HEIGHT {
            let u_id = unique_id(column as u8, row as u8, WIDTH as u8);
            let mut neighbors = Vec::new();
            for coord in neighbor {
                let n_column = column as i8 + coord.0;
                let n_row = row as i8 + coord.1;
                if n_column >= 0 && n_column < WIDTH as i8 && n_row >= 0 && n_row < HEIGHT as i8 {
                    let neighbor_id = unique_id(n_column as u8, n_row as u8, WIDTH as u8);
                    neighbors.push(neighbor_id);
                    let edge = Edge::new(u_id, neighbor_id);
                    edges.insert(edge);
                    edge_to_connected.insert(edge, false);
                }
            }
            neighbors.shuffle(&mut rng);
            let cell = Cell {
                id: u_id,
                column: column as u8,
                row: row as u8,
                neighbors,
            };
            grid[row][column] = u_id;
            id_to_cell.insert(u_id, cell);
        }
    }

    let start_id: u8 = 0;
    let mut visited: HashSet<u8> = HashSet::new();
    let mut frontier: HashSet<u8> = HashSet::new();

    dbg!(&grid);
    dbg!(&id_to_cell);
    dbg!(&edges);

    dig(
        start_id,
        &id_to_cell,
        &edges,
        &mut edge_to_connected,
        &mut visited,
        &mut frontier,
    );

    dbg!(&edge_to_connected);

    // for row in 0..HEIGHT {
    //     let mut line = String::new();
    //     for column in 0..WIDTH {
    //         let u_id = unique_id(column as u8, row as u8, WIDTH as u8);
    //         let c = if column + 1 == WIDTH {
    //             &format!("{}", u_id)
    //         } else {
    //             &format!("{}-----", u_id)
    //         };
    //         line.push_str(c);
    //     }
    //     println!("{}", line);
    //     if row + 1 < HEIGHT {
    //         println!("|     |     |");
    //         println!("|     |     |");
    //     }
    // }

    (id_to_cell, edge_to_connected)
}

pub fn dig(
    start_id: u8,
    id_to_cell: &HashMap<u8, Cell>,
    edges: &HashSet<Edge>,
    edge_to_connected: &mut HashMap<Edge, bool>,
    visited: &mut HashSet<u8>,
    frontier: &mut HashSet<u8>,
) {
    visited.insert(start_id);

    for neighbor in &id_to_cell.get(&start_id).unwrap().neighbors {
        if !visited.contains(neighbor) {
            let edge = Edge::new(start_id, *neighbor);
            if edge_to_connected.contains_key(&edge) {
                dbg!("connected {} {}", start_id, *neighbor);
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

fn unique_id(column: u8, row: u8, width: u8) -> u8 {
    row * width + column
}
