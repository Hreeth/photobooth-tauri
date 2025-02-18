use std::{fs, path::{Path, PathBuf}, process::Command};

use image::{imageops::FilterType::Lanczos3, GenericImage, GenericImageView, Rgba, RgbaImage};

#[tauri::command(async)]
pub async fn capture(output_path: &str) -> Result<String, String> {
    let result = Command::new("libcamera-still")
        .arg("--saturation")
        .arg("1.5")
        .arg("-t")
        .arg("5000")
        .arg("--autofocus-mode")
        .arg("continuous")
        .arg("--autofocus-range")
        .arg("normal")
        .arg("--denoise")
        .arg("cdn_off")
        .arg("--ev")
        .arg("0")
        .arg("-f")
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
pub async fn print(images: Vec<String>, output_path: &str, color_mode: &str, copies: usize) -> Result<(), String> {
    let strip_width = 1200;  // Total width of the photostrip
    let strip_height = 1800; // Total height of the photostrip

    let gap_cm = 0.15; // 0.15 cm gap
    let dpi = 300; // Standard printing DPI
    let gap_px = ((gap_cm / 2.54) * dpi as f32).round() as u32; // Convert cm to pixels

    let column_gap = gap_px; // 0.15 cm gap between columns
    let row_gap = gap_px; // 0.15 cm gap between rows

    let cell_width = (strip_width - column_gap) / 2; // Each photo takes half of the width minus column gap
    let cell_height = (strip_height - (row_gap * 3)) / 4; // Adjust height to fit 4 images + gaps

    let bg_color = if color_mode == "B&W" {
        Rgba([0, 0, 0, 255]) // Black background
    } else {
        Rgba([255, 255, 255, 255]) // White background
    };

    let mut canvas = RgbaImage::from_pixel(strip_width, strip_height, bg_color);

    for (i, img_path) in images.iter().enumerate().take(4) { // Only process the first 4 images
        let y_offset = i as u32 * (cell_height + row_gap); // Offset including row gap

        let photo = match image::open(img_path) {
            Ok(img) => {
                println!("Loading image: {:?}", img_path);
                let (width, height) = img.dimensions();
                let aspect_ratio = width as f32 / height as f32;

                // Resize while keeping aspect ratio
                let mut resized_width = cell_width;
                let mut resized_height = (resized_width as f32 / aspect_ratio).round() as u32;

                if resized_height > cell_height {
                    resized_height = cell_height;
                    resized_width = (resized_height as f32 * aspect_ratio).round() as u32;
                }

                // Resize image
                let resized = img.resize(resized_width, resized_height, Lanczos3);

                // Create a blank image with correct size
                let mut bordered_image = RgbaImage::from_pixel(cell_width, cell_height, bg_color);

                // Center the image
                let x_offset_center = (cell_width - resized_width) / 2;
                let y_offset_center = (cell_height - resized_height) / 2;

                if let Err(e) = bordered_image.copy_from(&resized, x_offset_center, y_offset_center) {
                    return Err(format!("Failed to place photo {} in bordered image: {}", i + 1, e));
                }

                bordered_image
            }
            Err(e) => return Err(format!("Failed to load photo {}: {}", img_path, e)),
        };

        // Place images in both left and right columns with column gap
        let left_x_offset = 0;
        let right_x_offset = cell_width + column_gap;

        if let Err(e) = canvas.copy_from(&photo, left_x_offset, y_offset) {
            return Err(format!("Failed to place photo {} in left column: {}", i + 1, e));
        }
        if let Err(e) = canvas.copy_from(&photo, right_x_offset, y_offset) {
            return Err(format!("Failed to duplicate photo {} in right column: {}", i + 1, e));
        }
    }

    // Convert to B&W if needed
    if color_mode == "B&W" {
        for pixel in canvas.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            let gray = (r as u32 + g as u32 + b as u32) / 3;
            *pixel = Rgba([gray as u8, gray as u8, gray as u8, a]);
        }
    }

    // Save the final image
    if let Err(e) = canvas.save(output_path) {
        return Err(format!("Failed to save print copy: {}", e));
    }

    // Print the image
    let print_res = Command::new("lp")
        .arg("-n")
        .arg((copies / 2).to_string()) // Print full copies
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