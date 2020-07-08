mod macros;
mod utils;
mod game;

use wasm_bindgen::prelude::*;
use game::player::Player;


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
    // Create Two Players
    //  Create 12 pieces for each player,  3x Tiny 3x Small, 3x Medium, 3x Large
    let p1 = Player::new(String::from("Alex"));
    let p2 = Player::new(String::from("Angelica"));
    log!("{:?}", p1);
    log!("{:?}", p2);
    // Create Game Board
    //  16 squares 4x4 grid
    //  Each square can have stack of 3 pieces
    // Create Game Manager
    //  Know whose turn it is
    //  Check if player one per move

}
