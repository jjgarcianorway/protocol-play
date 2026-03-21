// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's brief in-game comments for The Delivery.

use bevy::prelude::*;
use crate::anna_comments::*;
use super::types::*;

/// Set up Anna comments for The Delivery.
pub fn setup_delivery_anna(mut commands: Commands, font: Res<DeliveryFont>) {
    let pool: Vec<&str> = vec![
        "Right on target.",
        "Stay focused.",
        "The crew is counting on these supplies.",
        "Smooth delivery.",
        "Each pod is a promise to someone sleeping.",
        "The routing gets faster with practice.",
        "Careful with the purple ones. Cryo repairs are delicate.",
        "You're keeping 14,892 people alive. One pod at a time.",
        "Speed matters, but accuracy matters more.",
        "The ship feels different when supplies flow.",
        "I timed you. Faster than yesterday.",
        "Some of these pods contain medicine. Handle with care.",
    ];
    let queue = build_queue(&pool, 5);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0);
}

/// React to streaks in The Delivery.
pub fn delivery_anna_reactive(
    state: Res<DeliveryState>,
    mut anna: ResMut<AnnaComments>,
) {
    if anna.current.is_some() { return; }
    // Streak of 10+
    if state.is_changed() && state.streak >= 10 {
        if !anna.queue.iter().any(|(_, t)| t == "You're in the zone.") {
            anna.queue.push((1.0, "You're in the zone.".into()));
        }
    }
}
