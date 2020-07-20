use crate::wasm_bindgen::{JsCast, JsValue};

use std::f64;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};

use super::shapes::{Rectangle, Circle};
use crate::game::utils::coord::Coord;



#[derive(Debug, Clone)]
pub struct Graphics {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    rectangles: Vec<Rectangle>,
    circles: Vec<Circle>
}

impl Graphics {

    pub fn new(element: HtmlCanvasElement) -> Graphics {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        
        let tan: JsValue = JsValue::from_str("#6C5B7B");
        context.set_fill_style(&tan);
        context.fill_rect(0.0, 0.0, element.width() as f64, element.height() as f64);

        let rectangles = Graphics::draw_board(&context);
        let circles = Graphics::draw_hand(&context);
        Graphics { element, context, rectangles, circles }
    }

    pub fn get_clicked_rectangle(&self, x: f64, y: f64) -> Option<&Coord> {
        let mut rectangle = None;
        for r in self.rectangles.iter() {
            if self.context.is_point_in_path_with_path_2d_and_f64(r.get_path(), x, y) {
                rectangle = Some(r.get_coord());
                break;
            } 
        }
        rectangle
    }

    pub fn get_clicked_circle(&self, x: f64, y: f64) -> Option<&Circle> {
        let mut circle = None;
        for c in self.circles.iter() {
            if self.context.is_point_in_path_with_path_2d_and_f64(c.get_path(), x, y) {
                circle = Some(c);
                break;
            } 
        }
        circle
    }

    fn draw_hand(context: &CanvasRenderingContext2d) -> Vec<Circle> {
        fn piece_renderer(context: &CanvasRenderingContext2d, quadrant: usize, size: usize, player: usize) -> Circle {
            // let y = 100.0 * player  as f64;
            let y = match player {
                1 => 100.0,
                2 => 700.0,
                _ => 0.0,
            };
            let coord = match quadrant {
                1 => (200.0, y),
                2 => (300.0, y),
                3 => (400.0, y),
                _ => (0.0, 0.0),
            };
            let size = (10 * size) as f64 ;
            
            let path = Path2d::new().unwrap();
            path.arc(coord.0, coord.1, size, 0.0, 2.0 * f64::consts::PI).unwrap();
            
            let yellow = JsValue::from_str("#FFB85F");
            let yellow_border = JsValue::from_str("#FFA433");
            let red = JsValue::from_str("#F67280");
            let red_border = JsValue::from_str("#C4421A");
            
            if player == 1 {
                context.set_fill_style(&yellow);
                context.set_stroke_style(&yellow_border);
            } else {
                context.set_fill_style(&red);
                context.set_stroke_style(&red_border);
            }

            context.set_line_width(5.0);
            context.stroke_with_path(&path);
            context.fill_with_path_2d(&path);         
            

            
            let circle = Circle::new(path, quadrant as u8);
            circle
        }

        let mut circles: Vec<Circle> = Vec::with_capacity(12);
        
        let w = 400.0;
        for size in 1..5 {
            for player in 1..3 {
                for quadrant in 1..4 {
                        let circle = piece_renderer(context, quadrant, size, player);
                        circles.push(circle);
                }
            }
        }
        circles
    }

    fn draw_board(context: &CanvasRenderingContext2d) -> Vec<Rectangle> {
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
                
                let rectangle = Rectangle::new(path, coord);
                rectangles.push(rectangle);
            }
        }
        context.close_path();
        rectangles
    }
}