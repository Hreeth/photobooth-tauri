use std::{thread, time::Duration};

use tauri::{LogicalPosition, Manager};


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

      window.set_position(LogicalPosition { x: 0.0, y: 0.0 }).unwrap();

      thread::spawn(move || {
        let _ = window.set_fullscreen(false);
        thread::sleep(Duration::from_millis(2000));
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