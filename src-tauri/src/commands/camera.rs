use std::{path::PathBuf, sync::Arc};

use base64::Engine;
use tauri::Emitter;

use crate::{Result, state::AppState};

#[tauri::command]
pub fn start_camera(app: tauri::AppHandle, state: tauri::State<Arc<AppState>>) -> Result<()> {
    let mut camera = state.camera.lock().unwrap();

    if camera.is_streaming {
        return Ok(());
    }

    camera.driver.start_stream(app)?;
    camera.is_streaming = true;

    Ok(())
}

#[tauri::command]
pub fn capture(app: tauri::AppHandle, state: tauri::State<Arc<AppState>>) -> Result<String> {
    let path = {
        let mut camera = state.camera.lock().unwrap();

        if !camera.is_streaming {
            return Err("camera not started".into());
        }

        camera.driver.capture()?
    };

    {
        let mut session = state.session.lock().unwrap();

        if !session.can_capture() {
            let _ = std::fs::remove_file(&path);
            return Err("session is not ready for capture".into());
        }

        session.set_preview(PathBuf::from(&path));
    }

    let bytes = std::fs::read(&path).map_err(|e| format!("failed to read captured image: {e}"))?;

    let preview = base64::engine::general_purpose::STANDARD.encode(bytes);

    app.emit("take-preview", preview).map_err(|e| format!("failed to emit take preview: {e}"))?;

    Ok(path)
}

#[tauri::command]
pub fn retake(state: tauri::State<Arc<AppState>>) -> Result<()> {
    let rejected = {
        let mut session = state.session.lock().unwrap();
        session.retake()?
    };

    let _ = std::fs::remove_file(&rejected);

    Ok(())
}

#[tauri::command]
pub fn accept_photo(state: tauri::State<Arc<AppState>>) -> Result<bool> {
    let mut session = state.session.lock().unwrap();

    session.accept_preview()?;

    Ok(session.is_complete())
}

#[tauri::command]
pub fn stop_camera(state: tauri::State<Arc<AppState>>) -> Result<()> {
    let mut camera = state.camera.lock().unwrap();

    if !camera.is_streaming {
        return Ok(());
    }

    camera.driver.stop_stream()?;
    camera.is_streaming = false;

    Ok(())
}
