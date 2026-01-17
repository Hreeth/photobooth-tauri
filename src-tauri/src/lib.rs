use std::{thread, time::Duration};
use tauri::Manager; 

mod razorpay;
mod mail;
mod imaging;
mod config;
 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      razorpay::create_qr,
      razorpay::check_payment_status,
      imaging::capture,
      imaging::print,
      mail::store_email,
      mail::send_email,
      config::save_pricing,
      config::get_or_init_pricing,
      config::save_layouts,
      config::get_or_init_layouts
    ])
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();

      thread::spawn(move || {
          thread::sleep(Duration::from_millis(8000));
          let _ = window.set_fullscreen(true);
      });

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