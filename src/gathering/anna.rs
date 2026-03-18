// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's brief in-game comments for The Gathering.

use bevy::prelude::*;
use crate::anna_comments::*;
use super::types::*;

/// Set up Anna comments for The Gathering.
pub fn setup_gathering_anna(mut commands: Commands, font: Res<GatheringFont>) {
    let pool: Vec<&str> = vec![
        "Watch the big ones.",
        "The crystals are remnants of dead stars.",
        "Stay sharp out there.",
        "Every crystal counts.",
    ];
    let queue = build_queue(&pool, 3);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0);
}

/// Context-sensitive Anna comments that react to game state.
pub fn gathering_anna_reactive(
    ship: Res<ShipState>,
    mut anna: ResMut<AnnaComments>,
) {
    // Only trigger if no comment currently showing and queue has room
    if anna.current.is_some() { return; }

    let shield_pct = ship.shield / super::constants::SHIELD_MAX;
    let life_pct = ship.life / super::constants::LIFE_MAX;

    // Near death (life < 20%)
    if life_pct < 0.2 && life_pct > 0.0 {
        if !anna.queue.iter().any(|(_, t)| t == "Come back. Please.") {
            anna.queue.push((0.5, "Come back. Please.".into()));
        }
    }
    // Low shields (< 25%) but not near death
    else if shield_pct < 0.25 && life_pct >= 0.2 {
        if !anna.queue.iter().any(|(_, t)| t.contains("careful")) {
            anna.queue.push((1.0, "Be careful. I can't lose you.".into()));
        }
    }
}
