// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Spawn simple pop particles from a cell position (spread outward, fade fast).
pub fn spawn_pop_particles(
    commands: &mut Commands,
    cell_pos: Vec2,
    color: CrystalColor,
) {
    let (r, g, b) = color.rgb();
    let mut rng = rand::thread_rng();
    for _ in 0..POP_PARTICLE_COUNT {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(40.0..POP_PARTICLE_SPREAD * 3.0);
        let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let size = rng.gen_range(POP_PARTICLE_SIZE * 0.5..POP_PARTICLE_SIZE * 1.2);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(cell_pos.x - size / 2.0),
                top: Val::Px(cell_pos.y - size / 2.0),
                width: Val::Px(size),
                height: Val::Px(size),
                border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(r, g, b, 1.0)),
            PopParticle {
                velocity: vel,
                lifetime: POP_PARTICLE_LIFETIME,
            },
        ));
    }
}

/// Update pop particles — move outward and fade.
pub fn update_pop_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Node, &mut PopParticle, &mut BackgroundColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut node, mut particle, mut bg) in query.iter_mut() {
        particle.lifetime -= dt;
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let t = 1.0 - (particle.lifetime / POP_PARTICLE_LIFETIME);
        // Move outward
        if let Val::Px(x) = node.left {
            node.left = Val::Px(x + particle.velocity.x * dt);
        }
        if let Val::Px(y) = node.top {
            node.top = Val::Px(y + particle.velocity.y * dt);
        }
        // Fade out
        let alpha = (1.0 - t).max(0.0);
        if let Color::Srgba(ref mut c) = bg.0 {
            c.alpha = alpha;
        }
    }
}

/// Spawn floating "+N" text near a tank.
pub fn spawn_tank_float(
    commands: &mut Commands,
    font: &Handle<Font>,
    index: usize,
    amount: f32,
) {
    let (r, g, b) = CRYSTAL_COLORS[index];
    let label = format!("+{:.0}", amount);
    // Position near the tank area (right side of screen)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(40.0 + (4 - index) as f32 * (TANK_WIDTH + TANK_GAP)),
            top: Val::Percent(35.0),
            ..default()
        },
        TankFloatText { lifetime: TANK_FLOAT_LIFETIME },
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: TANK_FLOAT_FONT, ..default() },
        TextColor(Color::srgba(
            (r + 0.2).min(1.0), (g + 0.2).min(1.0), (b + 0.2).min(1.0), 1.0,
        )),
    ));
}

/// Animate floating tank text (rise + fade out).
pub fn animate_tank_floats(
    time: Res<Time>,
    mut query: Query<(Entity, &mut TankFloatText, &mut Node)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut ft, mut node) in query.iter_mut() {
        ft.lifetime -= dt;
        if ft.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else if let Val::Percent(pct) = node.top {
            node.top = Val::Percent(pct - TANK_FLOAT_RISE * dt * 0.05);
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
