// SPDX-License-Identifier: GPL-3.0-or-later

//! Resource depletion system — resources drain slowly over time,
//! creating urgency for the player to take action.

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;
use crate::save_state::{save_game_state, GameState};

/// System: drain resources each frame proportional to elapsed game-days.
/// Also handles periodic saving, crew loss when cryo is 0, and Anna warnings.
pub fn drain_resources(
    time: Res<Time>,
    mut drain: ResMut<DrainTimer>,
    mut ship: ResMut<ShipStatus>,
    mut gs: ResMut<GameState>,
    mut anna: ResMut<AnnaState>,
    running_game: Res<RunningGame>,
) {
    // Don't drain while a child game is running
    if running_game.0.is_some() {
        return;
    }

    let dt = time.delta_secs();
    drain.day_timer += dt;
    drain.save_timer += dt;

    // Calculate fractional days elapsed this frame
    let day_fraction = dt / DAY_DURATION_SECS;

    // Apply drain to each resource
    let mut any_hit_zero = false;

    let old_power = ship.power;
    ship.power = (ship.power - POWER_DRAIN * day_fraction).max(0.0);
    if ship.power <= 0.0 && old_power > 0.0 { any_hit_zero = true; }

    let old_life = ship.life_support;
    ship.life_support = (ship.life_support - LIFE_SUPPORT_DRAIN * day_fraction).max(0.0);
    if ship.life_support <= 0.0 && old_life > 0.0 { any_hit_zero = true; }

    let old_cryo = ship.cryo;
    ship.cryo = (ship.cryo - CRYO_DRAIN * day_fraction).max(0.0);
    if ship.cryo <= 0.0 && old_cryo > 0.0 { any_hit_zero = true; }

    let old_shields = ship.shields;
    ship.shields = (ship.shields - SHIELDS_DRAIN * day_fraction).max(0.0);
    if ship.shields <= 0.0 && old_shields > 0.0 { any_hit_zero = true; }

    let old_repair = ship.repair;
    ship.repair = (ship.repair - REPAIR_DRAIN * day_fraction).max(0.0);
    if ship.repair <= 0.0 && old_repair > 0.0 { any_hit_zero = true; }

    // Crew loss when cryo is at 0
    if ship.cryo <= 0.0 && drain.day_timer >= DAY_DURATION_SECS {
        let mut rng = rand::thread_rng();
        let loss = rng.gen_range(CRYO_ZERO_CREW_LOSS_MIN..=CRYO_ZERO_CREW_LOSS_MAX);
        ship.crew_count = ship.crew_count.saturating_sub(loss);
    }

    // Advance day counter
    if drain.day_timer >= DAY_DURATION_SECS {
        drain.day_timer -= DAY_DURATION_SECS;
        ship.day += 1;
        // Sync to GameState
        sync_ship_to_gs(&ship, &mut gs);
    }

    // Queue Anna warning if any resource just hit 0
    if any_hit_zero {
        queue_depletion_warning(&ship, &mut anna);
    }

    // Periodic save
    if drain.save_timer >= SAVE_INTERVAL_SECS {
        drain.save_timer -= SAVE_INTERVAL_SECS;
        sync_ship_to_gs(&ship, &mut gs);
        save_game_state(&gs);
    }
}

/// Sync ShipStatus values back into GameState.
fn sync_ship_to_gs(ship: &ShipStatus, gs: &mut GameState) {
    gs.power = ship.power;
    gs.life_support = ship.life_support;
    gs.cryo = ship.cryo;
    gs.shields = ship.shields;
    gs.repair = ship.repair;
    gs.crew_count = ship.crew_count;
    gs.day = ship.day;
    gs.distance_au = ship.distance_au;
}

/// Queue an Anna warning message when a resource hits zero.
fn queue_depletion_warning(ship: &ShipStatus, anna: &mut AnnaState) {
    let msg = if ship.power <= 0.0 {
        "Power is gone. I can barely keep the lights on."
    } else if ship.life_support <= 0.0 {
        "Life support has failed. We're running out of air."
    } else if ship.cryo <= 0.0 {
        "Cryogenic systems offline. We're losing crew."
    } else if ship.shields <= 0.0 {
        "Shields are down. We're completely exposed."
    } else if ship.repair <= 0.0 {
        "Repair systems depleted. The ship is falling apart."
    } else {
        return;
    };
    anna.queue.push((msg.to_string(), false));
}
