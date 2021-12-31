use wasm_bindgen::prelude::*;
// use image::io::Reader as ImageReader;
use image;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn resize(img: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let result = image::load_from_memory_with_format(&img, image::ImageFormat::Png).unwrap();
    let resized_image= result.resize_exact(width as u32, height as u32, image::imageops::FilterType::Lanczos3);

    let rgb_image = resized_image.into_rgb8();
    let mut result_vector = Vec::new();
    let (width, height) = rgb_image.dimensions();
    let png_encoder = image::codecs::png::PngEncoder::new(&mut result_vector);
    png_encoder.encode(&rgb_image, width as u32, height as u32, image::ColorType::Rgb8);
    result_vector
}