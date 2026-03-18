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
        "The ice ones are beautiful, aren't they?",
        "I mapped these fields from the bridge.",
        "You make it look easy.",
        "Don't fly too close to the metallic ones.",
    ];
    let queue = build_queue(&pool, 5);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0);
}

/// Tracks which reactive comments have already been shown to avoid repetition.
#[derive(Resource, Default, Clone)]
pub struct AnnaReactiveFlags {
    pub said_low_shield: bool,
    pub said_near_death: bool,
    pub said_first_crystal: bool,
    pub said_high_chain: bool,
    pub said_recovery: bool,
}

/// Context-sensitive Anna comments that react to game state.
pub fn gathering_anna_reactive(
    ship: Res<ShipState>,
    mut anna: ResMut<AnnaComments>,
    chain: Res<super::types::CrystalChain>,
    flags: Option<Res<AnnaReactiveFlags>>,
    mut commands: Commands,
) {
    // Only trigger if no comment currently showing
    if anna.current.is_some() { return; }

    let mut flags = match flags {
        Some(f) => f.clone(),
        None => {
            commands.insert_resource(AnnaReactiveFlags::default());
            AnnaReactiveFlags::default()
        }
    };

    let shield_pct = ship.shield / super::constants::SHIELD_MAX;
    let life_pct = ship.life / super::constants::LIFE_MAX;

    // First crystal collected
    if ship.crystals > 0 && !flags.said_first_crystal {
        flags.said_first_crystal = true;
        anna.queue.push((0.5, "Good. We need those.".into()));
    }
    // High chain multiplier
    else if chain.multiplier >= 2.0 && !flags.said_high_chain {
        flags.said_high_chain = true;
        anna.queue.push((0.3, "That chain bonus... impressive.".into()));
    }
    // Near death (life < 20%)
    else if life_pct < 0.2 && life_pct > 0.0 && !flags.said_near_death {
        flags.said_near_death = true;
        anna.queue.push((0.5, "Come back. Please.".into()));
    }
    // Low shields (< 25%) but not near death
    else if shield_pct < 0.25 && life_pct >= 0.2 && !flags.said_low_shield {
        flags.said_low_shield = true;
        anna.queue.push((1.0, "Be careful. I can't lose you.".into()));
    }
    // Recovered from low health
    else if life_pct >= 0.2 && flags.said_near_death && !flags.said_recovery {
        // Shield regen brought them back — but life doesn't regen, so this
        // won't trigger (keeping for future design). Reset low-shield flag
        // so it can re-trigger after recovery.
        flags.said_low_shield = false;
    }

    commands.insert_resource(flags);
}
