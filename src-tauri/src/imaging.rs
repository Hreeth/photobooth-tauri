use std::{fs, path::PathBuf, process::Command};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const DPI: f32 = 300.0f32;
const WIDTH: u32 = (4f32 * DPI).round() as u32;
const HEIGHT: u32 = (6f32 * DPI).round() as u32;
const BORDER: f32 = 0.15f32;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Layout {
    A,
    B,
    C,
}

#[tauri::command(async)]
pub async fn capture(output_path: &str) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        let mut cmd_base = Command::new("libcamera-still");
        let cmd = cmd_base
            .arg("-t")
            .arg("3000")
            .arg("--autofocus-mode")
            .arg("continuous")
            .arg("--autofocus-range")
            .arg("normal")
            .arg("--denoise")
            .arg("cdn_off")
            .arg("--shutter")
            .arg("18000")
            .arg("--gain")
            .arg("10")
            .arg("--ev")
            .arg("0")
            .arg("--roi")
            .arg("0.075,0.13,0.79,0.85")
            .arg("-p")
            .arg("-10,-10,1920,1080")
            .arg("-o")
            .arg(output_path);

        let result = cmd.output();

        return match result {
            Ok(output) => {
                let stdout_str = String::from_utf8_lossy(&output.stdout);
                let stderr_str = String::from_utf8_lossy(&output.stderr);

                if !output.status.success() {
                    println!("stderr: {}", stderr_str);
                }

                println!("stdout: {}", stdout_str);
                Ok(output_path.to_string())
            }
            Err(e) => return Err(format!("Failed to execute capture command: {}", e)),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let sample_path = "sample.jpg"; // Replace with the actual path of your sample image

        return match fs::copy(sample_path, output_path) {
            Ok(_) => {
                println!("Sample image copied to: {}", output_path);
                Ok(output_path.to_string())
            }
            Err(e) => Err(format!("Failed to copy sample image: {}", e)),
        }
    }
}

#[tauri::command(async)]
pub async fn print(
    app: AppHandle,
    images: Vec<String>,
    output_path: &str,
    color_mode: &str,
    copies: usize,
    layout: Layout,
) -> Result<(), String> {
    let bg_color = Rgba([240, 236, 230, 255]);

    let border_px = ((BORDER / 2.54) * DPI).round() as u32;

    let canvas = match layout {
        Layout::A => apply_layout_a(&app, images, color_mode, bg_color, border_px)?,
        Layout::B => apply_layout_b(&app, images, color_mode, bg_color, border_px)?,
        Layout::C => apply_layout_c(&app, images, color_mode, bg_color, border_px)?,
    };

    if let Err(e) = canvas.save(output_path) {
        eprintln!("Failed to save image: {}", e);
        return Err(format!("Failed to save image: {}", e));
    }

    let mut canvas2 = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

    let strip = match image::open(output_path) {
        Ok(img) => image::imageops::resize(
            &img,
            WIDTH - (2 * border_px),
            HEIGHT - (2 * border_px),
            image::imageops::FilterType::Lanczos3,
        ),
        Err(e) => {
            eprintln!("Failed to open image {}: {}", output_path, e);
            return Err(format!("Failed to open image {}: {}", output_path, e));
        }
    };

    if let Err(e) = canvas2.copy_from(&strip, border_px, border_px) {
        eprintln!("Failed to copy final strip to canvas2: {}", e);
        return Err(e.to_string());
    }

    if let Err(e) = canvas2.save(output_path) {
        eprintln!("Failed to save final image: {}", e);
        return Err(format!("Failed to save image: {}", e));
    }

    let print_res = match layout {
        Layout::A | Layout::B => Command::new("lp")
            .arg("-o")
            .arg("media=w288h432")
            .arg("-o")
            .arg("fit-to-page")
            .arg("-n")
            .arg(copies.to_string())
            .arg(output_path)
            .output(),
        Layout::C => Command::new("lp")
            .arg("-n")
            .arg(copies.to_string())
            .arg(output_path)
            .output(),
    };

    match print_res {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Failed to print: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return Err(format!(
                    "Failed to print: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute print command: {}", e);
            return Err(format!("Failed to execute print command: {}", e));
        }
    }

    Ok(())
}

fn apply_layout_a(
    app_handle: &AppHandle,
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let branding_height = ((2f32 / 2.54) * DPI).round() as u32;

    // 2 vertical photos
    let available_height = HEIGHT - branding_height - (3 * border_px);

    let cell_height = available_height / 2;
    let cell_width = WIDTH - (2 * border_px);

    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

    for (i, img_path) in images.iter().enumerate().take(2) {
        let y_offset = border_px + (i as u32 * (cell_height + border_px));

        let photo = match image::open(img_path) {
            Ok(img) => {
                let img = if color_mode == "B&W" {
                    img.grayscale()
                } else {
                    img
                };

                let (orig_w, orig_h) = img.dimensions();

                let cell_aspect = cell_width as f32 / cell_height as f32;

                let img_aspect = orig_w as f32 / orig_h as f32;

                let (crop_x, crop_y, crop_w, crop_h) = if img_aspect > cell_aspect {
                    let new_w = (orig_h as f32 * cell_aspect).round() as u32;

                    let x = (orig_w - new_w) / 2;

                    (x, 0, new_w, orig_h)
                } else {
                    let new_h = (orig_w as f32 / cell_aspect).round() as u32;

                    let y = (orig_h - new_h) / 2;

                    (0, y, orig_w, new_h)
                };

                let cropped =
                    image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();

                image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                )
            }
            Err(e) => {
                eprintln!("Failed to open image {}: {}", img_path, e);

                return Err(format!("Failed to open image {}: {}", img_path, e));
            }
        };

        if let Err(e) = canvas.copy_from(&photo, border_px, y_offset) {
            eprintln!("photo error: {}", e);

            return Err(format!("photo error: {}", e));
        }
    }

    apply_overlay(app_handle, &mut canvas, "layout_a.png")?;

    Ok(canvas)
}

fn apply_layout_b(
    app_handle: &AppHandle,
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    // Internally work in landscape orientation (6x4)
    let landscape_width = HEIGHT;
    let landscape_height = WIDTH;

    let branding_height = ((1f32 / 2.54) * DPI).round() as u32;

    let available_height = landscape_height - branding_height - (2 * border_px);

    let cell_width = (landscape_width - (3 * border_px)) / 2;
    let cell_height = available_height / 2;

    let mut canvas = RgbaImage::from_pixel(landscape_width, landscape_height, bg_color);

    for (i, img_path) in images.iter().enumerate().take(4) {
        let y_offset = border_px + (i as u32 / 2) * (cell_height + border_px);
        let x_offset = border_px + (i as u32 % 2) * (cell_width + border_px);

        let photo = match image::open(img_path) {
            Ok(img) => {
                let img = if color_mode == "B&W" {
                    img.grayscale()
                } else {
                    img
                };

                let (orig_w, orig_h) = img.dimensions();

                let cell_aspect = cell_width as f32 / cell_height as f32;
                let img_aspect = orig_w as f32 / orig_h as f32;

                let (crop_x, crop_y, crop_w, crop_h) = if img_aspect > cell_aspect {
                    let new_w = (orig_h as f32 * cell_aspect).round() as u32;
                    let x = (orig_w - new_w) / 2;
                    (x, 0, new_w, orig_h)
                } else {
                    let new_h = (orig_w as f32 / cell_aspect).round() as u32;
                    let y = (orig_h - new_h) / 2;
                    (0, y, orig_w, new_h)
                };

                let cropped =
                    image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();

                image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                )
            }
            Err(e) => {
                eprintln!("Failed to open image {}: {}", img_path, e);
                return Err(format!("Failed to open image {}: {}", img_path, e));
            }
        };

        if let Err(e) = canvas.copy_from(&photo, x_offset, y_offset) {
            eprintln!("photo error: {}", e);
            return Err(format!("photo error: {}", e));
        }
    }

    // Rotate back into portrait 4x6
    let mut rotated = image::imageops::rotate90(&canvas);

    apply_overlay(app_handle, &mut rotated, "layout_b.png")?;

    Ok(rotated)
}

fn apply_layout_c(
    app_handle: &AppHandle,
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let center_gap = border_px * 2;
    let branding_height = ((1.0f32 / 2.54) * DPI).round() as u32;

    let available_height = HEIGHT - branding_height - (4 * border_px);
    let cell_width = (WIDTH - (2 * border_px) - center_gap) / 2;
    let cell_height = available_height / 4;

    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

    for (i, img_path) in images.iter().enumerate().take(4) {
        let y_offset = border_px + i as u32 * (cell_height + border_px);

        let photo = match image::open(img_path) {
            Ok(img) => {
                let img = if color_mode == "B&W" {
                    img.grayscale()
                } else {
                    img
                };

                let (orig_w, orig_h) = img.dimensions();
                let cell_aspect = cell_width as f32 / cell_height as f32;
                let img_aspect = orig_w as f32 / orig_h as f32;

                let (crop_x, crop_y, crop_w, crop_h) = if img_aspect > cell_aspect {
                    let new_w = (orig_h as f32 * cell_aspect).round() as u32;
                    let x = (orig_w - new_w) / 2;
                    (x, 0, new_w, orig_h)
                } else {
                    let new_h = (orig_w as f32 / cell_aspect).round() as u32;
                    let y = (orig_h - new_h) / 2;
                    (0, y, orig_w, new_h)
                };

                let cropped =
                    image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();
                image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                )
            }
            Err(e) => {
                eprintln!("Failed to open image {}: {}", img_path, e);
                return Err(format!("Failed to open image {}: {}", img_path, e));
            }
        };

        let left_x = border_px;
        let right_x = border_px + cell_width + center_gap;

        if let Err(e) = canvas.copy_from(&photo, left_x, y_offset) {
            eprintln!("Left photo error: {}", e);
            return Err(format!("Left photo error: {}", e));
        }

        if let Err(e) = canvas.copy_from(&photo, right_x, y_offset) {
            eprintln!("Right photo error: {}", e);
            return Err(format!("Right photo error: {}", e));
        }
    }

    apply_overlay(app_handle, &mut canvas, "layout_c.png")?;

    Ok(canvas)
}

fn apply_overlay(app_handle: &AppHandle, canvas: &mut RgbaImage, filename: &str) -> Result<(), String> {
    let overlay_path = get_asset_path(app_handle, filename)?;

    let overlay = image::open(overlay_path)
        .map_err(|e| format!("Failed to open overlay: {}", e))?
        .to_rgba8();

    match (overlay.width(), overlay.height()) {
        (1200, 1800) => {
            image::imageops::overlay(canvas, &overlay, 0, 0);
        }

        (600, 1800) => {
            image::imageops::overlay(canvas, &overlay, 0, 0);
            image::imageops::overlay(canvas, &overlay, 600, 0);
        }

        _ => {
            return Err(format!(
                "Unexpected overlay size {}x{}",
                overlay.width(),
                overlay.height()
            ));
        }
    }

    Ok(())
}

fn get_asset_path(app_handle: &AppHandle, filename: &str) -> Result<PathBuf, String> {
    let resource_path = app_handle.path().resolve(
        format!("assets/{}", filename),
        tauri::path::BaseDirectory::Resource,
    );
    if let Err(e) = resource_path {
        return Err(format!("Failed to find resource: {}", e));
    }

    Ok(resource_path.unwrap())
}
