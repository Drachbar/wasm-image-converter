use image::{DynamicImage, ImageOutputFormat};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn convert_image(input: &[u8], width: u32, height: u32, format: &str) -> Vec<u8> {
    // Ladda bilden
    let img = image::load_from_memory(input).expect("Failed to load image");

    // Skapa en ny bild beroende på format
    let new_img = match format {
        "grayscale" => img.grayscale(),
        "invert" => {
            let mut img = img.to_rgba8();
            image::imageops::invert(&mut img);
            DynamicImage::ImageRgba8(img)
        },
        _ => img,
    };

    // Konvertera bilden till en byte-vektor
    let mut output = Vec::new();
    new_img.write_to(&mut output, ImageOutputFormat::Png).expect("Failed to write image");
    output
}