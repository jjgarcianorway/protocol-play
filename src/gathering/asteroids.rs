// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::mesh::*;
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

    let crystal_mesh = meshes.add(Sphere::new(1.0).mesh().ico(CRYSTAL_ICO_SUBDIVISIONS).unwrap());
    let crystal_material = materials.add(StandardMaterial {
        base_color: Color::srgb(CRYSTAL_COLOR.0, CRYSTAL_COLOR.1, CRYSTAL_COLOR.2),
        emissive: LinearRgba::new(CRYSTAL_COLOR.0, CRYSTAL_COLOR.1, CRYSTAL_COLOR.2, 1.0) * CRYSTAL_EMISSIVE_MULT,
        ..default()
    });

    GatheringAssets { asteroid_meshes, asteroid_materials, crystal_mesh, crystal_material }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    bounds: Res<ViewBounds>,
    assets: Res<GatheringAssets>,
    state: Res<ShipState>,
    difficulty: Res<Difficulty>,
) {
    if !state.alive { return; }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(ASTEROID_MIN_RADIUS..ASTEROID_MAX_RADIUS);
    let base_speed = rng.gen_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED) * difficulty.speed_mult;

    let mesh_idx = rng.gen_range(0..assets.asteroid_meshes.len());
    let mat_idx = rng.gen_range(0..assets.asteroid_materials.len());
    let rot_axis = Vec3::new(
        rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0),
    ).normalize_or(Vec3::Y);
    let rot_speed = rng.gen_range(ASTEROID_MIN_ROT_SPEED..ASTEROID_MAX_ROT_SPEED);

    let (x, y, velocity) = if rng.gen_range(0.0..1.0_f32) < difficulty.side_chance {
        // Side entry
        let from_left = rng.gen_range(0..2) == 0;
        let x = if from_left { -bounds.half_width - ASTEROID_SPAWN_BUFFER } else { bounds.half_width + ASTEROID_SPAWN_BUFFER };
        let y = rng.gen_range(-bounds.half_height * 0.5..bounds.half_height);
        let vx = if from_left { rng.gen_range(2.0..6.0) } else { rng.gen_range(-6.0..-2.0) };
        (x, y, Vec2::new(vx, -base_speed * 0.6))
    } else {
        // Top entry
        let x = rng.gen_range(-bounds.half_width..bounds.half_width);
        let y = bounds.half_height + ASTEROID_SPAWN_BUFFER + radius;
        let drift = rng.gen_range(-1.5..1.5);
        (x, y, Vec2::new(drift, -base_speed))
    };

    commands.spawn((
        Asteroid { radius, velocity, rot_axis, rot_speed },
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
        tf.translation.x += asteroid.velocity.x * dt;
        tf.translation.y += asteroid.velocity.y * dt;
        tf.rotate(Quat::from_axis_angle(asteroid.rot_axis, asteroid.rot_speed * dt));
        let margin = ASTEROID_DESPAWN_BUFFER + asteroid.radius;
        if tf.translation.y < -bounds.half_height - margin
            || tf.translation.x.abs() > bounds.half_width + margin * 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn asteroid_asteroid_collisions(
    mut query: Query<(&mut Asteroid, &Transform)>,
) {
    let mut combos: Vec<(Entity, Vec2, f32, Vec2)> = Vec::new();
    // Collect positions and velocities
    for (asteroid, tf) in query.iter() {
        let pos = tf.translation.truncate();
        combos.push((Entity::PLACEHOLDER, pos, asteroid.radius, asteroid.velocity));
    }
    // Check pairs
    let len = combos.len();
    let mut impulses: Vec<(usize, Vec2)> = Vec::new();
    for i in 0..len {
        for j in (i + 1)..len {
            let diff = combos[i].1 - combos[j].1;
            let dist = diff.length();
            let min_dist = combos[i].2 + combos[j].2;
            if dist < min_dist && dist > 0.001 {
                let normal = diff / dist;
                let rel_vel = combos[i].3 - combos[j].3;
                let vel_along = rel_vel.dot(normal);
                if vel_along < 0.0 {
                    let mass_i = combos[i].2 * combos[i].2;
                    let mass_j = combos[j].2 * combos[j].2;
                    let impulse = vel_along * ASTEROID_BOUNCE_FACTOR / (mass_i + mass_j);
                    impulses.push((i, -normal * impulse * mass_j));
                    impulses.push((j, normal * impulse * mass_i));
                }
            }
        }
    }
    // Apply impulses
    if impulses.is_empty() { return; }
    let mut idx = 0;
    for (mut asteroid, _) in query.iter_mut() {
        for (imp_idx, imp) in &impulses {
            if *imp_idx == idx { asteroid.velocity += *imp; }
        }
        idx += 1;
    }
}
