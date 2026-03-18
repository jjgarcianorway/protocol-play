// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's brief in-game comments for the Bot Game (player mode).

use bevy::prelude::*;
use crate::anna_comments::*;
use crate::types::GameFont;

/// Set up Anna comments for the Bot Game based on current progress.
pub fn setup_bot_anna(mut commands: Commands, font: Res<GameFont>) {
    let gs = crate::save_state::load_game_state();
    let level = gs.bot_level;
    let total_levels = 149u32; // campaign size

    let mut pool: Vec<&str> = Vec::new();

    // Early game (levels 0-49)
    if level < 50 {
        pool.extend_from_slice(&[
            "You're getting good at this.",
            "Another system back online.",
            "Each one matters.",
        ]);
    }
    // Mid game (levels 50-99)
    if level >= 30 && level < 100 {
        pool.extend_from_slice(&[
            "Every puzzle you solve... that's another system breathing.",
            "Keep going. The ship needs you.",
        ]);
    }
    // Progress-aware
    if level > 0 {
        let remaining = total_levels.saturating_sub(level);
        if remaining > 0 && remaining < 120 {
            pool.push("We're getting closer.");
        }
    }
    // Late game (levels 100+)
    if level >= 100 {
        pool.extend_from_slice(&[
            "We're close now.",
            "When this is over... I hope you'll still talk to me.",
            "Almost there. I can feel it.",
        ]);
    }

    // Fallback if somehow empty
    if pool.is_empty() {
        pool.push("I'm here if you need me.");
    }

    let queue = build_queue(&pool, 3);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0);
}
