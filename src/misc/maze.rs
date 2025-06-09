use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Cell {
    id: u8,
    column: u8,
    row: u8,
    neighbors: Vec<u8>,
}

#[derive(Hash, PartialEq, Eq)]
pub struct UndirectedEdge {
    a_id: u8,
    b_id: u8,
}

impl UndirectedEdge {
    pub fn new(a: u8, b: u8) -> Self {
        let (a_id, b_id) = if a < b { (a, b) } else { (b, a) };

        Self { a_id, b_id }
    }
}

#[test]
pub fn GenerateMaze() {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    let mut grid: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut id_to_cell: HashMap<u8, Cell> = HashMap::new();
    let mut edges: HashSet<UndirectedEdge> = HashSet::new();

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
                    let edge = UndirectedEdge::new(u_id, neighbor_id);
                    edges.insert(edge);
                }
            }
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

    Dig(start_id, &id_to_cell, &edges, &mut visited, &mut frontier);
}

pub fn Dig(
    start_id: u8,
    id_to_cell: &HashMap<u8, Cell>,
    edges: &HashSet<UndirectedEdge>,
    visited: &mut HashSet<u8>,
    frontier: &mut HashSet<u8>,
) {
    visited.insert(start_id);

    for neighbor in &id_to_cell.get(&start_id).unwrap().neighbors {
        if !visited.contains(neighbor) {
            let edge = UndirectedEdge::new(start_id, *neighbor);
            if edges.contains(&edge) {
                dbg!("connected {} {}", start_id, *neighbor);
            }
            Dig(*neighbor, id_to_cell, edges, visited, frontier);
        }
    }
}

fn unique_id(column: u8, row: u8, width: u8) -> u8 {
    row * width + column
}
