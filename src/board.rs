// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(dead_code)]

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

/// Auto-assign next teleport number for a given (type, color).
/// Finds first unpaired number (count==1), then first unused (count==0).
pub fn next_teleport_number(
    tiles: impl Iterator<Item = TileKind>, is_but: bool, color: usize,
) -> Option<usize> {
    let mut counts = [0u8; NUM_TELEPORTS];
    for kind in tiles {
        match (is_but, kind) {
            (false, TileKind::Teleport(c, n)) if c == color => { counts[n] += 1; }
            (true, TileKind::TeleportBut(c, n)) if c == color => { counts[n] += 1; }
            _ => {}
        }
    }
    if let Some(n) = counts.iter().position(|&c| c == 1) { return Some(n); }
    counts.iter().position(|&c| c == 0)
}

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
        | TileKind::Turn(_, _) | TileKind::TurnBut(_, _)
        | TileKind::Teleport(_, _) | TileKind::TeleportBut(_, _)
        | TileKind::Bounce(_) | TileKind::BounceBut(_)
        | TileKind::Door(_) | TileKind::Switch
        | TileKind::ColorSwitch(_) | TileKind::ColorSwitchBut(_)
        | TileKind::Painter(_)
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
        TileKind::Goal(ci) | TileKind::Bounce(ci) | TileKind::BounceBut(ci)
        | TileKind::Painter(ci) => {
            let mat = match kind {
                TileKind::Goal(_) => assets.goal_symbol_materials[ci].clone(),
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
        TileKind::Teleport(ci, num) | TileKind::TeleportBut(ci, num) => {
            let idx = if matches!(kind, TileKind::Teleport(_, _)) {
                num * NUM_TELEPORT_COLORS + ci
            } else { num * NUM_COLORS + ci };
            let mat = if matches!(kind, TileKind::Teleport(_, _)) {
                assets.teleport_symbol_materials[idx].clone()
            } else { assets.teleportbut_symbol_materials[idx].clone() };
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
        TileKind::ColorSwitch(ci) | TileKind::ColorSwitchBut(ci) => {
            let mat = match kind {
                TileKind::ColorSwitch(_) => assets.colorswitch_symbol_materials[ci].clone(),
                _ => assets.colorswitchbut_symbol_materials[ci].clone(),
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

/// Populate the board from a tile list, spawning tiles and marking inventory.
pub fn populate_board(
    commands: &mut Commands, size: u32,
    tile_data: &[(u32, u32, TileKind, bool)], assets: &GameAssets,
) {
    let mut present = std::collections::HashSet::new();
    for &(col, row, _, _) in tile_data { present.insert((col, row)); }
    for row in 0..size {
        for col in 0..size {
            if !present.contains(&(col, row)) {
                spawn_tile(commands, col, row, size, TileKind::Empty, assets);
            }
        }
    }
    for &(col, row, kind, is_marked) in tile_data {
        if col >= size || row >= size { continue; }
        let entity = spawn_tile(commands, col, row, size, kind, assets);
        if is_marked {
            commands.entity(entity).insert(InventoryMarker).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.marker_mesh.clone()),
                    MeshMaterial3d(assets.marker_material.clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0)),
                    InventoryMarkerVisual,
                ));
            });
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
    play_mode: Res<PlayMode>,
    mut validated: ResMut<LevelValidated>,
) -> Result {
    if *play_mode != PlayMode::Editing { return Ok(()); }
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let new_size = match button {
            BoardButton::Increase => (board_size.0 + 1).min(MAX_BOARD_SIZE),
            BoardButton::Decrease => board_size.0.saturating_sub(1).max(MIN_BOARD_SIZE),
        };
        if new_size == board_size.0 { continue; }
        board_size.0 = new_size; validated.0 = false;
        for entity in &tiles { commands.entity(entity).despawn(); }
        spawn_board(&mut commands, new_size, &assets);
        let mut text = size_text.single_mut()?;
        **text = format!("{}x{}", new_size, new_size);
    }
    Ok(())
}

// === Camera ===
pub fn adapt_camera(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection), (With<Camera3d>, Without<IconCamera>)>,
    board_size: Res<BoardSize>,
    expansion: Query<&Node, With<ExpansionContainer>>,
    play_mode: Res<PlayMode>,
    time: Res<Time>,
) -> Result {
    let window = windows.single()?;
    let (mut transform, projection) = cameras.single_mut()?;
    let (w, h) = (window.width(), window.height());
    let aspect = w / h;
    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => return Ok(()),
    };
    let playing = matches!(*play_mode, PlayMode::Playing | PlayMode::TestPlaying);
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let radius = board_bounding_radius(board_size.0);

    // UI pixel heights
    let vw = w / 100.0;
    let is_player = cfg!(feature = "player");
    let (top_px, bot_px) = if playing {
        (0.0, 0.0)
    } else if is_player {
        let inv = SLOT_HEIGHT_VW * vw + INVENTORY_PAD_VW * 2.0 * vw + INV_SLIDE_SHOW;
        (h * 0.06, inv + h * 0.12)
    } else {
        let exp = expansion.iter().next()
            .map(|n| match n.height { Val::Vw(v) => v * vw, _ => 0.0 }).unwrap_or(0.0);
        let inv = SLOT_HEIGHT_VW * vw + INVENTORY_PAD_VW * 2.0 * vw
            + COUNT_FONT + SLOT_BORDER_PX * 2.0 + INV_SLIDE_SHOW + exp;
        (TOP_BTN_SIZE + TOP_SLIDE_SHOW, inv)
    };

    // Usable viewport between UI elements
    let usable_h = (h - top_px - bot_px).max(100.0);

    // Distance: vertical uses reduced radius (isometric foreshortening)
    // Play mode needs more vertical room (no UI to constrain, board must fit fully)
    let rv = if playing { radius * 0.85 } else { radius * 0.7 };
    let usable_fov_v = half_fov_v * (usable_h / h);
    let dist_v = rv / usable_fov_v.sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;

    // Vertical position: shift board UP to clear inventory at bottom
    let look_y = if playing { 0.0 } else { -0.09 * distance };
    let look_at = Vec3::new(0.0, look_y, 0.0);

    let dir = camera_direction();
    let target = Transform::from_translation(look_at + dir * distance).looking_at(look_at, Vec3::Y);
    let speed = CAMERA_ZOOM_SPEED * time.delta_secs();
    transform.translation = transform.translation.lerp(target.translation, speed);
    transform.rotation = transform.rotation.slerp(target.rotation, speed);
    Ok(())
}
