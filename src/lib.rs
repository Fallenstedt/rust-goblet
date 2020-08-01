
extern crate js_sys;
extern crate wasm_bindgen;

mod macros;
mod utils;
mod game;

use std::rc::Rc;
use std::cell::{Cell, RefCell};   
use web_sys::HtmlCanvasElement;

use crate::wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;
use crate::game::manager::Manager;
use crate::game::ui::graphics::Graphics;
use crate::game::utils::{player_number_match};


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

    let pressed = Rc::new(Cell::new(false));
    let circle = Rc::new(Cell::new(-1));
    let initial_rectangle = Rc::new(Cell::new(-1));
    let original_circle_x_y = Rc::new(Cell::new((0.0,0.0)));

    // process mousedown
    {
        let graphics = graphics.clone();
        let manager = manager.clone();

        let pressed = pressed.clone();
        let circle = circle.clone();
        let original_circle_x_y = original_circle_x_y.clone();
        let initial_rectangle= initial_rectangle.clone();

        
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            let graphics = graphics.borrow();
            let manager = manager.borrow();

            circle.set(graphics.get_largest_clicked_circle_index(x, y));
            
            if circle.get() > -1 {
                
                let current_turn = manager.get_turn();
                let shape_owner = graphics.get_circle(circle.get() as usize).get_player();

                if player_number_match(shape_owner, current_turn) {
                    pressed.set(true);
                    initial_rectangle.set(graphics.get_clicked_rectangle_index(x, y));
                    original_circle_x_y.set(graphics.get_circle(circle.get() as usize).get_pos())
                } else {
                    pressed.set(false);
                    circle.set(-1);
                    initial_rectangle.set(-1);
                }                
            }

        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    // process mouse move
    {
        let pressed = pressed.clone();
        let circle = circle.clone();
        let graphics = graphics.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() == true && circle.get() > -1 {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;
                graphics.borrow_mut().update_circle_pos(circle.get() as usize, x, y);
                graphics.borrow_mut().draw_circles();
            
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }

    //process mouse up
    {
        let pressed = pressed.clone();
        let circle = circle.clone();
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
            if circle.get() < 0 {
                pressed.set(false);
                circle.set(-1);
                initial_rectangle.set(-1);
                return
            }

            // user didn't drop a circle on a rectangle
            if ending_rectangle < 0 {
                let (original_x, original_y) = original_circle_x_y.get();
                graphics.update_circle_pos(circle.get() as usize, original_x, original_y);
                graphics.draw_circles();

                pressed.set(false);
                circle.set(-1);
                initial_rectangle.set(-1);
                return
            }

            // piece came from hand
            let ending_rectangle = ending_rectangle as usize;
            if initial_rectangle.get() < 0 {
                log!("User is getting circle from hand");
                
                let coord = graphics.get_coord_for_rectangle(ending_rectangle);
                let quadrant = graphics.get_circle_quadrant(circle.get() as usize);               

                match manager.move_gobblet_from_hand_to_board(coord, quadrant) {
                    Some(gobblet) => {
                        // return piece to hand
                        manager.return_gobblet_to_hand(gobblet, quadrant);
                        
                        // repaint it back at the hand
                        let (original_x, original_y) = original_circle_x_y.get();
                        graphics.update_circle_pos(circle.get() as usize, original_x, original_y);
                        graphics.draw_circles();

                        // reset interaction state
                        pressed.set(false);
                        circle.set(-1);
                        initial_rectangle.set(-1);
                        return
                    },
                    None => graphics.position_circle_center_of_rectangle(ending_rectangle, circle.get() as usize)
                };
            } else {
                // piece came from board
                let source = graphics.get_coord_for_rectangle(initial_rectangle.get() as usize);
                let destination = graphics.get_coord_for_rectangle(ending_rectangle);

                match manager.move_gobblet_on_board(source, destination) {
                    None => graphics.position_circle_center_of_rectangle(ending_rectangle, circle.get() as usize),
                    Some(gobblet) => {
                        // return the piece to source
                        manager.return_gobblet_to_board(source, gobblet);

                        // repaint it at source rectangle
                        let (original_x, original_y) = original_circle_x_y.get();
                        graphics.update_circle_pos(circle.get() as usize, original_x, original_y);
                        graphics.draw_circles();

                        // reset interaction state
                        pressed.set(false);
                        circle.set(-1);
                        initial_rectangle.set(-1);
                        return
                    }
                };

            };
            
            pressed.set(false);
            circle.set(-1);
            initial_rectangle.set(-1);

            match manager.has_won() {
                Some(player) => {
                    log!("Game Over! {:?} won", player);
                    return
                },
                None => manager.change_turn()
            }


        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }    
}