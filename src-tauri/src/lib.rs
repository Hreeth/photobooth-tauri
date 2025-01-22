use std::env;

use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RazorpayOrder {
  amount: u64,
  currency: String,
  receipt: String,
  payment_capture: bool,
  method: String
}

#[derive(Deserialize)]
struct RazorpayOrderResposne {
  id: String,
  status: String,
  amount: u64,
  currency: String,
  receipt: String,
  upi_qr_code: Option<String>
}

#[tauri::command]
async fn create_order(amount: u64, receipt: String) -> Result<String, String> {
  dotenv::dotenv().ok();

  let key_id = env::var("RAZORPAY_KEY_ID").expect("Missing Razorpay Key ID");
  let ket_secret = env::var("RAZORPAY_KEY_SECRET").expect("Missing Razorpay Key Secret");

  let url = "https://api.razorpay.com/v1/orders";
  let client = Client::new();

  let order_payload = RazorpayOrder {
    amount,
    currency: "INR".to_string(),
    receipt,
    payment_capture: true,
    method: "upi".to_string()
  };

  let res = client
    .post(url)
    .basic_auth(key_id, Some(ket_secret))
    .json(&order_payload)
    .send()
    .await
    .map_err(|e| format!("Request failed: {}", e))?;

  if !res.status().is_success() {
    return Err(format!("Failed to create order: HTTP {}", res.status()))
  }

  let order_res: RazorpayOrderResposne = res.json().await.map_err(|e| format!("Parse error: {}", e))?;

  if let Some(upi_qr_code) = order_res.upi_qr_code {
    Ok(upi_qr_code)
  } else {
    Err("UPI QR Code not available".to_string())
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
