use crate::wasm_bindgen::{JsCast, JsValue};

use std::f64;


use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};

use super::shapes::{Rectangle, Circle};
use super::interaction::Interaction;
use crate::game::utils::coord::Coord;
use crate::game::utils::PlayerNumber;


#[derive(Debug, Clone)]
pub struct Graphics {
    pub interaction: Interaction,
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    rectangles: Vec<Rectangle>,
    circles: Vec<Circle>,
}

impl Graphics {

    pub fn new(element: HtmlCanvasElement) -> Graphics {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        let rectangles = Graphics::create_board(&context, &element);
        let circles = Graphics::create_hand(&context);
        let interaction = Interaction::new();
        Graphics { 
            interaction,
            element, 
            context, 
            rectangles, 
            circles, 
        }
    }

    pub fn get_clicked_rectangle_index(&self, x: f64, y: f64) -> isize {
        let mut index: isize = -1;
        for (i, r) in self.rectangles.iter().enumerate() {
            if self.context.is_point_in_path_with_path_2d_and_f64(r.get_path(), x, y) {
                index = i as isize;
                break;
            } 
        }
        index
    }

    pub fn get_coord_for_rectangle(&self, index: usize) -> &Coord {
        self.rectangles.get(index).unwrap().get_coord()
    }

    pub fn update_circle_pos(&mut self,  x: f64, y: f64) {
        let index = self.interaction.get_chosen_circle() as usize;
        let circle = self.circles.get_mut(index).unwrap();  
        circle.set_pos(x, y);
    }
        
    pub fn get_circle_quadrant(&self) -> u8  {
        let index = self.interaction.get_chosen_circle() as usize;
        self.circles.get(index).unwrap().get_quadrant()
    }

    pub fn get_circle(&self) -> &Circle  {
        let index = self.interaction.get_chosen_circle();
        return if index > -1 {
            self.circles.get(index as usize).unwrap()
        } else {
            panic!("Cannot get circle that is out of range");
        }
    }

    pub fn set_largest_clicked_circle(&self, x: f64, y: f64) {
        let mut index: isize = -1;
        let mut clicked_circles = Vec::new();
        
        for (i, c) in self.circles.iter().enumerate() {
            if self.context.is_point_in_path_with_path_2d_and_f64(c.get_path(), x, y) {
                clicked_circles.push((i, c));
                index = i as isize;
            } 
        }

        if clicked_circles.len() == 0 {
            self.interaction.set_chosen_circle(index);
            return;
        }
        
        // sort circles by largest -> smallest
        clicked_circles.sort_by(|a, b| b.1.get_size().partial_cmp(&a.1.get_size()).unwrap());
        index = clicked_circles.get(0).unwrap().0 as isize;
        self.interaction.set_chosen_circle(index)
    }
    
    pub fn position_circle_center_of_rectangle(&mut self, rectange_index: usize) {
        let circle_index = self.interaction.get_chosen_circle() as usize;

        let rectangle = self.rectangles.get(rectange_index).unwrap();
        let circle = self.circles.get_mut(circle_index).unwrap();
        let (x, y) = rectangle.get_pos();
        circle.set_pos(x, y);
        self.draw_circles();
    }


    pub fn draw_circles(&mut self) {
        self.redraw_board();

        let yellow = JsValue::from_str("#FFB85F");
        let yellow_border = JsValue::from_str("#FFA433");
        let red = JsValue::from_str("#F67280");
        let red_border = JsValue::from_str("#C4421A");

        for circle in &mut self.circles {
            let path = Path2d::new().unwrap();
            let (x, y) = circle.get_pos();
            let size = circle.get_size();

            match circle.get_player() {
                PlayerNumber::One => {
                    &self.context.set_fill_style(&yellow);
                    &self.context.set_stroke_style(&yellow_border);
                },
                PlayerNumber::Two => {
                    &self.context.set_fill_style(&red);
                    &self.context.set_stroke_style(&red_border);
                }
            };

            path.arc(x, y, size, 0.0, 2.0 * f64::consts::PI).unwrap();

            self.context.set_line_width(5.0);
            self.context.stroke_with_path(&path);
            self.context.fill_with_path_2d(&path);
            
            circle.set_path(path);
        }
    }

    fn redraw_board(&self) {
        let light_purple: JsValue = JsValue::from_str("#6C5B7B");
        self.context.set_fill_style(&light_purple);
        self.context.fill_rect(0.0, 0.0, self.element.width() as f64, self.element.height() as f64);

        // board
        let w = 400.0;
        let h = 400.0;
        let n_row = 4.0;
        let n_col = 4.0;
        
        let w: f64 = w / n_row; // width of block
        let h: f64 = h / n_col; // height of block
        
        // colors
        let sea = JsValue::from_str("#5f506c");
        let foam = JsValue::from_str("#867297");
        
        let offset = (100.0, 200.0);
        for i in 0..n_row as u8 { // row
            for j in 0..(n_col as u8) { // column
                // cast as floats
                let j = j as f64;
                let i = i as f64;
                
                if i % 2.0 == 0.0 {
                    if j % 2.0 == 0.0 { self.context.set_fill_style(&foam); } else { self.context.set_fill_style(&sea); };    
                } else {
                    if j % 2.0 == 0.0 { self.context.set_fill_style(&sea); } else { self.context.set_fill_style(&foam); };
                }

                let x = j * w + offset.0;
                let y = i * h + offset.1;
                
                let path = Path2d::new().unwrap();
                path.rect(x, y, w, h);
                self.context.fill_with_path_2d(&path);
            }
        }
    }

    fn create_hand(context: &CanvasRenderingContext2d) -> Vec<Circle> {
        fn piece_renderer(context: &CanvasRenderingContext2d, quadrant: usize, size: usize, player: PlayerNumber, y: f64) -> Circle {
            let coord = match quadrant {
                1 => (200.0, y),
                2 => (300.0, y),
                3 => (400.0, y),
                _ => (0.0, 0.0),
            };
            let size = (10 * size) as f64 ;
            
            let path = Path2d::new().unwrap();
            path.arc(coord.0, coord.1, size, 0.0, 2.0 * f64::consts::PI).unwrap();

            context.set_line_width(5.0);
            context.stroke_with_path(&path);
            context.fill_with_path_2d(&path);         
                        
            let circle = Circle::new(path, quadrant as u8, player, coord.0, coord.1, size);
            circle
        }

        let mut circles: Vec<Circle> = Vec::with_capacity(12);  
        let yellow = JsValue::from_str("#FFB85F");
        let yellow_border = JsValue::from_str("#FFA433");
        let red = JsValue::from_str("#F67280");
        let red_border = JsValue::from_str("#C4421A");
        
        for size in 1..5 {
            for player in 1..3 {

                let y: f64;
                let p: PlayerNumber;
                match player {
                    1 => {
                        context.set_fill_style(&yellow);
                        context.set_stroke_style(&yellow_border);
                        y = 100.0;
                        p = PlayerNumber::One;
                    },
                    2 => {
                        context.set_fill_style(&red);
                        context.set_stroke_style(&red_border);
                        y = 700.0;
                        p = PlayerNumber::Two;
                    },
                    _ => panic!("Cannot have more than two players!")
                };

                for quadrant in 1..4 {
                        let circle = piece_renderer(context, quadrant, size, p, y);
                        circles.push(circle);
                }
            }
        }
        circles
    }

    fn create_board(context: &CanvasRenderingContext2d, element: &HtmlCanvasElement) -> Vec<Rectangle> {
        let light_purple: JsValue = JsValue::from_str("#6C5B7B");
        context.set_fill_style(&light_purple);
        context.fill_rect(0.0, 0.0, element.width() as f64, element.height() as f64);

        // board
        let w = 400.0;
        let h = 400.0;
        let n_row = 4.0;
        let n_col = 4.0;
        
        let w: f64 = w / n_row; // width of block
        let h: f64 = h / n_col; // height of block
        
        // colors
        let sea = JsValue::from_str("#5f506c");
        let foam = JsValue::from_str("#867297");
        
        let offset = (100.0, 200.0);
        let mut rectangles: Vec<Rectangle> = Vec::with_capacity(16);
        for i in 0..n_row as u8 { // row
            for j in 0..(n_col as u8) { // column
                // cast as floats
                let j = j as f64;
                let i = i as f64;
                
                if i % 2.0 == 0.0 {
                    if j % 2.0 == 0.0 { context.set_fill_style(&foam); } else { context.set_fill_style(&sea); };    
                } else {
                    if j % 2.0 == 0.0 { context.set_fill_style(&sea); } else { context.set_fill_style(&foam); };
                }

                let x = j * w + offset.0;
                let y = i * h + offset.1;
                
                let coord = Coord::new(i as u8, j as u8);
                let path = Path2d::new().unwrap();
                path.rect(x, y, w, h);
                context.fill_with_path_2d(&path);
                
                let rectangle = Rectangle::new(path, coord, x + (0.5 * w), y + (0.5 * h));
                rectangles.push(rectangle);
            }
        }
        rectangles
    }
}