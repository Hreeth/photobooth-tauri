use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use tauri::Manager;

use crate::{
    Result,
    imaging::{
        Layout, LayoutMode,
        layout::{Slot, generate_slots},
    },
    state::LayoutKind,
};

pub const DPI: f32 = 300.0;
pub const WIDTH: u32 = (4f32 * DPI).round() as u32;
pub const HEIGHT: u32 = (6f32 * DPI).round() as u32;

pub fn compose(
    app_handle: &tauri::AppHandle,
    layout_kind: &LayoutKind,
    images: Vec<DynamicImage>,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    let layout = layout_kind.layout();

    match layout.mode {
        LayoutMode::Full => match layout_kind {
            LayoutKind::Full1x2 => {
                compose_portrait(app_handle, layout_kind, &layout, images, background)
            }

            LayoutKind::Full2x2 => {
                compose_landscape(app_handle, layout_kind, &layout, images, background)
            }

            _ => unreachable!("non-full layout with LayoutMode::Full"),
        },

        LayoutMode::Strip => compose_strip(app_handle, layout_kind, &layout, images, background),
    }
}

fn compose_portrait(
    app_handle: &tauri::AppHandle,
    layout_kind: &LayoutKind,
    layout: &Layout,
    images: Vec<DynamicImage>,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(WIDTH, HEIGHT, background);

    let slots = generate_slots(layout, WIDTH, HEIGHT);

    if slots.is_empty() {
        return Err("invalid layout: no slots generated".into());
    }

    place_all(&mut canvas, &images, &slots, 0)?;

    let _ = apply_overlay(app_handle, &mut canvas, layout_kind);

    Ok(DynamicImage::ImageRgba8(canvas))
}

fn compose_landscape(
    app_handle: &tauri::AppHandle,
    layout_kind: &LayoutKind,
    layout: &Layout,
    images: Vec<DynamicImage>,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    let landscape_width = HEIGHT;
    let landscape_height = WIDTH;

    let mut canvas: RgbaImage =
        ImageBuffer::from_pixel(landscape_width, landscape_height, background);

    let slots = generate_slots(layout, landscape_width, landscape_height);

    if slots.is_empty() {
        return Err("invalid layout: no slots generated".into());
    }

    place_all(&mut canvas, &images, &slots, 0)?;

    let _ = apply_overlay(app_handle, &mut canvas, layout_kind);

    let rotated = image::imageops::rotate90(&canvas);

    Ok(DynamicImage::ImageRgba8(rotated))
}

fn compose_strip(
    app_handle: &tauri::AppHandle,
    layout_kind: &LayoutKind,
    layout: &Layout,
    images: Vec<DynamicImage>,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(WIDTH, HEIGHT, background);

    let strip_width = WIDTH / 2;

    let slots = generate_slots(layout, strip_width, HEIGHT);

    if slots.is_empty() {
        return Err("invalid layout: no slots generated".into());
    }

    place_all(&mut canvas, &images, &slots, 0)?;

    place_all(&mut canvas, &images, &slots, strip_width)?;

    let _ = apply_overlay(app_handle, &mut canvas, layout_kind);

    Ok(DynamicImage::ImageRgba8(canvas))
}

fn place_all(
    canvas: &mut RgbaImage,
    images: &[DynamicImage],
    slots: &[Slot],
    x_offset: u32,
) -> Result<()> {
    if images.len() != slots.len() {
        return Err("image count mismatch with layout".into());
    }

    for (image, slot) in images.iter().zip(slots.iter()) {
        let fitted =
            image.resize_to_fill(slot.width, slot.height, image::imageops::FilterType::Lanczos3);

        overlay(canvas, &fitted, (slot.x + x_offset).into(), slot.y.into());
    }

    Ok(())
}

fn apply_overlay(
    app_handle: &tauri::AppHandle,
    canvas: &mut RgbaImage,
    layout_kind: &LayoutKind,
) -> Result<()> {
    let filename = format!("{:?}.png", layout_kind);

    let overlay_path = app_handle
        .path()
        .resolve(format!("assets/{filename}"), tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("failed to find overlay: {e}"))?;

    let event_overlay =
        image::open(&overlay_path).map_err(|e| format!("failed to open overlay: {e}"))?.to_rgba8();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let overlay_ratio = event_overlay.width() as f32 / event_overlay.height() as f32;

    let canvas_ratio = canvas_width as f32 / canvas_height as f32;

    if approximately_equal(overlay_ratio, canvas_ratio) {
        let resized = image::imageops::resize(
            &event_overlay,
            canvas_width,
            canvas_height,
            image::imageops::FilterType::Lanczos3,
        );

        overlay(canvas, &resized, 0, 0);

        return Ok(());
    }

    let half_width = canvas_width / 2;

    let half_ratio = half_width as f32 / canvas_height as f32;

    if approximately_equal(overlay_ratio, half_ratio) {
        let resized = image::imageops::resize(
            &event_overlay,
            half_width,
            canvas_height,
            image::imageops::FilterType::Lanczos3,
        );

        overlay(canvas, &resized, 0, 0);

        overlay(canvas, &resized, half_width as i64, 0);

        return Ok(());
    }

    Err(format!(
        "unexpected overlay aspect ratio {:.4} for canvas {}x{}",
        overlay_ratio, canvas_width, canvas_height
    )
    .into())
}

fn approximately_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}
