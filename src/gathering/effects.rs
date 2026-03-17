// SPDX-License-Identifier: GPL-3.0-or-later
//! Visual effects: asteroid trail wisps, collision sparks.

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Spawn faint dust trail particles behind large asteroids.
pub fn spawn_asteroid_trails(
    mut commands: Commands,
    asteroid_q: Query<(&Asteroid, &Transform)>,
    assets: Res<GatheringAssets>,
    state: Res<ShipState>,
    paused: Res<Paused>,
    time: Res<Time>,
) {
    if !state.alive || paused.0 { return; }
    let dt = time.delta_secs();
    for (asteroid, tf) in asteroid_q.iter() {
        if asteroid.radius < TRAIL_MIN_RADIUS { continue; }
        if rand::random::<f32>() >= TRAIL_PARTICLES_PER_SEC * dt { continue; }
        let offset = Vec3::new(
            rand::random::<f32>() * asteroid.radius * 0.5 - asteroid.radius * 0.25,
            asteroid.radius * 0.5,
            rand::random::<f32>() * 0.4 - 0.2,
        );
        commands.spawn((
            AsteroidTrailParticle { lifetime: TRAIL_PARTICLE_LIFETIME },
            Mesh3d(assets.trail_mesh.clone()),
            MeshMaterial3d(assets.trail_material.clone()),
            Transform::from_translation(tf.translation + offset),
        ));
    }
}

pub fn move_trail_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AsteroidTrailParticle, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut p, mut tf) in query.iter_mut() {
        p.lifetime -= dt;
        let frac = (p.lifetime / TRAIL_PARTICLE_LIFETIME).max(0.0);
        tf.scale = Vec3::splat(frac);
        if p.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_sparks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Spark, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut spark, mut tf) in query.iter_mut() {
        spark.lifetime -= dt;
        tf.translation += spark.velocity * dt;
        let life_frac = (spark.lifetime / SPARK_LIFETIME).max(0.0);
        tf.scale = Vec3::splat(life_frac);
        if spark.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_sparks(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    let mesh = meshes.add(Sphere::new(SPARK_SIZE));
    let (r, g, b) = SPARK_COLOR;
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(r, g, b),
        emissive: LinearRgba::new(r, g, b, 1.0) * SPARK_EMISSIVE,
        unlit: true,
        ..default()
    });

    let mut rng = rand::thread_rng();
    for _ in 0..SPARK_COUNT {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = SPARK_SPEED * rng.gen_range(0.5..1.5);
        let velocity = Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0);
        commands.spawn((
            Spark { velocity, lifetime: SPARK_LIFETIME },
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(position),
        ));
    }
}
