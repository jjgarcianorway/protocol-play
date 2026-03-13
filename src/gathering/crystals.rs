// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

#[derive(Component)]
pub struct NebulaLayer;

#[derive(Component)]
pub struct NebulaLight;

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
    let value = rng.gen_range(CRYSTAL_MIN_VALUE..CRYSTAL_MAX_VALUE);
    // Nebula size reflects crystal value
    let value_t = (value - CRYSTAL_MIN_VALUE) as f32
        / (CRYSTAL_MAX_VALUE - CRYSTAL_MIN_VALUE) as f32;
    let radius = CRYSTAL_MIN_RADIUS + value_t * (CRYSTAL_MAX_RADIUS - CRYSTAL_MIN_RADIUS);
    let x = rng.gen_range(-bounds.half_width * 0.7..bounds.half_width * 0.7);
    let y = bounds.half_height + ASTEROID_SPAWN_BUFFER + radius;

    let rot_axis = Vec3::new(
        rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0),
    ).normalize_or(Vec3::Z);

    commands.spawn((
        CrystalCloud { radius, value, remaining: 1.0, rot_axis, particle_timer: 0.0 },
        Transform::from_xyz(x, y, 0.0),
        Visibility::default(),
    )).with_children(|parent| {
        let n = assets.crystal_meshes.len().min(assets.crystal_materials.len());
        for i in 0..n {
            let num = if i < n / 3 { 3 } else { 2 };
            for _ in 0..num {
                let t = i as f32 / n.max(1) as f32;
                let spread = radius * (0.1 + t * 0.45);
                let ox = rng.gen_range(-spread..spread);
                let oy = rng.gen_range(-spread..spread);
                let oz = rng.gen_range(-spread * 0.4..spread * 0.4);
                let layer_scale = radius * (0.3 + t * 0.5) * rng.gen_range(0.6..1.4);
                parent.spawn((
                    NebulaLayer,
                    Mesh3d(assets.crystal_meshes[i].clone()),
                    MeshMaterial3d(assets.crystal_materials[i].clone()),
                    Transform::from_xyz(ox, oy, oz)
                        .with_scale(Vec3::splat(layer_scale))
                        .with_rotation(Quat::from_euler(
                            EulerRot::XYZ,
                            rng.gen_range(0.0..std::f32::consts::TAU),
                            rng.gen_range(0.0..std::f32::consts::TAU),
                            rng.gen_range(0.0..std::f32::consts::TAU),
                        )),
                ));
            }
        }
        // Central point light — brighter than particle lights
        parent.spawn((
            NebulaLight,
            PointLight {
                color: Color::srgb(0.2, 0.5, 1.0),
                intensity: CRYSTAL_POINT_LIGHT_INTENSITY * radius,
                range: CRYSTAL_POINT_LIGHT_RANGE * radius.sqrt(),
                shadows_enabled: false,
                ..default()
            },
            Transform::IDENTITY,
        ));
    });
}

pub fn move_crystals(
    mut commands: Commands,
    mut cloud_q: Query<
        (Entity, &mut CrystalCloud, &mut Transform),
        (Without<NebulaLayer>, Without<CrystalParticle>),
    >,
    children_q: Query<&Children>,
    mut light_q: Query<&mut PointLight, With<NebulaLight>>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
    state: Res<ShipState>,
    ship_q: Query<
        &Transform,
        (With<Ship>, Without<CrystalCloud>, Without<NebulaLayer>, Without<CrystalParticle>),
    >,
    assets: Res<GatheringAssets>,
) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    let ship_pos = ship_q.iter().next().map(|t| t.translation);
    let mut rng = rand::thread_rng();

    for (entity, mut cloud, mut tf) in cloud_q.iter_mut() {
        tf.translation.y -= SCROLL_SPEED * 0.5 * dt;
        tf.rotate(Quat::from_axis_angle(cloud.rot_axis, CRYSTAL_ROT_SPEED * dt));

        // Nebula shrinks as crystals are absorbed
        let s = cloud.remaining.max(0.01);
        let new_scale = Vec3::splat(s);
        if tf.scale != new_scale { tf.scale = new_scale; }

        // Dim the central point light proportionally
        if let Ok(children) = children_q.get(entity) {
            for child in children.iter() {
                if let Ok(mut light) = light_q.get_mut(child) {
                    let target_intensity = CRYSTAL_POINT_LIGHT_INTENSITY * cloud.radius * s;
                    if (light.intensity - target_intensity).abs() > 1.0 {
                        light.intensity = target_intensity;
                        light.range = CRYSTAL_POINT_LIGHT_RANGE * cloud.radius.sqrt() * s;
                    }
                }
            }
        }

        // Emit pollen-like particles during absorption
        if cloud.remaining < 1.0 {
            if let Some(sp) = ship_pos {
                cloud.particle_timer += dt;
                let interval = 1.0 / PARTICLE_EMIT_RATE;
                while cloud.particle_timer >= interval {
                    cloud.particle_timer -= interval;
                    emit_particle(
                        &mut commands, &assets, &mut rng,
                        tf.translation, sp, cloud.radius * s,
                    );
                }
            }
        }

        let margin = ASTEROID_DESPAWN_BUFFER + cloud.radius;
        if tf.translation.y < -bounds.half_height - margin || cloud.remaining <= 0.01 {
            commands.entity(entity).despawn();
        }
    }
}

fn emit_particle(
    commands: &mut Commands,
    assets: &GatheringAssets,
    rng: &mut impl Rng,
    cloud_pos: Vec3,
    ship_pos: Vec3,
    spread: f32,
) {
    let ci = rng.gen_range(0..assets.particle_materials.len());
    let offset = Vec3::new(
        rng.gen_range(-spread..spread) * PARTICLE_SPREAD,
        rng.gen_range(-spread..spread) * PARTICLE_SPREAD,
        rng.gen_range(-spread * 0.3..spread * 0.3),
    );
    let start = cloud_pos + offset;
    let dir = (ship_pos - start).normalize_or(Vec3::Y);
    let speed = PARTICLE_SPEED * rng.gen_range(0.7..1.3);

    commands.spawn((
        CrystalParticle { velocity: dir * speed, lifetime: PARTICLE_LIFETIME },
        Mesh3d(assets.particle_mesh.clone()),
        MeshMaterial3d(assets.particle_materials[ci].clone()),
        Transform::from_translation(start)
            .with_scale(Vec3::splat(PARTICLE_SIZE * rng.gen_range(0.5..1.5))),
    )).with_children(|parent| {
        parent.spawn((
            PointLight {
                color: Color::srgb(0.15, 0.4, 1.0),
                intensity: PARTICLE_LIGHT_INTENSITY,
                range: PARTICLE_LIGHT_RANGE,
                shadows_enabled: false,
                ..default()
            },
            Transform::IDENTITY,
        ));
    });
}

pub fn move_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CrystalParticle, &mut Transform)>,
    ship_q: Query<&Transform, (With<Ship>, Without<CrystalParticle>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let ship_pos = ship_q.iter().next().map(|t| t.translation);

    for (entity, mut particle, mut tf) in query.iter_mut() {
        particle.lifetime -= dt;

        // Home toward ship
        if let Some(sp) = ship_pos {
            let dir = (sp - tf.translation).normalize_or(Vec3::Y);
            let speed = particle.velocity.length();
            particle.velocity = particle.velocity.lerp(dir * speed, PARTICLE_HOMING * dt);
        }

        tf.translation += particle.velocity * dt;

        // Fade/shrink as lifetime decreases
        let life_frac = (particle.lifetime / PARTICLE_LIFETIME).max(0.0);
        let shrink = 1.0 - dt * 2.0 * (1.0 - life_frac);
        tf.scale *= shrink.max(0.1);

        let near_ship = ship_pos.map_or(false, |sp| tf.translation.distance(sp) < 1.5);
        if particle.lifetime <= 0.0 || near_ship {
            commands.entity(entity).despawn();
        }
    }
}

pub fn absorb_crystals(
    ship_q: Query<&Transform, With<Ship>>,
    mut crystal_q: Query<(&mut CrystalCloud, &Transform)>,
    mut state: ResMut<ShipState>,
    time: Res<Time>,
) -> Result {
    if !state.alive { return Ok(()); }
    let ship_tf = ship_q.single()?;
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
    Ok(())
}
