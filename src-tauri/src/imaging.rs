use std::{fs, path::PathBuf, process::Command};

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use chrono::Local;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
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
    C
}


#[tauri::command(async)]
pub async fn capture(
    output_path: &str,
    color_mode: &str
) -> Result<String, String> {
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
    
    // if color_mode != "B&W" {
    //     cmd
    //         .arg("--awbgains")
    //         .arg("1.8,3.2");
    // }
        
    let result = cmd.output();

    match result {
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

// #[tauri::command]
// pub fn capture(output_path: &str) -> Result<String, String> {
//     let sample_path = "sample.jpg"; // Replace with the actual path of your sample image

//     match fs::copy(sample_path, output_path) {
//         Ok(_) => {
//             println!("Sample image copied to: {}", output_path);
//             Ok(output_path.to_string())
//         }
//         Err(e) => Err(format!("Failed to copy sample image: {}", e)),
//     }
// }

#[tauri::command(async)]
pub async fn print(
    _app: AppHandle,
    images: Vec<String>,
    output_path: &str,
    color_mode: &str,
    copies: usize,
    layout: Layout
) -> Result<(), String> {
    let bg_color = if color_mode == "B&W" {
        Rgba([0, 0, 0, 255])
    } else {
        Rgba([255, 255, 255, 255])
    };
    
    let border_px = ((BORDER / 2.54) * DPI).round() as u32;

    let canvas = match layout {
        Layout::A => apply_layout_a(images, color_mode, bg_color, border_px)?,
        Layout::B => apply_layout_b(images, color_mode, bg_color, border_px)?,
        Layout::C => apply_layout_c(images, color_mode, bg_color, border_px)?,
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
        Layout::A | Layout::B => {
            Command::new("lp")
                .arg("-o")
                .arg("media=w288h432")
                .arg("-o")
                .arg("fit-to-page")
                .arg("-n")
                .arg(copies.to_string())
                .arg(output_path)
                .output()
        },
        Layout::C => {
            Command::new("lp")
                .arg("-n")
                .arg(copies.to_string())
                .arg(output_path)
                .output()
        }
    };

    match print_res {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Failed to print: {}", String::from_utf8_lossy(&output.stderr));
                return Err(format!("Failed to print: {}", String::from_utf8_lossy(&output.stderr)));
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
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let branding_height = ((2f32 / 2.54) * DPI).round() as u32;

    let cell_height = HEIGHT - branding_height - border_px;
    let cell_width = WIDTH - (2 * border_px);

    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

    let photo = match image::open(&images[0]) {
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

            let cropped = image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();
            image::imageops::resize(
                &cropped,
                cell_width,
                cell_height,
                image::imageops::FilterType::Lanczos3,
            )
        }
        Err(e) => {
            eprintln!("Failed to open image {}: {}", &images[0], e);
            return Err(format!("Failed to open image {}: {}", &images[0], e));
        }
    };
    
    if let Err(e) = canvas.copy_from(&photo, border_px, border_px) {
        eprintln!("photo error: {}", e);
        return Err(format!("photo error: {}", e));
    }

    if color_mode == "B&W" {
        for pixel in canvas.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            *pixel = Rgba([gray, gray, gray, a]);
        }
    }

    let date = Local::now().format("%d.%m.%Y").to_string();
    let label = "MEMORABOOTH".to_string();
    let label_font_src = include_bytes!("../fonts/Futura.ttf");
    let data_font_src = include_bytes!("../fonts/JMH Typewriter.ttf");
    let label_font = FontArc::try_from_slice(label_font_src as &[u8])
        .expect("Failed to load font");
    let date_font = FontArc::try_from_slice(data_font_src as &[u8])
        .expect("Failed to load font");

    let label_scale: PxScale = PxScale {
        x: 100.0,
        y: 100.0
    };
    let date_scale: PxScale = PxScale {
        x: 80.0,
        y: 80.0
    };

    let text_color = if color_mode == "B&W" {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    };

    let date_width: f32 = date.chars().map(|c| {
        let glyph_id = date_font.glyph_id(c);
        date_font.as_scaled(date_scale.y).h_advance(glyph_id)
    }).sum();
    let label_width: f32 = label.chars().map(|c| {
        let glyph_id = label_font.glyph_id(c);
        label_font.as_scaled(label_scale.y).h_advance(glyph_id)
    }).sum();

    let date_x = ((WIDTH as f32 - date_width) / 2.0) as i32;
    let label_x = ((WIDTH as f32 - label_width) / 2.0) as i32;

    let branding_start_y = HEIGHT - branding_height;
    let label_y = (branding_start_y + 20) as i32;
    let date_y = (branding_start_y + 110) as i32;

    
    // Draw title first (above)
    draw_text_mut(
        &mut canvas,
        text_color,
        label_x,
        label_y,
        label_scale,
        &label_font,
        &label
    );

    // Draw date below
    draw_text_mut(
        &mut canvas,
        text_color,
        date_x,
        date_y,
        date_scale,
        &date_font,
        &date
    );
    
    Ok(canvas)
}

fn apply_layout_b(
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let branding_height = ((2f32 / 2.54) * DPI).round() as u32;

    let available_height = HEIGHT - branding_height - (2 * border_px);
    let cell_width = (WIDTH - (3 * border_px)) / 2;
    let cell_height = available_height / 2;

    let mut canvas = RgbaImage::from_pixel(WIDTH, HEIGHT, bg_color);

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

                let cropped = image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();
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

    if color_mode == "B&W" {
        for pixel in canvas.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            *pixel = Rgba([gray, gray, gray, a]);
        }
    }

    let date = Local::now().format("%d.%m.%Y").to_string();
    let label = "MEMORABOOTH".to_string();
    let label_font_src = include_bytes!("../fonts/Futura.ttf");
    let data_font_src = include_bytes!("../fonts/JMH Typewriter.ttf");
    let label_font = FontArc::try_from_slice(label_font_src as &[u8])
        .expect("Failed to load font");
    let date_font = FontArc::try_from_slice(data_font_src as &[u8])
        .expect("Failed to load font");

    let label_scale: PxScale = PxScale {
        x: 100.0,
        y: 100.0
    };
    let date_scale: PxScale = PxScale {
        x: 80.0,
        y: 80.0
    };

    let text_color = if color_mode == "B&W" {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    };

    let date_width: f32 = date.chars().map(|c| {
        let glyph_id = date_font.glyph_id(c);
        date_font.as_scaled(date_scale.y).h_advance(glyph_id)
    }).sum();
    let label_width: f32 = label.chars().map(|c| {
        let glyph_id = label_font.glyph_id(c);
        label_font.as_scaled(label_scale.y).h_advance(glyph_id)
    }).sum();

    let date_x = ((WIDTH as f32 - date_width) / 2.0) as i32;
    let label_x = ((WIDTH as f32 - label_width) / 2.0) as i32;

    let branding_start_y = HEIGHT - branding_height;
    let label_y = (branding_start_y + 20) as i32;
    let date_y = (branding_start_y + 110) as i32;

    
    // Draw title first (above)
    draw_text_mut(
        &mut canvas,
        text_color,
        label_x,
        label_y,
        label_scale,
        &label_font,
        &label
    );

    // Draw date below
    draw_text_mut(
        &mut canvas,
        text_color,
        date_x,
        date_y,
        date_scale,
        &date_font,
        &date
    );
    
    Ok(canvas)
}

fn apply_layout_c(
    images: Vec<String>,
    color_mode: &str,
    bg_color: Rgba<u8>,
    border_px: u32
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

                let cropped = image::imageops::crop_imm(&img, crop_x, crop_y, crop_w, crop_h).to_image();
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

    if color_mode == "B&W" {
        for pixel in canvas.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
            *pixel = Rgba([gray, gray, gray, a]);
        }
    }

    let label = "MEMORABOOTH".to_string();
    let label_font_src = include_bytes!("../fonts/Futura.ttf");
    let label_font = FontArc::try_from_slice(label_font_src as &[u8])
        .expect("Failed to load font");

    let label_scale: PxScale = PxScale {
        x: 60.0,
        y: 60.0
    };

    let text_color = if color_mode == "B&W" {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    };

    let label_width: f32 = label.chars().map(|c| {
        let glyph_id = label_font.glyph_id(c);
        label_font.as_scaled(label_scale.y).h_advance(glyph_id)
    }).sum();

    let label_x = ((cell_width as f32 - label_width) / 2.0) as i32;

    let branding_start_y = HEIGHT - branding_height;
    let label_y = (branding_start_y + 6) as i32;

    
    // Draw title first
    draw_text_mut(
        &mut canvas,
        text_color,
        label_x,
        label_y,
        label_scale,
        &label_font,
        &label
    );
    draw_text_mut(
        &mut canvas,
        text_color,
        label_x + (cell_width + center_gap) as i32,
        label_y,
        label_scale,
        &label_font,
        &label
    );

    
    Ok(canvas)
}



fn _get_asset_path(app_handle: &AppHandle, filename: &str) -> Result<PathBuf, String> {
    let resource_path = app_handle.path().resolve(format!("assets/{}", filename), tauri::path::BaseDirectory::Resource);
    if let Err(e) = resource_path {
        return Err(format!("Failed to find resource: {}", e))
    }

    Ok(resource_path.unwrap())
}