// SPDX-License-Identifier: GPL-3.0-or-later

// Camera
pub const CAMERA_Z: f32 = 50.0;
pub const CAMERA_FOV: f32 = 45.0;

// Ship
pub const SHIP_LENGTH: f32 = 2.2;
pub const SHIP_WIDTH: f32 = 1.4;
pub const SHIP_HEIGHT: f32 = 0.45;
pub const SHIP_INERTIA: f32 = 6.0;
pub const SHIP_MAX_TILT: f32 = 0.45;
pub const SHIP_MAX_PITCH: f32 = 0.2;
pub const SHIP_TILT_SPEED: f32 = 5.0;
pub const SHIP_COLLISION_RADIUS: f32 = 0.7;
pub const SHIP_HULL_COLOR: [f32; 4] = [0.55, 0.6, 0.68, 1.0];
pub const SHIP_ACCENT_COLOR: [f32; 4] = [0.35, 0.4, 0.48, 1.0];
pub const SHIP_ENGINE_COLOR: [f32; 4] = [0.3, 0.7, 1.0, 1.0];
pub const SHIP_WINDOW_COLOR: [f32; 4] = [0.6, 0.85, 1.0, 1.0];

// Asteroids
pub const NUM_ASTEROID_MESHES: usize = 6;
pub const ASTEROID_ICO_SUBDIVISIONS: u32 = 2;
pub const ASTEROID_PERTURB: f32 = 0.25;
pub const ASTEROID_MIN_RADIUS: f32 = 0.8;
pub const ASTEROID_MAX_RADIUS: f32 = 5.0;
pub const ASTEROID_MIN_SPEED: f32 = 4.0;
pub const ASTEROID_MAX_SPEED: f32 = 14.0;
pub const ASTEROID_MIN_ROT_SPEED: f32 = 0.2;
pub const ASTEROID_MAX_ROT_SPEED: f32 = 1.5;
pub const ASTEROID_SPAWN_INTERVAL: f32 = 0.35;
pub const ASTEROID_SPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_DESPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_COLORS: [(f32, f32, f32); 5] = [
    (0.45, 0.40, 0.35), (0.35, 0.32, 0.30), (0.50, 0.42, 0.38),
    (0.40, 0.38, 0.42), (0.48, 0.45, 0.40),
];

// Damage
pub const SHIELD_MAX: f32 = 100.0;
pub const LIFE_MAX: f32 = 100.0;
pub const SHIELD_REGEN_RATE: f32 = 0.3;
pub const DAMAGE_SIZE_FACTOR: f32 = 4.0;
pub const DAMAGE_SPEED_FACTOR: f32 = 0.8;
pub const DAMAGE_GLANCING_MULT: f32 = 0.4;
pub const CONTROL_LOSS_DURATION: f32 = 0.5;
pub const CONTROL_LOSS_FACTOR: f32 = 0.3;
pub const SCREEN_SHAKE_DECAY: f32 = 8.0;

// Background stars
pub const NUM_STAR_LAYERS: usize = 4;
pub const STARS_PER_LAYER: [usize; NUM_STAR_LAYERS] = [20, 50, 100, 200];
pub const STAR_LAYER_SPEEDS: [f32; NUM_STAR_LAYERS] = [0.3, 0.8, 1.8, 3.5];
pub const STAR_LAYER_DEPTHS: [f32; NUM_STAR_LAYERS] = [-50.0, -35.0, -20.0, -8.0];
pub const STAR_SIZES: [f32; NUM_STAR_LAYERS] = [0.04, 0.07, 0.10, 0.15];
pub const STAR_BRIGHTNESS: [f32; NUM_STAR_LAYERS] = [0.3, 0.5, 0.7, 1.0];

// UI bars
pub const BAR_WIDTH_PX: f32 = 14.0;
pub const BAR_MARGIN_PX: f32 = 18.0;
pub const BAR_STROKE_PX: f32 = 2.0;
pub const BAR_TOP_PX: f32 = 40.0;
pub const BAR_BOTTOM_PX: f32 = 40.0;
pub const BAR_GAP_PX: f32 = 8.0;
pub const SHIELD_FULL_COLOR: (f32, f32, f32) = (0.4, 0.7, 1.0);
pub const SHIELD_LOW_COLOR: (f32, f32, f32) = (0.15, 0.3, 0.5);
pub const LIFE_FULL_COLOR: (f32, f32, f32) = (1.0, 0.9, 0.2);
pub const LIFE_LOW_COLOR: (f32, f32, f32) = (1.0, 0.15, 0.1);
pub const BAR_BG_COLOR: (f32, f32, f32, f32) = (0.1, 0.1, 0.12, 0.8);
pub const BAR_STROKE_COLOR: (f32, f32, f32, f32) = (0.3, 0.3, 0.35, 0.9);

// HUD
pub const HUD_FONT: f32 = 14.0;
pub const HUD_LABEL_COLOR: (f32, f32, f32, f32) = (0.7, 0.7, 0.75, 0.8);
pub const HUD_VALUE_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.9);
pub const HUD_MARGIN_PX: f32 = 12.0;
pub const HUD_GAP_PX: f32 = 6.0;

// Game
pub const SCROLL_SPEED: f32 = 5.0;
pub const CLEAR_COLOR_G: (f32, f32, f32) = (0.01, 0.01, 0.04);
pub const AMBIENT_COLOR_G: (f32, f32, f32) = (0.6, 0.6, 0.7);
pub const AMBIENT_BRIGHTNESS_G: f32 = 200.0;
pub const FADE_DURATION: f32 = 1.5;
pub const STATS_FONT: f32 = 18.0;
pub const STATS_TITLE_FONT: f32 = 28.0;
pub const STATS_CARD_BG: (f32, f32, f32) = (0.08, 0.08, 0.12);
pub const STATS_CARD_PAD: f32 = 40.0;
pub const STATS_CARD_GAP: f32 = 12.0;
pub const STATS_SUCCESS_COLOR: (f32, f32, f32) = (0.3, 0.7, 1.0);

// Crystal clouds
pub const CRYSTAL_MIN_RADIUS: f32 = 1.5;
pub const CRYSTAL_MAX_RADIUS: f32 = 3.5;
pub const CRYSTAL_ABSORB_RANGE: f32 = 4.0;
pub const CRYSTAL_ABSORB_RATE: f32 = 0.6;
pub const CRYSTAL_MIN_VALUE: u64 = 5_000;
pub const CRYSTAL_MAX_VALUE: u64 = 50_000;
pub const CRYSTAL_COLOR: (f32, f32, f32) = (0.2, 0.6, 1.0);
pub const CRYSTAL_EMISSIVE_MULT: f32 = 5.0;
pub const CRYSTAL_SPAWN_INTERVAL: f32 = 4.0;
pub const CRYSTAL_ICO_SUBDIVISIONS: u32 = 1;

// Difficulty
pub const DIFFICULTY_TIME_SCALE: f32 = 0.003;
pub const DIFFICULTY_CRYSTAL_SCALE: f32 = 0.00001;
pub const DIFFICULTY_MAX_SPAWN_MULT: f32 = 3.0;
pub const DIFFICULTY_MAX_SPEED_MULT: f32 = 1.8;
pub const DIFFICULTY_SIDE_SPAWN_CHANCE: f32 = 0.25;

// Asteroid-asteroid collision
pub const ASTEROID_BOUNCE_FACTOR: f32 = 0.8;

// Lighting
pub const DIR_LIGHT_DIR: [f32; 3] = [-0.4, -0.6, -0.8];
pub const DIR_LIGHT_BRIGHTNESS: f32 = 3000.0;
