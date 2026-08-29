use std::{process::Command, sync::Arc};

use crate::{Result, imaging::LayoutMode, state::AppState};

#[tauri::command]
pub fn print(state: tauri::State<Arc<AppState>>, mode: LayoutMode) -> Result<()> {
    let mut session = state.session.lock().unwrap();

    assert!(session.options.is_some(), "fatal: options must exist at this point");

    let options = session.options.as_ref().unwrap();
    let r#final = session.r#final.as_ref().ok_or("fatal: final image must exist at this point")?;

    let res = match mode {
        LayoutMode::Full => Command::new("lp")
            .args([
                "-o",
                "media=w288h432",
                "-o",
                "fit-to-page",
                "-n",
                &options.prints.to_string(),
                &r#final.to_string_lossy().to_string(),
            ])
            .output(),

        #[rustfmt::skip]
        LayoutMode::Strip => Command::new("lp")
            .args([
                "-n", &options.prints.to_string(), 
                &r#final.to_string_lossy().to_string()
            ])
            .output(),
    };

    match res {
        Ok(out) => {
            if !out.status.success() {
                return Err(format!("Failed to print: {}", String::from_utf8_lossy(&out.stderr)));
            }
        }

        Err(e) => {
            return Err(format!("Failed to execute print command: {e}"));
        }
    }

    session.status = crate::state::SessionStatus::Completed;

    Ok(())
}
