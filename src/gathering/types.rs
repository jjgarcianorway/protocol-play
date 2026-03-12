// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;

// === Components ===
#[derive(Component)] pub struct Ship;
#[derive(Component)] pub struct ShipPart;

#[derive(Component)]
pub struct Asteroid {
    pub radius: f32,
    pub speed: f32,
    pub rot_axis: Vec3,
    pub rot_speed: f32,
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

// === Resources ===
#[derive(Resource)]
pub struct ShipState {
    pub target: Vec2,
    pub velocity: Vec2,
    pub shield: f32,
    pub life: f32,
    pub crystals: u64,
    pub distance: f32,
    pub elapsed_time: f32,
    pub hits_taken: u32,
    pub control_loss_timer: f32,
    pub alive: bool,
}

impl Default for ShipState {
    fn default() -> Self {
        Self {
            target: Vec2::ZERO, velocity: Vec2::ZERO,
            shield: SHIELD_MAX, life: LIFE_MAX,
            crystals: 0, distance: 0.0, elapsed_time: 0.0,
            hits_taken: 0, control_loss_timer: 0.0, alive: true,
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
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GatheringState {
    #[default]
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct GatheringFont(pub Handle<Font>);
