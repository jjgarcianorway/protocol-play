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
    mut hit_flash: ResMut<HitFlash>,
    paused: Res<Paused>,
    mut commands: Commands,
) -> Result {
    if !state.alive || paused.0 { return Ok(()); }
    let ship_tf = ship_q.single()?;
    let ship_pos = ship_tf.translation.truncate();

    for (entity, ast_tf, asteroid) in asteroid_q.iter() {
        let ast_pos = ast_tf.translation.truncate();
        let dist = ship_pos.distance(ast_pos);
        let collision_dist = SHIP_COLLISION_RADIUS + asteroid.radius;

        if dist < collision_dist {
            let overlap = 1.0 - (dist / collision_dist).clamp(0.0, 1.0);
            let directness = if overlap > 0.5 { 1.0 } else { DAMAGE_GLANCING_MULT + (1.0 - DAMAGE_GLANCING_MULT) * (overlap / 0.5) };
            let speed = asteroid.velocity.length();
            let base_damage = (asteroid.radius * DAMAGE_SIZE_FACTOR + speed * DAMAGE_SPEED_FACTOR) * directness;
            let type_mult = match asteroid.asteroid_type {
                AsteroidType::Rock => 1.0,
                AsteroidType::Ice => ASTEROID_ICE_DAMAGE_MULT,
                AsteroidType::Metallic => ASTEROID_METALLIC_DAMAGE_MULT,
            };
            let damage = base_damage * type_mult;

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
            hit_flash.timer = HIT_FLASH_DURATION;
            commands.entity(entity).insert(HitCooldown(0.5));

            spawn_damage_direction(&mut commands, ship_pos, ast_pos);

            if state.life <= 0.0 {
                state.life = 0.0;
                state.alive = false;
            }
        }
    }
    Ok(())
}

/// Near-miss detection: asteroids close but not colliding recover shield.
pub fn check_near_misses(
    ship_q: Query<&Transform, With<Ship>>,
    asteroid_q: Query<(Entity, &Transform, &Asteroid), (Without<HitCooldown>, Without<NearMissCooldown>)>,
    mut state: ResMut<ShipState>,
    mut near_miss_flash: ResMut<NearMissFlash>,
    paused: Res<Paused>,
    mut commands: Commands,
    font: Res<GatheringFont>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) -> Result {
    if !state.alive || paused.0 { return Ok(()); }
    let ship_tf = ship_q.single()?;
    let ship_pos = ship_tf.translation.truncate();

    for (entity, ast_tf, asteroid) in asteroid_q.iter() {
        let ast_pos = ast_tf.translation.truncate();
        let dist = ship_pos.distance(ast_pos);
        let collision_dist = SHIP_COLLISION_RADIUS + asteroid.radius;
        let near_miss_inner = collision_dist + NEAR_MISS_MIN_GAP;
        let near_miss_outer = collision_dist + NEAR_MISS_RANGE;

        if dist > near_miss_inner && dist < near_miss_outer {
            let recovery = NEAR_MISS_SHIELD_RECOVERY;
            state.shield = (state.shield + recovery).min(SHIELD_MAX);
            state.near_misses += 1;
            near_miss_flash.timer = NEAR_MISS_FLASH_DURATION;
            commands.entity(entity).insert(NearMissCooldown(NEAR_MISS_COOLDOWN));

            // Spawn floating text
            if let Ok((camera, cam_gt)) = cameras.single() {
                if let Ok(vp) = camera.world_to_viewport(cam_gt, ship_tf.translation) {
                    let label = format!("Shield +{}", recovery as u32);
                    let color = Color::srgb(
                        NEAR_MISS_TEXT_COLOR.0, NEAR_MISS_TEXT_COLOR.1, NEAR_MISS_TEXT_COLOR.2,
                    );
                    commands.spawn((
                        FloatingText {
                            lifetime: FLOAT_TEXT_LIFETIME, max_lifetime: FLOAT_TEXT_LIFETIME,
                            text_color: NEAR_MISS_TEXT_COLOR,
                        },
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(vp.x - 50.0),
                            top: Val::Px(vp.y - 30.0),
                            ..default()
                        },
                        ZIndex(15),
                    )).with_child((
                        Text::new(label),
                        TextFont { font: font.0.clone(), font_size: FLOAT_TEXT_FONT - 2.0, ..default() },
                        TextColor(color),
                    ));
                }
            }
        }
    }
    Ok(())
}

pub fn tick_near_miss_cooldowns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut NearMissCooldown)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut cd) in query.iter_mut() {
        cd.0 -= dt;
        if cd.0 <= 0.0 { commands.entity(entity).remove::<NearMissCooldown>(); }
    }
}

fn spawn_damage_direction(commands: &mut Commands, ship_pos: Vec2, ast_pos: Vec2) {
    let diff = ast_pos - ship_pos;
    // Determine which edge to show the indicator on
    let (left, top, width, height) = if diff.x.abs() > diff.y.abs() {
        if diff.x > 0.0 {
            // Hit from right
            (Val::Auto, Val::Percent(20.0), Val::Px(DAMAGE_DIR_WIDTH), Val::Percent(60.0))
        } else {
            // Hit from left
            (Val::Px(0.0), Val::Percent(20.0), Val::Px(DAMAGE_DIR_WIDTH), Val::Percent(60.0))
        }
    } else if diff.y > 0.0 {
        // Hit from top
        (Val::Percent(20.0), Val::Px(0.0), Val::Percent(60.0), Val::Px(DAMAGE_DIR_WIDTH))
    } else {
        // Hit from bottom
        (Val::Percent(20.0), Val::Auto, Val::Percent(60.0), Val::Px(DAMAGE_DIR_WIDTH))
    };

    let mut node = Node {
        position_type: PositionType::Absolute,
        left,
        top,
        width,
        height,
        border_radius: BorderRadius::all(Val::Px(4.0)),
        ..default()
    };
    // Position right edge
    if diff.x > 0.0 && diff.x.abs() > diff.y.abs() {
        node.right = Val::Px(0.0);
    }
    // Position bottom edge
    if diff.y < 0.0 && diff.y.abs() >= diff.x.abs() {
        node.bottom = Val::Px(0.0);
    }

    commands.spawn((
        DamageDirectionIndicator { timer: DAMAGE_DIR_FADE_TIME },
        node,
        BackgroundColor(Color::srgba(1.0, 0.1, 0.05, DAMAGE_DIR_ALPHA)),
        ZIndex(9),
    ));
}

pub fn update_damage_direction(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageDirectionIndicator, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut indicator, mut bg) in query.iter_mut() {
        indicator.timer -= dt;
        if indicator.timer <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let alpha = (indicator.timer / DAMAGE_DIR_FADE_TIME) * DAMAGE_DIR_ALPHA;
        bg.0 = Color::srgba(1.0, 0.1, 0.05, alpha);
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
