// SPDX-License-Identifier: GPL-3.0-or-later

// Grid
pub const GRID_COLS: usize = 8;
pub const GRID_ROWS: usize = 8;
pub const CELL_SIZE: f32 = 56.0;
pub const CELL_GAP: f32 = 3.0;
pub const CELL_CORNER: f32 = 8.0;
pub const CELL_BORDER: f32 = 2.0;

// Colors (clear background matches other games)
pub const CLEAR_COLOR_C: (f32, f32, f32) = (0.05, 0.06, 0.12);

// Crystal colors
pub const CRYSTAL_RED: (f32, f32, f32) = (0.902, 0.098, 0.294);
pub const CRYSTAL_GREEN: (f32, f32, f32) = (0.130, 0.545, 0.130);
pub const CRYSTAL_BLUE: (f32, f32, f32) = (0.150, 0.250, 0.700);
pub const CRYSTAL_YELLOW: (f32, f32, f32) = (1.000, 0.882, 0.098);
pub const CRYSTAL_PURPLE: (f32, f32, f32) = (0.569, 0.118, 0.706);

// Crystal color array (indexed by CrystalColor::index())
pub const CRYSTAL_COLORS: [(f32, f32, f32); 5] = [
    CRYSTAL_RED, CRYSTAL_GREEN, CRYSTAL_BLUE, CRYSTAL_YELLOW, CRYSTAL_PURPLE,
];

// Resource names and icons
pub const RESOURCE_NAMES: [&str; 5] = ["Power", "Life Support", "Cryo", "Shields", "Repair"];
pub const RESOURCE_ICONS: [&str; 5] = ["\u{26A1}", "\u{1F4A7}", "\u{2744}", "\u{1F6E1}", "\u{1F527}"];

// Efficiency multipliers: (min_chain_size, multiplier)
pub const EFFICIENCY_TABLE: [(u32, f32); 6] = [
    (1, 0.3),
    (2, 0.7),
    (4, 1.0),
    (7, 1.5),
    (11, 2.0),
    (16, 3.0),
];

// Crystal pile
pub const INITIAL_PILE_SIZE: u64 = 500;

// Timing
pub const GRAVITY_DELAY: f32 = 0.15;
pub const CASCADE_DELAY: f32 = 0.5;
pub const PARTICLE_LIFETIME: f32 = 0.6;
pub const PARTICLE_SIZE: f32 = 8.0;

// UI layout
pub const GRID_LEFT_MARGIN: f32 = 200.0;
pub const TANK_WIDTH: f32 = 30.0;
pub const TANK_HEIGHT: f32 = 300.0;
pub const TANK_GAP: f32 = 8.0;
pub const TANK_CORNER: f32 = 6.0;
pub const TANK_BG: (f32, f32, f32, f32) = (0.15, 0.15, 0.20, 0.8);
pub const PILE_BAR_WIDTH: f32 = 30.0;
pub const PILE_BAR_HEIGHT: f32 = 300.0;

// Highlight
pub const HIGHLIGHT_BORDER_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.8);
pub const HIGHLIGHT_PULSE_SPEED: f32 = 4.0;

// Cell empty color
pub const CELL_EMPTY_COLOR: (f32, f32, f32, f32) = (0.10, 0.11, 0.18, 0.6);

// Font sizes
pub const TITLE_FONT: f32 = 28.0;
pub const CHAIN_SIZE_FONT: f32 = 22.0;
pub const TANK_LABEL_FONT: f32 = 18.0;
pub const TANK_PCT_FONT: f32 = 12.0;
pub const PILE_FONT: f32 = 14.0;
pub const RESULTS_TITLE_FONT: f32 = 36.0;
pub const RESULTS_FONT: f32 = 20.0;
pub const RESULTS_BTN_FONT: f32 = 18.0;

// Panel backgrounds
pub const RESULTS_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const BTN_BG_C: (f32, f32, f32) = (0.15, 0.18, 0.28);
pub const BTN_HOVER_C: (f32, f32, f32) = (0.22, 0.26, 0.38);

// Bloom
pub const BLOOM_INTENSITY_C: f32 = 0.15;
pub const BLOOM_LF_BOOST_C: f32 = 0.3;

// Max resource capacity (percentage-based, 1.0 = 100%)
pub const RESOURCE_MAX: f32 = 100.0;
