use crate::wasm_bindgen::{JsCast, JsValue};

use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};
use std::f64;


#[derive(Debug, Clone)]
pub struct Graphics {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Graphics {

    pub fn new(element: HtmlCanvasElement) -> Graphics {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        Graphics { element, context }
    }

    pub fn get_element(&self) -> &HtmlCanvasElement {
        &self.element
    }

    pub fn draw_hand(&self) {
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

    pub fn draw_board(&self) -> Vec<((f64, f64), (f64, f64))> {
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
        
        // convenience
        let context = &self.context;
        
        let offset = (100.0, 200.0);
        let mut pixels: Vec<((f64, f64), (f64, f64))> = Vec::with_capacity(16);
        for i in 0..n_row as u32 { // row
            for j in 0..(n_col as u32) { // column
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
                
                let top_left = (x, y);
                let bottom_right = (x + w, y + h);
                let rect_path = Path2d::new().unwrap();

                rect_path.rect(x, y, w, h);
                context.fill_with_path_2d(&rect_path);
                pixels.push((top_left, bottom_right));
            }
        }
        context.close_path();
        pixels
    }
}