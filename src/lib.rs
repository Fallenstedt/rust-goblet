
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
    let chosen_gobblet = manager.remove_piece_from_hand(1).unwrap();
    manager.add_piece_to_board(Coord::new(0, 0), chosen_gobblet);
}
