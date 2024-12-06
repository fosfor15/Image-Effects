use base64::prelude::*;
use image::load_from_memory;
use image::ImageFormat::Png;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Hello world from Rust, lib.rs and grayscale function".into());

    let base64_to_vector: Vec<u8> = BASE64_STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());
    
    let mut image: image::DynamicImage = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
    
    image = image.grayscale();
    log(&"Grayscale effect applied".into());
    
    let mut image_buffer: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut image_buffer), Png).unwrap();
    log(&"New image written".into());

    let encoded_image: String = BASE64_STANDARD.encode(&image_buffer);
    let image_url: String = format!("data:image/png;base64,{encoded_image}");

    return image_url;
}
