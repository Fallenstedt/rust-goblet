use crate::wasm_bindgen::{JsCast, JsValue};

use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};
use super::super::coord::Coord;
use std::f64;

#[derive(Debug, Clone)]
struct Rectangle {
    path: Path2d,
    coord: Coord,
}

impl Rectangle {
    pub fn new(path: Path2d, coord: Coord) -> Rectangle {
        Rectangle { path, coord }
    }

    pub fn get_path(&self) -> &Path2d {
        &self.path
    }

    pub fn get_coord(&self) -> &Coord {
        &self.coord
    }
}


#[derive(Debug, Clone)]
pub struct Graphics {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    rectangles: Vec<Rectangle>
}

impl Graphics {

    pub fn new(element: HtmlCanvasElement) -> Graphics {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

        let rectangles = Graphics::draw_board(&context);
        Graphics { element, context, rectangles }
    }

    pub fn get_element(&self) -> &HtmlCanvasElement {
        &self.element
    }

    pub fn has_clicked_rectangle(&self, x: f64, y: f64) -> Option<&Coord> {
        let mut rectangle = None;
        for (i, recentagle) in self.rectangles.iter().enumerate() {
            if *&self.context.is_point_in_path_with_path_2d_and_f64(recentagle.get_path(), x, y) {
                rectangle = Some(recentagle.get_coord());
                break;
            } 
        }
        rectangle
    }

    fn draw_hand(&self) {
        let context = &self.context;
        let yellow = JsValue::from_str("#FFB85F");
        let yellow_border = JsValue::from_str("#FFA433");
        
        context.set_fill_style(&yellow);
        context.set_stroke_style(&yellow_border);
        context.set_line_width(5.0);
        context.begin_path();
        context.arc(100.0, 100.0, 40.0, 0.0, 2.0 * f64::consts::PI).unwrap();
        context.fill(); 
        context.stroke();
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
        let sea = JsValue::from_str("#129793");
        let foam = JsValue::from_str("#9bd7d5");
        
        let offset = (100.0, 200.0);
        let mut pixels: Vec<((f64, f64), (f64, f64))> = Vec::with_capacity(16);
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