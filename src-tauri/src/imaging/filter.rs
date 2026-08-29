use image::RgbaImage;

use crate::state::FilterKind;

pub fn apply(image: &mut RgbaImage, filter: FilterKind) {
    match filter {
        FilterKind::BW => {
            for pixel in image.pixels_mut() {
                let [r, g, b, a] = pixel.0;

                let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;

                *pixel = image::Rgba([gray, gray, gray, a]);
            }
        }

        FilterKind::Color => {}

        FilterKind::HujiCam => {}

        FilterKind::Vintage => {}
    }
}
