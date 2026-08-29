use std::sync::Arc;

use crate::{
    Result,
    state::{AppState, Options},
};

#[tauri::command]
pub fn start_session(state: tauri::State<Arc<AppState>>, options: Options) -> Result<()> {
    let mut session = state.session.lock().unwrap();
    session.start(options);

    Ok(())
}

#[tauri::command]
pub fn reset_session(state: tauri::State<Arc<AppState>>) -> Result<()> {
    let mut session = state.session.lock().unwrap();
    session.reset();

    Ok(())
}
