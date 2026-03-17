// SPDX-License-Identifier: GPL-3.0-or-later

// === Background ===
pub const CLEAR_COLOR_M: (f32, f32, f32) = (0.04, 0.05, 0.10);

// === Bloom ===
pub const BLOOM_INTENSITY_M: f32 = 0.20;
pub const BLOOM_LF_BOOST_M: f32 = 0.35;

// === Stars ===
pub const NUM_STARS: usize = 80;
pub const STAR_MIN_SIZE: f32 = 0.03;
pub const STAR_MAX_SIZE: f32 = 0.08;
pub const STAR_DEPTH: f32 = -20.0;
pub const STAR_SPREAD_X: f32 = 60.0;
pub const STAR_SPREAD_Y: f32 = 35.0;
pub const STAR_TWINKLE_SPEED: f32 = 1.5;

// === Layout ===
pub const DASHBOARD_WIDTH_PCT: f32 = 55.0;
pub const GAMES_WIDTH_PCT: f32 = 45.0;
pub const SECTION_PAD: f32 = 20.0;
pub const SECTION_GAP: f32 = 12.0;

// === Ship title ===
pub const SHIP_NAME: &str = "ARK AURORA";
pub const SHIP_NAME_FONT: f32 = 32.0;
pub const SHIP_NAME_COLOR: (f32, f32, f32) = (0.7, 0.8, 0.95);

// === Resource bars ===
pub const BAR_HEIGHT: f32 = 18.0;
pub const BAR_WIDTH: f32 = 280.0;
pub const BAR_CORNER: f32 = 4.0;
pub const BAR_BG_COLOR: (f32, f32, f32, f32) = (0.12, 0.13, 0.18, 0.8);
pub const BAR_LABEL_FONT: f32 = 14.0;
pub const BAR_PCT_FONT: f32 = 13.0;
pub const BAR_LERP_SPEED: f32 = 3.0;

// Resource colors (same 5-color palette as crystals)
pub const RES_POWER_COLOR: (f32, f32, f32) = (0.902, 0.098, 0.294);
pub const RES_LIFE_COLOR: (f32, f32, f32) = (0.130, 0.545, 0.130);
pub const RES_CRYO_COLOR: (f32, f32, f32) = (0.150, 0.250, 0.700);
pub const RES_SHIELD_COLOR: (f32, f32, f32) = (1.000, 0.882, 0.098);
pub const RES_REPAIR_COLOR: (f32, f32, f32) = (0.569, 0.118, 0.706);

pub const RES_COLORS: [(f32, f32, f32); 5] = [
    RES_POWER_COLOR, RES_LIFE_COLOR, RES_CRYO_COLOR,
    RES_SHIELD_COLOR, RES_REPAIR_COLOR,
];
pub const RES_NAMES: [&str; 5] = ["Power", "Life Support", "Cryo", "Shields", "Repair"];
pub const RES_ICONS: [&str; 5] = ["PWR", "LIFE", "CRYO", "SHLD", "RPR"];

// === Crew & journey ===
pub const INFO_FONT: f32 = 15.0;
pub const INFO_COLOR: (f32, f32, f32) = (0.6, 0.65, 0.75);
pub const INFO_HIGHLIGHT_COLOR: (f32, f32, f32) = (0.85, 0.9, 1.0);

// === Game cards ===
pub const CARD_WIDTH: f32 = 300.0;
pub const CARD_HEIGHT: f32 = 80.0;
pub const CARD_PAD: f32 = 14.0;
pub const CARD_CORNER: f32 = 8.0;
pub const CARD_BG: (f32, f32, f32, f32) = (0.10, 0.11, 0.16, 0.9);
pub const CARD_HOVER_BG: (f32, f32, f32, f32) = (0.16, 0.18, 0.25, 0.95);
pub const CARD_DISABLED_ALPHA: f32 = 0.4;
pub const CARD_TITLE_FONT: f32 = 18.0;
pub const CARD_STATUS_FONT: f32 = 13.0;
pub const CARD_TITLE_COLOR: (f32, f32, f32) = (0.9, 0.92, 1.0);
pub const CARD_STATUS_COLOR: (f32, f32, f32) = (0.55, 0.6, 0.7);
pub const CARD_RECOMMENDED_COLOR: (f32, f32, f32) = (0.3, 0.8, 0.4);
pub const CARD_BORDER: f32 = 2.0;
pub const CARD_BORDER_COLOR: (f32, f32, f32, f32) = (0.25, 0.28, 0.38, 0.6);
pub const CARD_BORDER_HOVER: (f32, f32, f32, f32) = (0.5, 0.6, 0.8, 0.8);

// === Section headers ===
pub const SECTION_TITLE_FONT: f32 = 13.0;
pub const SECTION_TITLE_COLOR: (f32, f32, f32) = (0.4, 0.45, 0.55);

// === Anna ===
pub const ANNA_PANEL_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.85);
pub const ANNA_PANEL_CORNER: f32 = 10.0;
pub const ANNA_PANEL_PAD: f32 = 16.0;
pub const ANNA_CIRCLE_SIZE: f32 = 40.0;
pub const ANNA_CIRCLE_COLOR: (f32, f32, f32) = (0.3, 0.6, 0.9);
pub const ANNA_GLOW_BLUR: f32 = 15.0;
pub const ANNA_GLOW_SPREAD: f32 = 5.0;
pub const ANNA_NAME_FONT: f32 = 12.0;
pub const ANNA_NAME_COLOR: (f32, f32, f32) = (0.4, 0.7, 1.0);
pub const ANNA_MSG_FONT: f32 = 15.0;
pub const ANNA_MSG_COLOR: (f32, f32, f32) = (0.75, 0.8, 0.9);
pub const ANNA_FADE_SPEED: f32 = 2.0;
pub const ANNA_MSG_HOLD: f32 = 6.0;

// === Version ===
pub const VERSION_FONT_M: f32 = 11.0;
