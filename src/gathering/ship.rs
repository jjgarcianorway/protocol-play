// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::window::CursorOptions;
use super::constants::*;
use super::types::*;

pub fn spawn_ship(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ship_scene = asset_server.load("models/craft_speederA.glb#Scene0");
    commands.spawn((
        Ship, SceneRoot(ship_scene),
        Transform::from_scale(Vec3::splat(SHIP_MODEL_SCALE))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    // Shield bubble — separate entity that follows ship
    let bubble_mesh = meshes.add(Sphere::new(SHIELD_BUBBLE_RADIUS).mesh().ico(3).unwrap());
    let bubble_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(
            SHIELD_BUBBLE_COLOR.0, SHIELD_BUBBLE_COLOR.1, SHIELD_BUBBLE_COLOR.2, 0.0,
        ),
        emissive: LinearRgba::new(
            SHIELD_BUBBLE_COLOR.0, SHIELD_BUBBLE_COLOR.1, SHIELD_BUBBLE_COLOR.2, 1.0,
        ) * SHIELD_BUBBLE_EMISSIVE,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((
        ShieldBubble,
        Mesh3d(bubble_mesh),
        MeshMaterial3d(bubble_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

pub fn move_ship(
    windows: Query<&Window>,
    mut cursor_opts: Query<&mut CursorOptions>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut ship_q: Query<&mut Transform, With<Ship>>,
    mut state: ResMut<ShipState>,
    time: Res<Time>,
    paused: Res<Paused>,
) -> Result {
    if !state.alive || paused.0 { return Ok(()); }
    if let Ok(mut opts) = cursor_opts.single_mut() { if opts.visible { opts.visible = false; } }
    let window = windows.single()?;
    let (camera, cam_gt) = cameras.single()?;
    let Some(cursor) = window.cursor_position() else { return Ok(()); };
    let Ok(ray) = camera.viewport_to_world(cam_gt, cursor) else { return Ok(()); };

    if ray.direction.z.abs() < 1e-6 { return Ok(()); }
    let t = -ray.origin.z / ray.direction.z;
    let world_pos = ray.origin + ray.direction * t;
    state.target = Vec2::new(world_pos.x, world_pos.y);

    let dt = time.delta_secs();
    let inertia = if state.control_loss_timer > 0.0 {
        state.control_loss_timer = (state.control_loss_timer - dt).max(0.0);
        SHIP_INERTIA * (CONTROL_LOSS_FACTOR + (1.0 - CONTROL_LOSS_FACTOR)
            * (1.0 - state.control_loss_timer / CONTROL_LOSS_DURATION))
    } else {
        SHIP_INERTIA
    };

    let mut ship_tf = ship_q.single_mut()?;
    let current = Vec2::new(ship_tf.translation.x, ship_tf.translation.y);
    let diff = state.target - current;
    let new_pos = current + diff * (1.0 - (-inertia * dt).exp());
    state.velocity = (new_pos - current) / dt.max(0.001);

    ship_tf.translation.x = new_pos.x;
    ship_tf.translation.y = new_pos.y;

    let tilt = (-state.velocity.x * SHIP_TILT_FACTOR).clamp(-SHIP_MAX_TILT, SHIP_MAX_TILT);
    let pitch = (state.velocity.y * SHIP_PITCH_FACTOR).clamp(-SHIP_MAX_PITCH, SHIP_MAX_PITCH);
    // Base rotation: model faces +Z, we want it facing +Y (screen up)
    let base = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let tilt_rot = Quat::from_euler(EulerRot::XZY, pitch, 0.0, tilt);
    ship_tf.rotation = ship_tf.rotation.slerp(base * tilt_rot, (SHIP_TILT_SPEED * dt).min(1.0));
    Ok(())
}

pub fn restore_cursor(mut cursor_opts: Query<&mut CursorOptions>) -> Result {
    if let Ok(mut opts) = cursor_opts.single_mut() { if !opts.visible { opts.visible = true; } }
    Ok(())
}

pub fn update_shield_bubble(
    ship_q: Query<&Transform, With<Ship>>,
    mut bubble_q: Query<
        (&mut Transform, &MeshMaterial3d<StandardMaterial>),
        (With<ShieldBubble>, Without<Ship>),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<ShipState>,
    time: Res<Time>,
) -> Result {
    let ship_tf = ship_q.single()?;
    let (mut btf, bmat) = bubble_q.single_mut()?;
    btf.translation = ship_tf.translation;

    let shield_pct = (state.shield / SHIELD_MAX).clamp(0.0, 1.0);
    let pulse = 1.0 + (time.elapsed_secs() * SHIELD_BUBBLE_PULSE_SPEED).sin()
        * SHIELD_BUBBLE_PULSE_AMOUNT;
    btf.scale = Vec3::splat(pulse);

    if let Some(mat) = materials.get_mut(&bmat.0) {
        let alpha = shield_pct * SHIELD_BUBBLE_MAX_ALPHA;
        mat.base_color = Color::srgba(
            SHIELD_BUBBLE_COLOR.0, SHIELD_BUBBLE_COLOR.1, SHIELD_BUBBLE_COLOR.2, alpha,
        );
    }
    Ok(())
}

pub fn spawn_engine_particles(
    mut commands: Commands,
    ship_q: Query<&Transform, With<Ship>>,
    state: Res<ShipState>,
    paused: Res<Paused>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result {
    if !state.alive || paused.0 { return Ok(()); }
    let ship_tf = ship_q.single()?;
    let dt = time.delta_secs();
    let emit_count = (ENGINE_PARTICLES_PER_SEC * dt) as u32;
    if emit_count == 0 && rand::random::<f32>() > ENGINE_PARTICLES_PER_SEC * dt {
        return Ok(());
    }

    let mesh = meshes.add(Sphere::new(ENGINE_PARTICLE_SIZE));
    let (r, g, b) = ENGINE_COLOR;
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, 0.8),
        emissive: LinearRgba::new(r, g, b, 1.0) * ENGINE_PARTICLE_EMISSIVE,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let count = emit_count.max(1);
    for &(ox, oy, oz) in &ENGINE_OFFSETS {
        for _ in 0..count {
            let world_offset = ship_tf.rotation * Vec3::new(ox, oy, oz);
            let pos = ship_tf.translation + world_offset;
            let spread = Vec3::new(
                (rand::random::<f32>() - 0.5) * 0.5,
                -ENGINE_PARTICLE_SPEED + (rand::random::<f32>() - 0.5) * 1.0,
                (rand::random::<f32>() - 0.5) * 0.3,
            );
            commands.spawn((
                EngineParticle { lifetime: ENGINE_PARTICLE_LIFETIME, velocity: spread },
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_translation(pos).with_scale(Vec3::splat(1.0)),
            ));
        }
    }
    Ok(())
}

pub fn move_engine_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut EngineParticle, &mut Transform)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, mut p, mut tf) in query.iter_mut() {
        p.lifetime -= dt;
        tf.translation += p.velocity * dt;
        let life_frac = (p.lifetime / ENGINE_PARTICLE_LIFETIME).max(0.0);
        tf.scale = Vec3::splat(life_frac);
        if p.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
