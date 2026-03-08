// SPDX-License-Identifier: GPL-3.0-or-later

// === Camera ===
pub const CAMERA_ELEVATION: f32 = 30.0;
pub const CAMERA_AZIMUTH: f32 = 45.0;
pub const CAMERA_MARGIN: f32 = 1.05;

// === Board ===
pub const MIN_BOARD_SIZE: u32 = 3;
pub const MAX_BOARD_SIZE: u32 = 12;

// === Tile geometry ===
pub const FLOOR_TOP_Y: f32 = 0.125;
pub const EMPTY_MARKER_Y: f32 = -0.124;
pub const TILE_HEIGHT: f32 = 0.25;
pub const SYMBOL_OVERLAY_OFFSET: f32 = 0.002;
pub const DELETE_OVERLAY_OFFSET: f32 = 0.005;

// === Animation speeds ===
pub const ANIM_SPEED: f32 = 12.0;
pub const HOVER_ANIM_SPEED: f32 = 8.0;
pub const UI_ANIM_SPEED: f32 = 12.0;

// === UI sizes ===
pub const SLOT_VW: f32 = 4.5;
pub const SLOT_HEIGHT_VW: f32 = 5.6;
pub const ICON_VW: f32 = 3.4;
pub const ICON_SIZE: u32 = 128;
pub const TEX_SIZE: u32 = 128;
pub const TEX_BORDER: u32 = 6;
pub const TILE_TEX_SIZE: u32 = 1024;
pub const TILE_TEX_BORDER: u32 = 12;

// === Colors ===
pub const NUM_COLORS: usize = 10;

pub const SOURCE_COLORS: [(f32, f32, f32); NUM_COLORS] = [
    (0.95, 0.1, 0.1),   // Red
    (1.0, 0.5, 0.0),    // Orange
    (1.0, 0.88, 0.0),   // Yellow
    (0.35, 0.85, 0.2),  // Light Green
    (0.0, 0.45, 0.12),  // Dark Green
    (0.3, 0.7, 1.0),    // Light Blue
    (0.1, 0.15, 0.75),  // Dark Blue
    (1.0, 0.35, 0.55),  // Pink
    (0.6, 0.15, 0.85),  // Purple
    (0.55, 0.3, 0.08),  // Brown
];

// UI colors
pub const SLOT_BG: (f32, f32, f32) = (0.15, 0.15, 0.15);
pub const INVENTORY_BG: (f32, f32, f32, f32) = (0.08, 0.08, 0.08, 0.85);
pub const BORDER_SELECTED: (f32, f32, f32, f32) = (1.0, 1.0, 0.0, 0.8);
pub const BORDER_UNSELECTED: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.2);
pub const BORDER_HOVERED: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.8);
pub const GHOST_ALPHA: f32 = 0.6;
pub const DELETE_OVERLAY_COLOR: (f32, f32, f32, f32) = (0.9, 0.2, 0.2, 0.6);

// Texture colors
pub const TILE_GRAY: [u8; 4] = [180, 180, 180, 255];
pub const TILE_DARK: [u8; 4] = [60, 60, 60, 255];
pub const SYMBOL_STROKE: [u8; 4] = [80, 80, 80, 255];
pub const STROKE_EXPAND: f32 = 0.025;
pub const TURN_CENTER_BRIGHTNESS: f32 = 0.35;

// Bot dimensions
pub const BOT_SIZE: f32 = 0.35;
pub const BOT_EYE_W: f32 = 0.06;
pub const BOT_EYE_H: f32 = 0.065;
pub const BOT_EYE_D: f32 = 0.015;
