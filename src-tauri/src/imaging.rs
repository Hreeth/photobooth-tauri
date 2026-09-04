use std::{path::PathBuf, process::Command};

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
            .arg("0.075,0.15,0.79,0.85")
            .arg("-p")
            .arg("-10,-10,1920,1080")
            .arg("-o")
            .arg(output_path);

        // if color_mode != "B&W" {
        //     cmd
        //         .arg("--awbgains")
        //         .arg("1.8,3.2");
        // }

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

            Err(e) => Err(format!("Failed to execute capture command: {}", e)),
        };
    }

    #[cfg(not(target_os = "linux"))]
    {
        let sample_path = "sample.jpg";

        return match std::fs::copy(sample_path, output_path) {
            Ok(_) => {
                println!("Sample image copied to: {}", output_path);
                Ok(output_path.to_string())
            }

            Err(e) => Err(format!("Failed to copy sample image: {}", e)),
        };
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
    let bg_color = Rgba([28, 42, 89, 255]);

    let border_px = ((BORDER / 2.54) * DPI).round() as u32;

    let branding_path = get_asset_path(&app, "branding.png")?;

    let canvas = match layout {
        Layout::A => apply_layout_a(images, color_mode, bg_color, border_px, &branding_path)?,

        Layout::B => apply_layout_b(images, color_mode, bg_color, border_px, &branding_path)?,

        Layout::C => apply_layout_c(images, color_mode, bg_color, border_px, &branding_path)?,
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

fn draw_branding(
    canvas: &mut RgbaImage,
    branding_path: &PathBuf,
    area_x: u32,
    area_y: u32,
    area_width: u32,
    area_height: u32,
) -> Result<(), String> {
    if area_width == 0 || area_height == 0 {
        return Ok(());
    }

    let branding = match image::open(branding_path) {
        Ok(img) => img.into_rgba8(),

        Err(e) => {
            eprintln!(
                "Failed to open branding image {}: {}",
                branding_path.display(),
                e
            );

            return Err(format!(
                "Failed to open branding image {}: {}",
                branding_path.display(),
                e
            ));
        }
    };

    let (orig_w, orig_h) = branding.dimensions();

    if orig_w == 0 || orig_h == 0 {
        return Ok(());
    }

    let padding_x = ((area_width as f32) * 0.05).round() as u32;
    let padding_y = ((area_height as f32) * 0.08).round() as u32;

    let max_width = area_width.saturating_sub(padding_x * 2);
    let max_height = area_height.saturating_sub(padding_y * 2);

    if max_width == 0 || max_height == 0 {
        return Ok(());
    }

    let scale_x = max_width as f32 / orig_w as f32;
    let scale_y = max_height as f32 / orig_h as f32;

    let scale = scale_x.min(scale_y);

    let new_width = ((orig_w as f32 * scale).round() as u32).max(1);
    let new_height = ((orig_h as f32 * scale).round() as u32).max(1);

    let resized = image::imageops::resize(
        &branding,
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3,
    );

    let x = area_x + (area_width.saturating_sub(new_width)) / 2;
    let y = area_y + (area_height.saturating_sub(new_height)) / 2;

    image::imageops::overlay(canvas, &resized, x as i64, y as i64);

    Ok(())
}

fn apply_layout_a(
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
    branding_path: &PathBuf,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let branding_height = ((2f32 / 2.54) * DPI).round() as u32;

    let available_height = HEIGHT - branding_height - (3 * border_px);

    let cell_height = available_height / 2;
    let cell_width = WIDTH - (2 * border_px);

    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

    for (i, img_path) in images.iter().enumerate().take(2) {
        let y_offset = border_px + (i as u32 * (cell_height + border_px));

        let photo = match image::open(img_path) {
            Ok(img) => {
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

                let mut resized = image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                );

                if color_mode == "B&W" {
                    for pixel in resized.pixels_mut() {
                        let [r, g, b, a] = pixel.0;

                        let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;

                        *pixel = Rgba([gray, gray, gray, a]);
                    }
                }

                resized
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

    let branding_start_y = HEIGHT - branding_height;

    draw_branding(
        &mut canvas,
        branding_path,
        0,
        branding_start_y,
        WIDTH,
        branding_height,
    )?;

    Ok(canvas)
}

fn apply_layout_b(
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
    branding_path: &PathBuf,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
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

                let mut resized = image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                );

                if color_mode == "B&W" {
                    for pixel in resized.pixels_mut() {
                        let [r, g, b, a] = pixel.0;

                        let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;

                        *pixel = Rgba([gray, gray, gray, a]);
                    }
                }

                resized
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

    let branding_start_y = landscape_height - branding_height;

    draw_branding(
        &mut canvas,
        branding_path,
        0,
        branding_start_y,
        landscape_width,
        branding_height,
    )?;

    let rotated = image::imageops::rotate90(&canvas);

    Ok(rotated)
}

fn apply_layout_c(
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32,
    branding_path: &PathBuf,
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

                let mut resized = image::imageops::resize(
                    &cropped,
                    cell_width,
                    cell_height,
                    image::imageops::FilterType::Lanczos3,
                );

                if color_mode == "B&W" {
                    for pixel in resized.pixels_mut() {
                        let [r, g, b, a] = pixel.0;

                        let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;

                        *pixel = Rgba([gray, gray, gray, a]);
                    }
                }

                resized
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

    let branding_start_y = HEIGHT - branding_height;

    let left_branding_x = border_px;

    let right_branding_x = border_px + cell_width + center_gap;

    draw_branding(
        &mut canvas,
        branding_path,
        left_branding_x,
        branding_start_y,
        cell_width,
        branding_height,
    )?;

    draw_branding(
        &mut canvas,
        branding_path,
        right_branding_x,
        branding_start_y,
        cell_width,
        branding_height,
    )?;

    Ok(canvas)
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
