// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

#[derive(Component)]
pub struct HitCooldown(pub f32);

pub fn check_collisions(
    ship_q: Query<&Transform, With<Ship>>,
    asteroid_q: Query<(Entity, &Transform, &Asteroid), Without<HitCooldown>>,
    mut state: ResMut<ShipState>,
    mut shake: ResMut<ScreenShake>,
    mut commands: Commands,
) {
    if !state.alive { return; }
    let ship_tf = ship_q.single();
    let ship_pos = ship_tf.translation.truncate();

    for (entity, ast_tf, asteroid) in asteroid_q.iter() {
        let ast_pos = ast_tf.translation.truncate();
        let dist = ship_pos.distance(ast_pos);
        let collision_dist = SHIP_COLLISION_RADIUS + asteroid.radius;

        if dist < collision_dist {
            let overlap = 1.0 - (dist / collision_dist).clamp(0.0, 1.0);
            let directness = if overlap > 0.5 { 1.0 } else { DAMAGE_GLANCING_MULT + (1.0 - DAMAGE_GLANCING_MULT) * (overlap / 0.5) };
            let damage = (asteroid.radius * DAMAGE_SIZE_FACTOR + asteroid.speed * DAMAGE_SPEED_FACTOR) * directness;

            if state.shield > 0.0 {
                let shield_absorb = damage.min(state.shield);
                state.shield -= shield_absorb;
                let remaining = damage - shield_absorb;
                if remaining > 0.0 { state.life -= remaining; }
            } else {
                state.life -= damage;
            }

            state.hits_taken += 1;
            state.control_loss_timer = CONTROL_LOSS_DURATION;
            shake.intensity = (shake.intensity + damage * 0.15).min(3.0);
            commands.entity(entity).insert(HitCooldown(0.5));

            if state.life <= 0.0 {
                state.life = 0.0;
                state.alive = false;
            }
        }
    }
}

pub fn tick_hit_cooldowns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HitCooldown)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut cd) in query.iter_mut() {
        cd.0 -= dt;
        if cd.0 <= 0.0 { commands.entity(entity).remove::<HitCooldown>(); }
    }
}
