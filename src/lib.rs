
extern crate js_sys;
extern crate wasm_bindgen;

mod macros;
mod utils;
mod game;

use wasm_bindgen::prelude::*;
use game::manager::Manager;
use game::coord::Coord;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

#[wasm_bindgen]
pub fn new_game() {
    let mut manager = Manager::new(String::from("Alex"), String::from("Angelica"));
    
    let current_player = manager.get_current_player();
    let gobblet = current_player.remove_piece_from_hand(1).unwrap();
    let gobblet2 = current_player.remove_piece_from_hand(1).unwrap();
    let board = manager.get_board();
    
    /**
     * problems
     * 1. Pieces on board have no names. Who owns which piece?
     * 2. Pieces can stack in incorret order
    */
    board.add_piece_to_board(Coord::new(1, 1), gobblet);
    board.add_piece_to_board(Coord::new(1, 1), gobblet2);

    log!("{:?}", board)
}
