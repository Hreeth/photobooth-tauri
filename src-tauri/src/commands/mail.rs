use ab_glyph::{FontArc, PxScale};
use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::Local;
use image::{GenericImage, GenericImageView, Rgba, RgbaImage, imageops::FilterType::Lanczos3};
use imageproc::drawing::draw_text_mut;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{Value, json, to_string_pretty};
use std::{
    error::Error,
    fs::{self, File, OpenOptions, read, remove_file},
    io::{Read, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use uuid::Uuid;

use crate::{
    Result, imaging::LayoutMode, state::AppState, utils::{assets_dir, output_dir, remove_bleed},
};

static IS_SENDING: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

const TEMPLATE_KEY: &str =
    "2518b.45ebd6f14385fb13.k1.2d7bc653-223a-11f1-8c35-cabf48e1bf81.19cfd571733";

fn emails_path() -> PathBuf {
    assets_dir().join("emails.json")
}

#[tauri::command]
pub fn store_email(state: tauri::State<Arc<AppState>>, user_email: String) -> Result<String> {
    let state = state.inner().clone();

    tauri::async_runtime::spawn(async move {
        let result = (|| -> Result<()> {
            let (photo_paths, final_path, session_id, layout_mode) = {
                let session = state.session.lock().unwrap();

                let final_path =
                    session.r#final.clone().ok_or("fatal: final image not available")?;

                let options =
                    session.options.as_ref().ok_or("fatal: session options not available")?;

                (session.photos.clone(), final_path, session.session_id, options.layout.mode())
            };

            store_email_req(user_email, photo_paths, final_path, session_id, layout_mode)
        })();

        if let Err(e) = result {
            eprintln!("Failed to store email: {e}");
            return;
        }

        if IS_SENDING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return;
        }

        let res = send_email_req().await;

        IS_SENDING.store(false, Ordering::SeqCst);

        if let Err(e) = res {
            eprintln!("Failed to send emails: {e}");
        }
    });

    Ok("Email stored successfully".to_string())
}

fn store_email_req(
    user_email: String,
    photo_paths: Vec<PathBuf>,
    final_path: PathBuf,
    session_id: Uuid,
    layout_mode: LayoutMode,
) -> Result<()> {
    let new_photo_paths =
        format_files(user_email.clone(), photo_paths, final_path, session_id, layout_mode)
            .map_err(|e| format!("Failed to process new paths: {e}"))?;

    let json_path = emails_path();

    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let mut emails: Vec<Value> = if json_path.exists() {
        let mut file = File::open(&json_path).map_err(|e| format!("Failed to open file: {e}"))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {e}"))?;

        serde_json::from_slice(&buffer).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    if emails.iter().any(|e| e["email"] == user_email) {
        return Err("Email already stored".to_string());
    }

    emails.push(json!({
        "email": user_email,
        "photos": new_photo_paths,
    }));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&json_path)
        .map_err(|e| format!("Failed to open file for writing: {e}"))?;

    file.write_all(
        to_string_pretty(&emails)
            .map_err(|e| format!("Failed to serialize emails: {e}"))?
            .as_bytes(),
    )
    .map_err(|e| format!("Failed to write to file: {e}"))?;

    Ok(())
}

#[tauri::command]
pub fn send_email() -> Result<String> {
    if IS_SENDING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("Email sending already in progress.".to_string());
    }

    tauri::async_runtime::spawn(async move {
        let res = send_email_req().await;

        IS_SENDING.store(false, Ordering::SeqCst);

        if let Err(e) = res {
            eprintln!("Failed to send emails: {e}");
        }
    });

    Ok("Started sending emails".into())
}

async fn send_email_req() -> Result<String> {
    let api_key = dotenv_codegen::dotenv!("ZEPTOMAIL_API_KEY");

    let json_path = emails_path();

    if !json_path.exists() {
        return Err("No pending emails.".to_string());
    }

    let zepto_url = "https://api.zeptomail.in/v1.1/email/template";

    let client = Client::new();

    let mut file = File::open(&json_path).map_err(|e| format!("Failed to open file: {e}"))?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {e}"))?;

    let mut emails: Vec<Value> = serde_json::from_slice(&buffer).unwrap_or_else(|_| vec![]);

    if emails.is_empty() {
        return Err("No pending emails.".to_string());
    }

    let mut successful_emails = vec![];

    for email in &emails {
        let user_email = email["email"].as_str().unwrap_or_default();

        let photo_paths_arr = email["photos"].as_array().cloned().unwrap_or_default();

        let mut attachments = vec![];

        for path in photo_paths_arr.iter().filter_map(|p| p.as_str()) {
            if let Ok(data) = read(path) {
                let base64_encoded = BASE64_STANDARD.encode(data);

                let path_buf = PathBuf::from(path);

                let full_name =
                    path_buf.file_name().and_then(|f| f.to_str()).unwrap_or("unknown.png");

                let filename_slice = full_name.get(37..).unwrap_or(full_name);

                attachments.push(json!({
                    "name": filename_slice,
                    "content": base64_encoded,
                    "mime_type": "image/png"
                }));
            }
        }

        let email_data = json!({
            "template_key": TEMPLATE_KEY,
            "from": {
                "address": "memories@memorabooth.com",
                "name": "Memorabooth"
            },
            "to": [{
                "email_address": {
                    "address": user_email
                }
            }],
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
                return Err(format!(
                    "Failed to send email: {}",
                    res.text().await.unwrap_or_default()
                ));
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
        .map_err(|e| format!("Failed to open file for writing: {e}"))?;

    file.write_all(
        to_string_pretty(&emails)
            .map_err(|e| format!("Failed to serialize emails: {e}"))?
            .as_bytes(),
    )
    .map_err(|e| format!("Failed to update file: {e}"))?;

    Ok("Emails sent successfully via ZeptoMail!".to_string())
}

fn format_files(
    user_email: String,
    photo_paths: Vec<PathBuf>,
    final_path: PathBuf,
    session_id: Uuid,
    layout_mode: LayoutMode,
) -> Result<Vec<String>, Box<dyn Error>> {
    let email_prefix = user_email.split('@').next().unwrap_or("unknown");

    let storage_dir = output_dir();

    fs::create_dir_all(&storage_dir)?;

    let mut formatted_paths = Vec::new();

    let polaroid_size: u32 = 1280;
    let border_width: u32 = 30;

    let font_data = include_bytes!("../../fonts/Spacetype - Garet Book.otf");

    let font = FontArc::try_from_slice(font_data as &[u8])?;

    let date_text = Local::now().format("%d-%m-%Y").to_string();

    let mut polaroid_images: Vec<RgbaImage> = Vec::new();

    for (index, photo_path) in photo_paths.iter().enumerate() {
        let new_filename = format!("{}_{}_polaroid_{}.png", session_id, email_prefix, index + 1);

        let new_path = storage_dir.join(&new_filename);

        let img = image::open(photo_path)?;

        let (width, height) = img.dimensions();

        let aspect_ratio = width as f32 / height as f32;

        let resized_width = polaroid_size - (2 * border_width);

        let resized_height = (resized_width as f32 / aspect_ratio) as u32;

        let resized = img.resize(resized_width, resized_height, Lanczos3);

        let polaroid_height = resized_height + (2 * border_width) + 120;

        let mut polaroid =
            RgbaImage::from_pixel(polaroid_size, polaroid_height, Rgba([255, 255, 255, 255]));

        polaroid.copy_from(&resized, border_width, border_width)?;

        draw_text_mut(
            &mut polaroid,
            Rgba([78, 52, 46, 255]),
            (border_width + 20) as i32,
            (polaroid_height - border_width - 80) as i32,
            PxScale::from(70.0),
            &font,
            &date_text,
        );

        draw_text_mut(
            &mut polaroid,
            Rgba([78, 52, 46, 255]),
            (polaroid_size - 600) as i32,
            (polaroid_height - border_width - 80) as i32,
            PxScale::from(70.0),
            &font,
            "M E M O R A B O O T H",
        );

        polaroid.save(&new_path)?;

        formatted_paths.push(new_path.to_string_lossy().to_string());

        polaroid_images.push(polaroid);
    }

    let final_filename = format!("{}_{}_final.png", session_id, email_prefix);

    let final_output_path = storage_dir.join(final_filename);

    let final_image = image::open(&final_path)?;

    let final_image = remove_bleed(final_image, layout_mode)?;

    final_image.save(&final_output_path)?;

    formatted_paths.push(final_output_path.to_string_lossy().to_string());

    Ok(formatted_paths)
}
