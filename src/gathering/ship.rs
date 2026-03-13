// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::window::CursorOptions;
use super::constants::*;
use super::types::*;

pub fn spawn_ship(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let hull_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(SHIP_HULL_COLOR.0, SHIP_HULL_COLOR.1, SHIP_HULL_COLOR.2),
        perceptual_roughness: 0.3, metallic: 0.7, ..default()
    });
    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(SHIP_ACCENT_COLOR.0, SHIP_ACCENT_COLOR.1, SHIP_ACCENT_COLOR.2),
        perceptual_roughness: 0.4, metallic: 0.6, ..default()
    });
    let engine_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(SHIP_ENGINE_COLOR.0, SHIP_ENGINE_COLOR.1, SHIP_ENGINE_COLOR.2),
        emissive: LinearRgba::new(
            SHIP_ENGINE_COLOR.0, SHIP_ENGINE_COLOR.1, SHIP_ENGINE_COLOR.2, 1.0,
        ) * SHIP_ENGINE_EMISSIVE,
        ..default()
    });
    let cockpit_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(
            SHIP_COCKPIT_COLOR.0, SHIP_COCKPIT_COLOR.1, SHIP_COCKPIT_COLOR.2, 0.85,
        ),
        emissive: LinearRgba::new(
            SHIP_COCKPIT_COLOR.0, SHIP_COCKPIT_COLOR.1, SHIP_COCKPIT_COLOR.2, 1.0,
        ) * SHIP_COCKPIT_EMISSIVE,
        alpha_mode: AlphaMode::Blend, ..default()
    });
    let wing_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(SHIP_WING_COLOR.0, SHIP_WING_COLOR.1, SHIP_WING_COLOR.2),
        perceptual_roughness: 0.35, metallic: 0.8, ..default()
    });

    // All meshes oriented along Y (screen up = nose, screen down = engines)
    let hull_mesh = meshes.add(Capsule3d::new(SHIP_HULL_R, SHIP_HULL_LEN));
    let nacelle_mesh = meshes.add(Capsule3d::new(SHIP_NACELLE_R, SHIP_NACELLE_LEN));
    let engine_mesh = meshes.add(Sphere::new(SHIP_ENGINE_R).mesh().ico(3).unwrap());
    let cockpit_mesh = meshes.add(Sphere::new(SHIP_COCKPIT_R).mesh().ico(3).unwrap());
    let wing_mesh = meshes.add(Cuboid::new(SHIP_WING_SPAN, SHIP_WING_THICK, SHIP_WING_CHORD));

    commands.spawn((
        Ship, Transform::default(), Visibility::default(),
    )).with_children(|parent| {
        // Main hull — capsule along Y (default), nose at +Y
        parent.spawn((Mesh3d(hull_mesh), MeshMaterial3d(hull_mat.clone()),
            Transform::default()));
        // Cockpit — sphere near nose
        parent.spawn((Mesh3d(cockpit_mesh), MeshMaterial3d(cockpit_mat),
            Transform::from_xyz(0.0, SHIP_HULL_LEN * 0.35, -SHIP_HULL_R * 0.3)));
        // Wings — flat box, slightly behind center
        parent.spawn((Mesh3d(wing_mesh), MeshMaterial3d(wing_mat),
            Transform::from_xyz(0.0, -SHIP_HULL_LEN * 0.05, 0.0)));
        // Left nacelle
        parent.spawn((Mesh3d(nacelle_mesh.clone()), MeshMaterial3d(accent_mat.clone()),
            Transform::from_xyz(-SHIP_NACELLE_OFF, -SHIP_NACELLE_Y, 0.0)));
        // Right nacelle
        parent.spawn((Mesh3d(nacelle_mesh), MeshMaterial3d(accent_mat),
            Transform::from_xyz(SHIP_NACELLE_OFF, -SHIP_NACELLE_Y, 0.0)));
        // Left engine glow
        parent.spawn((Mesh3d(engine_mesh.clone()), MeshMaterial3d(engine_mat.clone()),
            Transform::from_xyz(-SHIP_NACELLE_OFF, -SHIP_NACELLE_Y - SHIP_NACELLE_LEN * 0.55, 0.0)));
        // Right engine glow
        parent.spawn((Mesh3d(engine_mesh.clone()), MeshMaterial3d(engine_mat.clone()),
            Transform::from_xyz(SHIP_NACELLE_OFF, -SHIP_NACELLE_Y - SHIP_NACELLE_LEN * 0.55, 0.0)));
        // Center engine glow
        parent.spawn((Mesh3d(engine_mesh), MeshMaterial3d(engine_mat),
            Transform::from_xyz(0.0, -SHIP_HULL_LEN * 0.55, 0.0)));
    });
}

pub fn move_ship(
    windows: Query<&Window>,
    mut cursor_opts: Query<&mut CursorOptions>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut ship_q: Query<&mut Transform, With<Ship>>,
    mut state: ResMut<ShipState>,
    time: Res<Time>,
) -> Result {
    if !state.alive { return Ok(()); }
    // Hide OS cursor during gameplay
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
    let target_rot = Quat::from_euler(EulerRot::XZY, pitch, 0.0, tilt);
    ship_tf.rotation = ship_tf.rotation.slerp(target_rot, (SHIP_TILT_SPEED * dt).min(1.0));
    Ok(())
}

pub fn restore_cursor(mut cursor_opts: Query<&mut CursorOptions>) -> Result {
    if let Ok(mut opts) = cursor_opts.single_mut() { if !opts.visible { opts.visible = true; } }
    Ok(())
}
