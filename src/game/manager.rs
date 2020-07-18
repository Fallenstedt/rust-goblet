
use super::logic::player::Player;
use super::logic::board::Board;
use super::logic::coord::Coord;
use super::logic::gobblet::Gobblet;
use super::ui::graphics::Graphics;

use js_sys::Math;
use web_sys::HtmlCanvasElement;

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
    graphics: Graphics,
}

impl Manager {
    pub fn new(name1: String, name2: String, canvas: HtmlCanvasElement) -> Manager {
        // logic
        let player1 = Player::new(name1);
        let player2 = Player::new(name2);
        let board = Board::new();
        let turn = Manager::random_turn(); 

        // graphics
        let graphics = Graphics::new(canvas);
        graphics.draw_board();

        Manager{ player1, player2, board, turn, graphics }
    }

    pub fn remove_piece_from_hand(&mut self, section: u8) -> Option<Gobblet> {
        let p = self.get_mut_current_player();
        let chosen_gobblet = p.remove_piece_from_hand(section);
        chosen_gobblet
    }

    pub fn add_piece_to_board(&mut self, coord: Coord, gobblet: Gobblet) -> Option<Gobblet> {
        self.board.add_piece_to_board(coord, gobblet)
    }

    pub fn remove_piece_from_board(&mut self, coord: Coord) -> Option<Gobblet> {
        let current_player = self.get_mut_current_player().get_name();
        let piece = self.board.remove_piece_from_board(coord, current_player);
        piece
    }

    pub fn has_won(&self) -> bool {
        self.board.has_won(self.get_current_player().get_name())
    }

    pub fn get_current_player(&self) -> &Player {
        match &self.turn {
            Turn::Player1 => &self.player1,
            Turn::Player2 => &self.player2
        }
    }

    pub fn get_mut_current_player(&mut self) -> &mut Player {
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
