// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::*;

// === Animation ===
pub fn animate_scale(
    mut query: Query<(&mut Transform, &TargetScale, Option<&GhostPreview>, Option<&TileHighlight>)>,
    time: Res<Time>,
) {
    for (mut transform, target, ghost, highlight) in &mut query {
        let speed = if ghost.is_some() || highlight.is_some() {
            HOVER_ANIM_SPEED
        } else {
            ANIM_SPEED
        };
        transform.scale = transform.scale.lerp(target.0, speed * time.delta_secs());
        if transform.scale.distance(target.0) < 0.01 {
            transform.scale = target.0;
        }
    }
}

pub fn animate_node_width(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Node, &NodeWidthAnim)>,
    time: Res<Time>,
) {
    for (entity, mut node, anim) in &mut query {
        let current = match node.width {
            Val::Vw(w) | Val::Px(w) => w,
            _ => anim.target,
        };
        let new_w = current + (anim.target - current) * UI_ANIM_SPEED * time.delta_secs();
        if (new_w - anim.target).abs() < 0.1 {
            if anim.despawn_at_zero && anim.target < 0.1 {
                commands.entity(entity).despawn_recursive();
            } else {
                node.width = Val::Vw(anim.target);
                commands.entity(entity).remove::<NodeWidthAnim>();
            }
        } else {
            node.width = Val::Vw(new_w);
        }
    }
}

pub fn cleanup_despawned(mut commands: Commands, query: Query<(Entity, &Transform), With<DespawnAtZeroScale>>) {
    for (entity, transform) in &query {
        if transform.scale.length() < 0.02 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// === Mouse Hover ===
pub fn update_hovered_cell(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    board_size: Res<BoardSize>,
    mut hovered: ResMut<HoveredCell>,
    ui_interactions: Query<&Interaction, With<Button>>,
) {
    for interaction in &ui_interactions {
        if *interaction != Interaction::None {
            hovered.0 = None;
            return;
        }
    }
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    let Some(cursor) = window.cursor_position() else { hovered.0 = None; return; };
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor) else { hovered.0 = None; return; };
    let dir = ray.direction.as_vec3();
    if dir.y.abs() < 1e-6 { hovered.0 = None; return; }
    let t = -ray.origin.y / dir.y;
    if t < 0.0 { hovered.0 = None; return; }
    let hit = ray.origin + dir * t;
    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let col = (hit.x + offset + 0.5).floor() as i32;
    let row = (hit.z + offset + 0.5).floor() as i32;
    if col >= 0 && col < board_size.0 as i32 && row >= 0 && row < board_size.0 as i32 {
        hovered.0 = Some((col as u32, row as u32));
    } else {
        hovered.0 = None;
    }
}

// === Ghost & Highlight ===
pub fn update_ghost_and_highlight(
    mut commands: Commands,
    hovered: Res<HoveredCell>,
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    mut ghost_q: Query<
        (&mut Transform, &mut TargetScale, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>),
        (With<GhostPreview>, Without<TileHighlight>),
    >,
    mut ghost_overlay_q: Query<
        (&mut Transform, &mut MeshMaterial3d<StandardMaterial>),
        (With<GhostSymbolOverlay>, Without<GhostPreview>, Without<TileHighlight>, Without<Tile>),
    >,
    mut highlight_q: Query<
        (&mut Transform, &mut TargetScale),
        (With<TileHighlight>, Without<GhostPreview>, Without<Tile>),
    >,
    mut hidden_tile: ResMut<HiddenTileEntity>,
    mut ghost_cell: ResMut<GhostCell>,
    mut tile_scale_q: Query<
        &mut TargetScale,
        (With<Tile>, Without<GhostPreview>, Without<TileHighlight>, Without<DespawnAtZeroScale>),
    >,
    placed_sources: Res<PlacedSources>,
) {
    // Restore previous suppressed tile
    if let Some(old_entity) = hidden_tile.0.take() {
        if let Ok(mut target) = tile_scale_q.get_mut(old_entity) {
            target.0 = Vec3::ONE;
        }
    }

    let (mut ghost_tf, mut ghost_target, mut ghost_mesh, mut ghost_mat) = ghost_q.single_mut();
    let (mut overlay_tf, mut overlay_mat) = ghost_overlay_q.single_mut();
    let (mut hl_tf, mut hl_target) = highlight_q.single_mut();
    let mut show_overlay = false;

    let Some((col, row)) = hovered.0 else {
        if ghost_target.0.length() > 0.5 && selected_tool.0 == Tool::Delete {
            commands.spawn((
                Mesh3d(assets.ghost_delete_mesh.clone()),
                MeshMaterial3d(assets.ghost_delete_material.clone()),
                Transform::from_translation(ghost_tf.translation).with_scale(ghost_tf.scale),
                TargetScale(Vec3::ZERO), DespawnAtZeroScale,
            ));
        }
        ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO;
        hl_target.0 = Vec3::ZERO;
        ghost_cell.0 = None;
        return;
    };

    let tile_info = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile_info else {
        ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO;
        hl_target.0 = Vec3::ZERO;
        ghost_cell.0 = None;
        return;
    };

    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let world_x = col as f32 - offset;
    let world_z = row as f32 - offset;

    let cell_changed = ghost_cell.0 != Some((col, row));
    if cell_changed {
        if selected_tool.0 == Tool::Delete && ghost_target.0.length() > 0.5 {
            commands.spawn((
                Mesh3d(assets.ghost_delete_mesh.clone()),
                MeshMaterial3d(assets.ghost_delete_material.clone()),
                Transform::from_translation(ghost_tf.translation).with_scale(ghost_tf.scale),
                TargetScale(Vec3::ZERO), DespawnAtZeroScale,
            ));
        }
        ghost_tf.scale = Vec3::ZERO;
        hl_tf.scale = Vec3::ZERO;
        ghost_cell.0 = Some((col, row));
    }

    let hl_y = FLOOR_TOP_Y + 0.01;
    hl_tf.translation = Vec3::new(world_x, hl_y, world_z);
    hl_target.0 = Vec3::ONE;

    let can_place_floor = matches!(kind, TileKind::Empty | TileKind::Source(_, _) | TileKind::Turn(_, _));
    let can_place_source = matches!(kind, TileKind::Empty | TileKind::Floor | TileKind::Turn(_, _));
    let can_place_turn = matches!(kind, TileKind::Empty | TileKind::Floor | TileKind::Source(_, _));
    let can_delete = matches!(kind, TileKind::Floor | TileKind::Source(_, _) | TileKind::Turn(_, _));

    match selected_tool.0 {
        Tool::Floor if can_place_floor => {
            ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
            ghost_target.0 = Vec3::ONE;
            if let Ok(mut target) = tile_scale_q.get_mut(entity) {
                target.0 = Vec3::ZERO;
                hidden_tile.0 = Some(entity);
            }
        }
        Tool::Source if can_place_source => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
                    ghost_tf.rotation = Quat::from_rotation_y(dir.rotation());
                    *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
                    *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
                    *overlay_mat = MeshMaterial3d(assets.ghost_symbol_materials[ci].clone());
                    show_overlay = true;
                    ghost_target.0 = Vec3::ONE;
                    if let Ok(mut target) = tile_scale_q.get_mut(entity) {
                        target.0 = Vec3::ZERO;
                        hidden_tile.0 = Some(entity);
                    }
                } else {
                    ghost_target.0 = Vec3::ZERO;
                }
            } else {
                ghost_target.0 = Vec3::ZERO;
            }
        }
        Tool::Turn if can_place_turn => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
                ghost_tf.rotation = Quat::from_rotation_y(dir.rotation());
                *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
                *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
                *overlay_mat = MeshMaterial3d(assets.ghost_turn_materials[ci].clone());
                show_overlay = true;
                ghost_target.0 = Vec3::ONE;
                if let Ok(mut target) = tile_scale_q.get_mut(entity) {
                    target.0 = Vec3::ZERO;
                    hidden_tile.0 = Some(entity);
                }
            } else {
                ghost_target.0 = Vec3::ZERO;
            }
        }
        Tool::Delete if can_delete => {
            ghost_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + DELETE_OVERLAY_OFFSET, world_z);
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.ghost_delete_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_delete_material.clone());
            ghost_target.0 = Vec3::ONE;
        }
        _ => { ghost_target.0 = Vec3::ZERO; }
    }

    overlay_tf.scale = if show_overlay { Vec3::ONE } else { Vec3::ZERO };
}

// === Tile Placement ===
pub fn handle_tile_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
    mut placed_sources: ResMut<PlacedSources>,
    play_mode: Res<PlayMode>,
    ghost_q: Query<&Transform, With<GhostPreview>>,
) {
    if *play_mode == PlayMode::Playing { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else { return; };
    let tile = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile else { return; };
    let ghost_scale = ghost_q.single().scale;
    // Free the color if overwriting a source tile
    if let TileKind::Source(ci, _) = kind {
        if selected_tool.0 != Tool::Source || inv_state.color_index != Some(*ci) {
            placed_sources.0.remove(ci);
        }
    }

    match (selected_tool.0, *kind) {
        (Tool::Floor, TileKind::Empty | TileKind::Source(_, _) | TileKind::Turn(_, _)) => {
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Floor, &assets, ghost_scale);
        }
        (Tool::Source, TileKind::Empty | TileKind::Floor | TileKind::Source(_, _) | TileKind::Turn(_, _)) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    placed_sources.0.insert(ci);
                    inv_state.last_placed_color = Some(ci);
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                    spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Source(ci, dir), &assets, ghost_scale);
                    let next = (1..NUM_COLORS)
                        .map(|offset| (ci + offset) % NUM_COLORS)
                        .find(|c| !placed_sources.0.contains(c));
                    inv_state.color_index = next;
                    if next.is_none() {
                        selected_tool.0 = Tool::Floor;
                    }
                }
            }
        }
        (Tool::Turn, TileKind::Empty | TileKind::Floor | TileKind::Source(_, _) | TileKind::Turn(_, _)) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                inv_state.last_placed_color = Some(ci);
                commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Turn(ci, dir), &assets, ghost_scale);
            }
        }
        (Tool::Delete, TileKind::Floor | TileKind::Source(_, _) | TileKind::Turn(_, _)) => {
            if let TileKind::Source(ci, _) = kind {
                if inv_state.level >= 2 {
                    inv_state.color_index = Some(*ci);
                }
            }
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
        _ => {}
    }
}
