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
        TileKind::Floor | TileKind::Source(_, _) | TileKind::Turn(_, _) => 0.0,
    };
    Vec3::new(col as f32 - offset, y, row as f32 - offset)
}

// === Spawning ===
pub fn spawn_tile(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets,
) {
    spawn_tile_at_scale(commands, col, row, board_size, kind, assets, Vec3::ZERO);
}

pub fn spawn_tile_at_scale(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets, initial_scale: Vec3,
) {
    let pos = tile_world_pos(col, row, board_size, &kind);
    match kind {
        TileKind::Empty => {
            commands.spawn((
                Mesh3d(assets.empty_mesh.clone()), MeshMaterial3d(assets.empty_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
        }
        TileKind::Floor => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
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
            });
        }
        TileKind::Turn(ci, dir) => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.turn_symbol_mesh.clone()),
                    MeshMaterial3d(assets.turn_symbol_materials[ci].clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            });
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
    mut placed_sources: ResMut<PlacedSources>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let new_size = match button {
            BoardButton::Increase => (board_size.0 + 1).min(MAX_BOARD_SIZE),
            BoardButton::Decrease => board_size.0.saturating_sub(1).max(MIN_BOARD_SIZE),
        };
        if new_size == board_size.0 { continue; }
        board_size.0 = new_size;
        placed_sources.0.clear();
        for entity in &tiles { commands.entity(entity).despawn(); }
        spawn_board(&mut commands, new_size, &assets);
        let mut text = size_text.single_mut();
        **text = format!("{}x{}", new_size, new_size);
    }
}

// === Play/Stop ===
pub fn play_stop_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<PlayStopButton>, Changed<Interaction>)>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    bots: Query<Entity, With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut button_image: Query<&mut ImageNode, With<PlayButtonImage>>,
    play_icons: Res<PlayIcons>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        match *play_mode {
            PlayMode::Editing => {
                *play_mode = PlayMode::Playing;
                let mut img = button_image.single_mut();
                img.image = play_icons.stop.clone();
                for (coord, kind) in &tiles {
                    if let TileKind::Source(ci, dir) = kind {
                        let pos = tile_world_pos(coord.col, coord.row, board_size.0, kind);
                        let bot_y = FLOOR_TOP_Y + BOT_SIZE / 2.0;
                        commands.spawn((
                            Mesh3d(assets.bot_mesh.clone()),
                            MeshMaterial3d(assets.bot_materials[*ci].clone()),
                            Transform::from_translation(Vec3::new(pos.x, bot_y, pos.z))
                                .with_rotation(Quat::from_rotation_y(dir.rotation()))
                                .with_scale(Vec3::ZERO),
                            TargetScale(Vec3::ONE),
                            Bot,
                        )).with_children(|parent| {
                            parent.spawn((
                                Mesh3d(assets.eye_mesh.clone()),
                                MeshMaterial3d(assets.eye_material.clone()),
                                Transform::from_translation(Vec3::new(-0.07, 0.04, -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + 0.001))),
                            ));
                            parent.spawn((
                                Mesh3d(assets.eye_mesh.clone()),
                                MeshMaterial3d(assets.eye_material.clone()),
                                Transform::from_translation(Vec3::new(0.07, 0.04, -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + 0.001))),
                            ));
                        });
                    }
                }
            }
            PlayMode::Playing => {
                *play_mode = PlayMode::Editing;
                let mut img = button_image.single_mut();
                img.image = play_icons.play.clone();
                for entity in &bots {
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                }
            }
        }
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
