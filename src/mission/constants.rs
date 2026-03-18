// SPDX-License-Identifier: GPL-3.0-or-later

// === Background ===
pub const CLEAR_COLOR_M: (f32, f32, f32) = (0.04, 0.05, 0.10);

// === Bloom ===
pub const BLOOM_INTENSITY_M: f32 = 0.20;
pub const BLOOM_LF_BOOST_M: f32 = 0.35;

// === Stars ===
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
pub const ANNA_STORY_MSG_COLOR: (f32, f32, f32) = (0.9, 0.85, 0.75);
pub const ANNA_FADE_SPEED: f32 = 2.0;
pub const ANNA_MSG_HOLD: f32 = 5.0;
pub const ANNA_STORY_HOLD: f32 = 8.0;

// Anna mood glow colors
pub const ANNA_WARNING_COLOR: (f32, f32, f32) = (0.9, 0.5, 0.2);
pub const ANNA_STORY_COLOR: (f32, f32, f32) = (0.9, 0.85, 0.7);
pub const ANNA_GLITCH_COLOR: (f32, f32, f32) = (0.3, 0.6, 0.9);
pub const ANNA_WARNING_PULSE_SPEED: f32 = 4.0;
pub const ANNA_NORMAL_PULSE_SPEED: f32 = 1.2;
pub const ANNA_GLITCH_SPEED: f32 = 12.0;

// === Resource drain rates (per game-day) ===
pub const POWER_DRAIN: f32 = 0.5;
pub const LIFE_SUPPORT_DRAIN: f32 = 0.3;
pub const CRYO_DRAIN: f32 = 0.2;
pub const SHIELDS_DRAIN: f32 = 0.1;
pub const REPAIR_DRAIN: f32 = 0.4;
pub const DAY_DURATION_SECS: f32 = 10.0;
pub const SAVE_INTERVAL_SECS: f32 = 30.0;

/// Drain rates indexed the same as RES_NAMES (Power, Life Support, Cryo, Shields, Repair).
pub const RES_DRAIN_RATES: [f32; 5] = [
    POWER_DRAIN, LIFE_SUPPORT_DRAIN, CRYO_DRAIN, SHIELDS_DRAIN, REPAIR_DRAIN,
];

/// Crew lost per day when cryo is at 0.
pub const CRYO_ZERO_CREW_LOSS_MIN: u32 = 1;
pub const CRYO_ZERO_CREW_LOSS_MAX: u32 = 5;

// === Drain rate label ===
pub const DRAIN_LABEL_FONT: f32 = 11.0;
pub const DRAIN_LABEL_COLOR: (f32, f32, f32) = (0.6, 0.4, 0.4);

// === Version ===
pub const VERSION_FONT_M: f32 = 11.0;

// === Settings overlay ===
pub const SETTINGS_OVERLAY_BG: (f32, f32, f32, f32) = (0.02, 0.03, 0.06, 0.92);
pub const SETTINGS_PANEL_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.95);
pub const SETTINGS_PANEL_CORNER: f32 = 12.0;
pub const SETTINGS_PANEL_MAX_W: f32 = 700.0;
pub const SETTINGS_PANEL_MAX_H: f32 = 500.0;
pub const SETTINGS_PANEL_PAD: f32 = 24.0;
pub const SETTINGS_TAB_WIDTH: f32 = 160.0;
pub const SETTINGS_TAB_PAD: f32 = 12.0;
pub const SETTINGS_TAB_CORNER: f32 = 6.0;
pub const SETTINGS_TAB_FONT: f32 = 15.0;
pub const SETTINGS_TAB_BG: (f32, f32, f32, f32) = (0.10, 0.11, 0.16, 0.8);
pub const SETTINGS_TAB_ACTIVE_BG: (f32, f32, f32, f32) = (0.18, 0.20, 0.28, 0.95);
pub const SETTINGS_TAB_COLOR: (f32, f32, f32) = (0.55, 0.6, 0.7);
pub const SETTINGS_TAB_ACTIVE_COLOR: (f32, f32, f32) = (0.85, 0.9, 1.0);
pub const SETTINGS_TITLE_FONT: f32 = 22.0;
pub const SETTINGS_TITLE_COLOR: (f32, f32, f32) = (0.8, 0.85, 0.95);
pub const SETTINGS_HEADING_FONT: f32 = 16.0;
pub const SETTINGS_HEADING_COLOR: (f32, f32, f32) = (0.7, 0.75, 0.85);
pub const SETTINGS_BODY_FONT: f32 = 14.0;
pub const SETTINGS_BODY_COLOR: (f32, f32, f32) = (0.6, 0.65, 0.75);
pub const SETTINGS_NOTE_FONT: f32 = 12.0;
pub const SETTINGS_NOTE_COLOR: (f32, f32, f32) = (0.45, 0.5, 0.6);
pub const SETTINGS_BTN_PAD_X: f32 = 18.0;
pub const SETTINGS_BTN_PAD_Y: f32 = 10.0;
pub const SETTINGS_BTN_CORNER: f32 = 6.0;
pub const SETTINGS_BTN_FONT: f32 = 14.0;
pub const SETTINGS_BTN_BG: (f32, f32, f32, f32) = (0.12, 0.13, 0.18, 0.9);
pub const SETTINGS_BTN_HOVER_BG: (f32, f32, f32, f32) = (0.20, 0.22, 0.30, 0.95);
pub const SETTINGS_BTN_BORDER: (f32, f32, f32, f32) = (0.25, 0.28, 0.38, 0.6);
pub const SETTINGS_BTN_HOVER_BORDER: (f32, f32, f32, f32) = (0.5, 0.6, 0.8, 0.8);
pub const SETTINGS_BTN_COLOR: (f32, f32, f32) = (0.8, 0.83, 0.9);
pub const SETTINGS_LANG_SELECTED_BG: (f32, f32, f32, f32) = (0.15, 0.25, 0.45, 0.9);
pub const SETTINGS_LANG_SELECTED_BORDER: (f32, f32, f32, f32) = (0.4, 0.6, 0.9, 0.8);
pub const SETTINGS_DANGER_BG: (f32, f32, f32, f32) = (0.25, 0.08, 0.08, 0.9);
pub const SETTINGS_DANGER_BORDER: (f32, f32, f32, f32) = (0.7, 0.2, 0.2, 0.7);
pub const SETTINGS_DANGER_COLOR: (f32, f32, f32) = (0.95, 0.4, 0.4);
pub const SETTINGS_FADE_DURATION: f32 = 0.3;
pub const SETTINGS_GLOW_COLOR: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.12);
pub const SETTINGS_GLOW_SPREAD: f32 = 8.0;
pub const SETTINGS_GLOW_BLUR: f32 = 20.0;
pub const SETTINGS_COMING_SOON_COLOR: (f32, f32, f32) = (0.5, 0.45, 0.35);

// === Main Menu ===
pub const MENU_NUM_STARS: usize = 250;
pub const MENU_STAR_SPREAD_X: f32 = 80.0;
pub const MENU_STAR_SPREAD_Y: f32 = 50.0;
pub const MENU_STAR_MIN_SIZE: f32 = 0.02;
pub const MENU_STAR_MAX_SIZE: f32 = 0.10;
pub const MENU_STAR_DEPTH: f32 = -25.0;
pub const MENU_STAR_DRIFT_SPEED_MIN: f32 = 0.1;
pub const MENU_STAR_DRIFT_SPEED_MAX: f32 = 0.8;

pub const MENU_TITLE_FONT: f32 = 52.0;
pub const MENU_SUBTITLE_FONT: f32 = 16.0;
pub const MENU_SUBTITLE_COLOR: (f32, f32, f32, f32) = (0.6, 0.65, 0.75, 0.8);
pub const MENU_BUTTON_FONT: f32 = 20.0;
pub const MENU_BUTTON_WIDTH: f32 = 320.0;
pub const MENU_BUTTON_HEIGHT: f32 = 52.0;
pub const MENU_BUTTON_CORNER: f32 = 10.0;
pub const MENU_BUTTON_BORDER: f32 = 1.5;
pub const MENU_BUTTON_BG: (f32, f32, f32, f32) = (0.08, 0.09, 0.14, 0.85);
pub const MENU_BUTTON_BORDER_COLOR: (f32, f32, f32, f32) = (0.3, 0.35, 0.5, 0.5);
pub const MENU_BUTTON_HOVER_BG: (f32, f32, f32, f32) = (0.14, 0.16, 0.24, 0.95);
pub const MENU_BUTTON_HOVER_BORDER: (f32, f32, f32, f32) = (0.5, 0.6, 0.85, 0.8);
pub const MENU_BUTTON_HOVER_GLOW: (f32, f32, f32, f32) = (0.4, 0.5, 0.8, 0.25);
pub const MENU_BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.85, 0.9, 1.0);
pub const MENU_BUTTON_GAP: f32 = 12.0;

pub const MENU_QUOTE_FONT: f32 = 14.0;
pub const MENU_QUOTE_COLOR: (f32, f32, f32, f32) = (0.65, 0.7, 0.8, 0.7);

pub const MENU_TITLE_FADE_START: f32 = 0.0;
pub const MENU_TITLE_FADE_END: f32 = 2.0;
pub const MENU_BUTTON_FADE_START: f32 = 2.0;
pub const MENU_BUTTON_FADE_STAGGER: f32 = 0.3;
pub const MENU_QUOTE_FADE_START: f32 = 3.5;
pub const MENU_QUOTE_FADE_END: f32 = 4.5;
pub const MENU_FADE_OUT_DURATION: f32 = 1.0;

pub const MENU_CONFIRM_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const MENU_CONFIRM_BORDER: (f32, f32, f32, f32) = (0.4, 0.45, 0.6, 0.7);
pub const MENU_CONFIRM_TEXT_COLOR: (f32, f32, f32) = (0.8, 0.85, 0.95);
pub const MENU_CONFIRM_FONT: f32 = 17.0;
pub const MENU_CONFIRM_BTN_FONT: f32 = 16.0;

// === Credits ===
pub const CREDITS_SCROLL_SPEED: f32 = 50.0; // pixels per second
pub const CREDITS_FADE_IN_DURATION: f32 = 1.5;
pub const CREDITS_FADE_OUT_DURATION: f32 = 2.0;
pub const CREDITS_TITLE_FONT: f32 = 48.0;
pub const CREDITS_TAGLINE_FONT: f32 = 18.0;
pub const CREDITS_HEADING_FONT: f32 = 24.0;
pub const CREDITS_BODY_FONT: f32 = 16.0;
pub const CREDITS_SMALL_FONT: f32 = 14.0;
pub const CREDITS_QUOTE_FONT: f32 = 16.0;
pub const CREDITS_HEADING_COLOR: (f32, f32, f32) = (0.75, 0.82, 0.95);
pub const CREDITS_BODY_COLOR: (f32, f32, f32) = (0.7, 0.73, 0.8);
pub const CREDITS_TAGLINE_COLOR: (f32, f32, f32, f32) = (0.6, 0.65, 0.75, 0.85);
pub const CREDITS_QUOTE_COLOR: (f32, f32, f32, f32) = (0.65, 0.7, 0.8, 0.7);
pub const CREDITS_TITLE_COLOR: (f32, f32, f32) = (1.0, 1.0, 1.0);
pub const CREDITS_SKIP_FONT: f32 = 12.0;
pub const CREDITS_SKIP_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.5);
pub const CREDITS_CHARACTER_NAME_COLOR: (f32, f32, f32) = (0.8, 0.85, 0.95);
pub const CREDITS_CHARACTER_DESC_COLOR: (f32, f32, f32, f32) = (0.6, 0.65, 0.75, 0.8);
pub const CREDITS_SECTION_GAP: f32 = 40.0;
pub const CREDITS_LINE_GAP: f32 = 6.0;
pub const CREDITS_FINAL_CREW_COLOR: (f32, f32, f32) = (0.85, 0.8, 0.7);

// === Loading Screen ===
pub const LOADING_BAR_WIDTH: f32 = 400.0;
pub const LOADING_BAR_HEIGHT: f32 = 4.0;
pub const LOADING_BAR_CORNER: f32 = 2.0;
pub const LOADING_BAR_BG: (f32, f32, f32, f32) = (0.15, 0.16, 0.20, 1.0);
pub const LOADING_BAR_FILL: (f32, f32, f32) = (0.3, 0.7, 0.9);
pub const LOADING_BAR_GLOW_COLOR: (f32, f32, f32, f32) = (0.3, 0.7, 0.9, 0.35);
pub const LOADING_BAR_GLOW_BLUR: f32 = 12.0;
pub const LOADING_BAR_GLOW_SPREAD: f32 = 4.0;
pub const LOADING_STEP_FONT: f32 = 16.0;
pub const LOADING_STEP_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.8);
pub const LOADING_DETAIL_FONT: f32 = 13.0;
pub const LOADING_DETAIL_COLOR: (f32, f32, f32) = (0.5, 0.6, 0.7);
pub const LOADING_READY_COLOR: (f32, f32, f32) = (0.7, 0.85, 1.0);
pub const LOADING_TYPEWRITER_SPEED: f32 = 30.0; // chars per second
pub const LOADING_DURATION: f32 = 4.0; // total seconds for progress bar
pub const LOADING_HOLD_DURATION: f32 = 1.0; // pause after completion
pub const LOADING_DETAIL_FADE_SPEED: f32 = 2.0; // alpha per second for detail fade-in
pub const LOADING_READY_PULSE_SPEED: f32 = 2.5;
