use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum LayoutMode {
    Full,
    Strip,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LayoutGrid {
    pub cols: u8,
    pub rows: u8,
}

impl LayoutGrid {
    pub fn new(cols: u8, rows: u8) -> Self {
        assert!(rows > 0 && cols > 0, "bug: shouldn't be possible at all");

        Self { cols, rows }
    }

    pub fn total(&self) -> usize {
        (self.cols * self.rows) as usize
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LayoutBounds {
    pub borders: [u32; 4],
    pub gap: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Layout {
    pub grid: LayoutGrid,
    pub mode: LayoutMode,
    pub bounds: LayoutBounds,
}

#[derive(Debug)]
pub struct Slot {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn generate_slots(layout: &Layout, width: u32, height: u32) -> Vec<Slot> {
    let cols = layout.grid.cols;
    let rows = layout.grid.rows;

    let gap = layout.bounds.gap;
    let [top, right, bottom, left] = layout.bounds.borders;

    let usable_width = width - left - right;
    let usable_height = height - top - bottom;

    let gap_x = gap * (cols - 1) as u32;
    let gap_y = gap * (rows - 1) as u32;

    let cell_width = (usable_width - gap_x) / cols as u32;
    let cell_height = (usable_height - gap_y) / rows as u32;

    let mut slots = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            let col = col as u32;
            let row = row as u32;

            let x = left + col * (cell_width + gap);
            let y = top + row * (cell_height + gap);

            slots.push(Slot { x, y, width: cell_width, height: cell_height });
        }
    }

    slots
}
