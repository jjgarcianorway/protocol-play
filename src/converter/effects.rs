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

        let ease = t * t;
        let pos = particle.start.lerp(particle.target, ease);

        node.left = Val::Px(pos.x);
        node.top = Val::Px(pos.y);

        let alpha = if t > 0.7 { 1.0 - (t - 0.7) / 0.3 } else { 1.0 };
        let c = CRYSTAL_COLORS[particle.color_index];
        *bg = BackgroundColor(Color::srgba(c.0, c.1, c.2, alpha));

        if t >= 1.0 {
            tanks.levels[particle.color_index] += 0.5;
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn cascade feedback text ("Cascade x2!", etc.)
pub fn spawn_cascade_text(
    commands: &mut Commands,
    font: &Handle<Font>,
    cascade_count: u32,
) {
    let label = format!("Cascade x{}!", cascade_count);
    let scale = 1.0 + (cascade_count as f32 - 1.0) * 0.15;
    let font_size = CASCADE_TEXT_FONT * scale.min(1.6);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            top: Val::Percent(35.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        CascadeText { lifetime: CASCADE_TEXT_LIFETIME },
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size, ..default() },
        TextColor(Color::srgba(1.0, 0.9, 0.3, 1.0)),
    ));
}

/// Animate cascade text (rise + fade out).
pub fn animate_cascade_text(
    time: Res<Time>,
    mut query: Query<(Entity, &mut CascadeText, &mut Node)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut ct, mut node) in query.iter_mut() {
        ct.lifetime -= dt;
        if ct.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            if let Val::Percent(pct) = node.top {
                node.top = Val::Percent(pct - CASCADE_TEXT_RISE_SPEED * dt * 0.05);
            }
        }
    }
}

/// Animate star dots with subtle twinkle.
pub fn animate_stars(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &Node), With<StarDot>>,
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
