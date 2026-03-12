// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

pub fn spawn_crystals(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<CrystalSpawnTimer>,
    bounds: Res<ViewBounds>,
    assets: Res<GatheringAssets>,
    state: Res<ShipState>,
) {
    if !state.alive { return; }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(CRYSTAL_MIN_RADIUS..CRYSTAL_MAX_RADIUS);
    let value = rng.gen_range(CRYSTAL_MIN_VALUE..CRYSTAL_MAX_VALUE);
    let x = rng.gen_range(-bounds.half_width * 0.7..bounds.half_width * 0.7);
    let y = bounds.half_height + ASTEROID_SPAWN_BUFFER + radius;

    commands.spawn((
        CrystalCloud { radius, value, remaining: 1.0 },
        Mesh3d(assets.crystal_mesh.clone()),
        MeshMaterial3d(assets.crystal_material.clone()),
        Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(radius)),
    ));
}

pub fn move_crystals(
    mut commands: Commands,
    mut query: Query<(Entity, &CrystalCloud, &mut Transform)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    for (entity, cloud, mut tf) in query.iter_mut() {
        tf.translation.y -= SCROLL_SPEED * 0.5 * dt;
        tf.scale = Vec3::splat(cloud.radius * cloud.remaining);
        let margin = ASTEROID_DESPAWN_BUFFER + cloud.radius;
        if tf.translation.y < -bounds.half_height - margin || cloud.remaining <= 0.01 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn absorb_crystals(
    ship_q: Query<&Transform, With<Ship>>,
    mut crystal_q: Query<(&mut CrystalCloud, &Transform)>,
    mut state: ResMut<ShipState>,
    time: Res<Time>,
) {
    if !state.alive { return; }
    let ship_tf = ship_q.single();
    let ship_pos = ship_tf.translation.truncate();
    let dt = time.delta_secs();

    for (mut cloud, tf) in crystal_q.iter_mut() {
        let cloud_pos = tf.translation.truncate();
        let dist = ship_pos.distance(cloud_pos);
        if dist < CRYSTAL_ABSORB_RANGE + cloud.radius * cloud.remaining {
            let absorb = CRYSTAL_ABSORB_RATE * dt;
            let absorbed_fraction = absorb.min(cloud.remaining);
            cloud.remaining -= absorbed_fraction;
            let crystals_gained = (cloud.value as f32 * absorbed_fraction) as u64;
            state.crystals += crystals_gained;
        }
    }
}
