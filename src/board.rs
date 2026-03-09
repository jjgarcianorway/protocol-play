// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

// === Helpers ===
pub fn camera_direction() -> Vec3 {
    let elev = CAMERA_ELEVATION.to_radians();
    let azim = CAMERA_AZIMUTH.to_radians();
    Vec3::new(elev.cos() * azim.sin(), elev.sin(), elev.cos() * azim.cos())
}

pub fn board_bounding_radius(size: u32) -> f32 {
    let half = size as f32 / 2.0;
    (half * half + 0.35 * 0.35 + half * half).sqrt()
}

pub fn tile_world_pos(col: u32, row: u32, board_size: u32, kind: &TileKind) -> Vec3 {
    let offset = (board_size as f32 - 1.0) / 2.0;
    let y = match kind {
        TileKind::Empty => EMPTY_MARKER_Y,
        TileKind::Floor | TileKind::Source(_, _) | TileKind::Goal(_)
        | TileKind::Turn(_, _) | TileKind::TurnBut(_, _) | TileKind::Teleport(_)
        | TileKind::Bounce(_) | TileKind::BounceBut(_)
        | TileKind::Door(_) | TileKind::Switch | TileKind::Painter(_)
        | TileKind::Arrow(_, _) | TileKind::ArrowBut(_, _) => 0.0,
    };
    Vec3::new(col as f32 - offset, y, row as f32 - offset)
}

// === Spawning ===
pub fn spawn_tile(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets,
) -> Entity {
    spawn_tile_at_scale(commands, col, row, board_size, kind, assets, Vec3::ZERO)
}

pub fn spawn_tile_at_scale(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets, initial_scale: Vec3,
) -> Entity {
    let pos = tile_world_pos(col, row, board_size, &kind);
    match kind {
        TileKind::Empty => {
            commands.spawn((
                Mesh3d(assets.empty_mesh.clone()), MeshMaterial3d(assets.empty_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).id()
        }
        TileKind::Floor => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).id()
        }
        TileKind::Source(ci, dir) => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.source_symbol_mesh.clone()),
                    MeshMaterial3d(assets.source_symbol_materials[ci].clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
        TileKind::Goal(ci) | TileKind::Teleport(ci) | TileKind::Bounce(ci) | TileKind::BounceBut(ci)
        | TileKind::Painter(ci) => {
            let mat = match kind {
                TileKind::Goal(_) => assets.goal_symbol_materials[ci].clone(),
                TileKind::Teleport(_) => assets.teleport_symbol_materials[ci].clone(),
                TileKind::Bounce(_) => assets.bounce_symbol_materials[ci].clone(),
                TileKind::Painter(_) => assets.painter_symbol_materials[ci].clone(),
                _ => assets.bouncebot_symbol_materials[ci].clone(),
            };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.goal_symbol_mesh.clone()), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
        TileKind::Turn(ci, dir) | TileKind::TurnBut(ci, dir) => {
            let (mesh, mat) = if matches!(kind, TileKind::Turn(_, _)) {
                (assets.turn_symbol_mesh.clone(), assets.turn_symbol_materials[ci].clone())
            } else {
                (assets.turnbut_symbol_mesh.clone(), assets.turnbut_symbol_materials[ci].clone())
            };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(mesh), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
        TileKind::Door(open) => {
            let mat = if open { assets.door_open_material.clone() } else { assets.door_closed_material.clone() };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.goal_symbol_mesh.clone()), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
        TileKind::Switch => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.goal_symbol_mesh.clone()), MeshMaterial3d(assets.switch_material.clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
        TileKind::Arrow(ci, dir) | TileKind::ArrowBut(ci, dir) => {
            let (mesh, mat) = if matches!(kind, TileKind::Arrow(_, _)) {
                (assets.arrow_symbol_mesh.clone(), assets.arrow_symbol_materials[ci].clone())
            } else {
                (assets.arrowbut_symbol_mesh.clone(), assets.arrowbut_symbol_materials[ci].clone())
            };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(mesh), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            }).id()
        }
    }
}

pub fn spawn_board(commands: &mut Commands, size: u32, assets: &GameAssets) {
    for row in 0..size {
        for col in 0..size {
            spawn_tile(commands, col, row, size, TileKind::Empty, assets);
        }
    }
}

// === Board Size Buttons ===
pub fn button_interaction(
    mut commands: Commands,
    mut board_size: ResMut<BoardSize>,
    interaction_query: Query<(&Interaction, &BoardButton), Changed<Interaction>>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut size_text: Query<&mut Text, With<BoardSizeText>>,
    mut placed_teleports: ResMut<PlacedTeleports>,
    play_mode: Res<PlayMode>,
    mut validated: ResMut<LevelValidated>,
) {
    if *play_mode != PlayMode::Editing { return; }
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let new_size = match button {
            BoardButton::Increase => (board_size.0 + 1).min(MAX_BOARD_SIZE),
            BoardButton::Decrease => board_size.0.saturating_sub(1).max(MIN_BOARD_SIZE),
        };
        if new_size == board_size.0 { continue; }
        board_size.0 = new_size; validated.0 = false;
        placed_teleports.0 = [0; 10];
        for entity in &tiles { commands.entity(entity).despawn_recursive(); }
        spawn_board(&mut commands, new_size, &assets);
        let mut text = size_text.single_mut();
        **text = format!("{}x{}", new_size, new_size);
    }
}

// === Camera ===
pub fn adapt_camera(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection), With<Camera3d>>,
    board_size: Res<BoardSize>,
) {
    let window = windows.single();
    let (mut transform, projection) = cameras.single_mut();
    let aspect = window.width() / window.height();
    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => return,
    };
    let radius = board_bounding_radius(board_size.0);
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let dist_v = radius / half_fov_v.sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;
    let dir = camera_direction();
    *transform = Transform::from_translation(dir * distance).looking_at(Vec3::ZERO, Vec3::Y);
}
