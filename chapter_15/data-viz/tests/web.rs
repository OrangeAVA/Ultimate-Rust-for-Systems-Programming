//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use data_viz::generate_realtime_chart;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_generate_realtime_chart() {
    // Sample data
    let x_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_values = vec![10.0, 20.0, 15.0, 25.0, 30.0];

    // Call the generate_realtime_chart function
    let result = generate_realtime_chart(x_values.clone(), y_values.clone());

    // You can add more assertions or checks based on your requirements
}
