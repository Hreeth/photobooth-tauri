use std::process::Command;

use image::{imageops::FilterType::Lanczos3, GenericImage, GenericImageView, Rgba, RgbaImage};

#[tauri::command(async)]
pub async fn capture(output_path: &str) -> Result<String, String> {
    let result = Command::new("libcamera-still")
        .arg("--saturation")
        .arg("1.5")
        .arg("-t")
        .arg("4000")
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

#[tauri::command(async)]
pub async fn print(images: Vec<String>, output_path: &str, color_mode: &str, copies: usize) -> Result<(), String> {
    let strip_width = 1200;
    let strip_height = 1800;
    
    let border_width = 20; // Increased border to ensure symmetry
    let cell_width = (strip_width / 2) - (2 * border_width); // 600px minus borders
    let cell_height = (strip_height / 4) - (2 * border_width); // 450px minus borders

    let mut canvas = RgbaImage::from_pixel(strip_width, strip_height, image::Rgba([255, 255, 255, 255]));

    for (i, img_path) in images.iter().enumerate() {
        let y_offset = i as u32 * (strip_height / 4); // Row position

        let photo = match image::open(img_path) {
            Ok(img) => {
                println!("Loading image: {:?}", img_path);
                let (width, height) = img.dimensions();
                let aspect_ratio = width as f32 / height as f32;

                // Resize while keeping the image within bounds
                let mut resized_width = cell_width;
                let mut resized_height = (resized_width as f32 / aspect_ratio) as u32;

                if resized_height > cell_height {
                    resized_height = cell_height;
                    resized_width = (resized_height as f32 * aspect_ratio) as u32;
                }

                // Resize the image to fit exactly within its space
                let resized = img.resize(resized_width, resized_height, Lanczos3);

                // Create a blank bordered image with correct dimensions
                let mut bordered_image = RgbaImage::from_pixel(cell_width, cell_height, Rgba([255, 255, 255, 255]));

                // Ensure the image is centered in its slot
                let x_offset_center = (cell_width - resized_width) / 2;
                let y_offset_center = (cell_height - resized_height) / 2;

                if let Err(e) = bordered_image.copy_from(&resized, x_offset_center, y_offset_center) {
                    return Err(format!("Failed to place photo {} in bordered image: {}", i + 1, e));
                }

                bordered_image
            }
            Err(e) => return Err(format!("Failed to load photo {}: {}", img_path, e)),
        };

        // Ensure equal left and right placement without exceeding dimensions
        let left_x_offset = border_width;
        let right_x_offset = strip_width / 2 + border_width;

        // Check that right_x_offset does not exceed bounds
        if right_x_offset + cell_width > strip_width {
            return Err(format!("Error: Right column image exceeds strip width."));
        }

        // Place images in both left and right columns
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


    // Save the final image to the output path
    if let Err(e) = canvas.save(output_path) {
        return Err(format!("Failed to save print copy: {}", e));
    }

    // Print the image
    let print_res = Command::new("lp")
        .arg("-n")
        .arg((copies / 2).to_string()) // Full copies since it's a single page
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