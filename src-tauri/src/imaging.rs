use std::{fs, path::PathBuf, process::Command};

use ab_glyph::{FontArc, PxScale};
use chrono::Local;
use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use tauri::{AppHandle, Manager};

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
        .arg("--ev")
        .arg("0")
        .arg("--rotation")
        .arg("180")
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
    copies: usize
) -> Result<(), String> {
    let dpi = 300.0f32;

    let strip_width = (4f32 * dpi).round() as u32;
    let strip_height = (6f32 * dpi).round() as u32;

    let border_cm = 0.15f32;
    let border_px = ((border_cm / 2.54) * dpi).round() as u32;

    let center_gap = border_px * 2;
    let branding_height = ((1.0f32 / 2.54) * dpi).round() as u32;

    let available_height = strip_height - branding_height - (4 * border_px);
    let cell_width = (strip_width - (2 * border_px) - center_gap) / 2;
    let cell_height = available_height / 4;

    let bg_color = if color_mode == "B&W" {
        Rgba([0, 0, 0, 255])
    } else {
        Rgba([255, 255, 255, 255])
    };

    let mut canvas = RgbaImage::from_pixel(strip_width, strip_height, bg_color);

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

    let date_text = Local::now().format("%d.%m.%Y").to_string();
    let font_data = include_bytes!("../fonts/JMH Typewriter.ttf");
    let font = FontArc::try_from_slice(font_data as &[u8])
        .expect("Failed to load font");

    let scale = PxScale {
        x: 70.0,
        y: 70.0
    };

    let text_color = if color_mode == "B&W" {
        Rgba([255, 255, 255, 255])
    } else {
        Rgba([0, 0, 0, 255])
    };

    let padding_x = border_px + 155;
    let padding_y = border_px + 80;

    draw_text_mut(
        &mut canvas,
        text_color,
        padding_x.try_into().unwrap(),
        ((strip_height - padding_y)).try_into().unwrap(),
        scale,
        &font,
        &date_text
    );
    draw_text_mut(
        &mut canvas,
        text_color,
        padding_x.try_into().unwrap(),
        ((strip_height - padding_y)).try_into().unwrap(),
        scale,
        &font,
        &date_text
    );

    draw_text_mut(
        &mut canvas,
        text_color,
        (padding_x + cell_width + center_gap).try_into().unwrap(),
        ((strip_height - padding_y)).try_into().unwrap(),
        scale,
        &font,
        &date_text
    );

    if let Err(e) = canvas.save(output_path) {
        eprintln!("Failed to save image: {}", e);
        return Err(format!("Failed to save image: {}", e));
    }

    let mut canvas2 = RgbaImage::from_pixel(strip_width, strip_height, bg_color);

    let strip = match image::open(output_path) {
        Ok(img) => image::imageops::resize(
            &img,
            strip_width - (2 * border_px),
            strip_height - (2 * border_px),
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

    let print_res = Command::new("lp")
        .arg("-n")
        .arg((copies / 2).to_string())
        .arg(output_path)
        .output();

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

fn _get_asset_path(app_handle: &AppHandle, filename: &str) -> Result<PathBuf, String> {
    let resource_path = app_handle.path().resolve(format!("assets/{}", filename), tauri::path::BaseDirectory::Resource);
    if let Err(e) = resource_path {
        return Err(format!("Failed to find resource: {}", e))
    }

    Ok(resource_path.unwrap())
}