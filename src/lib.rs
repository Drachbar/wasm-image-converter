use std::io::Cursor;

use image::{ColorType, ImageEncoder, load_from_memory};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::webp::WebPEncoder;
use image::imageops::FilterType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_image(
    data: &[u8],
    width: u32,
    height: u32,
    format: &str
) -> Result<Vec<u8>, JsValue> {
    // Ladda bilden från byte-datan
    let img = load_from_memory(data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    // Ändra storlek på bilden
    let resized_img = img.resize_exact(width, height, FilterType::Lanczos3);

    // Spara bilden i det önskade formatet
    let mut buffer = Cursor::new(Vec::new());
    match format {
        "png" => {
            let encoder = PngEncoder::new(&mut buffer);
            encoder.write_image(&resized_img.to_rgba8(), resized_img.width(), resized_img.height(), ColorType::Rgba8.into())
                .map_err(|e| JsValue::from_str(&format!("Failed to write PNG image: {}", e)))?;
        }
        "jpeg" | "jpg" => {
            let rgb_img = resized_img.to_rgb8();
            let encoder = JpegEncoder::new(&mut buffer);
            encoder.write_image(&rgb_img, rgb_img.width(), rgb_img.height(), ColorType::Rgb8.into())
                .map_err(|e| JsValue::from_str(&format!("Failed to write JPEG image: {}", e)))?;
        }
        "webp" => {
            let encoder = WebPEncoder::new_lossless(&mut buffer);
            encoder.encode(&resized_img.to_rgba8(), resized_img.width(), resized_img.height(), ColorType::Rgba8.into())
                .map_err(|e| JsValue::from_str(&format!("Failed to write WebP image: {}", e)))?;
        }
        _ => return Err(JsValue::from_str("Unsupported format")),
    };

    Ok(buffer.into_inner())
}