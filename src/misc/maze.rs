use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug)]
pub struct Cell {
    id: u8,
    column: u8,
    row: u8,
    neighbors: Vec<u8>,
}

pub struct UndirectedEdge {
    aId: u8,
    bId: u8,
}

#[test]
pub fn GenerateMaze() {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;
    let mut grid: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut id_to_cell: HashMap<u8, Cell> = HashMap::new();

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
                    neighbors.push(unique_id(n_column as u8, n_row as u8, WIDTH as u8));
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

    visited.insert(start_id);

    dbg!(grid);
    dbg!(id_to_cell);

    // Dig(&mut visited, &mut frontier);
}

pub fn Dig(visited: &mut HashSet<u8>, frontier: &mut HashSet<u8>) {
    todo!()
}

fn unique_id(column: u8, row: u8, width: u8) -> u8 {
    row * width + column
}
