use crate::wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::f64;

pub struct Canvas {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d
}

impl Canvas {

    pub fn new(element: HtmlCanvasElement) -> Canvas {
        let context = element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
        Canvas { element, context }
    }

    pub fn draw_board(&self) {
        // board
        let w = (*&self.element.width() - 200) as f64;
        let h = (*&self.element.height() - 200) as f64;
        let n_row = 4.0;
        let n_col = 4.0;
        
        let w: f64 = w / n_row; // width of block
        let h: f64 = h / n_col; // height of block
        
        // colors
        let sea = JsValue::from_str("#129793");
        let foam = JsValue::from_str("#9bd7d5");
        
        // convenience
        let context = &self.context;
        
        context.begin_path();
        context.set_fill_style(&sea);
        context.fill_rect(0.0, 0.0, w * n_row, h * n_col);
        
        context.set_fill_style(&foam);
        for i in 0..n_row as u32 { // row
            for j in 0..(n_col as u32 / 2) { // column
                // cast as floats
                let j = j as f64;
                let i = i as f64;

                let offset = if i % 2.0 == 0.0 { 0.0 } else { w };
                let x = 2.0 * j * w + offset;
                let y = i * h;
                
                context.rect(x, y, w, h);
                context.fill();
            }
        }
    }
}
