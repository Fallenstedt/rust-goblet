use std::cell::{Cell};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interaction {
    pressed: Rc<Cell<bool>>,
    chosen_circle: Rc<Cell<isize>>,
    initial_rectangle: Rc<Cell<isize>>,
    ending_rectangle: Rc<Cell<isize>>
}

impl Interaction {
    pub fn new() -> Interaction {
        let pressed = Rc::new(Cell::new(false));
        let chosen_circle = Rc::new(Cell::new(-1));
        let initial_rectangle = Rc::new(Cell::new(-1));
        let ending_rectangle = Rc::new(Cell::new(-1));
        
        Interaction {
            pressed,
            chosen_circle,
            initial_rectangle,
            ending_rectangle
        }
    }

    pub fn set_pressed(&mut self, state: bool) {
        &self.pressed.set(state);
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed.get()
    }

    pub fn set_initial_rectangle(&self, index: isize) {
        self.initial_rectangle.set(index);
    }

    pub fn get_initial_rectangle(&self) -> isize {
        self.initial_rectangle.get()
    }

    pub fn get_chosen_circle(&self) -> isize {
        self.chosen_circle.get()
    }

    pub fn set_chosen_circle(&self, value: isize) {
        self.chosen_circle.set(value)
    }

    pub fn reset_state(&self) {
        self.pressed.set(false);
        self.chosen_circle.set(-1);
        self.initial_rectangle.set(-1);
        self.ending_rectangle.set(-1);
    }

}