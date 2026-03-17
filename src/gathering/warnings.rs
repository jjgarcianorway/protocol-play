// SPDX-License-Identifier: GPL-3.0-or-later
//! Feature #1: Screen-edge asteroid warning indicators.
//! Shows red chevron arrows on screen edges when side-velocity asteroids approach.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Update or spawn warning indicators for asteroids approaching from off-screen.
pub fn update_warning_indicators(
    mut commands: Commands,
    asteroid_q: Query<(Entity, &Asteroid, &Transform)>,
    mut indicator_q: Query<(Entity, &WarningIndicator, &mut Node, &mut BackgroundColor)>,
    bounds: Res<ViewBounds>,
    state: Res<ShipState>,
    paused: Res<Paused>,
    ship_q: Query<&Transform, With<Ship>>,
) {
    if !state.alive || paused.0 {
        // Despawn all indicators when paused or dead
        for (entity, _, _, _) in indicator_q.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    let ship_y = ship_q.iter().next().map(|t| t.translation.y).unwrap_or(0.0);

    // Track which asteroid entities still have valid warnings
    let mut active_asteroids: Vec<Entity> = Vec::new();

    for (ast_entity, asteroid, ast_tf) in asteroid_q.iter() {
        let pos = ast_tf.translation;
        let vel = asteroid.velocity;

        // Only warn for asteroids with significant side velocity
        if vel.x.abs() < 1.5 { continue; }

        // Must be off-screen horizontally
        let screen_edge = bounds.half_width;
        if pos.x.abs() < screen_edge { continue; }

        // Check if asteroid would enter the viewport within WARNING_LEAD_TIME
        let time_to_edge = if vel.x > 0.0 && pos.x < -screen_edge {
            (-screen_edge - pos.x) / vel.x
        } else if vel.x < 0.0 && pos.x > screen_edge {
            (screen_edge - pos.x) / vel.x
        } else {
            continue;
        };

        if time_to_edge < 0.0 || time_to_edge > WARNING_LEAD_TIME { continue; }

        // Check if asteroid is within ship's vertical range (generous margin)
        let future_y = pos.y + vel.y * time_to_edge;
        let y_range = bounds.half_height * 0.8;
        if (future_y - ship_y).abs() > y_range { continue; }

        active_asteroids.push(ast_entity);

        // Calculate indicator properties
        let proximity = 1.0 - (time_to_edge / WARNING_LEAD_TIME).clamp(0.0, 1.0);
        let alpha = proximity * 0.8;
        let from_left = pos.x < 0.0;

        // Calculate vertical position on screen (0..100%)
        let y_pct = ((pos.y + bounds.half_height) / (bounds.half_height * 2.0))
            .clamp(0.05, 0.95);
        // Screen Y is inverted (top = 0)
        let screen_y_pct = (1.0 - y_pct) * 100.0;

        // Check if indicator already exists for this asteroid
        let mut found = false;
        for (_, indicator, mut node, mut bg) in indicator_q.iter_mut() {
            if indicator.asteroid_entity == ast_entity {
                found = true;
                // Update position and alpha
                node.top = Val::Percent(screen_y_pct);
                if from_left {
                    node.left = Val::Px(WARNING_MARGIN_PX);
                    node.right = Val::Auto;
                } else {
                    node.right = Val::Px(WARNING_MARGIN_PX);
                    node.left = Val::Auto;
                }
                let (r, g, b) = WARNING_ARROW_COLOR;
                bg.0 = Color::srgba(r, g, b, alpha);
                break;
            }
        }

        if !found {
            let (r, g, b) = WARNING_ARROW_COLOR;
            let mut node = Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(screen_y_pct),
                width: Val::Px(WARNING_ARROW_SIZE * 0.6),
                height: Val::Px(WARNING_ARROW_SIZE),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            };
            if from_left {
                node.left = Val::Px(WARNING_MARGIN_PX);
            } else {
                node.right = Val::Px(WARNING_MARGIN_PX);
            }
            commands.spawn((
                WarningIndicator { asteroid_entity: ast_entity },
                node,
                BackgroundColor(Color::srgba(r, g, b, alpha)),
                ZIndex(8),
            ));
        }
    }

    // Despawn indicators for asteroids that no longer qualify
    for (entity, indicator, _, _) in indicator_q.iter() {
        if !active_asteroids.contains(&indicator.asteroid_entity) {
            commands.entity(entity).despawn();
        }
    }
}
