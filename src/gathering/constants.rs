// SPDX-License-Identifier: GPL-3.0-or-later

// Camera
pub const CAMERA_Z: f32 = 50.0;
pub const CAMERA_FOV: f32 = 45.0;

// Ship geometry (smooth capsules + spheres)
pub const SHIP_HULL_R: f32 = 0.35;
pub const SHIP_HULL_LEN: f32 = 1.6;
pub const SHIP_NACELLE_R: f32 = 0.18;
pub const SHIP_NACELLE_LEN: f32 = 0.9;
pub const SHIP_NACELLE_OFF: f32 = 0.7;
pub const SHIP_NACELLE_Y: f32 = 0.2;
pub const SHIP_ENGINE_R: f32 = 0.14;
pub const SHIP_COCKPIT_R: f32 = 0.22;
pub const SHIP_WING_SPAN: f32 = 1.8;
pub const SHIP_WING_THICK: f32 = 0.06;
pub const SHIP_WING_CHORD: f32 = 0.6;
pub const SHIP_INERTIA: f32 = 6.0;
pub const SHIP_MAX_TILT: f32 = 1.4;
pub const SHIP_MAX_PITCH: f32 = 0.3;
pub const SHIP_TILT_SPEED: f32 = 8.0;
pub const SHIP_TILT_FACTOR: f32 = 0.07;
pub const SHIP_PITCH_FACTOR: f32 = 0.02;
pub const SHIP_COLLISION_RADIUS: f32 = 0.9;

// Ship colors (tuples for srgb)
pub const SHIP_HULL_COLOR: (f32, f32, f32) = (0.65, 0.68, 0.75);
pub const SHIP_ACCENT_COLOR: (f32, f32, f32) = (0.45, 0.50, 0.58);
pub const SHIP_ENGINE_COLOR: (f32, f32, f32) = (0.3, 0.75, 1.0);
pub const SHIP_ENGINE_EMISSIVE: f32 = 6.0;
pub const SHIP_COCKPIT_COLOR: (f32, f32, f32) = (0.5, 0.85, 1.0);
pub const SHIP_COCKPIT_EMISSIVE: f32 = 3.0;
pub const SHIP_WING_COLOR: (f32, f32, f32) = (0.5, 0.52, 0.58);

// Asteroids
pub const NUM_ASTEROID_MESHES: usize = 12;
pub const ASTEROID_ICO_SUBDIVISIONS: [u32; 4] = [1, 2, 2, 3];
pub const ASTEROID_PERTURB_MIN: f32 = 0.12;
pub const ASTEROID_PERTURB_MAX: f32 = 0.35;
pub const ASTEROID_ELONGATION_MIN: f32 = 0.7;
pub const ASTEROID_ELONGATION_MAX: f32 = 1.5;
pub const ASTEROID_MIN_RADIUS: f32 = 0.8;
pub const ASTEROID_MAX_RADIUS: f32 = 5.0;
pub const ASTEROID_MIN_SPEED: f32 = 2.0;
pub const ASTEROID_MAX_SPEED: f32 = 7.0;
pub const ASTEROID_MIN_ROT_SPEED: f32 = 0.2;
pub const ASTEROID_MAX_ROT_SPEED: f32 = 1.5;
pub const ASTEROID_SPAWN_INTERVAL: f32 = 0.50;
pub const ASTEROID_SPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_DESPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_COLORS: [(f32, f32, f32); 8] = [
    (0.45, 0.40, 0.35), (0.35, 0.32, 0.30), (0.50, 0.42, 0.38),
    (0.40, 0.38, 0.42), (0.48, 0.45, 0.40), (0.30, 0.28, 0.32),
    (0.55, 0.48, 0.40), (0.38, 0.35, 0.28),
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

// Crystal nebula clouds
pub const CRYSTAL_MIN_RADIUS: f32 = 1.5;
pub const CRYSTAL_MAX_RADIUS: f32 = 3.5;
pub const CRYSTAL_ABSORB_RANGE: f32 = 4.0;
pub const CRYSTAL_ABSORB_RATE: f32 = 0.6;
pub const CRYSTAL_MIN_VALUE: u64 = 5_000;
pub const CRYSTAL_MAX_VALUE: u64 = 50_000;
pub const CRYSTAL_SPAWN_INTERVAL: f32 = 4.0;
pub const CRYSTAL_ICO_SUBDIVISIONS: u32 = 2;
pub const CRYSTAL_NEBULA_LAYERS: usize = 8;
pub const CRYSTAL_NEBULA_PERTURB: f32 = 0.3;
pub const CRYSTAL_CORE_ALPHA: f32 = 0.25;
pub const CRYSTAL_OUTER_ALPHA: f32 = 0.04;
pub const CRYSTAL_CORE_EMISSIVE: f32 = 12.0;
pub const CRYSTAL_OUTER_EMISSIVE: f32 = 4.0;
pub const CRYSTAL_ROT_SPEED: f32 = 0.3;
pub const CRYSTAL_POINT_LIGHT_INTENSITY: f32 = 800.0;
pub const CRYSTAL_POINT_LIGHT_RANGE: f32 = 12.0;
pub const CRYSTAL_COLORS: [(f32, f32, f32); 5] = [
    (0.1, 0.4, 1.0),    // deep blue core
    (0.25, 0.55, 1.0),   // bright blue
    (0.45, 0.25, 0.9),   // purple tint
    (0.15, 0.7, 0.85),   // cyan
    (0.3, 0.35, 1.0),    // indigo
];

// Crystal absorption particles (pollen-like)
pub const PARTICLE_SIZE: f32 = 0.18;
pub const PARTICLE_SPEED: f32 = 14.0;
pub const PARTICLE_EMIT_RATE: f32 = 18.0;
pub const PARTICLE_LIFETIME: f32 = 1.5;
pub const PARTICLE_HOMING: f32 = 8.0;
pub const PARTICLE_LIGHT_INTENSITY: f32 = 40.0;
pub const PARTICLE_LIGHT_RANGE: f32 = 3.5;
pub const PARTICLE_EMISSIVE: f32 = 18.0;
pub const PARTICLE_SPREAD: f32 = 0.5;

// Difficulty
pub const DIFFICULTY_TIME_SCALE: f32 = 0.003;
pub const DIFFICULTY_CRYSTAL_SCALE: f32 = 0.00001;
pub const DIFFICULTY_MAX_SPAWN_MULT: f32 = 3.0;
pub const DIFFICULTY_MAX_SPEED_MULT: f32 = 1.8;
pub const DIFFICULTY_SIDE_SPAWN_CHANCE: f32 = 0.25;

// Asteroid-asteroid collision
pub const ASTEROID_BOUNCE_FACTOR: f32 = 0.8;
pub const ASTEROID_SEPARATION_SPEED: f32 = 3.0;

// Lighting
pub const DIR_LIGHT_DIR: [f32; 3] = [-0.4, -0.6, -0.8];
pub const DIR_LIGHT_BRIGHTNESS: f32 = 3000.0;
