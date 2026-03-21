// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's brief in-game comments for The Converter.

use bevy::prelude::*;
use crate::anna_comments::*;
use super::types::*;

/// Set up Anna comments for The Converter.
pub fn setup_converter_anna(mut commands: Commands, font: Res<ConverterFont>) {
    let pool: Vec<&str> = vec![
        "I love watching the chains cascade.",
        "Keep going.",
        "Resources flowing. The ship thanks you.",
        "You have a good eye for patterns.",
        "The bigger the chain, the more efficient the conversion.",
        "Each crystal holds energy from a star that died billions of years ago.",
        "Four or more to cascade. Think ahead.",
        "The tanks are filling. The ship can feel it.",
        "You're turning starlight into survival.",
        "I designed this converter. Watching you use it is... satisfying.",
        "Look for the cascade opportunities. They multiply.",
        "These crystals are the only fuel we have. Make them count.",
    ];
    let queue = build_queue(&pool, 5);
    commands.insert_resource(AnnaComments { queue, current: None });
    spawn_anna_ui(&mut commands, &font.0);
}

/// React to big chains in the converter.
pub fn converter_anna_reactive(
    stats: Res<ConversionStats>,
    mut anna: ResMut<AnnaComments>,
) {
    if anna.current.is_some() { return; }
    // Big chain reaction
    if stats.is_changed() && stats.best_chain >= 8 {
        if !anna.queue.iter().any(|(_, t)| t == "Impressive.") {
            anna.queue.push((0.5, "Impressive.".into()));
        }
    }
}
