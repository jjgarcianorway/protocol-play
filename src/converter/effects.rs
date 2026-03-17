// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Spawn burst particles from a cell position toward the resource tank.
pub fn spawn_burst_particles(
    commands: &mut Commands,
    cell_pos: Vec2,
    tank_pos: Vec2,
    color: CrystalColor,
    count: u32,
) {
    let (r, g, b) = color.rgb();
    let base_color = Color::srgb(r, g, b);
    for i in 0..count.min(6) {
        let offset = Vec2::new(
            (i as f32 - 2.5) * 4.0,
            (i as f32 % 3.0 - 1.0) * 4.0,
        );
        let start = cell_pos + offset;
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(start.x),
                top: Val::Px(start.y),
                width: Val::Px(PARTICLE_SIZE),
                height: Val::Px(PARTICLE_SIZE),
                border_radius: BorderRadius::all(Val::Px(PARTICLE_SIZE / 2.0)),
                ..default()
            },
            BackgroundColor(base_color),
            BurstParticle {
                target: tank_pos,
                start,
                lifetime: 0.0,
                max_lifetime: PARTICLE_LIFETIME + i as f32 * 0.05,
                color_index: color.index(),
            },
        ));
    }
}

/// Update burst particles — move toward target tank, despawn when done.
pub fn update_burst_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Node, &mut BurstParticle, &mut BackgroundColor)>,
    mut tanks: ResMut<ResourceTanks>,
) {
    let dt = time.delta_secs();
    for (entity, mut node, mut particle, mut bg) in query.iter_mut() {
        particle.lifetime += dt;
        let t = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);

        // Ease-in curve for acceleration toward target
        let ease = t * t;
        let pos = particle.start.lerp(particle.target, ease);

        node.left = Val::Px(pos.x);
        node.top = Val::Px(pos.y);

        // Fade out near end
        let alpha = if t > 0.7 { 1.0 - (t - 0.7) / 0.3 } else { 1.0 };
        let c = CRYSTAL_COLORS[particle.color_index];
        *bg = BackgroundColor(Color::srgba(c.0, c.1, c.2, alpha));

        if t >= 1.0 {
            // Add to tank
            tanks.levels[particle.color_index] += 0.5;
            commands.entity(entity).despawn();
        }
    }
}

/// Animate cell backgrounds on hover (pulse border).
pub fn animate_highlight_pulse(
    time: Res<Time>,
    hovered: Res<HoveredGroup>,
    mut query: Query<(&GridCell, &mut BorderColor)>,
) {
    let t = time.elapsed_secs();
    let pulse = ((t * HIGHLIGHT_PULSE_SPEED).sin() * 0.3 + 0.7).clamp(0.4, 1.0);

    for (cell, mut border) in query.iter_mut() {
        let is_hovered = hovered.cells.iter().any(|&(r, c)| r == cell.row && c == cell.col);
        if is_hovered {
            let (hr, hg, hb, _) = HIGHLIGHT_BORDER_COLOR;
            *border = BorderColor::all(Color::srgba(hr, hg, hb, pulse));
        } else {
            *border = BorderColor::all(Color::NONE);
        }
    }
}
