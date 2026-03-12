// SPDX-License-Identifier: GPL-3.0-or-later

// === Camera ===
pub const CAMERA_ELEVATION: f32 = 30.0;
pub const CAMERA_AZIMUTH: f32 = 45.0;
pub const CAMERA_MARGIN: f32 = 0.90;

// === Board ===
pub const MIN_BOARD_SIZE: u32 = 3;
pub const MAX_BOARD_SIZE: u32 = 12;

// === Tile geometry ===
pub const FLOOR_TOP_Y: f32 = 0.0625;
pub const EMPTY_MARKER_Y: f32 = 0.0;
pub const TILE_HEIGHT: f32 = 0.125;
pub const SYMBOL_OVERLAY_OFFSET: f32 = 0.002;
pub const DELETE_OVERLAY_OFFSET: f32 = 0.005;
pub const HIGHLIGHT_Y_OFFSET: f32 = 0.01;
pub const MARKER_Y_OFFSET: f32 = 0.008;
pub const OVERLAY_MESH_THICKNESS: f32 = 0.001;

// === Animation speeds ===
pub const ANIM_SPEED: f32 = 12.0;
pub const HOVER_ANIM_SPEED: f32 = 8.0;
pub const UI_ANIM_SPEED: f32 = 12.0;
pub const HOVER_FADE_SPEED: f32 = 6.0;

// Animation snap thresholds
pub const SCALE_SNAP: f32 = 0.01;
pub const WIDTH_SNAP: f32 = 0.1;
pub const SLIDE_SNAP: f32 = 0.5;
pub const FADE_SNAP: f32 = 0.005;
pub const DESPAWN_SCALE: f32 = 0.02;

// === UI slide positions ===
pub const INV_SLIDE_SHOW: f32 = 4.0;
pub const INV_SLIDE_HIDE: f32 = -500.0;
pub const TOP_SLIDE_SHOW: f32 = 10.0;
pub const BANNER_SLIDE_HIDE: f32 = -40.0;

// === UI sizes ===
pub const SLOT_VW: f32 = 4.9;
pub const SLOT_HEIGHT_VW: f32 = 4.4;
pub const ICON_VW: f32 = 4.3;
pub const SLOT_BORDER_PX: f32 = 2.5;
pub const ICON_SIZE: u32 = 128;
pub const TOP_BTN_SIZE: f32 = 44.0;
pub const TOP_BTN_FONT: f32 = 26.0;
pub const LABEL_FONT: f32 = 16.0;
pub const COUNT_FONT: f32 = 14.0;
pub const DIALOG_TITLE_FONT: f32 = 20.0;
pub const DIALOG_BODY_FONT: f32 = 16.0;
pub const TEX_SIZE: u32 = 128;
pub const TEX_BORDER: u32 = 6;
pub const TILE_TEX_SIZE: u32 = 1024;
pub const TILE_TEX_BORDER: u32 = 16;
pub const PLAY_BTN_SIZE: f32 = 52.0;
pub const PLAY_BTN_BORDER: f32 = 3.0;
pub const BANNER_HEIGHT: f32 = 36.0;
pub const INVENTORY_PAD_VW: f32 = 0.2;
pub const EXPANSION_HEIGHT_VW: f32 = SLOT_HEIGHT_VW + INVENTORY_PAD_VW * 2.0;
pub const INVENTORY_GAP_VW: f32 = 0.15;
pub const BTN_MARGIN: f32 = 2.0;
pub const TEXT_BTN_PAD: (f32, f32) = (14.0, 8.0);
pub const TEXT_BTN_LEFT_MARGIN: f32 = 12.0;
pub const BTN_SIDE_MARGIN: f32 = 8.0;
pub const COUNT_TEXT_ALPHA: f32 = 0.7;
pub const DISABLED_BTN_ALPHA: f32 = 0.25;

// Marker/highlight textures
pub const MARKER_TEX_SIZE: u32 = 64;
pub const HIGHLIGHT_TEX_BORDER: u32 = 4;
pub const HIGHLIGHT_TEX_COLOR: [u8; 4] = [255, 255, 255, 200];
pub const INV_MARKER_BORDER: u32 = 3;
pub const INV_MARKER_COLOR: [u8; 4] = [255, 220, 0, 200];

// Empty tile marker (soft grid borders)
pub const EMPTY_MARKER_BORDER: u32 = 2;
pub const EMPTY_MARKER_COLOR: [u8; 4] = [80, 90, 110, 120];

// Vignette
pub const VIGNETTE_SIZE: u32 = 256;
pub const VIGNETTE_ALPHA: f32 = 0.4;

// Lighting
pub const LIGHT_ILLUMINANCE: f32 = 2000.0;
pub const LIGHT_ELEVATION: f32 = -0.8;
pub const LIGHT_AZIMUTH: f32 = 0.4;

// Dialog fade
pub const DIALOG_FADE_TARGET: f32 = 0.85;

// === Colors ===
pub const NUM_COLORS: usize = 10;
pub const GREY_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7);
pub const NUM_TURN_COLORS: usize = NUM_COLORS + 1;
pub const NUM_TELEPORTS: usize = 10;
pub const NUM_TELEPORT_COLORS: usize = NUM_COLORS + 1;
pub const NUM_BOUNCE_COLORS: usize = NUM_COLORS + 1;
pub const NUM_SWITCH_COLORS: usize = NUM_COLORS + 1;
pub const NUM_ARROW_COLORS: usize = NUM_COLORS + 1;

pub const COLOR_NAMES: [&str; NUM_COLORS] = [
    "Red", "Orange", "Yellow", "Light Green", "Dark Green",
    "Light Blue", "Dark Blue", "Pink", "Purple", "Brown",
];

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
pub const SLOT_BG: (f32, f32, f32) = (0.70, 0.70, 0.70);
pub const L2L3_DIVIDER_WIDTH: f32 = 2.0;
pub const L2L3_DIVIDER_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.25);
pub const BTN_BG: (f32, f32, f32) = (0.18, 0.20, 0.25);
pub const INVENTORY_L1_BG: (f32, f32, f32, f32) = (0.38, 0.38, 0.38, 0.90);
pub const INVENTORY_EXP_BG: (f32, f32, f32, f32) = (0.48, 0.48, 0.48, 0.90);
pub const BORDER_SELECTED: (f32, f32, f32, f32) = (1.0, 1.0, 0.0, 0.8);
pub const BORDER_UNSELECTED: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.2);
pub const BORDER_HOVERED: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.8);
pub const GHOST_ALPHA: f32 = 0.6;
pub const DELETE_OVERLAY_COLOR: (f32, f32, f32, f32) = (0.9, 0.2, 0.2, 0.6);
pub const TEST_INVENTORY_BG: (f32, f32, f32, f32) = (0.12, 0.08, 0.02, 0.90);
pub const TEST_BANNER_BG: (f32, f32, f32, f32) = (0.85, 0.55, 0.0, 0.9);
pub const TEST_BANNER_TEXT: (f32, f32, f32) = (0.1, 0.05, 0.0);
pub const COUNT_AVAIL_COLOR: (f32, f32, f32) = (0.4, 0.4, 0.4);
pub const COUNT_EMPTY_COLOR: (f32, f32, f32) = (0.4, 0.4, 0.4);

// Dialog colors
pub const DIALOG_PANEL_BG: (f32, f32, f32) = (0.15, 0.15, 0.18);
pub const DIALOG_INPUT_BG: (f32, f32, f32) = (0.08, 0.08, 0.10);
pub const DIALOG_INPUT_BORDER: (f32, f32, f32) = (0.4, 0.4, 0.5);
pub const CONFIRM_BTN_BG: (f32, f32, f32) = (0.15, 0.5, 0.2);
pub const STOP_TEST_BTN_BG: (f32, f32, f32) = (0.45, 0.15, 0.15);
pub const MARK_ACTIVE_BG: (f32, f32, f32) = (0.35, 0.30, 0.10);
pub const TOOLTIP_COLOR: (f32, f32, f32) = (0.10, 0.10, 0.12);
pub const DIALOG_PAD: f32 = 24.0;
pub const DIALOG_BTN_PAD: (f32, f32) = (20.0, 8.0);
pub const DIALOG_ROW_GAP: f32 = 16.0;
pub const DIALOG_MIN_WIDTH: f32 = 320.0;
pub const DIALOG_INPUT_WIDTH: f32 = 260.0;
pub const DIALOG_INPUT_HEIGHT: f32 = 36.0;
pub const DIALOG_INPUT_PAD: f32 = 8.0;
pub const DIALOG_INPUT_BORDER_PX: f32 = 2.0;
pub const DIALOG_BTN_GAP: f32 = 12.0;
pub const DIALOG_INPUT_TEXT: (f32, f32, f32) = (0.9, 0.9, 0.9);
pub const DIALOG_EMPTY_TEXT: (f32, f32, f32) = (0.5, 0.5, 0.5);
pub const DIALOG_LIST_GAP: f32 = 4.0;
pub const DIALOG_LIST_WIDTH: f32 = 280.0;
pub const DIALOG_LIST_MAX_H: f32 = 50.0;
pub const DIALOG_CANCEL_TOP_MARGIN: f32 = 8.0;
pub const LOAD_DIALOG_ROW_GAP: f32 = 12.0;
pub const MAX_LEVEL_NAME: usize = 40;

// Simulation overlay
pub const SIM_ERROR_COLOR: (f32, f32, f32) = (0.95, 0.15, 0.15);
pub const SIM_SUCCESS_COLOR: (f32, f32, f32) = (0.15, 0.95, 0.25);
pub const SIM_OVERLAY_BG: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.5);
pub const SIM_CARD_BG: (f32, f32, f32) = (0.12, 0.12, 0.12);
pub const SIM_CARD_PAD: f32 = 30.0;
pub const SIM_CARD_GAP: f32 = 20.0;
pub const SIM_MSG_FONT: f32 = 28.0;
pub const SIM_BTN_BG: (f32, f32, f32) = (0.3, 0.3, 0.3);
pub const SIM_BTN_PAD: (f32, f32) = (30.0, 12.0);
pub const SIM_BTN_FONT: f32 = 20.0;

// Texture colors (dark navy/slate blue — matches Unity reference)
pub const TILE_GRAY: [u8; 4] = [52, 58, 72, 255];
pub const TILE_DARK: [u8; 4] = [28, 32, 45, 255];
pub const SYMBOL_STROKE: [u8; 4] = [38, 42, 55, 255];
pub const STROKE_EXPAND: f32 = 0.025;
pub const TURN_CENTER_BRIGHTNESS: f32 = 0.35;
pub const ICON_WHITE: [u8; 4] = [100, 100, 100, 255];
pub const ICON_DARK_BG: [u8; 4] = [22, 26, 38, 255];
pub const ISO_SIDE_COLOR: [u8; 4] = [35, 40, 55, 255];
pub const ISO_BOTTOM_COLOR: [u8; 4] = [20, 24, 36, 255];
pub const ISO_MARGIN: f32 = 0.08;
pub const DELETE_ICON_COLOR: [u8; 4] = [220, 60, 60, 255];
pub const PLAY_ICON_COLOR: [u8; 4] = [80, 200, 80, 255];
pub const STOP_ICON_COLOR: [u8; 4] = [220, 60, 60, 255];

// Bot dimensions
pub const BOT_SIZE: f32 = 0.35;
pub const BOT_EYE_W: f32 = 0.06;
pub const BOT_EYE_H: f32 = 0.065;
pub const BOT_EYE_D: f32 = 0.015;
pub const BOT_EYE_SPACING: f32 = 0.07;
pub const BOT_EYE_Y_OFFSET: f32 = 0.04;

// Bot movement
pub const BOT_START_DELAY: f32 = 0.5;
pub const BOT_CRUISE_SPEED: f32 = 1.0;   // tiles per second
pub const BOT_ACCEL: f32 = 1.0;           // tiles/s²
pub const BOT_TURN_DURATION: f32 = 0.3;  // seconds for 90° rotation
pub const BOT_BOUNCE_SPEED: f32 = 8.0;   // bounce cycles per second at goal
pub const BOT_BOUNCE_HEIGHT: f32 = 0.15; // max bounce height
pub const FALL_PAUSE: f32 = 0.2;         // seconds before falling
pub const FALL_DURATION: f32 = 0.6;      // seconds for fall animation
pub const FALL_DISTANCE: f32 = 5.0;      // units bot falls downward

// Teleport animation
pub const TELEPORT_SHRINK_DONE: f32 = 0.03;
pub const TELEPORT_GROW_DONE: f32 = 0.97;

// Painter
pub const PAINT_TRANSITION_SPEED: f32 = 2.0; // color transitions per second

// Crush animation (bot crushed by closing door)
pub const CRUSH_DURATION: f32 = 0.4;
pub const CRUSH_EXPAND: f32 = 1.5;

// Status bar
pub const STATUS_FONT: f32 = 15.0;
pub const STATUS_FADE_SPEED: f32 = 6.0;

// Cursor & scrollbar
pub const CURSOR_BLINK_RATE: f32 = 1.8;
pub const SCROLLBAR_WIDTH: f32 = 6.0;
pub const SCROLLBAR_COLOR: (f32, f32, f32, f32) = (0.6, 0.6, 0.7, 0.6);
pub const SCROLLBAR_TRACK_COLOR: (f32, f32, f32, f32) = (0.25, 0.25, 0.30, 0.3);
pub const SCROLLBAR_MIN_H: f32 = 20.0;
pub const LOAD_ENTRY_HOVER_BG: (f32, f32, f32) = (0.28, 0.30, 0.35);
pub const DELETE_BTN_COLOR: (f32, f32, f32) = (0.6, 0.25, 0.25);
pub const DELETE_BTN_HOVER: (f32, f32, f32) = (0.8, 0.2, 0.2);

// Player navigation
pub const LEVEL_NAME_FONT: f32 = 15.0;
pub const LEVEL_NAME_MIN_W: f32 = 160.0;
pub const NAV_ARROW_FONT: f32 = 20.0;

// Visual quality
pub const CLEAR_COLOR: (f32, f32, f32) = (0.78, 0.90, 0.86);
pub const AMBIENT_COLOR: (f32, f32, f32) = (0.75, 0.82, 0.90);
pub const AMBIENT_BRIGHTNESS: f32 = 250.0;
pub const BLOOM_INTENSITY: f32 = 0.15;
pub const BLOOM_LF_BOOST: f32 = 0.3;
pub const EMISSIVE_STRENGTH: f32 = 5.0;
pub const FLOOR_TINT: (f32, f32, f32) = (1.0, 1.0, 1.0);
pub const UI_CORNER_RADIUS: f32 = 6.0;
