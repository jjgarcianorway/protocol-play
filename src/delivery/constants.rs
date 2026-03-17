// SPDX-License-Identifier: GPL-3.0-or-later

// Background
pub const CLEAR_COLOR_D: (f32, f32, f32) = (0.05, 0.06, 0.12);

// Bloom (matching Gathering: 0.20 intensity, 0.4 LF boost)
pub const BLOOM_INTENSITY_D: f32 = 0.20;
pub const BLOOM_LF_BOOST_D: f32 = 0.4;

// Resource colors (same palette as converter)
pub const POD_RED: (f32, f32, f32) = (0.902, 0.098, 0.294);
pub const POD_GREEN: (f32, f32, f32) = (0.130, 0.545, 0.130);
pub const POD_BLUE: (f32, f32, f32) = (0.150, 0.250, 0.700);
pub const POD_YELLOW: (f32, f32, f32) = (1.000, 0.882, 0.098);
pub const POD_PURPLE: (f32, f32, f32) = (0.569, 0.118, 0.706);

pub const POD_COLORS: [(f32, f32, f32); 5] = [
    POD_RED, POD_GREEN, POD_BLUE, POD_YELLOW, POD_PURPLE,
];

pub const RESOURCE_NAMES: [&str; 5] = ["Power", "Life Support", "Cryo", "Shields", "Repair"];
pub const RESOURCE_ICONS: [&str; 5] = ["\u{26A1}", "\u{1F4A7}", "\u{2744}", "\u{1F6E1}", "\u{1F527}"];

// Pod sizes and layout
pub const POD_SIZE: f32 = 40.0;
pub const POD_CORNER: f32 = 12.0;
pub const POD_GLOW_BLUR: f32 = 12.0;
pub const POD_GLOW_SPREAD: f32 = 4.0;
pub const POD_ICON_FONT: f32 = 18.0;

// Pod trail
pub const POD_TRAIL_COUNT: usize = 3;
pub const POD_TRAIL_SPACING: f32 = 12.0;
pub const POD_TRAIL_SIZE_DECAY: f32 = 0.7;
pub const POD_TRAIL_ALPHA_DECAY: f32 = 0.5;

// Deposit slot layout
pub const SLOT_COUNT: usize = 5;
pub const SLOT_WIDTH: f32 = 100.0;
pub const SLOT_HEIGHT: f32 = 70.0;
pub const SLOT_GAP: f32 = 16.0;
pub const SLOT_CORNER: f32 = 10.0;
pub const SLOT_BORDER: f32 = 3.0;
pub const SLOT_BOTTOM_MARGIN: f32 = 40.0;
pub const SELECTED_BORDER_COLOR: (f32, f32, f32, f32) = (1.0, 0.9, 0.2, 1.0);
pub const UNSELECTED_BORDER_COLOR: (f32, f32, f32, f32) = (0.3, 0.3, 0.4, 0.6);

// Slot glow effect (matching pod hint)
pub const SLOT_MATCH_GLOW_BLUR: f32 = 16.0;
pub const SLOT_MATCH_GLOW_SPREAD: f32 = 4.0;
pub const SLOT_MATCH_PULSE_SPEED: f32 = 3.0;

// Fall zone (vertical area where pods fall)
pub const FALL_ZONE_TOP: f32 = 80.0;
pub const FALL_ZONE_HEIGHT: f32 = 400.0;

// Timing and difficulty
pub const INITIAL_FALL_DURATION: f32 = 2.5;
pub const MIN_FALL_DURATION: f32 = 0.8;
pub const INITIAL_SPAWN_INTERVAL: f32 = 2.0;
pub const MIN_SPAWN_INTERVAL: f32 = 0.6;
pub const TOTAL_PODS: u32 = 100;
pub const ROUTE_ANIM_DURATION: f32 = 0.3;

// Scoring and streaks
pub const STREAK_TIER_1: u32 = 5;
pub const STREAK_TIER_2: u32 = 10;
pub const STREAK_TIER_3: u32 = 20;
pub const STREAK_MULT_1: f32 = 1.2;
pub const STREAK_MULT_2: f32 = 1.5;
pub const STREAK_MULT_3: f32 = 2.0;

// Streak milestone popup
pub const STREAK_POPUP_LIFETIME: f32 = 1.5;
pub const STREAK_POPUP_FONT: f32 = 36.0;
pub const STREAK_POPUP_RISE: f32 = 40.0;

// Flash effects
pub const CORRECT_FLASH_DURATION: f32 = 0.3;
pub const WRONG_FLASH_DURATION: f32 = 0.4;
pub const CORRECT_FLASH_COLOR: (f32, f32, f32, f32) = (0.2, 0.9, 0.2, 0.5);
pub const WRONG_FLASH_COLOR: (f32, f32, f32, f32) = (0.9, 0.1, 0.1, 0.5);

// Font sizes
pub const TITLE_FONT_D: f32 = 28.0;
pub const HUD_FONT: f32 = 20.0;
pub const STREAK_FONT: f32 = 26.0;
pub const SLOT_LABEL_FONT: f32 = 14.0;
pub const SLOT_ICON_FONT: f32 = 24.0;
pub const RESULTS_TITLE_FONT_D: f32 = 36.0;
pub const RESULTS_FONT_D: f32 = 20.0;
pub const RESULTS_BTN_FONT_D: f32 = 18.0;
pub const SPEED_METER_FONT: f32 = 12.0;

// Panel backgrounds
pub const RESULTS_BG_D: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const BTN_BG_D: (f32, f32, f32) = (0.15, 0.18, 0.28);
pub const BTN_HOVER_D: (f32, f32, f32) = (0.22, 0.26, 0.38);

// HUD
pub const HUD_TOP_MARGIN: f32 = 16.0;
pub const HUD_SIDE_MARGIN: f32 = 24.0;

// Star background (UI-based dots)
pub const STAR_COUNT_D: usize = 60;
pub const STAR_MIN_SIZE_D: f32 = 1.0;
pub const STAR_MAX_SIZE_D: f32 = 3.0;
pub const STAR_MIN_ALPHA_D: f32 = 0.15;
pub const STAR_MAX_ALPHA_D: f32 = 0.5;
