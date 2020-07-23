use crate::wasm_bindgen::JsCast;
use crate::wasm_bindgen::prelude::*;

use super::logic::player::Player;
use super::logic::board::Board;
use super::ui::graphics::Graphics;
use super::ui::shapes::{Circle, Rectangle};

use std::rc::Rc;
use std::cell::Cell;      
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
    pub fn start_game(&self, canvas: HtmlCanvasElement) {
        let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        let context = Rc::new(context);
        let pressed = Rc::new(Cell::new(false));
        let circle = Rc::new(Cell::new(false));
        let rectangle: Rc<Cell<Option<Rectangle>>> = Rc::new(Cell::new(None));
        let graphics = self.graphics.clone();

        // process mousedown
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let circle = circle.clone();
            let rectangle = rectangle.clone();
            
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;
                
                pressed.set(true);
                circle.set(graphics.get_clicked_circle(x, y).is_some());
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }
        // process mouse move
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let circle = circle.clone();
            let graphics = self.graphics.clone();

            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if pressed.get() && circle.get() {
                    let x = event.offset_x() as f64;
                    let y = event.offset_y() as f64;
                    log!("{:?} {:?}",x, y);
                    let c = graphics.get_clicked_circle(x, y).unwrap();
                    log!("{:?}!", c);
                    // c.get_path().move_to(x, y);
                }
            }) as Box<dyn FnMut(_)>);
            canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }

        //process mouse up
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = event.offset_x() as f64;
                let y = event.offset_y() as f64;
                pressed.set(false);
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
