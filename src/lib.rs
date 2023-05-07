use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Data {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[wasm_bindgen]
pub enum CandleColor {
    Red,
    Green,
    Black,
    White,
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct ChartOptions {
    margin: f64,
    line_width: f64,
    candle_color_a: CandleColor,
    candle_color_b: CandleColor,
}

// Add the implementation of the ChartCandle struct here
#[wasm_bindgen]
pub struct ChartCandle {
    canvas: HtmlCanvasElement,
    ctx: Option<CanvasRenderingContext2d>,
    width: f64,
    height: f64,
    one_percent_of_window_height: f64,
    options: ChartOptions,
    data: Vec<Data>,
}

#[wasm_bindgen]
impl ChartCandle {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, options: ChartOptions) -> Self {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let one_percent_of_window_height = height / 100.0;
        let data = Vec::new();

        ChartCandle {
            canvas,
            ctx: Some(ctx),
            width,
            height,
            one_percent_of_window_height,
            options,
            data,
        }
    }
    // Implement the formating method
    pub fn formating(&self, data: Vec<Data>) -> Vec<Data> {
        // Your Rust implementation of the formating method
    }

    // Implement the setData method
    pub fn set_data(&mut self, data: Vec<Data>) {
        // Your Rust implementation of the setData method
    }

    // Implement the cleanData method
    pub fn clean_data(&mut self) {
        // Your Rust implementation of the cleanData method
    }

    // Implement the draw method
    pub fn draw(&mut self) {
        // Your Rust implementation of the draw method
    }
}
