use crate::wasm_bindgen::prelude::*;
use crate::game::utils::coord::Coord;
use crate::game::utils::{PlayerNumber, player_number_match};
use super::logic::player::Player;
use super::logic::gobblet::{Gobblet};
use super::logic::board::{Board};

use js_sys::Math;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Manager {
    player1: Player,
    player2: Player,
    board: Board,
    turn: PlayerNumber,
}

impl Manager {
    pub fn new(name1: String, name2: String) -> Manager {
        let board = Board::new();
        let player1 = Player::new(name1, PlayerNumber::One);
        let player2 = Player::new(name2, PlayerNumber::Two); 

        Manager{ player1, player2, board, turn:  Manager::random_turn() }
    }

    pub fn move_gobblet_from_hand_to_coord(&mut self, coord: &Coord, quadrant: u8) -> Result<(), &str> {
        let player = self.get_mut_player();

        let gobblet = match player.remove_piece_from_hand(quadrant) {
            Some(gobblet) => {
                log!("Gobblet from hand {:#?}", gobblet);
                gobblet
            },
            None => panic!("Failed to obtain gobblet. This shouldn't have happened")
        };

        match self.board.add_piece_to_board(coord, gobblet) {
            None => Ok(()),
            Some(gobblet) => Err("Unable to add piece to board, returning piece to hand")
        }
    }

    pub fn move_gobblet_on_board(&mut self, source: &Coord, destination: &Coord) -> Result<(), &str> {
        let gobblet = match self.board.remove_piece_from_board(source, &self.turn) {
            Some(g) => g,
            None => panic!("Expected piece to exist on board")
        };

        match self.board.add_piece_to_board(destination, gobblet) {
            None => Ok(()),
            Some(g) => {
                self.board.add_piece_to_board(source, g).unwrap();
                Err("Unable to move piece to board, returning piece to board")
            }
        }
    }

    pub fn has_won(&self) -> Option<PlayerNumber> {
        if self.board.has_won(PlayerNumber::One) {
            return Some(PlayerNumber::One)
        } else if self.board.has_won(PlayerNumber::Two) {
            return Some(PlayerNumber::Two)
        }
        return None
    }

    pub fn get_turn(&self) -> &PlayerNumber {
        &self.turn
    }

    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
        log!("Next turn. {:?}", self.turn);
    }

    fn get_player(&self) -> &Player {
        if player_number_match(&self.turn, &PlayerNumber::One) {
            &self.player1
        } else {
            &self.player2
        }
    }

    fn get_mut_player(&mut self) -> &mut Player {
        if player_number_match(&self.turn, &PlayerNumber::One) {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    fn random_turn() -> PlayerNumber {
        return if Math::random() > 0.5 {
            PlayerNumber::One
        } else {
            PlayerNumber::Two
        }
    }
}

