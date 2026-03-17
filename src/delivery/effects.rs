// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Spawn star background dots.
pub fn spawn_star_background(commands: &mut Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..STAR_COUNT_D {
        let x = rng.gen_range(1.0..99.0);
        let y = rng.gen_range(1.0..99.0);
        let size = rng.gen_range(STAR_MIN_SIZE_D..STAR_MAX_SIZE_D);
        let alpha = rng.gen_range(STAR_MIN_ALPHA_D..STAR_MAX_ALPHA_D);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(x),
                top: Val::Percent(y),
                width: Val::Px(size),
                height: Val::Px(size),
                border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.7, 0.75, 1.0, alpha)),
            StarDotD,
        ));
    }
}

/// Animate star dots with subtle twinkle.
pub fn animate_stars(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &Node), With<StarDotD>>,
) {
    let t = time.elapsed_secs();
    let mut i = 0u32;
    for (mut bg, node) in query.iter_mut() {
        let phase = i as f32 * 2.7;
        let twinkle = ((t * 1.5 + phase).sin() * 0.5 + 0.5).clamp(0.2, 1.0);
        let base_alpha = match node.left {
            Val::Percent(p) => (p * 0.01).sin().abs() * 0.35 + 0.15,
            _ => 0.3,
        };
        *bg = BackgroundColor(Color::srgba(0.7, 0.75, 1.0, base_alpha * twinkle));
        i += 1;
    }
}

/// Animate streak popup text (rise + fade).
pub fn animate_streak_popups(
    time: Res<Time>,
    mut query: Query<(Entity, &mut StreakPopup, &mut Node)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut popup, mut node) in query.iter_mut() {
        popup.lifetime -= dt;
        if popup.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            if let Val::Percent(pct) = node.top {
                node.top = Val::Percent(
                    pct - STREAK_POPUP_RISE * dt * 0.03,
                );
            }
        }
    }
}

/// Spawn streak milestone popup.
pub fn spawn_streak_popup(
    commands: &mut Commands,
    font: &Handle<Font>,
    streak: u32,
) {
    let label = format!("STREAK x{}!", streak);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            top: Val::Percent(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        StreakPopup { lifetime: STREAK_POPUP_LIFETIME },
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: STREAK_POPUP_FONT, ..default() },
        TextColor(Color::srgba(1.0, 0.85, 0.2, 1.0)),
    ));
}
