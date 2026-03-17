// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::mesh::*;
use bevy::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Build a flat-shaded icosphere with vertex perturbation for a rocky asteroid look.
fn make_asteroid_mesh(rng: &mut impl Rng) -> Mesh {
    let subdiv = ASTEROID_ICO_SUBDIVISIONS[rng.gen_range(0..ASTEROID_ICO_SUBDIVISIONS.len())];
    let smooth = Sphere::new(1.0).mesh().ico(subdiv).unwrap();
    let Some(VertexAttributeValues::Float32x3(src_pos)) = smooth.attribute(Mesh::ATTRIBUTE_POSITION) else {
        return smooth;
    };

    let perturb = rng.gen_range(ASTEROID_PERTURB_MIN..ASTEROID_PERTURB_MAX);
    // Random elongation along one axis for shape variety
    let elongation = [
        rng.gen_range(ASTEROID_ELONGATION_MIN..ASTEROID_ELONGATION_MAX),
        rng.gen_range(ASTEROID_ELONGATION_MIN..ASTEROID_ELONGATION_MAX),
        rng.gen_range(ASTEROID_ELONGATION_MIN..ASTEROID_ELONGATION_MAX),
    ];

    // Perturb vertex positions on the smooth mesh first
    let perturbed: Vec<[f32; 3]> = src_pos.iter().map(|p| {
        [
            p[0] * elongation[0] + rng.gen_range(-perturb..perturb),
            p[1] * elongation[1] + rng.gen_range(-perturb..perturb),
            p[2] * elongation[2] + rng.gen_range(-perturb..perturb),
        ]
    }).collect();

    // Get indices
    let indices: Vec<u32> = match smooth.indices() {
        Some(Indices::U32(idx)) => idx.clone(),
        Some(Indices::U16(idx)) => idx.iter().map(|&i| i as u32).collect(),
        None => return smooth,
    };

    // Build flat-shaded mesh: duplicate vertices per triangle, compute face normals
    let tri_count = indices.len() / 3;
    let mut positions = Vec::with_capacity(tri_count * 3);
    let mut normals = Vec::with_capacity(tri_count * 3);
    let mut new_indices = Vec::with_capacity(tri_count * 3);

    for tri in 0..tri_count {
        let i0 = indices[tri * 3] as usize;
        let i1 = indices[tri * 3 + 1] as usize;
        let i2 = indices[tri * 3 + 2] as usize;

        let p0 = Vec3::from(perturbed[i0]);
        let p1 = Vec3::from(perturbed[i1]);
        let p2 = Vec3::from(perturbed[i2]);

        let face_normal = (p1 - p0).cross(p2 - p0).normalize_or(Vec3::Y);
        let n = face_normal.to_array();

        let base = (tri * 3) as u32;
        positions.push(p0.to_array());
        positions.push(p1.to_array());
        positions.push(p2.to_array());
        normals.push(n);
        normals.push(n);
        normals.push(n);
        new_indices.push(base);
        new_indices.push(base + 1);
        new_indices.push(base + 2);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(new_indices));
    mesh
}

pub fn create_asteroid_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> GatheringAssets {
    let mut rng = rand::thread_rng();
    let mut asteroid_meshes = Vec::with_capacity(NUM_ASTEROID_MESHES);
    let mut asteroid_materials = Vec::with_capacity(ASTEROID_COLORS.len());

    for _ in 0..NUM_ASTEROID_MESHES {
        asteroid_meshes.push(meshes.add(make_asteroid_mesh(&mut rng)));
    }

    for &(r, g, b) in &ASTEROID_COLORS {
        asteroid_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            perceptual_roughness: ASTEROID_ROCK_ROUGHNESS,
            metallic: ASTEROID_ROCK_METALLIC,
            ..default()
        }));
    }

    // Ice asteroid materials
    let mut ice_materials = Vec::with_capacity(ASTEROID_ICE_COLORS.len());
    for &(r, g, b) in &ASTEROID_ICE_COLORS {
        ice_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            perceptual_roughness: ASTEROID_ICE_ROUGHNESS,
            metallic: ASTEROID_ICE_METALLIC,
            emissive: LinearRgba::new(r * 0.3, g * 0.4, b * 0.5, 1.0) * ASTEROID_ICE_EMISSIVE,
            ..default()
        }));
    }

    // Metallic asteroid materials
    let mut metallic_materials = Vec::with_capacity(ASTEROID_METALLIC_COLORS.len());
    for &(r, g, b) in &ASTEROID_METALLIC_COLORS {
        metallic_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            perceptual_roughness: ASTEROID_METALLIC_ROUGHNESS,
            metallic: ASTEROID_METALLIC_METALLIC,
            ..default()
        }));
    }

    // Nebula crystal cloud layers — unlit glowing spheres
    let mut crystal_meshes = Vec::with_capacity(CRYSTAL_NEBULA_LAYERS);
    let mut crystal_materials = Vec::with_capacity(CRYSTAL_NEBULA_LAYERS);
    for i in 0..CRYSTAL_NEBULA_LAYERS {
        let t = i as f32 / (CRYSTAL_NEBULA_LAYERS - 1) as f32;
        let scale = 0.5 + t * 0.7;
        let mut mesh = Sphere::new(scale).mesh().ico(CRYSTAL_ICO_SUBDIVISIONS).unwrap();
        if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                let perturb = CRYSTAL_NEBULA_PERTURB * (0.3 + t * 0.7);
                pos[0] += rng.gen_range(-perturb..perturb);
                pos[1] += rng.gen_range(-perturb..perturb);
                pos[2] += rng.gen_range(-perturb..perturb);
            }
        }
        crystal_meshes.push(meshes.add(mesh));

        let ci = i % CRYSTAL_COLORS.len();
        let (r, g, b) = CRYSTAL_COLORS[ci];
        let alpha = CRYSTAL_CORE_ALPHA + (CRYSTAL_OUTER_ALPHA - CRYSTAL_CORE_ALPHA) * t;
        let emissive_str = CRYSTAL_CORE_EMISSIVE + (CRYSTAL_OUTER_EMISSIVE - CRYSTAL_CORE_EMISSIVE) * t;
        crystal_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgba(r, g, b, alpha),
            emissive: LinearRgba::new(r, g, b, 1.0) * emissive_str,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        }));
    }

    // Per-color crystal materials (5 color groups, each with CRYSTAL_NEBULA_LAYERS materials)
    let mut crystal_materials_by_color = Vec::with_capacity(CRYSTAL_RESOURCE_COLORS.len());
    for &(cr, cg, cb) in &CRYSTAL_RESOURCE_COLORS {
        let mut color_mats = Vec::with_capacity(CRYSTAL_NEBULA_LAYERS);
        for i in 0..CRYSTAL_NEBULA_LAYERS {
            let t = i as f32 / (CRYSTAL_NEBULA_LAYERS - 1) as f32;
            let alpha = CRYSTAL_CORE_ALPHA + (CRYSTAL_OUTER_ALPHA - CRYSTAL_CORE_ALPHA) * t;
            let emissive_str = CRYSTAL_CORE_EMISSIVE + (CRYSTAL_OUTER_EMISSIVE - CRYSTAL_CORE_EMISSIVE) * t;
            color_mats.push(materials.add(StandardMaterial {
                base_color: Color::srgba(cr, cg, cb, alpha),
                emissive: LinearRgba::new(cr, cg, cb, 1.0) * emissive_str,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                cull_mode: None,
                ..default()
            }));
        }
        crystal_materials_by_color.push(color_mats);
    }

    // Absorption particle mesh and materials — small glowing spheres
    let particle_mesh = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let mut particle_materials = Vec::with_capacity(CRYSTAL_COLORS.len());
    for &(r, g, b) in &CRYSTAL_COLORS {
        particle_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgba(r, g, b, 0.9),
            emissive: LinearRgba::new(r, g, b, 1.0) * PARTICLE_EMISSIVE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }));
    }

    // Per-color particle materials
    let mut particle_materials_by_color = Vec::with_capacity(CRYSTAL_RESOURCE_COLORS.len());
    for &(cr, cg, cb) in &CRYSTAL_RESOURCE_COLORS {
        particle_materials_by_color.push(materials.add(StandardMaterial {
            base_color: Color::srgba(cr, cg, cb, 0.9),
            emissive: LinearRgba::new(cr, cg, cb, 1.0) * PARTICLE_EMISSIVE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }));
    }

    // Trail particle mesh and material for large asteroids
    let trail_mesh = meshes.add(Sphere::new(TRAIL_PARTICLE_SIZE));
    let (tr, tg, tb) = TRAIL_PARTICLE_COLOR;
    let trail_material = materials.add(StandardMaterial {
        base_color: Color::srgba(tr, tg, tb, TRAIL_PARTICLE_ALPHA),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    GatheringAssets {
        asteroid_meshes, asteroid_materials, ice_materials, metallic_materials,
        crystal_meshes, crystal_materials, crystal_materials_by_color,
        particle_mesh, particle_materials, particle_materials_by_color,
        trail_mesh, trail_material,
    }
}

pub fn spawn_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AsteroidSpawnTimer>,
    bounds: Res<ViewBounds>,
    assets: Res<GatheringAssets>,
    state: Res<ShipState>,
    difficulty: Res<Difficulty>,
    paused: Res<Paused>,
) {
    if !state.alive || paused.0 { return; }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::thread_rng();
    // Feature #7: Scale asteroid sizes with difficulty
    let c = difficulty.combined;
    let min_r = ASTEROID_EARLY_MIN_R + (ASTEROID_LATE_MIN_R - ASTEROID_EARLY_MIN_R) * c.max(0.0).min(1.0);
    let max_r = ASTEROID_EARLY_MAX_R + (ASTEROID_LATE_MAX_R - ASTEROID_EARLY_MAX_R) * c.max(0.0).min(1.0);
    let radius = rng.gen_range(min_r..max_r);
    let base_speed = rng.gen_range(ASTEROID_MIN_SPEED..ASTEROID_MAX_SPEED) * difficulty.speed_mult;

    // Select asteroid type: 60% rock, 25% ice, 15% metallic
    let type_roll: f32 = rng.gen_range(0.0..1.0);
    let asteroid_type = if type_roll < 0.60 {
        AsteroidType::Rock
    } else if type_roll < 0.85 {
        AsteroidType::Ice
    } else {
        AsteroidType::Metallic
    };

    let mesh_idx = rng.gen_range(0..assets.asteroid_meshes.len());
    let material = match asteroid_type {
        AsteroidType::Rock => {
            let i = rng.gen_range(0..assets.asteroid_materials.len());
            assets.asteroid_materials[i].clone()
        }
        AsteroidType::Ice => {
            let i = rng.gen_range(0..assets.ice_materials.len());
            assets.ice_materials[i].clone()
        }
        AsteroidType::Metallic => {
            let i = rng.gen_range(0..assets.metallic_materials.len());
            assets.metallic_materials[i].clone()
        }
    };

    let rot_axis = Vec3::new(
        rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0),
    ).normalize_or(Vec3::Y);
    let rot_speed = rng.gen_range(ASTEROID_MIN_ROT_SPEED..ASTEROID_MAX_ROT_SPEED);

    let (x, y, velocity) = if rng.gen_range(0.0..1.0_f32) < difficulty.side_chance {
        let from_left = rng.gen_range(0..2) == 0;
        let x = if from_left { -bounds.half_width - ASTEROID_SPAWN_BUFFER } else { bounds.half_width + ASTEROID_SPAWN_BUFFER };
        let y = rng.gen_range(-bounds.half_height * 0.5..bounds.half_height);
        let vx = if from_left { rng.gen_range(2.0..6.0) } else { rng.gen_range(-6.0..-2.0) };
        (x, y, Vec2::new(vx, -base_speed * 0.6))
    } else {
        let x = rng.gen_range(-bounds.half_width..bounds.half_width);
        let y = bounds.half_height + ASTEROID_SPAWN_BUFFER + radius;
        let drift = rng.gen_range(-1.5..1.5);
        (x, y, Vec2::new(drift, -base_speed))
    };

    commands.spawn((
        Asteroid { radius, velocity, rot_axis, rot_speed, asteroid_type },
        Mesh3d(assets.asteroid_meshes[mesh_idx].clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(radius)),
    ));
}

pub fn move_asteroids(
    mut commands: Commands,
    mut query: Query<(Entity, &Asteroid, &mut Transform)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
    paused: Res<Paused>,
) {
    if !state.alive || paused.0 { return; }
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
    mut query: Query<(Entity, &mut Asteroid, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();
    let mut entities: Vec<Entity> = Vec::new();
    let mut combos: Vec<(Vec2, f32, Vec2)> = Vec::new();
    for (entity, asteroid, tf) in query.iter() {
        entities.push(entity);
        combos.push((tf.translation.truncate(), asteroid.radius, asteroid.velocity));
    }
    let len = combos.len();
    let mut impulses: Vec<(usize, Vec2)> = Vec::new();
    let mut separations: Vec<(usize, Vec2)> = Vec::new();

    for i in 0..len {
        for j in (i + 1)..len {
            let diff = combos[i].0 - combos[j].0;
            let dist = diff.length();
            let min_dist = combos[i].1 + combos[j].1;
            if dist < min_dist && dist > 0.001 {
                let normal = diff / dist;
                let overlap = min_dist - dist;

                let mass_i = combos[i].1 * combos[i].1;
                let mass_j = combos[j].1 * combos[j].1;
                let total = mass_i + mass_j;
                let sep = normal * overlap * ASTEROID_SEPARATION_SPEED * dt;
                separations.push((i, sep * (mass_j / total)));
                separations.push((j, -sep * (mass_i / total)));

                let rel_vel = combos[i].2 - combos[j].2;
                let vel_along = rel_vel.dot(normal);
                if vel_along < 0.0 {
                    let impulse = vel_along * ASTEROID_BOUNCE_FACTOR / total;
                    impulses.push((i, -normal * impulse * mass_j));
                    impulses.push((j, normal * impulse * mass_i));

                    // Spawn sparks at collision point
                    let contact = Vec3::new(
                        (combos[i].0.x + combos[j].0.x) * 0.5,
                        (combos[i].0.y + combos[j].0.y) * 0.5,
                        0.0,
                    );
                    super::effects::spawn_sparks(&mut commands, &mut meshes, &mut materials, contact);
                }
            }
        }
    }

    if impulses.is_empty() && separations.is_empty() { return; }
    // Spark spawning already happened above during collision detection
    for &(idx, imp) in &impulses {
        if let Ok((_, mut asteroid, _)) = query.get_mut(entities[idx]) {
            asteroid.velocity += imp;
        }
    }
    for &(idx, sep) in &separations {
        if let Ok((_, _, mut tf)) = query.get_mut(entities[idx]) {
            tf.translation.x += sep.x;
            tf.translation.y += sep.y;
        }
    }
}

