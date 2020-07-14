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

    pub fn remove_piece_from_board(&mut self, coord: Coord, player: String) -> Option<Gobblet> {
        let r = *coord.get_row() as usize;
        let c = *coord.get_column() as usize;
        let cell = &mut self.cells[r][c];

        return if cell.can_remove(player) {
            Some(cell.remove())
        } else {
            None
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

    pub fn remove(&mut self) -> Gobblet {
        self.state.pop().unwrap()
    }

    pub fn can_add(&self, pending_gobblet: &Gobblet) -> bool {
        if &self.state.is_empty() == &true {
            return true;
        }
        
        let sizes = [GobbletSize::Tiny, GobbletSize::Small, GobbletSize::Medium, GobbletSize::Large];
        let top_piece = &self.get_top_piece();
        let index_top = &sizes.iter().position(|&g: &GobbletSize| &g == top_piece.get_size()).unwrap();
        let index_pending = &sizes.iter().position(|&d: &GobbletSize| &d == pending_gobblet.get_size()).unwrap();
        
        index_pending > index_top
    }

    pub fn can_remove(&self, player: String) -> bool {
        if &self.state.is_empty() == &true {
            return false;
        }

        let top_piece = &self.get_top_piece();
        top_piece.get_name() == player
    }

    fn get_top_piece(&self) -> &Gobblet {
        &self.state[self.state.len() - 1]
    }
    
}


#[cfg(test)]
mod board_tests {
    use super::Board;
    use super::Coord;
    use super::super::gobblet::{GobbletSize, Gobblet};

    #[test]
    fn add_piece_to_board_should_return_none_if_added_successfully() {
        // Given
        let mut b = Board::new();
        b.add_piece_to_board(Coord::new(1, 3), Gobblet::new(GobbletSize::Medium, String::from("Alex")));

        // When
        let r = b.add_piece_to_board(Coord::new(1, 3), Gobblet::new(GobbletSize::Large, String::from("Alex")));

        match r {
            Some(_) => assert_eq!(false, true, "Piece was reutrned!"),
            None => assert_eq!(true, true)
        };
    }
}


#[cfg(test)]
mod cell_tests {
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