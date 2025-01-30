use std::{env, thread, time::Duration};

use tauri::{LogicalSize, Manager};

mod razorpay;
mod imaging;
 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      razorpay::create_qr,
      razorpay::check_payment_status,
      imaging::capture,
      imaging::print
    ])
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();
      let _ = window.hide();
      let _ = window.show();

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Trace)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}