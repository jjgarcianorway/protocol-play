// SPDX-License-Identifier: GPL-3.0-or-later

// Background
pub const CLEAR_COLOR_O: (f32, f32, f32) = (0.04, 0.05, 0.10);

// Bloom
pub const BLOOM_INTENSITY_O: f32 = 0.18;
pub const BLOOM_LF_BOOST_O: f32 = 0.35;

// Orb colors (RGBA-like tuples)
pub const ORB_ORANGE: (f32, f32, f32) = (0.949, 0.451, 0.051);
pub const ORB_CYAN: (f32, f32, f32) = (0.200, 0.749, 0.902);
pub const ORB_PINK: (f32, f32, f32) = (0.949, 0.400, 0.549);
pub const ORB_PURPLE: (f32, f32, f32) = (0.549, 0.102, 0.702);

pub const ORB_COLORS: [(f32, f32, f32); 4] = [
    ORB_ORANGE, ORB_CYAN, ORB_PINK, ORB_PURPLE,
];

// Deck
pub const ORB_VALUES: u8 = 10;
pub const ORB_SUITS: usize = 4;
pub const DECK_SIZE: usize = (ORB_VALUES as usize) * ORB_SUITS;
pub const HAND_SIZE: usize = 3;
pub const TABLE_INITIAL: usize = 4;

// Layout sizes
pub const ORB_SIZE: f32 = 72.0;
pub const ORB_CORNER: f32 = 16.0;
pub const ORB_BORDER: f32 = 2.5;
pub const ORB_FONT: f32 = 30.0;
pub const ORB_GAP: f32 = 12.0;

// Table area
pub const TABLE_PANEL_W: f32 = 620.0;
pub const TABLE_PANEL_H: f32 = 120.0;
pub const TABLE_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.6);
pub const TABLE_BORDER: (f32, f32, f32, f32) = (0.18, 0.20, 0.30, 0.3);
pub const TABLE_CORNER: f32 = 12.0;

// Hand area
pub const HAND_PANEL_W: f32 = 300.0;
pub const HAND_PANEL_H: f32 = 100.0;
pub const HAND_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.5);

// Treasure display
pub const TREASURE_W: f32 = 120.0;
pub const TREASURE_H: f32 = 60.0;
pub const TREASURE_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.5);

// NPC back orb
pub const ORB_BACK_COLOR: (f32, f32, f32) = (0.15, 0.16, 0.22);

// Se cayo timer
pub const SE_CAYO_DURATION: f32 = 3.0;
pub const SE_CAYO_PULSE_SPEED: f32 = 6.0;

// NPC difficulty (default: medium)
pub const NPC_REACT_CHANCE: f32 = 0.75;
pub const NPC_REACT_DELAY: f32 = 1.5;
pub const NPC_TURN_DELAY: f32 = 1.0;

// Font sizes
pub const TITLE_FONT_O: f32 = 28.0;
pub const LABEL_FONT_O: f32 = 18.0;
pub const SMALL_FONT_O: f32 = 14.0;
pub const RESULTS_TITLE_FONT_O: f32 = 36.0;
pub const RESULTS_FONT_O: f32 = 20.0;
pub const RESULTS_BTN_FONT_O: f32 = 18.0;
pub const TREASURE_FONT: f32 = 22.0;
pub const STATUS_FONT_O: f32 = 16.0;
pub const VERSION_FONT_O: f32 = 11.0;

// Ronda glow
pub const RONDA_GLOW_COLOR: (f32, f32, f32, f32) = (1.0, 0.9, 0.3, 0.8);
pub const RONDA_GLOW_SPREAD: f32 = 8.0;
pub const RONDA_GLOW_BLUR: f32 = 12.0;

// Mesa limpia flash
pub const MESA_LIMPIA_DURATION: f32 = 0.6;
pub const MESA_LIMPIA_COLOR: (f32, f32, f32, f32) = (1.0, 0.85, 0.2, 0.5);

// Panel backgrounds
pub const RESULTS_BG_O: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const BTN_BG_O: (f32, f32, f32) = (0.15, 0.18, 0.28);
pub const BTN_HOVER_O: (f32, f32, f32) = (0.22, 0.26, 0.38);

// Star background
pub const STAR_COUNT_O: usize = 50;

// Selection highlight
pub const SELECTED_BORDER: (f32, f32, f32, f32) = (1.0, 0.92, 0.3, 0.9);

// Win threshold
pub const WIN_THRESHOLD: i32 = 21;
