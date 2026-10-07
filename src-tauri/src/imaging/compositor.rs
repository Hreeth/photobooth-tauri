use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage, imageops::overlay};
use tauri::AppHandle;

use crate::{
    Result,
    imaging::{
        Layout, LayoutMode,
        branding::{draw_branding, draw_branding_strip},
        layout::{Slot, generate_slots},
    },
    state::LayoutKind,
};

pub const DPI: f32 = 300.0;
pub const WIDTH: u32 = (4f32 * DPI).round() as u32;
pub const HEIGHT: u32 = (6f32 * DPI).round() as u32;

pub fn compose(
    app_handle: &AppHandle,
    layout_kind: &LayoutKind,
    images: Vec<DynamicImage>,
    background: Rgba<u8>,
) -> Result<DynamicImage> {
    let layout = layout_kind.layout();

    match layout.mode {
        LayoutMode::Full => match layout_kind {
            LayoutKind::Full1x2 => compose_portrait(app_handle, &layout, images, background),

            LayoutKind::Full2x2 => compose_landscape(app_handle, &layout, images, background),

            _ => unreachable!("non-full layout with LayoutMode::Full"),
        },

        LayoutMode::Strip => compose_strip(app_handle, &layout, images, background),
    }
}

fn compose_portrait(
    app_handle: &AppHandle,
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

    if layout.bounds.branding {
        draw_branding(app_handle, &mut canvas, layout, background == Rgba([0, 0, 0, 255]));
    }

    Ok(DynamicImage::ImageRgba8(canvas))
}

fn compose_landscape(
    app_handle: &AppHandle,
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

    if layout.bounds.branding {
        draw_branding(app_handle, &mut canvas, layout, background == Rgba([0, 0, 0, 255]));
    }

    let rotated = image::imageops::rotate90(&canvas);

    Ok(DynamicImage::ImageRgba8(rotated))
}

fn compose_strip(
    app_handle: &AppHandle,
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

    if layout.bounds.branding {
        draw_branding_strip(app_handle, &mut canvas, layout, background == Rgba([0, 0, 0, 255]));
    }

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
