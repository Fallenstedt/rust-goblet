use super::player::Player;
use super::board::Board;
use super::coord::Coord;
use super::gobblet::Gobblet;

use js_sys::Math;

#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2
}

#[derive(Debug)]
pub struct Manager {
    player1: Player,
    player2: Player,
    board: Board,
    turn: Turn,
}

impl Manager {
    pub fn new(name1: String, name2: String) -> Manager {
        let player1 = Player::new(name1);
        let player2 = Player::new(name2);
        let board = Board::new();
        let turn = Manager::random_turn(); 

        Manager{ player1, player2, board, turn }
    }
    pub fn remove_piece_from_hand(&mut self, section: u8) -> Option<Gobblet> {
        let p = self.get_current_player();
        let chosen_gobblet = p.remove_piece_from_hand(section);
        chosen_gobblet
    }

    pub fn add_piece_to_board(&mut self, coord: Coord, gobblet: Gobblet) -> Option<Gobblet> {
        self.board.add_piece_to_board(coord, gobblet)
    }

    pub fn remove_piece_from_board(mut self, coord: Coord) -> Option<Gobblet> {
        let current_player = self.get_current_player().get_name();
        let piece = self.board.remove_piece_from_board(coord, current_player);
        piece
    }

    pub fn get_current_player(&mut self) -> &mut Player {
        match &self.turn {
            Turn::Player1 => &mut self.player1,
            Turn::Player2 => &mut self.player2
        }
    }


    fn random_turn() -> Turn {
        return if Math::random() > 0.5 {
            Turn::Player1
        } else {
            Turn::Player2
        }
    }
}
