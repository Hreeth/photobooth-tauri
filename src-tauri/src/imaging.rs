use std::process::Command;

use image::{imageops::FilterType::Lanczos3, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use palette::{FromColor, Hsl, IntoColor, Srgb};
use tauri::path;

#[tauri::command(async)]
pub async fn capture(output_path: &str) -> Result<String, String> {
    let result = Command::new("libcamera-still")
        .arg("--saturation")
        .arg("1.2")
        .arg("-t")
        .arg("0")
        .arg("--quality")
        .arg("100")
        .arg("--immediate")
        .arg("--nopreview")
        .arg("-o")
        .arg(output_path)
        .output();

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

#[tauri::command(async)]
pub async fn print(images: Vec<String>, output_path: &str, color_mode: &str, copies: usize) -> Result<(), String> {
    let strip_width = 1200;
    let strip_height = 1800;

    let border_width = 10;

    let mut canvas = RgbaImage::from_pixel(strip_width, strip_height, image::Rgba([255, 255, 255, 255]));

    for (i, img_path) in images.iter().enumerate() {
        let row = i / 2;
        let col = i % 2;

        let photo = match image::open(img_path) {
            Ok(img) => {
                println!("{:?}", img_path);
                let (width, height) = img.dimensions();
                let aspect_ratio = width as f32 / height as f32;

                let resized_width = (strip_width / 2) - 2 * border_width;
                let resized_height = (resized_width as f32 / aspect_ratio) as u32;

                let resized = img.resize(resized_width, resized_height, Lanczos3);

                let mut bordered_image = RgbaImage::from_pixel(resized_width, resized_height, Rgba([255, 255, 255, 255]));
                bordered_image.copy_from(&resized, border_width as u32, border_width as u32).unwrap();

                bordered_image
            }
            Err(e) => return Err(format!("Failed to load photo {}: {}", img_path, e))
        };

        let x_offset = col as u32 * (strip_width / 2);
        let y_offset = row as u32 * (strip_height / 2);

        if let Err(e) = canvas.copy_from(&photo, x_offset, y_offset) {
            return Err(format!("Failed to place photo {}: {}", i + 1, e));
        }
    }

    if color_mode == "B&W" {
        for pixel in canvas.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let gray = (r as u32 + g as u32 + b as u32) / 3;
            *pixel = Rgba([gray as u8, gray as u8, gray as u8, a]);
        }
    }

    // Save the final image to the output path
    if let Err(e) = canvas.save(output_path) {
        return Err(format!("Failed to save print copy: {}", e));
    }

    // Print the image (half the number of copies for two-sided printing)
    let print_res = Command::new("lp")
        .arg("-n")
        .arg((copies / 2).to_string())
        .arg(output_path)
        .output();

    match print_res {
        Ok(output) => {
            if !output.status.success() {
                return Err(format!(
                    "Failed to print: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        Err(e) => return Err(format!("Failed to execute print command: {}", e)),
    }

    Ok(())
}