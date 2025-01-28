use std::{fs, process::Command};

use image::{imageops::FilterType::Lanczos3, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use palette::{FromColor, Hsl, IntoColor, Srgb};

#[tauri::command(async)]
pub async fn capture(output_path: &str) -> Result<String, String> {
    // Use the Tauri application's resource directory or a default placeholder image
    let sample_photo_path = "sample.jpg"; // Ensure this file exists in your app directory
    
    if let Err(e) = fs::copy(sample_photo_path, output_path) {
        return Err(format!("Failed to take photo: {}", e));
    }

    Ok(output_path.to_string())
    // let result = Command::new("libcamera-still")
    //     .arg("-o")
    //     .arg(output_path)
    //     .arg("--immediate")
    //     .arg("--saturation")
    //     .arg("1.2")
    //     .arg("--quality")
    //     .arg("100")
    //     .output();

    // match result {
    //     Ok(output) => {
    //         let stdout_str = String::from_utf8_lossy(&output.stdout);
    //         let stderr_str = String::from_utf8_lossy(&output.stderr);
    
    //         if !output.status.success() {
    //             println!("stderr: {}", stderr_str);
    //         }
            
    //         println!("stdout: {}", stdout_str);
    //         Ok(output_path.to_string())
    //     }
    //     Err(e) => return Err(format!("Failed to execute print command: {}", e)),
    // }
}

#[tauri::command(async)]
pub async fn print(images: Vec<String>, color_mode: &str, copies: usize) -> Result<(), String> {
    let strip_width = 600;
    let strip_height = 1800;

    let border_width = 10;

    let mut canvas = RgbaImage::from_pixel(strip_width, strip_height, image::Rgba([255, 255, 255, 255]));

    for (i, img_path) in images.iter().enumerate() {
        let photo = match image::open(img_path) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let aspect_ratio = width as f32 / height as f32;

                let resized_width = strip_width - 2 * border_width;

                let resized = img.resize(resized_width, (resized_width as f32 / aspect_ratio) as u32, Lanczos3).crop(0, 0, strip_width, strip_height / 4 - 2 * border_width);

                let mut bordered_image = RgbaImage::from_pixel(strip_width, strip_height / 4, Rgba([255, 255, 255, 255]));
                bordered_image.copy_from(&resized, border_width  as u32, border_width as u32).unwrap();

                bordered_image
            }
            Err(e) => return Err(format!("Failed to load photo {}: {}", img_path, e))
        };

        let y_offset = i as u32 * (strip_height / 4);
        if let Err(e) = canvas.copy_from(&photo, 0, y_offset) {
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

    if let Err(e) = canvas.save("print_strip.png") {
        return Err(format!("Failed to save print copy: {}", e));
    }

    let print_res = Command::new("lp")
        .arg("-n")
        .arg((copies / 2).to_string())
        .arg("print_strip.png")
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

fn saturate(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in image.pixels_mut() {
        let rgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0
        );

        let mut hsl: Hsl = rgb.into_color();

        hsl.saturation = (hsl.saturation * 1.2).min(1.0);

        let adjusted_rgb = Srgb::from_color(hsl);
        pixel[0] = (adjusted_rgb.red * 255.0).round() as u8;
        pixel[1] = (adjusted_rgb.green * 255.0).round() as u8;
        pixel[2] = (adjusted_rgb.blue * 255.0).round() as u8;
    }
}