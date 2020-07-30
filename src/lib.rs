
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
pub fn start_game(canvas: HtmlCanvasElement, name1: String, name2: String) {
    let graphics = Rc::new(RefCell::new(Graphics::new(canvas.clone())));
    let manager = Rc::new(RefCell::new(Manager::new(name1, name2)));

    let pressed = Rc::new(Cell::new(false));
    let circle = Rc::new(Cell::new(-1));
    let initial_rectangle = Rc::new(Cell::new(-1));

    // process mousedown
    {
        let graphics = graphics.clone();
        let manager = manager.clone();
        let pressed = pressed.clone();
        let circle = circle.clone();
        let initial_rectangle= initial_rectangle.clone();
        
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            let graphics = graphics.borrow();
            let manager = manager.borrow();

            circle.set(graphics.get_largest_clicked_circle_index(x, y));
            
            if circle.get() > -1 {
                let shape = graphics.get_circle(circle.get() as usize);
                let current_turn = manager.get_turn();
                let shape_owner = shape.get_player();

                if matches!(shape_owner, current_turn) {
                    pressed.set(true);
                    initial_rectangle.set(graphics.get_clicked_rectangle_index(x, y));
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
        let graphics = graphics.clone();
        let manager = manager.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;            
            let mut graphics = graphics.borrow_mut();
            let mut manager = manager.borrow_mut();
            let ending_rectangle = graphics.get_clicked_rectangle_index(x, y);
            
            // user didn't click on a circle
            // user didn't drop a circle on a rectangle
            if circle.get() < 0 || ending_rectangle < 0 {
                log!("User didn't click a circle or an end rectangle");
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

                manager.move_gobblet_from_hand_to_coord(coord, quadrant).unwrap();
                graphics.position_circle_center_of_rectangle(ending_rectangle, circle.get() as usize);
            } else {
            // piece came from board
                
            };

            pressed.set(false);
            circle.set(-1);
            initial_rectangle.set(-1);

        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
    
}