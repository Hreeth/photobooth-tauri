use std::sync::Arc;

use image::DynamicImage;

use crate::{
    Result,
    imaging::{apply, compose},
    state::{AppState, FilterKind, SessionStatus},
    utils::{apply_bleed, capture_dir},
};

#[tauri::command]
pub async fn process_session(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<()> {
    let state = state.inner().clone();

    tauri::async_runtime::spawn_blocking(move || process_session_inner(&app_handle, state))
        .await
        .map_err(|e| format!("process_session task failed: {e}"))?
}

fn process_session_inner(app_handle: &tauri::AppHandle, state: Arc<AppState>) -> Result<()> {
    let (photos, layout, filter_kind);

    {
        let mut session = state.session.lock().unwrap();

        if !session.is_complete() {
            return Err("session is not complete".into());
        }

        let options = session.options.as_ref().ok_or("session options not set")?;

        photos = session.photos.clone();
        layout = options.layout;
        filter_kind = options.filter;

        session.status = SessionStatus::Completed;
    }

    let mut images = Vec::with_capacity(photos.len());

    for path in photos {
        let image = image::open(&path)
            .map_err(|e| format!("failed to open photo {}: {e}", path.display()))?
            .into_rgba8();

        images.push(DynamicImage::ImageRgba8(image));
    }

    for image in &mut images {
        apply_filter(image, filter_kind);
    }

    let background = image::Rgba([4, 7, 7, 255]);

    let composed = compose(app_handle, &layout, images, background)?;

    let output_path = capture_dir().join("final.jpg");

    let bleed = ((0.15 / 2.54) * crate::imaging::DPI).round() as u32;

    let final_image = apply_bleed(&composed, bleed, background.0);

    final_image
        .save_with_format(&output_path, image::ImageFormat::Jpeg)
        .map_err(|e| format!("failed to save final image: {e}"))?;

    {
        let mut session = state.session.lock().unwrap();
        session.r#final = Some(output_path);
    }

    Ok(())
}

fn apply_filter(image: &mut DynamicImage, filter_kind: FilterKind) {
    if let Some(image) = image.as_mut_rgba8() {
        apply(image, filter_kind);
    }
}
