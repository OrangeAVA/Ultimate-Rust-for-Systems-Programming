use image::RgbaImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn apply_grayscale(input: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    // Convert the input RGBA image to Grayscale
    let mut img = RgbaImage::from_vec(width, height, input).expect("Invalid image dimensions");

    for pixel in img.pixels_mut() {
        let gray_value =
            pixel.0[0] as f32 * 0.299 + pixel.0[1] as f32 * 0.587 + pixel.0[2] as f32 * 0.114;
        pixel.0 = [
            gray_value as u8,
            gray_value as u8,
            gray_value as u8,
            pixel.0[3],
        ];
    }

    // Convert the Grayscale image back to a flat Vec<u8>
    let output = img.into_raw();
    output
}
