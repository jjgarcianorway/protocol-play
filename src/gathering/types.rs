// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;

// === Components ===
#[derive(Component)] pub struct Ship;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CrystalColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl CrystalColor {
    pub const ALL: [CrystalColor; 5] = [
        CrystalColor::Red, CrystalColor::Green, CrystalColor::Blue,
        CrystalColor::Yellow, CrystalColor::Purple,
    ];

    pub fn rgb(&self) -> (f32, f32, f32) {
        match self {
            CrystalColor::Red    => (0.902, 0.098, 0.294),
            CrystalColor::Green  => (0.130, 0.545, 0.130),
            CrystalColor::Blue   => (0.150, 0.250, 0.700),
            CrystalColor::Yellow => (1.000, 0.882, 0.098),
            CrystalColor::Purple => (0.569, 0.118, 0.706),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CrystalColor::Red    => "Power",
            CrystalColor::Green  => "Life Support",
            CrystalColor::Blue   => "Cryo",
            CrystalColor::Yellow => "Shields",
            CrystalColor::Purple => "Repair",
        }
    }

    pub fn resource_icon(&self) -> &str {
        match self {
            CrystalColor::Red    => "\u{26A1}",
            CrystalColor::Green  => "\u{1F4A7}",
            CrystalColor::Blue   => "\u{2744}\u{FE0F}",
            CrystalColor::Yellow => "\u{1F6E1}\u{FE0F}",
            CrystalColor::Purple => "\u{1F527}",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            CrystalColor::Red    => 0,
            CrystalColor::Green  => 1,
            CrystalColor::Blue   => 2,
            CrystalColor::Yellow => 3,
            CrystalColor::Purple => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AsteroidType {
    Rock,
    Ice,
    Metallic,
}

#[derive(Component)]
pub struct Asteroid {
    pub radius: f32,
    pub velocity: Vec2,
    pub rot_axis: Vec3,
    pub rot_speed: f32,
    pub asteroid_type: AsteroidType,
}

#[derive(Component)]
pub struct NearMissCooldown(pub f32);

#[derive(Component)]
pub struct AsteroidTrailParticle {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct Star {
    pub layer: usize,
}

#[derive(Component)] pub struct ShieldBarFill;
#[derive(Component)] pub struct LifeBarFill;
#[derive(Component)] pub struct DistanceText;
#[derive(Component)] pub struct TimeText;
#[derive(Component)] pub struct GameOverScreen;
#[derive(Component)] pub struct TryAgainButton;
#[derive(Component)] pub struct CrystalText;
#[derive(Component)] pub struct PauseScreen;
#[derive(Component)] pub struct ShieldBubble;

#[derive(Component)]
pub struct CrystalCloud {
    pub radius: f32,
    pub value: u64,
    pub remaining: f32,
    pub rot_axis: Vec3,
    pub particle_timer: f32,
    pub color: CrystalColor,
}

#[derive(Component)]
pub struct CrystalParticle {
    pub velocity: Vec3,
    pub lifetime: f32,
}

// === Resources ===
#[derive(Resource)]
pub struct ShipState {
    pub target: Vec2,
    pub velocity: Vec2,
    pub shield: f32,
    pub life: f32,
    pub crystals: u64,
    pub crystals_red: u64,
    pub crystals_green: u64,
    pub crystals_blue: u64,
    pub crystals_yellow: u64,
    pub crystals_purple: u64,
    pub distance: f32,
    pub elapsed_time: f32,
    pub hits_taken: u32,
    pub near_misses: u32,
    pub max_chain: f32,
    pub control_loss_timer: f32,
    pub alive: bool,
}

impl ShipState {
    pub fn add_crystals(&mut self, amount: u64, color: CrystalColor) {
        self.crystals += amount;
        match color {
            CrystalColor::Red    => self.crystals_red += amount,
            CrystalColor::Green  => self.crystals_green += amount,
            CrystalColor::Blue   => self.crystals_blue += amount,
            CrystalColor::Yellow => self.crystals_yellow += amount,
            CrystalColor::Purple => self.crystals_purple += amount,
        }
    }

    pub fn crystals_by_color(&self, color: CrystalColor) -> u64 {
        match color {
            CrystalColor::Red    => self.crystals_red,
            CrystalColor::Green  => self.crystals_green,
            CrystalColor::Blue   => self.crystals_blue,
            CrystalColor::Yellow => self.crystals_yellow,
            CrystalColor::Purple => self.crystals_purple,
        }
    }
}

impl Default for ShipState {
    fn default() -> Self {
        Self {
            target: Vec2::ZERO, velocity: Vec2::ZERO,
            shield: SHIELD_MAX, life: LIFE_MAX,
            crystals: 0,
            crystals_red: 0, crystals_green: 0, crystals_blue: 0,
            crystals_yellow: 0, crystals_purple: 0,
            distance: 0.0, elapsed_time: 0.0,
            hits_taken: 0, near_misses: 0, max_chain: 1.0,
            control_loss_timer: 0.0, alive: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub intensity: f32,
    pub offset: Vec3,
}

#[derive(Resource)]
pub struct ViewBounds {
    pub half_width: f32,
    pub half_height: f32,
}

impl Default for ViewBounds {
    fn default() -> Self {
        Self { half_width: 30.0, half_height: 20.0 }
    }
}

#[derive(Resource)]
pub struct AsteroidSpawnTimer(pub Timer);

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(ASTEROID_SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct GatheringAssets {
    pub asteroid_meshes: Vec<Handle<Mesh>>,
    pub asteroid_materials: Vec<Handle<StandardMaterial>>,
    pub ice_materials: Vec<Handle<StandardMaterial>>,
    pub metallic_materials: Vec<Handle<StandardMaterial>>,
    pub crystal_meshes: Vec<Handle<Mesh>>,
    pub crystal_materials: Vec<Handle<StandardMaterial>>,
    pub crystal_materials_by_color: Vec<Vec<Handle<StandardMaterial>>>,
    pub particle_mesh: Handle<Mesh>,
    pub particle_materials: Vec<Handle<StandardMaterial>>,
    pub particle_materials_by_color: Vec<Handle<StandardMaterial>>,
    pub trail_mesh: Handle<Mesh>,
    pub trail_material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct CrystalSpawnTimer(pub Timer);

impl Default for CrystalSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(CRYSTAL_SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct Difficulty {
    pub spawn_mult: f32,
    pub speed_mult: f32,
    pub side_chance: f32,
    pub combined: f32,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GatheringState {
    #[default]
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct GatheringFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct HitFlash {
    pub timer: f32,
}

#[derive(Resource, Default)]
pub struct NearMissFlash {
    pub timer: f32,
}

#[derive(Component)]
pub struct FloatingText {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub text_color: (f32, f32, f32),
}

#[derive(Component)]
pub struct Spark {
    pub velocity: Vec3,
    pub lifetime: f32,
}

#[derive(Component)]
pub struct EngineParticle {
    pub lifetime: f32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct NebulaPlane;

#[derive(Resource, Default)]
pub struct Paused(pub bool);

// === Warning indicators (screen-edge asteroid warnings) ===
#[derive(Component)]
pub struct WarningIndicator {
    pub asteroid_entity: Entity,
}

// === Damage direction indicator ===
#[derive(Component)]
pub struct DamageDirectionIndicator {
    pub timer: f32,
}

// === Crystal magnet range ring ===
#[derive(Component)]
pub struct MagnetRing;

// === Crystal chain bonus ===
#[derive(Resource)]
pub struct CrystalChain {
    pub timer: f32,
    pub multiplier: f32,
}

impl Default for CrystalChain {
    fn default() -> Self {
        Self { timer: 0.0, multiplier: 1.0 }
    }
}

// === Ship damage smoke/sparks ===
#[derive(Component)]
pub struct DamageSmoke {
    pub lifetime: f32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct DamageSpark {
    pub lifetime: f32,
    pub velocity: Vec3,
}

#[derive(Resource)]
pub struct BestStats {
    pub best_distance_au: f32,
    pub best_crystals: u64,
    pub best_time_days: u32,
    pub total_sessions: u32,
}

impl Default for BestStats {
    fn default() -> Self {
        Self { best_distance_au: 0.0, best_crystals: 0, best_time_days: 0, total_sessions: 0 }
    }
}
