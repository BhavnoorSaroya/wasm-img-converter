use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

#[wasm_bindgen]
pub fn convert_jpg_to_png(jpg_data: &[u8]) -> Vec<u8> {
    // Decode the JPG image from the byte slice
    let img = image::load(Cursor::new(jpg_data), ImageFormat::Jpeg)
        .expect("Failed to load image");

    // Convert the image to PNG format and collect into a byte vector
    let mut png_data = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
        .expect("Failed to write PNG image");

    png_data
}

