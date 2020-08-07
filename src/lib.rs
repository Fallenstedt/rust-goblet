extern crate js_sys;
extern crate wasm_bindgen;

mod game;
mod macros;
mod utils;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use web_sys::HtmlCanvasElement;

use crate::game::manager::Manager;
use crate::game::ui::graphics::Graphics;
use crate::game::utils::player_number_match;
use crate::wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;

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

// TODO move all the complexity of interaction into graphics.
#[wasm_bindgen]
pub fn start_game(canvas: HtmlCanvasElement, name1: String, name2: String) {
    let graphics = Rc::new(RefCell::new(Graphics::new(canvas.clone())));
    let manager = Rc::new(RefCell::new(Manager::new(name1, name2)));

    let initial_rectangle = Rc::new(Cell::new(-1));
    let original_circle_x_y = Rc::new(Cell::new((0.0, 0.0)));

    // process mousedown
    {
        let graphics = graphics.clone();
        let manager = manager.clone();

        let original_circle_x_y = original_circle_x_y.clone();
        let initial_rectangle = initial_rectangle.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            let mut graphics = graphics.borrow_mut();
            let manager = manager.borrow();
            
            graphics.set_largest_clicked_circle(x, y);
            if graphics.get_chosen_circle() > -1 {
                let current_turn = manager.get_turn();
                let shape_owner = graphics.get_circle().get_player();

                if player_number_match(shape_owner, current_turn) {
                    graphics.set_pressed(true);
                    initial_rectangle.set(graphics.get_clicked_rectangle_index(x, y));
                    original_circle_x_y.set(graphics.get_circle().get_pos())
                } else {
                    graphics.set_pressed(false);
                    graphics.set_chosen_circle(-1);
                    initial_rectangle.set(-1);
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    // process mouse move
    {
        let graphics = graphics.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut graphics = graphics.borrow_mut();

            if graphics.is_pressed() && graphics.get_chosen_circle() > -1 {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;
                graphics.update_circle_pos(x, y);
                graphics.draw_circles();
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    //process mouse up
    {
        let initial_rectangle = initial_rectangle.clone();
        let original_circle_x_y = original_circle_x_y.clone();
        let graphics = graphics.clone();
        let manager = manager.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            let mut graphics = graphics.borrow_mut();
            let mut manager = manager.borrow_mut();
            let ending_rectangle = graphics.get_clicked_rectangle_index(x, y);
            // user didn't click on circle
            if graphics.get_chosen_circle() < 0 {
                graphics.set_pressed(false);
                initial_rectangle.set(-1);
                return;
            }

            // user didn't drop a circle on a rectangle
            if ending_rectangle < 0 {
                let (original_x, original_y) = original_circle_x_y.get();
                graphics.update_circle_pos(original_x, original_y);
                graphics.draw_circles();

                graphics.set_pressed(false);
                graphics.set_chosen_circle(-1);
                initial_rectangle.set(-1);
                return;
            }

            // piece came from hand
            let ending_rectangle = ending_rectangle as usize;
            match initial_rectangle.get() < 0 {
                true => {
                    let coord = graphics.get_coord_for_rectangle(ending_rectangle);
                    let quadrant = graphics.get_circle_quadrant();

                    match manager.move_gobblet_from_hand_to_board(coord, quadrant) {
                        Some(gobblet) => {
                            // return piece to hand
                            manager.return_gobblet_to_hand(gobblet, quadrant);
                            // repaint it back at the hand
                            let (original_x, original_y) = original_circle_x_y.get();
                            graphics.update_circle_pos(
                                original_x,
                                original_y,
                            );
                            graphics.draw_circles();
                            // reset interaction state
                            graphics.set_pressed(false);
                            graphics.set_chosen_circle(-1);
                            initial_rectangle.set(-1);
                            return;
                        }
                        None => graphics.position_circle_center_of_rectangle(
                            ending_rectangle,
                        ),
                    };
                }
                false => {
                    // piece came from board
                    let source = graphics.get_coord_for_rectangle(initial_rectangle.get() as usize);
                    let destination = graphics.get_coord_for_rectangle(ending_rectangle);

                    match manager.move_gobblet_on_board(source, destination) {
                        None => graphics.position_circle_center_of_rectangle(
                            ending_rectangle,
                        ),
                        Some(gobblet) => {
                            // return the piece to source
                            manager.return_gobblet_to_board(source, gobblet);

                            // repaint it at source rectangle
                            let (original_x, original_y) = original_circle_x_y.get();
                            graphics.update_circle_pos(
                                original_x,
                                original_y,
                            );
                            graphics.draw_circles();

                            // reset interaction state
                            graphics.set_pressed(false);
                            graphics.set_chosen_circle(-1);
                            initial_rectangle.set(-1);
                            return;
                        }
                    };
                }
            };
            
            graphics.set_pressed(false);
            graphics.set_chosen_circle(-1);
            initial_rectangle.set(-1);

            match manager.has_won() {
                Some(player) => {
                    log!("Game Over! {:?} won", player);
                    return;
                }
                None => manager.change_turn(),
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
