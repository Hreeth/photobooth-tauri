use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use crate::imaging::Layout;

const BRANDING_WIDTH_RATIO: f32 = 0.8;

pub fn draw_branding(canvas: &mut RgbaImage, layout: &Layout) {
    let [_, right, bottom, left] = layout.bounds.borders;

    let branding_height = bottom;

    if branding_height == 0 {
        return;
    }

    let usable_width = canvas.width().saturating_sub(left + right);

    if usable_width == 0 {
        return;
    }

    let font_src = include_bytes!("../../fonts/Burgundia.otf");
    let font = FontArc::try_from_slice(font_src).expect("failed to load branding font");

    let label = "memora.";

    // First size the branding according to the available branding height.
    let mut scale = font_scale_for_height(&font, branding_height as f32 * 0.8);

    // Determine how wide that text actually is at this scale.
    let label_width = text_width(&font, scale, label);

    // Branding may occupy at most 75% of the usable width.
    let max_width = usable_width as f32 * BRANDING_WIDTH_RATIO;

    if label_width > max_width {
        let factor = max_width / label_width;

        scale.x *= factor;
        scale.y *= factor;
    }

    let scaled = font.as_scaled(scale.y);

    let final_width = text_width(&font, scale, label);

    // Center inside the usable area.
    let x = left as f32 + (usable_width as f32 - final_width) / 2.0;

    let visual_height = scaled.ascent() - scaled.descent();

    let branding_start_y = canvas.height() - branding_height;

    let vertical_padding = (branding_height as f32 - visual_height) / 2.0;

    let y = branding_start_y as f32 + vertical_padding;

    let text_color = branding_color(canvas);

    draw_text_mut(canvas, text_color, x.round() as i32, y.round() as i32, scale, &font, label);
}

pub fn draw_branding_strip(canvas: &mut RgbaImage, layout: &Layout) {
    // The strip is duplicated, so branding must also be duplicated.
    let strip_width = canvas.width() / 2;

    let [_, right, bottom, left] = layout.bounds.borders;

    if bottom == 0 {
        return;
    }

    let usable_width = strip_width.saturating_sub(left + right);

    if usable_width == 0 {
        return;
    }

    let font_src = include_bytes!("../../fonts/Burgundia.otf");
    let font = FontArc::try_from_slice(font_src).expect("failed to load branding font");

    let label = "memora.";

    // Size from the branding height first.
    let mut scale = font_scale_for_height(&font, bottom as f32 * 0.8);

    let label_width = text_width(&font, scale, label);

    // Each branding instance can occupy at most 75%
    // of the usable width of one strip.
    let max_width = usable_width as f32 * BRANDING_WIDTH_RATIO;

    if label_width > max_width {
        let factor = max_width / label_width;

        scale.x *= factor;
        scale.y *= factor;
    }

    let scaled = font.as_scaled(scale.y);

    let final_width = text_width(&font, scale, label);

    let visual_height = scaled.ascent() - scaled.descent();

    let branding_start_y = canvas.height() - bottom;

    let vertical_padding = (bottom as f32 - visual_height) / 2.0;

    let y = branding_start_y as f32 + vertical_padding;

    let text_color = branding_color(canvas);

    // First strip.
    let x1 = left as f32 + (usable_width as f32 - final_width) / 2.0;

    // Second strip.
    let x2 = strip_width as f32 + x1;

    draw_text_mut(canvas, text_color, x1.round() as i32, y.round() as i32, scale, &font, label);

    draw_text_mut(canvas, text_color, x2.round() as i32, y.round() as i32, scale, &font, label);
}

fn text_width(font: &FontArc, scale: PxScale, text: &str) -> f32 {
    let scaled = font.as_scaled(scale);

    text.chars()
        .map(|c| {
            let glyph_id = font.glyph_id(c);
            scaled.h_advance(glyph_id)
        })
        .sum()
}

fn branding_color(canvas: &RgbaImage) -> Rgba<u8> {
    let mut total = 0u64;
    let mut count = 0u64;

    for pixel in canvas.pixels() {
        total += pixel[0] as u64;
        total += pixel[1] as u64;
        total += pixel[2] as u64;
        count += 3;
    }

    let average = if count == 0 { 255 } else { total / count };

    if average > 128 { Rgba([0, 0, 0, 255]) } else { Rgba([255, 255, 255, 255]) }
}

fn font_scale_for_height(font: &FontArc, target_height: f32) -> PxScale {
    let scaled = font.as_scaled(PxScale { x: 1.0, y: 1.0 });

    let unit_height = scaled.ascent() - scaled.descent();

    let scale_factor = target_height / unit_height;

    PxScale { x: scale_factor, y: scale_factor }
}
