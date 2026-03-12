// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

pub fn spawn_ship(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let hull_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(SHIP_HULL_COLOR[0], SHIP_HULL_COLOR[1], SHIP_HULL_COLOR[2], SHIP_HULL_COLOR[3]),
        ..default()
    });
    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(SHIP_ACCENT_COLOR[0], SHIP_ACCENT_COLOR[1], SHIP_ACCENT_COLOR[2], SHIP_ACCENT_COLOR[3]),
        ..default()
    });
    let engine_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(SHIP_ENGINE_COLOR[0], SHIP_ENGINE_COLOR[1], SHIP_ENGINE_COLOR[2], SHIP_ENGINE_COLOR[3]),
        emissive: LinearRgba::new(SHIP_ENGINE_COLOR[0], SHIP_ENGINE_COLOR[1], SHIP_ENGINE_COLOR[2], 1.0) * 2.0,
        ..default()
    });
    let window_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(SHIP_WINDOW_COLOR[0], SHIP_WINDOW_COLOR[1], SHIP_WINDOW_COLOR[2], SHIP_WINDOW_COLOR[3]),
        emissive: LinearRgba::new(SHIP_WINDOW_COLOR[0], SHIP_WINDOW_COLOR[1], SHIP_WINDOW_COLOR[2], 1.0) * 1.5,
        ..default()
    });

    let hull_mesh = meshes.add(Cuboid::new(SHIP_WIDTH, SHIP_HEIGHT, SHIP_LENGTH));
    let pod_mesh = meshes.add(Cuboid::new(SHIP_WIDTH * 0.2, SHIP_HEIGHT * 0.8, SHIP_LENGTH * 0.45));
    let engine_mesh = meshes.add(Cuboid::new(SHIP_WIDTH * 0.15, SHIP_HEIGHT * 0.6, SHIP_LENGTH * 0.12));
    let cockpit_mesh = meshes.add(Cuboid::new(SHIP_WIDTH * 0.4, SHIP_HEIGHT * 0.6, SHIP_LENGTH * 0.15));

    commands.spawn((
        Ship, Transform::default(), Visibility::default(),
    )).with_children(|parent| {
        parent.spawn((ShipPart, Mesh3d(hull_mesh), MeshMaterial3d(hull_mat.clone()),
            Transform::default()));
        parent.spawn((ShipPart, Mesh3d(pod_mesh.clone()), MeshMaterial3d(accent_mat.clone()),
            Transform::from_xyz(-SHIP_WIDTH * 0.45, 0.0, -SHIP_LENGTH * 0.15)));
        parent.spawn((ShipPart, Mesh3d(pod_mesh), MeshMaterial3d(accent_mat),
            Transform::from_xyz(SHIP_WIDTH * 0.45, 0.0, -SHIP_LENGTH * 0.15)));
        parent.spawn((ShipPart, Mesh3d(engine_mesh.clone()), MeshMaterial3d(engine_mat.clone()),
            Transform::from_xyz(-SHIP_WIDTH * 0.45, 0.0, -SHIP_LENGTH * 0.45)));
        parent.spawn((ShipPart, Mesh3d(engine_mesh), MeshMaterial3d(engine_mat),
            Transform::from_xyz(SHIP_WIDTH * 0.45, 0.0, -SHIP_LENGTH * 0.45)));
        parent.spawn((ShipPart, Mesh3d(cockpit_mesh), MeshMaterial3d(window_mat),
            Transform::from_xyz(0.0, SHIP_HEIGHT * 0.3, SHIP_LENGTH * 0.3)));
    });
}

pub fn move_ship(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut ship_q: Query<&mut Transform, With<Ship>>,
    mut state: ResMut<ShipState>,
    time: Res<Time>,
) -> Result {
    if !state.alive { return Ok(()); }
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
        SHIP_INERTIA * (CONTROL_LOSS_FACTOR + (1.0 - CONTROL_LOSS_FACTOR) * (1.0 - state.control_loss_timer / CONTROL_LOSS_DURATION))
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

    let tilt = (-state.velocity.x * 0.02).clamp(-SHIP_MAX_TILT, SHIP_MAX_TILT);
    let pitch = (state.velocity.y * 0.01).clamp(-SHIP_MAX_PITCH, SHIP_MAX_PITCH);
    let target_rot = Quat::from_euler(EulerRot::XZY, pitch, 0.0, tilt);
    ship_tf.rotation = ship_tf.rotation.slerp(target_rot, (SHIP_TILT_SPEED * dt).min(1.0));
    Ok(())
}
