use serde::{Deserialize, Serialize};

use crate::imaging::{Layout, LayoutBounds, LayoutGrid, LayoutMode};

/// Layout variants in the format ```{size}{grid}```
///
/// Example: ```Full1x2``` represents a layout with ```Full``` size and ```1x2``` grid
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayoutKind {
    #[serde(rename = "Full1x2")]
    Full1x2,
    #[serde(rename = "Full2x2")]
    Full2x2,
    #[serde(rename = "Strip1x3")]
    Strip1x3,
    #[serde(rename = "Strip1x4")]
    Strip1x4,
}

impl LayoutKind {
    pub fn grid(&self) -> LayoutGrid {
        match self {
            LayoutKind::Full1x2 => LayoutGrid::new(1, 2),
            LayoutKind::Full2x2 => LayoutGrid::new(2, 2),
            LayoutKind::Strip1x3 => LayoutGrid::new(1, 3),
            LayoutKind::Strip1x4 => LayoutGrid::new(1, 4),
        }
    }

    pub fn mode(&self) -> LayoutMode {
        match self {
            LayoutKind::Full1x2 => LayoutMode::Full,
            LayoutKind::Full2x2 => LayoutMode::Full,
            LayoutKind::Strip1x3 => LayoutMode::Strip,
            LayoutKind::Strip1x4 => LayoutMode::Strip,
        }
    }

    pub fn bounds(&self) -> LayoutBounds {
        match self {
            LayoutKind::Full1x2 => LayoutBounds { borders: [98, 44, 286, 44], gap: 44 },
            LayoutKind::Full2x2 => LayoutBounds { borders: [18, 18, 118, 18], gap: 18 },
            LayoutKind::Strip1x3 => LayoutBounds { borders: [181, 50, 377, 50], gap: 18 },
            LayoutKind::Strip1x4 => LayoutBounds { borders: [44, 18, 112, 18], gap: 12 },
        }
    }

    pub fn layout(&self) -> Layout {
        Layout { grid: self.grid(), mode: self.mode(), bounds: self.bounds() }
    }
}

/// Filter variants
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FilterKind {
    /// B&W filter
    #[serde(rename = "B&W")]
    BW,

    /// Color filter
    #[serde(rename = "Color")]
    Color,

    /// future ref
    #[serde(rename = "HujiCam")]
    HujiCam,

    /// future ref
    #[serde(rename = "Vintage")]
    Vintage,
}
