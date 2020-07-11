use super::gobblet::Gobblet;
use super::coord::Coord;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Board::build_cells() }
    }

    pub fn add_piece_to_board(&mut self, coord: Coord, gobblet: Gobblet) {
        let r = *coord.get_row() as usize;
        let c = *coord.get_column() as usize;
        
        &self.cells[r][c].add(gobblet);
    }

    // Create 2 dimenson array of cells. 
    // index in first vec represents row
    // index in second vec represent column
    // [
    //  [c, c, c, c],
    //  [c, c, c, c],
    //  [c, c, c, c],
    //  [c, c, c, c]
    // ]
    fn build_cells() -> Vec<Vec<Cell>> {
        vec![vec![Cell::new(); 4]; 4]
    }
}

#[derive(Debug, Clone)]
struct Cell {
    state: Vec<Gobblet>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { state: Vec::with_capacity(4)}
    }

    pub fn add(&mut self, gobblet: Gobblet) {
        self.state.push(gobblet);
    }
}

