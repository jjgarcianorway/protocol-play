// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::render::mesh::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

pub fn create_asteroid_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> GatheringAssets {
    let mut rng = rand::thread_rng();
    let mut asteroid_meshes = Vec::with_capacity(NUM_ASTEROID_MESHES);
    let mut asteroid_materials = Vec::with_capacity(ASTEROID_COLORS.len());

    for _ in 0..NUM_ASTEROID_MESHES {
        let mut mesh = Sphere::new(1.0).mesh().ico(ASTEROID_ICO_SUBDIVISIONS).unwrap();
        if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                pos[0] += rng.gen_range(-ASTEROID_PERTURB..ASTEROID_PERTURB);
                pos[1] += rng.gen_range(-ASTEROID_PERTURB..ASTEROID_PERTURB);
                pos[2] += rng.gen_range(-ASTEROID_PERTURB..ASTEROID_PERTURB);
            }
        }
        mesh.compute_normals();
        asteroid_meshes.push(meshes.add(mesh));
    }

    for &(r, g, b) in &ASTEROID_COLORS {
        asteroid_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            perceptual_roughness: 0.85,
            ..default()
        }));
    }

    GatheringAssets { asteroid_meshes, asteroid_materials }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    bounds: Res<ViewBounds>,
    assets: Res<GatheringAssets>,
    state: Res<ShipState>,
) {
    if !state.alive { return; }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(ASTEROID_MIN_RADIUS..ASTEROID_MAX_RADIUS);
    let speed = rng.gen_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED);
    let x = rng.gen_range(-bounds.half_width..bounds.half_width);
    let y = bounds.half_height + ASTEROID_SPAWN_BUFFER + radius;

    let mesh_idx = rng.gen_range(0..assets.asteroid_meshes.len());
    let mat_idx = rng.gen_range(0..assets.asteroid_materials.len());

    let rot_axis = Vec3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    ).normalize_or(Vec3::Y);
    let rot_speed = rng.gen_range(ASTEROID_MIN_ROT_SPEED..ASTEROID_MAX_ROT_SPEED);

    commands.spawn((
        Asteroid { radius, speed, rot_axis, rot_speed },
        Mesh3d(assets.asteroid_meshes[mesh_idx].clone()),
        MeshMaterial3d(assets.asteroid_materials[mat_idx].clone()),
        Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(radius)),
    ));
}

pub fn move_asteroids(
    mut commands: Commands,
    mut query: Query<(Entity, &Asteroid, &mut Transform)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    for (entity, asteroid, mut tf) in query.iter_mut() {
        tf.translation.y -= asteroid.speed * dt;
        tf.rotate(Quat::from_axis_angle(asteroid.rot_axis, asteroid.rot_speed * dt));
        if tf.translation.y < -bounds.half_height - ASTEROID_DESPAWN_BUFFER - asteroid.radius {
            commands.entity(entity).despawn();
        }
    }
}
