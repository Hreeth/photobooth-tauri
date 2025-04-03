use ab_glyph::{FontArc, PxScale};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use image::{imageops::FilterType::Lanczos3, GenericImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use reqwest::Client;
use serde_json::{json, to_string_pretty, Value};
use std::{error::Error, fs::{self, read, remove_file, File, OpenOptions}, io::{Read, Write}, path::PathBuf};

#[tauri::command]
pub fn store_email(document_path: &str, user_email: String, photo_paths: Vec<String>) -> Result<String, String> {
    let json_path: PathBuf = PathBuf::from(document_path).join("emails.json");

    let new_photo_paths = format_files((document_path).to_string(), user_email.clone(), photo_paths);
    if let Err(e) = new_photo_paths {
        return Err(format!("Failed to process new paths: {}", e))
    }

    let mut emails: Vec<Value> = if json_path.exists() {
        let mut file = File::open(&json_path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
        serde_json::from_slice(&buffer).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    if emails.iter().any(|e| e["email"] == user_email) {
        return Err("Email already stored".to_string());
    };

    let new_email = json!({
        "email":  user_email,
        "photos": new_photo_paths.unwrap()
    });
    emails.push(new_email);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&json_path)
        .map_err(|e| format!("Failed to open file for writing: {}", e))?;

    file.write_all(to_string_pretty(&emails).unwrap().as_bytes()).map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok("Email stores successfully".to_string())
}

#[tauri::command(async)]
pub async fn send_email(document_path: String) ->  Result<String, String> {
    let document_path_clone = document_path.clone();
    
    let res = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(send_email_req(document_path_clone))
    })
    .await
    .unwrap_or_else(|e| Err(format!("Task execution failed: {}", e)))?;

    Ok(res)
}

async fn send_email_req(document_path: String) -> Result<String, String> {
    let api_key = dotenv_codegen::dotenv!("ZEPTOMAIL_API_KEY");

    let json_path: PathBuf = PathBuf::from(&document_path).join("emails.json");
    let zepto_url = "https://api.zeptomail.in/v1.1/email";

    let client = Client::new();

    let mut emails: Vec<Value> = if json_path.exists() {
        let mut file = File::open(&json_path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
        serde_json::from_slice(&buffer).unwrap_or_else(|_| vec![])
    } else {
        return Err("No pending emails.".to_string());
    };

    let mut successful_emails = vec![];

    for email in &emails {
        let user_email = email["email"].as_str().unwrap_or_default();
        let photo_paths_arr = email["photos"].as_array().unwrap_or(&Vec::new()).clone();

        let mut attachments = vec![];

        for path in photo_paths_arr.iter().filter_map(|p| p.as_str()) {
            if let Ok(data) = read(path) {
                let base64_encoded = BASE64_STANDARD.encode(data);
                let filename = PathBuf::from(path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown.png")
                    .to_string();

                attachments.push(json!({
                    "name": filename,
                    "content": base64_encoded,
                    "mime_type": "image/png"
                }));
            }
        }

        let email_data = json!({
            "from": {
                "address": "memories@memorabooth.com",
                "name": "Memorabooth"
            },
            "to": [{
                "email_address": {
                    "address": user_email
                }
            }],
            "subject": "Your memories at the Memora Photobooth!",
            "htmlbody": "<p>Here are your photos from the photobooth!</p>",
            "attachments": attachments
        });

        let res = client
            .post(zepto_url)
            .header("Authorization", format!("Zoho-enczapikey {}", api_key))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&email_data)
            .send()
            .await;

        match res {
            Ok(res) if res.status().is_success() => {
                successful_emails.push(email.clone());

                for path in photo_paths_arr.iter().filter_map(|p| p.as_str()) {
                    if let Err(e) = remove_file(path) {
                        eprintln!("Failed to delete file {}: {}", path, e);
                    }
                }
            }
            Ok(res) => {
                return Err(format!("Failed to send email: {:?}", res.text().await.unwrap_or_default()));
            }
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        }
    }

    emails.retain(|email| !successful_emails.contains(email));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&json_path)
        .map_err(|e| format!("Failed to open file for writing: {}", e))?;

    file.write_all(to_string_pretty(&emails).unwrap().as_bytes())
        .map_err(|e| format!("Failed to update file: {}", e))?;

    Ok("Emails sent successfully via ZeptoMail!".to_string())
}

fn format_files(document_path: String, user_email: String, photo_paths: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let email_prefix = user_email.split('@').next().unwrap_or("unknown");
    let storage_dir = PathBuf::from(&document_path).join("Memora Photobooth");

    fs::create_dir_all(&storage_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    let mut renamed_paths = Vec::new();
    let polaroid_size: u32 = 1280; // Size for polaroid image
    let border_width: u32 = 30;
    let collage_size = (1280, 1920); // Collage final size

    // Load a font (you should provide a path to a .ttf font file)
    let font_data = include_bytes!("../fonts/Spacetype - Garet Book.otf");
    let font = FontArc::try_from_slice(font_data as &[u8])?;

    let date_text = Local::now().format("%d-%m-%Y").to_string(); // Current date

    let mut polaroid_images: Vec<image::ImageBuffer<Rgba<u8>, Vec<u8>>> = vec![];

    for (index, photo_path) in photo_paths.iter().enumerate().take(4) {
        let new_filename = format!("{}_polaroid_{}.png", email_prefix, index + 1);
        let new_path = storage_dir.join(&new_filename);

        let img = image::open(photo_path).map_err(|e| format!("Failed to open photo: {}", e))?;
        let (width, height) = img.dimensions();
        let aspect_ratio = width as f32 / height as f32;

        let resized_width = polaroid_size - (2 * border_width);
        let resized_height = (resized_width as f32 / aspect_ratio) as u32;

        let resized = img.resize(resized_width, resized_height, Lanczos3);

        let polaroid_height = resized_height + (2 * border_width) + 120;

        // Create Polaroid-style canvas
        let mut polaroid = RgbaImage::from_pixel(polaroid_size, polaroid_height, Rgba([255, 255, 255, 255]));

        polaroid.copy_from(&resized, border_width, border_width).map_err(|e| format!("Failed to place photo: {}", e))?;

        // Add date (Red)
        draw_text_mut(
            &mut polaroid,
            Rgba([78, 52, 46, 255]), // Red
            (border_width + 20).try_into().unwrap(),
            (polaroid_height - border_width - 80).try_into().unwrap(),
            PxScale::from(70.0),
            &font,
            &date_text,
        );

        // Add "Memorabooth" (Blue)
        draw_text_mut(
            &mut polaroid,
            Rgba([78, 52, 46, 255]), // Blue
            ((polaroid_size - 600)).try_into().unwrap(),
            (polaroid_height - border_width - 80).try_into().unwrap(),
            PxScale::from(70.0),
            &font,
            "M E M O R A B O O T H",
        );

        // Save the polaroid image
        polaroid.save(&new_path).map_err(|e| format!("Failed to save polaroid image: {}", e))?;
        renamed_paths.push(new_path.to_string_lossy().to_string());
        polaroid_images.push(polaroid);
    }

    // Create the final collage
    let gap_px = 20;
    let padded_collage_size = (collage_size.0 + (2 * gap_px), collage_size.1 + (2 * gap_px));
    let collage_path = storage_dir.join(format!("{}_collage.png", email_prefix));
    let mut collage = RgbaImage::from_pixel(padded_collage_size.0, padded_collage_size.1, Rgba([255, 255, 255, 255]));

    let cell_width = (collage_size.0 - gap_px) / 2;
    let cell_height = (collage_size.1 - 100 - (gap_px * 3)) / 4;

    for (i, photo) in photo_paths.iter().enumerate() {
        let y_offset = gap_px + (i as u32 * (cell_height + gap_px));

        let img = image::open(photo).map_err(|e| format!("Failed to load image: {}", e))?;
        let resized = image::imageops::resize(&img, cell_width, cell_height, Lanczos3);

        let left_x_offset = gap_px;
        let right_x_offset = cell_width + (2 * gap_px);

        collage.copy_from(&resized, left_x_offset, y_offset)
            .map_err(|e| format!("Failed to place photo in left column: {}", e))?;
        collage.copy_from(&resized, right_x_offset, y_offset)
            .map_err(|e| format!("Failed to place photo in right column: {}", e))?;
    }

    // Add "Memorabooth" text at the bottom, centered
    draw_text_mut(
        &mut collage,
        Rgba([78, 52, 46, 255]), // Blue
        ((padded_collage_size.0 / 2) - 250).try_into().unwrap(),
        (padded_collage_size.1 - 80 - gap_px).try_into().unwrap(),
        PxScale::from(60.0),
        &font,
        "M E M O R A B O O T H",
    );

    // Save the collage
    collage.save(&collage_path).map_err(|e| format!("Failed to save collage: {}", e))?;
    renamed_paths.push(collage_path.to_string_lossy().to_string());

    Ok(renamed_paths)
}