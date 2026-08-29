use std::{sync::Arc, thread, time::Duration};
use tauri::Manager;

use crate::state::AppState;

mod camera;
mod commands;
mod imaging;
mod state;
mod utils;

type Result<T, E = String> = std::result::Result<T, E>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::razorpay::create_qr,
            commands::razorpay::check_payment_status,
            commands::camera::start_camera,
            commands::camera::stop_camera,
            commands::camera::capture,
            commands::camera::retake,
            commands::camera::accept_photo,
            commands::imaging::process_session,
            commands::lifecycle::start_session,
            commands::lifecycle::reset_session,
            commands::print::print,
            commands::mail::store_email,
            commands::mail::send_email,
            commands::config::save_config,
            commands::config::get_or_init_config,
            commands::config::save_layouts,
            commands::config::get_or_init_layouts,
            commands::config::save_pages,
            commands::config::get_or_init_pages,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            let _ = std::fs::create_dir_all(utils::assets_dir());
            let _ = std::fs::create_dir_all(utils::output_dir());
            let _ = std::fs::create_dir_all(utils::capture_dir());

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(8000));
                let _ = window.set_fullscreen(true);
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default().level(log::LevelFilter::Trace).build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
