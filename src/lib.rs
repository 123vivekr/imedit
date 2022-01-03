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
    png_encoder.encode(&rgb_image, width as u32, height as u32, image::ColorType::Rgb8).unwrap();
    result_vector
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_resized_image() {
        let (want_width, want_height) = (200, 100);

        let sample_image_filename = "sample_image.png";
        let image_vector = std::fs::read(sample_image_filename).unwrap();

        let resized_image_vector = resize(image_vector, want_width, want_height);

        let result = image::load_from_memory_with_format(&resized_image_vector, image::ImageFormat::Png).unwrap();
        let (got_width, got_height) = result.to_rgba8().dimensions();
        assert_eq!((want_width, want_height), (got_width as usize, got_height as usize));
    }
}