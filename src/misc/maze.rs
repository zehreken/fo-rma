use std::collections::HashMap;

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

pub fn GenerateMaze() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut grid: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut id_to_cell: HashMap<u8, Cell> = HashMap::new();

    for column in 0..WIDTH {
        for row in 0..HEIGHT {
            let unique_id = unique_id(column as u8, row as u8, WIDTH as u8);
            let cell = Cell {
                id: unique_id,
                column: column as u8,
                row: row as u8,
                neighbors: Vec::new(),
            };
            grid[row][column] = unique_id;
            id_to_cell.insert(unique_id, cell);
        }
    }

    todo!()
}

pub fn Dig() {
    todo!()
}

fn unique_id(column: u8, row: u8, width: u8) -> u8 {
    row * width + column
}
