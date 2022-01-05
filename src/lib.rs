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
pub fn resize(image_vector: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let image =
        image::load_from_memory_with_format(&image_vector, image::ImageFormat::Png).unwrap();
    let resized_image = image.resize_exact(
        width as u32,
        height as u32,
        image::imageops::FilterType::Lanczos3,
    );

    let rgb_image = resized_image.into_rgb8();
    let mut result_image_vector = Vec::new();
    let (width, height) = rgb_image.dimensions();
    let png_encoder = image::codecs::png::PngEncoder::new(&mut result_image_vector);
    png_encoder
        .encode(
            &rgb_image,
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    result_image_vector
}

#[wasm_bindgen]
pub fn compress(image_vector: Vec<u8>) -> Vec<u8> {
    let image =
        image::load_from_memory_with_format(&image_vector, image::ImageFormat::Png).unwrap();

    let rgb_image = image.into_rgb8();
    let mut result_image_vector = Vec::new();
    let (width, height) = rgb_image.dimensions();
    let png_encoder = image::codecs::png::PngEncoder::new_with_quality(
        &mut result_image_vector,
        image::codecs::png::CompressionType::Best,
        image::codecs::png::FilterType::NoFilter,
    );
    png_encoder
        .encode(
            &rgb_image,
            width as u32,
            height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    result_image_vector
}

#[cfg(test)]
mod test {
    use super::*;
    use more_asserts;
    use std::os::unix::fs::MetadataExt;

    #[test]
    fn should_return_resized_image() {
        let (want_width, want_height) = (200, 100);
        let sample_image_filename = "sample_image.png";
        let image_vector = std::fs::read(sample_image_filename).unwrap();

        let resized_image_vector = resize(image_vector, want_width, want_height);

        let resized_image =
            image::load_from_memory_with_format(&resized_image_vector, image::ImageFormat::Png)
                .unwrap();
        let (got_width, got_height) = resized_image.to_rgba8().dimensions();
        assert_eq!(
            (want_width, want_height),
            (got_width as usize, got_height as usize)
        );
    }

    #[test]
    fn should_return_compressed_image_with_compression_ratio_above_threshold() {
        let threshold_compression_ratio = 0.8;
        let sample_image_filename = "sample_image.png";
        let sample_image_file = std::fs::File::open(sample_image_filename).unwrap();
        let sample_image_metadata = sample_image_file.metadata().unwrap();
        let original_file_size = sample_image_metadata.size() as f32 / 1024 as f32;
        let image_vector = std::fs::read(sample_image_filename).unwrap();

        let compressed_image_vector = compress(image_vector);

        let compressed_image =
            image::load_from_memory_with_format(&compressed_image_vector, image::ImageFormat::Png)
                .unwrap();
        let compressed_image_filename = "compressed_image.png";
        compressed_image.save(compressed_image_filename).unwrap();
        let compressed_image_file = std::fs::File::open(compressed_image_filename).unwrap();
        let compressed_image_metadata = compressed_image_file.metadata().unwrap();
        let compressed_file_size = compressed_image_metadata.size() as f32 / 1024 as f32;
        let compression_ratio = compressed_file_size / original_file_size;
        more_asserts::assert_gt!(compression_ratio, threshold_compression_ratio);

        // delete temporary compressed image file
        std::fs::remove_file(compressed_image_filename).unwrap();
    }
}
