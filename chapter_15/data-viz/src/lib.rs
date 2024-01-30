use image::RgbImage;
use plotters::prelude::*;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_realtime_chart(x_values: Vec<f64>, y_values: Vec<f64>) -> Vec<u8> {
    let data: Vec<(f64, f64)> = x_values.into_iter().zip(y_values).collect();

    let width = 1600;
    let height = 1400;

    let mut image = RgbImage::new(width, height);

    {
        let root = BitMapBackend::with_buffer(
            &mut image,
            (width.try_into().unwrap(), height.try_into().unwrap()),
        )
        .into_drawing_area();

        let _ = root.fill(&WHITE);

        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_2d(0.0..10.0, 0.0..100.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(data, &RED).point_size(4))
            .unwrap();
    }

    let mut buffer = Vec::new();
    for pixel in image.pixels() {
        buffer.push(pixel.0[0]);
        buffer.push(pixel.0[1]);
        buffer.push(pixel.0[2]);
    }
    buffer
}
