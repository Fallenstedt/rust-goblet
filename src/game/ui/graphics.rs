use crate::wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::f64;

#[derive(Debug)]
pub struct Graphics {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d
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

    pub fn draw_board(&self) {
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
        let pos = (100.0, 200.0);
        context.begin_path();
        context.set_fill_style(&sea);
        context.fill_rect(pos.0, pos.1, w * n_row, h * n_col);
        
        context.set_fill_style(&foam);
        for i in 0..n_row as u32 { // row
            for j in 0..(n_col as u32 / 2) { // column
                // cast as floats
                let j = j as f64;
                let i = i as f64;

                let offset = if i % 2.0 == 0.0 { 0.0 } else { w };
                let x = 2.0 * j * w + offset + pos.0;
                let y = i * h + pos.1;
                
                context.rect(x, y, w, h);
                context.fill();
            }
        }
    }
}
