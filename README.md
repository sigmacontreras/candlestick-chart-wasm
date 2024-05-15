# Stock Market Graph with WebAssembly and Rust

This project demonstrates how to create a simple stock market graph using WebAssembly (Wasm) and Rust. It utilizes the `wasm_bindgen` and `serde` crates to interact between JavaScript and Rust.

## Overview

The core functionality of this project revolves around a Rust implementation that generates a candlestick chart on an HTML canvas. The chart is drawn based on provided stock market data, and the colors of the candles can be customized.

## Features

- **Candlestick Chart**: Visualizes stock market data as candlestick patterns.
- **Customizable Colors**: Users can define colors for bullish and bearish candles.
- **WebAssembly Integration**: Rust code is compiled to WebAssembly for use in the browser.
- **Interactivity**: Supports setting and drawing data dynamically.

## Installation

To run this project locally, follow these steps:

1. Clone this repository to your local machine.
2. Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
3. Install `wasm-pack` by running `cargo install wasm-pack`.
4. Build the project by running `wasm-pack build`.
5. Serve the `index.html` file using a local server. You can use Python's built-in HTTP server by running `python3 -m http.server`.

## Usage

To use the stock market graph, follow these steps:

1. Include the `index.js` and `chart_candle_bg.wasm` files in your HTML file.
2. Create an HTML canvas element with an appropriate ID.
3. Instantiate the `ChartCandle` class and pass the canvas element and chart options.
4. Provide stock market data in the required format and call the `set_data` method.
5. Call the `draw` method to render the chart on the canvas.

Example usage:

```javascript
const canvas = document.getElementById("chart");
const options = ChartOptions.new(10, 5, CandleColor.Red, CandleColor.Green);
const chart = new ChartCandle(canvas, options);
const data = [/* Array of stock market data */];
chart.set_data(data);
chart.draw();
```

## Inspiration

This project was based on the following code:

- [Candlestick Chart Sandbox](https://codesandbox.io/p/sandbox/j795rj77l5?file=%2Fsrc%2Findex.js)