use std::{env, process::Command};

#[tauri::command]
pub fn open_keyboard() {
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
    Command::new("setsid")
        .arg("wvkbd-mobintl")
        .arg("-l")
        .arg("simple")
        .env("WAYLAND_DISPLAY", wayland_display)
        .spawn()
        .expect("Failed to open keyboard");
}

#[tauri::command]
pub fn close_keyboard() {
    Command::new("pkill")
        .arg("wvkbd-mobintl")
        .spawn()
        .expect("Failed to close keyboard");
}
