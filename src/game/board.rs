use super::gobblet::{Gobblet, GobbletSize};
use super::coord::Coord;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Board::build_cells() }
    }

    pub fn add_piece_to_board(&mut self, coord: Coord, gobblet: Gobblet) -> Option<Gobblet> {
        let r = *coord.get_row() as usize;
        let c = *coord.get_column() as usize;
        let cell = &mut self.cells[r][c];

        return if cell.can_add(&gobblet) {
            cell.add(gobblet);
            None
        } else {
            Some(gobblet)
        }
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

    fn can_add(&self, pending_gobblet: &Gobblet) -> bool {
        if self.state.is_empty() {
            return true;
        }
        
        let sizes = [GobbletSize::Tiny, GobbletSize::Small, GobbletSize::Medium, GobbletSize::Large];
        let top_piece: &GobbletSize = &self.state[self.state.len() - 1].size;
        let index_top = &sizes.iter().position(|&g: &GobbletSize| &g == top_piece).unwrap();
        let index_pending = &sizes.iter().position(|&d: &GobbletSize| &d == &pending_gobblet.size).unwrap();
        
        index_pending > index_top
    }
}



// Player Tests
#[cfg(test)]
mod tests {
    use super::Cell;
    use super::super::gobblet::{GobbletSize, Gobblet};

    #[test]
    fn can_add_should_return_true_if_cell_is_empty() {
        let c = Cell::new();
        let r = c.can_add(&Gobblet::new(GobbletSize::Tiny, String::from("Angelica")));
        assert_eq!(r, true);
    }

    #[test]
    fn can_add_should_return_true_if_gobblet_is_larger_than_top_piece() {
        // Given Tiny Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Tiny, String::from("Angelica")));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, String::from("Angelica")));
        let m = c.can_add(&Gobblet::new(GobbletSize::Medium, String::from("Angelica")));
        let l = c.can_add(&Gobblet::new(GobbletSize::Large, String::from("Angelica")));

        assert_eq!(s, true);
        assert_eq!(m, true);
        assert_eq!(l, true);
    }


    #[test]
    fn can_add_should_return_false_if_gobblet_is_smaller_than_top_piece() {
        // Given Large Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Large, String::from("Angelica")));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, String::from("Angelica")));
        let m = c.can_add(&Gobblet::new(GobbletSize::Medium, String::from("Angelica")));
        let l = c.can_add(&Gobblet::new(GobbletSize::Large, String::from("Angelica")));

        assert_eq!(s, false);
        assert_eq!(m, false);
        assert_eq!(l, false);
    }

    #[test]
    fn can_add_should_return_false_if_gobblet_is_same_size() {
        // Given Large Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Small, String::from("Angelica")));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, String::from("Angelica")));

        assert_eq!(s, false);
    }
}