use crate::wasm_bindgen::prelude::*;
use crate::game::utils::coord::Coord;
use super::logic::player::Player;
use super::logic::gobblet::{Gobblet};
use super::logic::board::{Board};

use js_sys::Math;

#[derive(Debug, Clone, Copy)]
pub enum PlayerNumber {
    One,
    Two
}

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

    pub fn move_gobblet_from_hand_to_coord(&mut self, coord: &Coord, quadrant: u8) -> Result<(), Gobblet> {
        let player = self.get_mut_player();

        let gobblet = match player.remove_piece_from_hand(quadrant) {
            Some(gobblet) => {
                log!("Gobblet from hand {:#?}", gobblet);
                gobblet
            },
            None => panic!("Failed to obtain gobblet. This shouldn't have happened")
        };

        match self.board.add_piece_to_board(coord, gobblet) {
            None => {
                self.change_turn();
                Ok(())
            },
            Some(gobblet) => Err(gobblet)
        }
    }

    pub fn get_turn(&self) -> &PlayerNumber {
        &self.turn
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            PlayerNumber::One => PlayerNumber::Two,
            PlayerNumber::Two => PlayerNumber::One,
        };
        log!("Next turn. {:?}", self.turn);
    }

    fn get_mut_player(&mut self) -> &mut Player {
        if matches!(self.turn, PlayerNumber::One) {
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
