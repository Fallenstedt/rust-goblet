use crate::wasm_bindgen::JsCast;
use crate::wasm_bindgen::prelude::*;

use super::logic::player::Player;
use super::logic::board::Board;
use super::ui::graphics::Graphics;
use super::ui::shapes::{Circle, Rectangle};

use std::rc::Rc;
use std::cell::{Cell, RefCell};      
use js_sys::Math;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};


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
}

#[wasm_bindgen]
impl Manager {

    #[wasm_bindgen(constructor)]
    pub fn new(name1: String, name2: String) -> Manager {
        let board = Board::new();
        let player1 = Player::new(name1);
        let player2 = Player::new(name2); 

        Manager{ player1, player2, board, turn:  Manager::random_turn() }
    }

    #[wasm_bindgen(method)]
    pub fn start_game(&self, canvas: HtmlCanvasElement) {
        let graphics = Rc::new(RefCell::new(Graphics::new(canvas.clone())));
        let pressed = Rc::new(Cell::new(false));
        let circle = Rc::new(Cell::new(-1));
        let rectangle: Rc<Cell<Option<Rectangle>>> = Rc::new(Cell::new(None));

        // process mousedown
        {
            let pressed = pressed.clone();
            let circle = circle.clone();
            let graphics = graphics.clone();
            
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;

                pressed.set(true);
                circle.set(graphics.borrow().get_largest_clicked_circle_index(x, y));

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
                if pressed.get() && circle.get() > -1 {
                    let x = event.offset_x() as f64;
                    let y = event.offset_y() as f64;
                    
                    graphics.borrow_mut().update_circle_coords(circle.get() as usize, x, y);
                    graphics.borrow_mut().draw_circles();
                
                }
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }

        //process mouse up
        {
            let pressed = pressed.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if circle.get() > -1 {
                    let x = event.offset_x() as f64;
                    let y = event.offset_y() as f64;
                    
                    // get square hovering over
                    // check if cell can be dropped
                    // if yes, drop and update game state
                    // if no, reset everyone's state
                }

                pressed.set(false);
                circle.set(-1);
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
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
