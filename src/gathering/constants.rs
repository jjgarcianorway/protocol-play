// SPDX-License-Identifier: GPL-3.0-or-later

// Camera
pub const CAMERA_Z: f32 = 50.0;
pub const CAMERA_FOV: f32 = 45.0;

// Ship dynamics
pub const SHIP_INERTIA: f32 = 12.0;
pub const SHIP_MAX_TILT: f32 = 1.4;
pub const SHIP_MAX_PITCH: f32 = 0.3;
pub const SHIP_TILT_SPEED: f32 = 8.0;
pub const SHIP_TILT_FACTOR: f32 = 0.07;
pub const SHIP_PITCH_FACTOR: f32 = 0.02;
pub const SHIP_COLLISION_RADIUS: f32 = 0.9;
pub const SHIP_MODEL_SCALE: f32 = 0.6;

// Asteroids
pub const NUM_ASTEROID_MESHES: usize = 12;
pub const ASTEROID_ICO_SUBDIVISIONS: [u32; 4] = [1, 2, 2, 3];
pub const ASTEROID_PERTURB_MIN: f32 = 0.12;
pub const ASTEROID_PERTURB_MAX: f32 = 0.35;
pub const ASTEROID_ELONGATION_MIN: f32 = 0.7;
pub const ASTEROID_ELONGATION_MAX: f32 = 1.5;
pub const ASTEROID_MIN_SPEED: f32 = 2.0;
pub const ASTEROID_MAX_SPEED: f32 = 7.0;
pub const ASTEROID_MIN_ROT_SPEED: f32 = 0.2;
pub const ASTEROID_MAX_ROT_SPEED: f32 = 1.5;
pub const ASTEROID_SPAWN_INTERVAL: f32 = 0.8;
pub const ASTEROID_SPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_DESPAWN_BUFFER: f32 = 8.0;
pub const ASTEROID_COLORS: [(f32, f32, f32); 8] = [
    (0.38, 0.40, 0.45), // cool grey
    (0.30, 0.32, 0.38), // dark blue-grey
    (0.42, 0.40, 0.48), // purple-grey
    (0.35, 0.38, 0.42), // slate
    (0.45, 0.43, 0.50), // lavender stone
    (0.28, 0.30, 0.38), // deep blue-grey
    (0.40, 0.38, 0.45), // mauve stone
    (0.33, 0.35, 0.40), // steel
];

// Damage
pub const SHIELD_MAX: f32 = 60.0;
pub const LIFE_MAX: f32 = 100.0;
pub const SHIELD_REGEN_RATE: f32 = 0.5;
pub const SHIELD_REGEN_DELAY: f32 = 5.0; // seconds without being hit before regen starts
pub const DAMAGE_SIZE_FACTOR: f32 = 0.8;
pub const DAMAGE_SPEED_FACTOR: f32 = 0.15;
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
pub const HUD_FONT: f32 = 16.0;
pub const HUD_LABEL_COLOR: (f32, f32, f32, f32) = (0.7, 0.7, 0.75, 0.8);
pub const HUD_VALUE_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 0.9);
pub const HUD_MARGIN_PX: f32 = 12.0;
pub const HUD_GAP_PX: f32 = 6.0;

// Game
pub const SCROLL_SPEED: f32 = 5.0;
pub const CLEAR_COLOR_G: (f32, f32, f32) = (0.02, 0.03, 0.08);
pub const AMBIENT_COLOR_G: (f32, f32, f32) = (0.6, 0.6, 0.7);
pub const AMBIENT_BRIGHTNESS_G: f32 = 200.0;
pub const FADE_DURATION: f32 = 1.5;
pub const STATS_FONT: f32 = 20.0;
pub const STATS_TITLE_FONT: f32 = 32.0;
pub const STATS_CARD_BG: (f32, f32, f32) = (0.08, 0.08, 0.12);
pub const STATS_CARD_PAD: f32 = 40.0;
pub const STATS_CARD_GAP: f32 = 12.0;
pub const STATS_SUCCESS_COLOR: (f32, f32, f32) = (0.3, 0.8, 0.7);

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
pub const CRYSTAL_POINT_LIGHT_INTENSITY: f32 = 400.0;
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
pub const PARTICLE_LIGHT_INTENSITY: f32 = 15.0;
pub const PARTICLE_LIGHT_RANGE: f32 = 3.5;
pub const PARTICLE_EMISSIVE: f32 = 18.0;
pub const PARTICLE_SPREAD: f32 = 0.5;

// Difficulty
pub const DIFFICULTY_TIME_SCALE: f32 = 0.003;
pub const DIFFICULTY_CRYSTAL_SCALE: f32 = 0.00001;
pub const DIFFICULTY_MAX_SPAWN_MULT: f32 = 3.5;
pub const DIFFICULTY_MAX_SPEED_MULT: f32 = 2.0;
pub const DIFFICULTY_SIDE_SPAWN_CHANCE: f32 = 0.25;

// Asteroid-asteroid collision
pub const ASTEROID_BOUNCE_FACTOR: f32 = 0.8;
pub const ASTEROID_SEPARATION_SPEED: f32 = 6.0;

// Lighting
pub const DIR_LIGHT_DIR: [f32; 3] = [-0.4, -0.6, -0.8];
pub const DIR_LIGHT_BRIGHTNESS: f32 = 3000.0;

// Hit flash
pub const HIT_FLASH_DURATION: f32 = 0.3;
pub const HIT_COOLDOWN_SECS: f32 = 0.5;

// Floating text
pub const FLOAT_TEXT_LIFETIME: f32 = 1.5;
pub const FLOAT_TEXT_RISE_SPEED: f32 = 60.0;
pub const FLOAT_TEXT_FONT: f32 = 22.0;
// Pause
pub const PAUSE_OVERLAY_ALPHA: f32 = 0.7;
pub const PAUSE_FONT: f32 = 48.0;

// Engine trail particles
pub const ENGINE_PARTICLE_SIZE: f32 = 0.08;
pub const ENGINE_PARTICLE_LIFETIME: f32 = 0.4;
pub const ENGINE_PARTICLES_PER_SEC: f32 = 30.0;
pub const ENGINE_PARTICLE_SPEED: f32 = 3.0;
pub const ENGINE_PARTICLE_EMISSIVE: f32 = 8.0;
pub const ENGINE_COLOR: (f32, f32, f32) = (0.3, 0.75, 1.0);
pub const ENGINE_OFFSETS: [(f32, f32, f32); 3] = [
    (-0.42, -0.39, 0.0),
    (0.42, -0.39, 0.0),
    (0.0, -0.51, 0.0),
];

// Background nebula planes
pub const NEBULA_SCROLL_SPEED_MULT: f32 = 0.3;
pub const NEBULA_CONFIGS: [(f32, f32, f32, f32, f32); 3] = [
    // (z, alpha, r, g, b) — far behind everything
    (-70.0, 0.06, 0.05, 0.25, 0.35),  // teal
    (-65.0, 0.07, 0.2, 0.08, 0.35),   // purple
    (-75.0, 0.05, 0.08, 0.12, 0.35),  // deep blue
];
pub const NEBULA_SIZE: f32 = 50.0;

// Asteroid collision sparks
pub const SPARK_COUNT: usize = 4;
pub const SPARK_SPEED: f32 = 8.0;
pub const SPARK_LIFETIME: f32 = 0.3;
pub const SPARK_SIZE: f32 = 0.12;
pub const SPARK_EMISSIVE: f32 = 15.0;
pub const SPARK_COLOR: (f32, f32, f32) = (1.0, 0.7, 0.2);

// Shield bubble
pub const SHIELD_BUBBLE_RADIUS: f32 = 1.3;
pub const SHIELD_BUBBLE_MAX_ALPHA: f32 = 0.008;
pub const SHIELD_BUBBLE_EMISSIVE: f32 = 0.15;
pub const SHIELD_BUBBLE_COLOR: (f32, f32, f32) = (0.4, 0.7, 1.0);
pub const SHIELD_BUBBLE_PULSE_SPEED: f32 = 2.0;
pub const SHIELD_BUBBLE_PULSE_AMOUNT: f32 = 0.03;

// Warning indicators (screen-edge asteroid approach)
pub const WARNING_LEAD_TIME: f32 = 3.0;
pub const WARNING_ARROW_SIZE: f32 = 20.0;
pub const WARNING_ARROW_COLOR: (f32, f32, f32) = (1.0, 0.2, 0.15);
pub const WARNING_MARGIN_PX: f32 = 8.0;

// Magnet range ring
pub const MAGNET_RING_ALPHA: f32 = 0.08;
pub const MAGNET_RING_NEARBY_MULT: f32 = 2.0;
pub const MAGNET_RING_FADE_SPEED: f32 = 4.0;

// Damage direction indicator
pub const DAMAGE_DIR_FADE_TIME: f32 = 0.5;
#[allow(dead_code)]
pub const DAMAGE_DIR_WIDTH: f32 = 80.0;
pub const DAMAGE_DIR_ALPHA: f32 = 0.6;
pub const DAMAGE_DIR_COLOR: (f32, f32, f32) = (1.0, 0.1, 0.05);

// Pause overlay fade
pub const PAUSE_FADE_IN_SECS: f32 = 0.15;

// Game over card fade
pub const GAME_OVER_CARD_FADE_SECS: f32 = 0.6;

// Warning indicator pulse
pub const WARNING_PULSE_SPEED: f32 = 6.0;
pub const WARNING_PULSE_AMOUNT: f32 = 0.3;

// Background color shift with difficulty
pub const CLEAR_COLOR_WARM: (f32, f32, f32) = (0.04, 0.02, 0.06);

// Crystal chain bonus
pub const CHAIN_TIMEOUT: f32 = 5.0;
pub const CHAIN_MULTIPLIERS: [f32; 4] = [1.0, 1.2, 1.5, 2.0];
pub const CHAIN_TEXT_COLOR: (f32, f32, f32) = (1.0, 0.85, 0.2);

// Ship damage smoke/sparks
pub const SMOKE_PARTICLE_SIZE: f32 = 0.15;
pub const SMOKE_LIFETIME: f32 = 0.8;
pub const SMOKE_SPEED: f32 = 1.5;
pub const SMOKE_COLOR: (f32, f32, f32, f32) = (0.15, 0.12, 0.1, 0.5);
pub const SPARK_DAMAGE_SIZE: f32 = 0.06;
pub const SPARK_DAMAGE_LIFETIME: f32 = 0.4;
pub const SPARK_DAMAGE_SPEED: f32 = 4.0;
pub const SPARK_DAMAGE_COLOR: (f32, f32, f32) = (1.0, 0.6, 0.1);
pub const SMOKE_BASE_RATE: f32 = 5.0;
pub const SPARK_DAMAGE_BASE_RATE: f32 = 3.0;

// Asteroid size scaling with difficulty
pub const ASTEROID_EARLY_MIN_R: f32 = 0.8;
pub const ASTEROID_EARLY_MAX_R: f32 = 2.5;
pub const ASTEROID_LATE_MIN_R: f32 = 2.0;
pub const ASTEROID_LATE_MAX_R: f32 = 5.0;

// Crystal placement bias toward asteroids
pub const CRYSTAL_ASTEROID_BIAS: f32 = 0.3;

// Near-miss shield recovery
pub const NEAR_MISS_RANGE: f32 = 0.8; // outer edge beyond collision
pub const NEAR_MISS_MIN_GAP: f32 = 0.3; // inner edge beyond collision (closest)
pub const NEAR_MISS_SHIELD_RECOVERY: f32 = 0.0; // no shield from near misses
pub const NEAR_MISS_COOLDOWN: f32 = 2.0; // seconds before same asteroid can trigger again
pub const NEAR_MISS_TEXT_COLOR: (f32, f32, f32) = (0.5, 0.85, 1.0); // light blue
pub const NEAR_MISS_FLASH_DURATION: f32 = 0.15;
pub const NEAR_MISS_FLASH_COLOR: (f32, f32, f32) = (0.2, 0.9, 0.7); // teal

// Asteroid types
pub const ASTEROID_ROCK_ROUGHNESS: f32 = 0.7;
pub const ASTEROID_ROCK_METALLIC: f32 = 0.15;
pub const ASTEROID_ICE_ROUGHNESS: f32 = 0.3;
pub const ASTEROID_ICE_METALLIC: f32 = 0.1;
pub const ASTEROID_ICE_EMISSIVE: f32 = 0.8;
pub const ASTEROID_ICE_DAMAGE_MULT: f32 = 0.75;
pub const ASTEROID_METALLIC_ROUGHNESS: f32 = 0.2;
pub const ASTEROID_METALLIC_METALLIC: f32 = 0.9;
pub const ASTEROID_METALLIC_DAMAGE_MULT: f32 = 1.3;
pub const ASTEROID_ICE_COLORS: [(f32, f32, f32); 4] = [
    (0.70, 0.85, 1.00),
    (0.60, 0.80, 0.95),
    (0.75, 0.90, 1.00),
    (0.55, 0.75, 0.92),
];
pub const ASTEROID_METALLIC_COLORS: [(f32, f32, f32); 4] = [
    (0.55, 0.55, 0.60),
    (0.45, 0.45, 0.52),
    (0.50, 0.50, 0.58),
    (0.40, 0.42, 0.50),
];

// Asteroid trail wisps (large asteroids only)
pub const TRAIL_MIN_RADIUS: f32 = 3.0;
pub const TRAIL_PARTICLES_PER_SEC: f32 = 7.0;
pub const TRAIL_PARTICLE_LIFETIME: f32 = 0.3;
pub const TRAIL_PARTICLE_SIZE: f32 = 0.1;
pub const TRAIL_PARTICLE_ALPHA: f32 = 0.25;
pub const TRAIL_PARTICLE_COLOR: (f32, f32, f32) = (0.4, 0.38, 0.35);

// Crystal value scaling with difficulty
pub const CRYSTAL_DIFFICULTY_SCALE: f32 = 0.5;

// Crystal resource colors (5 distinct types)
pub const CRYSTAL_RESOURCE_COLORS: [(f32, f32, f32); 5] = [
    (0.902, 0.098, 0.294), // Red — Power
    (0.130, 0.545, 0.130), // Green — Life Support
    (0.150, 0.250, 0.700), // Blue — Cryo
    (1.000, 0.882, 0.098), // Yellow — Shields
    (0.569, 0.118, 0.706), // Purple — Repair
];

