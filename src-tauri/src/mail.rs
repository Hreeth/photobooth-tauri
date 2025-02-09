use dotenv::dotenv;
use reqwest::Client;
use serde_json::{json, to_string_pretty, Value};
use std::{env, error::Error, fs::{self, read, remove_file, rename, File, OpenOptions}, io::{Read, Write}, path::PathBuf};
use reqwest::multipart::{Form, Part};

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
pub async fn send_email(document_path: String) -> Result<String, String> {
    dotenv().ok();

    let api_key = env::var("MAILGUN_API_KEY").expect("Missing Mailgun API Key");

    let json_path: PathBuf = PathBuf::from(&document_path).join("emails.json");
    let mailgun_url = format!("https://api.mailgun.net/v3/{}/messages", "memorabooth.com");
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

        let photo_paths: Vec<&str> = photo_paths_arr
            .iter()
            .filter_map(|p| p.as_str())
            .collect();

        let mut form = Form::new()
            .text("from", "Memora Photobooth <memories@memoraphotobooth.com>")
            .text("to", user_email.to_string())
            .text("subject", "Your memories at the Memora Photobooth!")
            .text("html", "<p>Here are your photos from the photobooth!</p>");

        for path in &photo_paths {
            if let Ok(data) = read(path) {
                let filename = PathBuf::from(path)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown.png")
                    .to_string();

                let part = Part::bytes(data)
                    .file_name(filename.clone())
                    .mime_str("image/png")
                    .map_err(|e| format!("Failed to create attachment part: {}", e))?;

                form = form.part("attachment", part);
            }
        }

        let res = client
            .post(&mailgun_url)
            .basic_auth("api", Some(api_key.clone()))
            .multipart(form)
            .send()
            .await;

        match res {
            Ok(res) if res.status().is_success() => {
                successful_emails.push(email.clone());

                for photo_path in &photo_paths {
                    if let Err(e) = remove_file(photo_path) {
                        eprintln!("Failed to delete file {}: {}", photo_path, e);
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

    emails.retain(|email| !successful_emails.contains(&email));

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&json_path)
        .map_err(|e| format!("Failed to open file for writing: {}", e))?;

    file.write_all(to_string_pretty(&emails).unwrap().as_bytes())
        .map_err(|e| format!("Failed to update file: {}", e))?;

    Ok("Emails sent successfully".to_string())
}

fn format_files(document_path: String, user_email: String, photo_paths: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let email_prefix = user_email.split("@").next().unwrap_or("unknown");
    let storage_dir = PathBuf::from(&document_path).join("Memora Photobooth");

    fs::create_dir_all(&storage_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    let mut renamed_paths = Vec::new();

    for (index, photo_path) in photo_paths.iter().enumerate() {
        let new_filename = format!("{}_photo_{}.png", email_prefix, index + 1);
        let new_path = storage_dir.join(&new_filename);

        rename(photo_path, &new_path).map_err(|e| format!("Failed to rename file: {}", e))?;

        renamed_paths.push(new_path.to_string_lossy().to_string());
    }

    Ok(renamed_paths)
}