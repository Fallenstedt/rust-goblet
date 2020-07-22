use super::logic::player::Player;
use super::logic::board::Board;
use super::ui::graphics::Graphics;

use js_sys::Math;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};


#[derive(Debug)]
pub enum Turn {
    Player1,
    Player2
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Manager {
    player1: Player,
    player2: Player,
    board: Board,
    turn: Turn,
    graphics: Graphics,
}

#[wasm_bindgen]
impl Manager {

    #[wasm_bindgen(constructor)]
    pub fn new(name1: String, name2: String, canvas: HtmlCanvasElement) -> Manager {
        let graphics = Graphics::new(canvas, &name1, &name2);
        let board = Board::new();
        let player1 = Player::new(name1);
        let player2 = Player::new(name2); 

        Manager{ player1, player2, board, turn:  Manager::random_turn(), graphics }
    }


    #[wasm_bindgen(method)]
    pub fn proccess_click_event(&self, x: f64, y: f64) {
        log!("Clicked at {:?}, {:?}",x,y);
        
        let has_clicked_circle = match self.graphics.get_clicked_circle(x, y) {
            Some(_) => true,
            None => false,
        };

        let has_clicked_rectangle = match self.graphics.get_clicked_rectangle(x, y) {
            Some(_) => true,
            None => false,
        };
        

        match (has_clicked_rectangle, has_clicked_circle) {
            (true, true) => log!("Circle and Rectangle clicked, piece is on board"),
            (true, false) => log!("Rectangle clicked, nothing found"),
            (false, true) => log!("Circle clicked, piece is on hand"),
            (false, false) => log!("What are you doing"),
        };
  

        // if click on circle no rectangle
        // then get current player and quadrant
    }

    // pub fn remove_piece_from_hand(&mut self, section: u8) -> Option<Gobblet> {
    //     let p = self.get_mut_current_player();
    //     let chosen_gobblet = p.remove_piece_from_hand(section);
    //     chosen_gobblet
    // }

    // pub fn add_piece_to_board(&mut self, coord: Coord, gobblet: Gobblet) -> Option<Gobblet> {
    //     self.board.add_piece_to_board(coord, gobblet)
    // }

    // pub fn remove_piece_from_board(&mut self, coord: Coord) -> Option<Gobblet> {
    //     let current_player = self.get_mut_current_player().get_name();
    //     let piece = self.board.remove_piece_from_board(coord, current_player);
    //     piece
    // }
    
    // pub fn has_won(&self) -> bool {
    //     self.board.has_won(self.get_current_player().get_name())
    // }

    // pub fn get_current_player(&self) -> &Player {
    //     match &self.turn {
    //         Turn::Player1 => &self.player1,
    //         Turn::Player2 => &self.player2
    //     }
    // }

    // pub fn get_mut_current_player(&mut self) -> &mut Player {
    //     match &self.turn {
    //         Turn::Player1 => &mut self.player1,
    //         Turn::Player2 => &mut self.player2
    //     }
    // }

    fn random_turn() -> Turn {
        return if Math::random() > 0.5 {
            Turn::Player1
        } else {
            Turn::Player2
        }
    }
}
