use crate::game::utils::coord::Coord;
use crate::game::utils::{PlayerNumber, player_number_match};
use crate::game::logic::gobblet::{Gobblet, GobbletSize};

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Board::build_cells() }
    }

    pub fn add_piece_to_board(&mut self, coord: &Coord, gobblet: Gobblet) -> Option<Gobblet> {
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

    pub fn remove_piece_from_board(&mut self, coord: &Coord, player: &PlayerNumber) -> Option<Gobblet> {
        let r = *coord.get_row() as usize;
        let c = *coord.get_column() as usize;
        let cell = &mut self.cells[r][c];

        return if cell.can_remove(&player) {
            Some(cell.remove())
        } else {
            None
        }
    }

    pub fn has_won(&self, number: PlayerNumber) -> bool {
        let mut rows: [u8; 4] = [0, 0, 0, 0];
        let mut columns: [u8; 4] = [0, 0, 0, 0];
        let mut diagonal: u8 = 0;
        let mut anti_diagonal: u8 = 0;
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
               if cell.is_empty() {
                   continue;
               }
                // check rows,
                // check columns,
                if player_number_match(cell.get_top_piece().get_player_number(), &number) {
                    rows[r] += 1;
                    columns[c] += 1;
                }

                // check diagonal,
                if r == c && player_number_match(cell.get_top_piece().get_player_number(), &number)  {
                    diagonal += 1;
                }
                
                // check anti diagonal
                if r + c == 3 && player_number_match(cell.get_top_piece().get_player_number(), &number) {
                    anti_diagonal += 1
                }
            }
        }

        return rows.contains(&4) || columns.contains(&4) || diagonal == 4 || anti_diagonal == 4
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
pub struct Cell {
    state: Vec<Gobblet>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { state: Vec::with_capacity(4) }
    }

    pub fn add(&mut self, gobblet: Gobblet) {
        &self.state.push(gobblet);
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

    pub fn can_remove(&self, player: &PlayerNumber) -> bool {
        if &self.state.is_empty() == &true {
            return false;
        }

        let top_piece = &self.get_top_piece();
        player_number_match(top_piece.get_player_number(), player)
    }

    fn is_empty(&self) -> bool {
        &self.state.len() == &0
    }

    pub fn get_top_piece(&self) -> &Gobblet {
        &self.state[self.state.len() - 1]
    }
    
}


#[cfg(test)]
mod board_tests {
    use super::{PlayerNumber, Board, Coord, Gobblet, GobbletSize};
    
    #[test]
    fn add_piece_to_board_should_return_none_if_added_successfully() {
        // Given
        let mut b = Board::new();
        b.add_piece_to_board(&Coord::new(1, 3), Gobblet::new(GobbletSize::Medium, PlayerNumber::One));

        // When
        let r = b.add_piece_to_board(&Coord::new(1, 3), Gobblet::new(GobbletSize::Large, PlayerNumber::One));

        match r {
            Some(_) => assert_eq!(false, true, "Piece was reutrned!"),
            None => assert_eq!(true, true)
        };
    }

    #[test]
    fn has_won_should_return_false_if_no_one_has_won() {
        let b = Board::new();
        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, false);
    }

    #[test]
    fn has_won_should_return_true_if_a_row_is_filled() {
        let mut b = Board::new();
        let gobblet = Gobblet::new(GobbletSize::Large, PlayerNumber::One);
        for i in 0..4 {
            b.add_piece_to_board(&Coord::new(1, i), gobblet.clone());
        }
        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, true);
    }
    
    #[test]
    fn has_won_should_return_true_if_a_column_is_filled() {
        let mut b = Board::new();
        let gobblet = Gobblet::new(GobbletSize::Large, PlayerNumber::One);
        for i in 0..4 {
            b.add_piece_to_board(&Coord::new(i, 1), gobblet.clone());
        }
        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, true);
    }

    #[test]
    fn has_won_should_return_true_if_diagonal_filled() {
        let mut b = Board::new();
        let gobblet = Gobblet::new(GobbletSize::Large, PlayerNumber::One);
        for i in 0..4 {
            b.add_piece_to_board(&Coord::new(i, i), gobblet.clone());
        }
        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, true);
    }

    #[test]
    fn has_won_should_return_true_if_anti_diagonal_filled() {
        let mut b = Board::new();
        let gobblet = Gobblet::new(GobbletSize::Large, PlayerNumber::One);

        b.add_piece_to_board(&Coord::new(0, 3), gobblet.clone());
        b.add_piece_to_board(&Coord::new(1, 2), gobblet.clone());
        b.add_piece_to_board(&Coord::new(2, 1), gobblet.clone());
        b.add_piece_to_board(&Coord::new(3, 0), gobblet.clone());

        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, true);
    }

    #[test]
    fn has_won_should_return_false_if_anti_diagonal_not_filled() {
        let mut b = Board::new();
        let gobblet = Gobblet::new(GobbletSize::Large, PlayerNumber::One);

        b.add_piece_to_board(&Coord::new(0, 3), gobblet.clone());
        b.add_piece_to_board(&Coord::new(1, 2), gobblet.clone());
        b.add_piece_to_board(&Coord::new(2, 2), gobblet.clone());
        b.add_piece_to_board(&Coord::new(3, 0), gobblet.clone());

        let r = b.has_won(PlayerNumber::One);
        assert_eq!(r, false);
    }
}


#[cfg(test)]
mod cell_tests {
    use super::{Cell, Gobblet, GobbletSize, PlayerNumber};

    #[test]
    fn can_add_should_return_true_if_cell_is_empty() {
        let c = Cell::new();
        let r = c.can_add(&Gobblet::new(GobbletSize::Tiny, PlayerNumber::Two));
        assert_eq!(r, true);
    }

    #[test]
    fn can_add_should_return_true_if_gobblet_is_larger_than_top_piece() {
        // Given Tiny Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Tiny, PlayerNumber::Two));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, PlayerNumber::Two));
        let m = c.can_add(&Gobblet::new(GobbletSize::Medium, PlayerNumber::Two));
        let l = c.can_add(&Gobblet::new(GobbletSize::Large, PlayerNumber::Two));

        assert_eq!(s, true);
        assert_eq!(m, true);
        assert_eq!(l, true);
    }


    #[test]
    fn can_add_should_return_false_if_gobblet_is_smaller_than_top_piece() {
        // Given Large Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Large, PlayerNumber::Two));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, PlayerNumber::Two));
        let m = c.can_add(&Gobblet::new(GobbletSize::Medium, PlayerNumber::Two));
        let l = c.can_add(&Gobblet::new(GobbletSize::Large, PlayerNumber::Two));

        assert_eq!(s, false);
        assert_eq!(m, false);
        assert_eq!(l, false);
    }

    #[test]
    fn can_add_should_return_false_if_gobblet_is_same_size() {
        // Given Large Piece in cell
        let mut c = Cell::new();
        c.add(Gobblet::new(GobbletSize::Small, PlayerNumber::Two));

        // When Small, Medium, Large 
        let s = c.can_add(&Gobblet::new(GobbletSize::Small, PlayerNumber::Two));

        assert_eq!(s, false);
    }
}