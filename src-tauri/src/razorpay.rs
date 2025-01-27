use std::env;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RazorpayQrRequest {
  #[serde(rename = "type")]
  pub qr_type: String,
  pub usage: String,
  pub fixed_amount: bool,
  pub payment_amount: u64,
  pub close_by: u64
}

#[derive(Serialize, Deserialize)]
pub struct RazorpayQrResponse {
  pub id: String,
  pub image_url: String,
  pub close_by: u64
}

#[derive(Deserialize)]
pub struct RazorpayPollingResponse {
  pub close_reason: Option<String>
}

#[tauri::command(async)]
pub async fn create_qr(amount: u64, close_by_secs: i64) -> Result<RazorpayQrResponse, String> {
  dotenv().ok();

  let key_id = env::var("RAZORPAY_KEY_ID").expect("Missing Razorpay Key ID");
  let key_secret = env::var("RAZORPAY_KEY_SECRET").expect("Missing Razorpay Key Secret");

  let url = "https://api.razorpay.com/v1/payments/qr_codes";
  let client = Client::new();

  let close_by = (Utc::now() + Duration::seconds(close_by_secs)).timestamp() as u64;

  let qr_payload = RazorpayQrRequest {
    qr_type: "upi_qr".to_string(),
    usage: "single_use".to_string(),
    fixed_amount: true,
    payment_amount: amount,
    close_by
  };

  let res = client
    .post(url)
    .basic_auth(key_id, Some(key_secret))
    .json(&qr_payload)
    .send()
    .await
    .map_err(|e| format!("Payment failed: {}", e))?;

  if !res.status().is_success() {
      return Err(format!("Failed to create QR code: {}", res.text().await.unwrap()));
  }

  let qr_res: RazorpayQrResponse = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
  Ok(qr_res)
}

#[tauri::command(async)]
pub async fn check_payment_status(qr_code_id: String) -> Result<bool, String> {
  dotenv().ok();

  let key_id = env::var("RAZORPAY_KEY_ID").expect("Missing Razorpay Key ID");
  let key_secret = env::var("RAZORPAY_KEY_SECRET").expect("Missing Razorpay Key Secret");

  let url = format!("https://api.razorpay.com/v1/payments/qr_codes/{}", qr_code_id);
  let client = Client::new();

  let res = client
    .get(url)
    .basic_auth(key_id, Some(key_secret))
    .send()
    .await
    .map_err(|e| format!("Failed to fetch payment details: {}", e))?;

  if !res.status().is_success() {
    return Err(format!("Failed to create QR code: {}", res.text().await.unwrap()));
  }

  let res_data: RazorpayPollingResponse = res.json().await.map_err(|e| format!("Parse error: {}", e))?;

  match res_data.close_reason.as_deref() {
    Some("paid") => Ok(true),
    _ => Ok(false)
  }
}