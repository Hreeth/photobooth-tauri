use std::path::PathBuf;

use image::{RgbaImage, imageops::FilterType};
use tauri::{AppHandle, Manager};

use crate::imaging::Layout;

const BRANDING_WIDTH_RATIO: f32 = 0.8;
const BRANDING_HEIGHT_RATIO: f32 = 0.8;

pub fn draw_branding(
    app_handle: &AppHandle,
    canvas: &mut RgbaImage,
    layout: &Layout,
    inverted: bool,
) {
    let [_, right, bottom, left] = layout.bounds.borders;

    if bottom == 0 {
        return;
    }

    let usable_width = canvas.width().saturating_sub(left + right);

    if usable_width == 0 {
        return;
    }

    let branding_path = match get_asset_path(app_handle, "branding.png") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut branding = match image::open(&branding_path) {
        Ok(image) => image.to_rgba8(),
        Err(e) => {
            eprintln!("Failed to load branding image {:?}: {}", branding_path, e);
            return;
        }
    };

    if inverted {
        invert_branding(&mut branding);
    }

    let original_width = branding.width();
    let original_height = branding.height();

    if original_width == 0 || original_height == 0 {
        return;
    }

    // Same conceptual sizing as the old text:
    // target height = 80% of the branding area.
    let max_height = bottom as f32 * BRANDING_HEIGHT_RATIO;

    // And the branding may occupy at most 80% of the usable width.
    let max_width = usable_width as f32 * BRANDING_WIDTH_RATIO;

    // Scale uniformly while respecting BOTH constraints.
    let scale = (max_width / original_width as f32).min(max_height / original_height as f32);

    let final_width = (original_width as f32 * scale).round() as u32;
    let final_height = (original_height as f32 * scale).round() as u32;

    if final_width == 0 || final_height == 0 {
        return;
    }

    branding = image::imageops::resize(&branding, final_width, final_height, FilterType::Lanczos3);

    // Horizontally centered inside the usable area.
    let x = left + (usable_width - final_width) / 2;

    // Vertically centered inside the branding area.
    let branding_start_y = canvas.height().saturating_sub(bottom);

    let y = branding_start_y + (bottom - final_height) / 2;

    image::imageops::overlay(canvas, &branding, x as i64, y as i64);
}

pub fn draw_branding_strip(
    app_handle: &AppHandle,
    canvas: &mut RgbaImage,
    layout: &Layout,
    inverted: bool,
) {
    let strip_width = canvas.width() / 2;

    let [_, right, bottom, left] = layout.bounds.borders;

    if bottom == 0 {
        return;
    }

    let usable_width = strip_width.saturating_sub(left + right);

    if usable_width == 0 {
        return;
    }

    let branding_path = match get_asset_path(app_handle, "branding.png") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut branding = match image::open(&branding_path) {
        Ok(image) => image.to_rgba8(),
        Err(e) => {
            eprintln!("Failed to load branding image {:?}: {}", branding_path, e);
            return;
        }
    };

    if inverted {
        invert_branding(&mut branding);
    }

    let original_width = branding.width();
    let original_height = branding.height();

    if original_width == 0 || original_height == 0 {
        return;
    }

    let max_height = bottom as f32 * BRANDING_HEIGHT_RATIO;
    let max_width = usable_width as f32 * BRANDING_WIDTH_RATIO;

    let scale = (max_width / original_width as f32).min(max_height / original_height as f32);

    let final_width = (original_width as f32 * scale).round() as u32;
    let final_height = (original_height as f32 * scale).round() as u32;

    if final_width == 0 || final_height == 0 {
        return;
    }

    branding = image::imageops::resize(&branding, final_width, final_height, FilterType::Lanczos3);

    let branding_start_y = canvas.height().saturating_sub(bottom);

    let y = branding_start_y + (bottom - final_height) / 2;

    // First branding.
    let x1 = left + (usable_width - final_width) / 2;

    // Second branding, exactly one strip-width to the right.
    let x2 = strip_width + x1;

    image::imageops::overlay(canvas, &branding, x1 as i64, y as i64);

    image::imageops::overlay(canvas, &branding, x2 as i64, y as i64);
}

fn invert_branding(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
        // Keep alpha unchanged.
    }
}

fn get_asset_path(app_handle: &AppHandle, filename: &str) -> Result<PathBuf, String> {
    app_handle
        .path()
        .resolve(format!("assets/{filename}"), tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Failed to find resource: {e}"))
}
