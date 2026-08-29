use std::path::PathBuf;

use image::{DynamicImage, RgbaImage, imageops::overlay};

use crate::imaging::{HEIGHT, WIDTH};

pub fn assets_dir() -> PathBuf {
    dirs::document_dir().unwrap().join("Memorabooth")
}

pub fn output_dir() -> PathBuf {
    dirs::picture_dir().unwrap().join("Memorabooth").join("outputs")
}

pub fn capture_dir() -> PathBuf {
    dirs::picture_dir().unwrap().join("Memorabooth").join("captures")
}

pub fn apply_bleed(img: &DynamicImage, bleed_px: u32, bg: [u8; 4]) -> DynamicImage {
    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, image::Rgba(bg));

    let resized = image::imageops::resize(
        img,
        WIDTH - (2 * bleed_px),
        HEIGHT - (2 * bleed_px),
        image::imageops::FilterType::Lanczos3,
    );

    overlay(&mut canvas, &resized, bleed_px.into(), bleed_px.into());

    DynamicImage::ImageRgba8(canvas)
}
