// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

impl Default for Difficulty {
    fn default() -> Self {
        Self { spawn_mult: 1.0, speed_mult: 1.0, side_chance: 0.0 }
    }
}

pub fn update_difficulty(
    state: Res<ShipState>,
    mut difficulty: ResMut<Difficulty>,
    mut timer: ResMut<AsteroidSpawnTimer>,
) {
    if !state.alive { return; }
    let time_factor = state.elapsed_time * DIFFICULTY_TIME_SCALE;
    let crystal_factor = state.crystals as f32 * DIFFICULTY_CRYSTAL_SCALE;
    let combined = (time_factor + crystal_factor).min(1.0);

    let new_spawn = 1.0 + combined * (DIFFICULTY_MAX_SPAWN_MULT - 1.0);
    let new_speed = 1.0 + combined * (DIFFICULTY_MAX_SPEED_MULT - 1.0);
    difficulty.spawn_mult = new_spawn;
    difficulty.speed_mult = new_speed;
    difficulty.side_chance = combined * DIFFICULTY_SIDE_SPAWN_CHANCE;

    let interval = ASTEROID_SPAWN_INTERVAL / new_spawn;
    timer.0.set_duration(std::time::Duration::from_secs_f32(interval));
}
