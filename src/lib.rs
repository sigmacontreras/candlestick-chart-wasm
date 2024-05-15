use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Data {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CandleColor {
    Red,
    Green,
    Black,
    White,
}

impl fmt::Display for CandleColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_str = match self {
            CandleColor::Red => "red",
            CandleColor::Green => "green",
            CandleColor::Black => "black",
            CandleColor::White => "white",
        };
        write!(f, "{}", color_str)
    }
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
    ctx: Option<CanvasRenderingContext2d>,
    width: f64,
    height: f64,
    one_percent_of_window_height: f64,
    options: ChartOptions,
    data: Vec<Data>,
}

#[wasm_bindgen]
impl ChartOptions {
    pub fn new(
        margin: f64,
        line_width: f64,
        candle_color_a: CandleColor,
        candle_color_b: CandleColor,
    ) -> Self {
        ChartOptions {
            margin,
            line_width,
            candle_color_a,
            candle_color_b,
        }
    }
}

#[wasm_bindgen]
impl ChartCandle {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, options: ChartOptions) -> Self {
        let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let one_percent_of_window_height = height / 100.0;
        let data = Vec::new();

        ChartCandle {
            ctx: Some(ctx),
            width,
            height,
            one_percent_of_window_height,
            options,
            data,
        }
    }
    // Implement the formatting method
    pub fn formatting(&self, data: JsValue) -> JsValue {
        let data: Vec<Data> = from_value(data).unwrap();

        let mut max = f64::MIN;
        let mut min = f64::MAX;

        // Find max and min values
        for el in &data {
            max = max.max(el.high);
            min = min.min(el.low);
        }

        let max_min_diff = max - min;
        let one_percent_of_max_min_diff = max_min_diff / 100.0;

        let formatted_data: Vec<Data> = data
            .into_iter()
            .map(|el| Data {
                low: (100.0 - (max_min_diff - (max - el.low)) / one_percent_of_max_min_diff)
                    * self.one_percent_of_window_height,
                high: (100.0 - (max_min_diff - (max - el.high)) / one_percent_of_max_min_diff)
                    * self.one_percent_of_window_height,
                open: (100.0 - (max_min_diff - (max - el.open)) / one_percent_of_max_min_diff)
                    * self.one_percent_of_window_height,
                close: (100.0 - (max_min_diff - (max - el.close)) / one_percent_of_max_min_diff)
                    * self.one_percent_of_window_height,
            })
            .collect();

        to_value(&formatted_data).unwrap()
    }

    // Implement the setData method
    pub fn set_data(&mut self, data: JsValue) {
        let formatted_data: JsValue = self.formatting(data);
        self.data = from_value(formatted_data).unwrap();
    }

    // Implement the cleanData method
    pub fn clean_data(&mut self) {
        self.data.clear();
    }

    // Implement the draw method
    pub fn draw(&mut self) {
        if let Some(ctx) = &self.ctx {
            ctx.clear_rect(0.0, 0.0, self.width.into(), self.height.into());
            let w_p = self.width as f64 / self.data.len() as f64;

            for (i, el) in self.data.iter().enumerate() {
                let stroke_color = if el.close > el.open {
                    &self.options.candle_color_a
                } else if el.close < el.open {
                    &self.options.candle_color_b
                } else {
                    &CandleColor::White
                };

                let x = i as f64 * (self.width as f64 / self.data.len() as f64);
                let x = x + self.width as f64 / self.data.len() as f64 / 2.0;

                let y_o = el.open;
                let y_c = if el.open == el.close {
                    el.close + ctx.line_width()
                } else {
                    el.close
                };

                ctx.set_stroke_style(&JsValue::from_str(&stroke_color.to_string()));
                ctx.set_line_width((w_p / 100.0) * self.options.line_width as f64);

                ctx.begin_path();
                ctx.move_to(x, el.high);
                ctx.line_to(x, el.low);
                ctx.stroke();

                ctx.set_line_width((w_p / 100.0) * (100.0 - self.options.margin as f64));
                ctx.begin_path();
                ctx.move_to(x, y_o);
                ctx.line_to(x, y_c);
                ctx.stroke();
            }
        }
    }
}
