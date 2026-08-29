use std::path::PathBuf;

use image::{DynamicImage, RgbaImage, imageops::overlay};

use crate::imaging::{HEIGHT, LayoutMode, WIDTH};

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

pub fn remove_bleed(
    img: image::DynamicImage,
    layout_mode: LayoutMode,
) -> crate::Result<image::DynamicImage> {
    // 0.15 cm bleed on each side.
    let bleed = ((0.15 / 2.54) * crate::imaging::DPI).round() as u32;

    let width = img.width();
    let height = img.height();

    if width <= bleed * 2 || height <= bleed * 2 {
        return Err("final image is too small to remove bleed".into());
    }

    let cropped = img.crop_imm(
        bleed,
        bleed,
        width - bleed * 2,
        height - bleed * 2,
    );

    let cropped = match layout_mode {
        LayoutMode::Strip => {
            let half_width = cropped.width() / 2;

            cropped.crop_imm(
                0,
                0,
                half_width,
                cropped.height(),
            )
        }

        _ => cropped,
    };

    Ok(cropped)
}