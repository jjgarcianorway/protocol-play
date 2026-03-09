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
    placed_goals: Res<PlacedGoals>,
    placed_teleports: Res<PlacedTeleports>,
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

    hl_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + 0.01, world_z);
    hl_target.0 = Vec3::ONE;

    // Compute ghost mode: Some((rotation, optional overlay material)) for tile-placing tools
    let ghost_mode = match selected_tool.0 {
        Tool::Floor if !matches!(kind, TileKind::Floor) => Some((Quat::IDENTITY, None)),
        Tool::Source if !matches!(kind, TileKind::Source(_, _)) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) { Some((Quat::from_rotation_y(dir.rotation()), Some(assets.ghost_symbol_materials[ci].clone()))) }
                else { None }
            } else { None }
        }
        Tool::Goal if !matches!(kind, TileKind::Goal(_)) => {
            if let Some(ci) = inv_state.color_index {
                if !placed_goals.0.contains(&ci) { Some((Quat::IDENTITY, Some(assets.ghost_goal_materials[ci].clone()))) }
                else { None }
            } else { None }
        }
        Tool::Turn if !matches!(kind, TileKind::Turn(_, _)) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                Some((Quat::from_rotation_y(dir.rotation()), Some(assets.ghost_turn_materials[ci].clone())))
            } else { None }
        }
        Tool::TurnBut if !matches!(kind, TileKind::TurnBut(_, _)) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                Some((Quat::from_rotation_y(dir.rotation()), Some(assets.ghost_turnbut_materials[ci].clone())))
            } else { None }
        }
        Tool::Teleport if !matches!(kind, TileKind::Teleport(_)) => {
            if let Some(ci) = inv_state.color_index {
                if placed_teleports.0[ci] < 2 { Some((Quat::IDENTITY, Some(assets.ghost_teleport_materials[ci].clone()))) }
                else { None }
            } else { None }
        }
        Tool::Bounce if !matches!(kind, TileKind::Bounce(_)) => {
            if let Some(ci) = inv_state.color_index {
                Some((Quat::IDENTITY, Some(assets.ghost_bounce_materials[ci].clone())))
            } else { None }
        }
        Tool::BounceBut if !matches!(kind, TileKind::BounceBut(_)) => {
            if let Some(ci) = inv_state.color_index {
                Some((Quat::IDENTITY, Some(assets.ghost_bouncebot_materials[ci].clone())))
            } else { None }
        }
        _ => None,
    };

    if let Some((rotation, overlay_mat_opt)) = ghost_mode {
        ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
        ghost_tf.rotation = rotation;
        *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
        *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
        if let Some(mat) = overlay_mat_opt {
            *overlay_mat = MeshMaterial3d(mat);
            show_overlay = true;
        }
        ghost_target.0 = Vec3::ONE;
        if let Ok(mut target) = tile_scale_q.get_mut(entity) {
            target.0 = Vec3::ZERO;
            hidden_tile.0 = Some(entity);
        }
    } else if selected_tool.0 == Tool::Delete && !matches!(kind, TileKind::Empty) {
        ghost_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + DELETE_OVERLAY_OFFSET, world_z);
        ghost_tf.rotation = Quat::IDENTITY;
        *ghost_mesh = Mesh3d(assets.ghost_delete_mesh.clone());
        *ghost_mat = MeshMaterial3d(assets.ghost_delete_material.clone());
        ghost_target.0 = Vec3::ONE;
    } else {
        ghost_target.0 = Vec3::ZERO;
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
    mut placed_goals: ResMut<PlacedGoals>,
    mut placed_teleports: ResMut<PlacedTeleports>,
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
    // Free tracked resources when overwriting tiles
    if let TileKind::Source(ci, _) = kind {
        if selected_tool.0 != Tool::Source || inv_state.color_index != Some(*ci) {
            placed_sources.0.remove(ci);
        }
    }
    if let TileKind::Goal(ci) = kind {
        if selected_tool.0 != Tool::Goal || inv_state.color_index != Some(*ci) {
            placed_goals.0.remove(ci);
        }
    }
    if let TileKind::Teleport(num) = kind {
        if selected_tool.0 != Tool::Teleport || inv_state.color_index != Some(*num) {
            placed_teleports.0[*num] = placed_teleports.0[*num].saturating_sub(1);
        }
    }

    // Determine if placement target is valid (not same tile type)
    let not_self = |tool: Tool, kind: &TileKind| match tool {
        Tool::Floor => !matches!(kind, TileKind::Floor),
        Tool::Source => !matches!(kind, TileKind::Source(_, _)),
        Tool::Goal => !matches!(kind, TileKind::Goal(_)),
        Tool::Turn => !matches!(kind, TileKind::Turn(_, _)),
        Tool::TurnBut => !matches!(kind, TileKind::TurnBut(_, _)),
        Tool::Teleport => !matches!(kind, TileKind::Teleport(_)),
        Tool::Bounce => !matches!(kind, TileKind::Bounce(_)),
        Tool::BounceBut => !matches!(kind, TileKind::BounceBut(_)),
        Tool::Delete => !matches!(kind, TileKind::Empty),
    };
    if !not_self(selected_tool.0, kind) { return; }

    let despawn = |commands: &mut Commands, entity: Entity| {
        commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
    };

    match selected_tool.0 {
        Tool::Floor => {
            despawn(&mut commands, entity);
            spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Floor, &assets, ghost_scale);
        }
        Tool::Source => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    placed_sources.0.insert(ci);
                    inv_state.last_placed_color = Some(ci);
                    despawn(&mut commands, entity);
                    spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Source(ci, dir), &assets, ghost_scale);
                    let next = (1..NUM_COLORS).map(|o| (ci + o) % NUM_COLORS).find(|c| !placed_sources.0.contains(c));
                    inv_state.color_index = next;
                    if next.is_none() { selected_tool.0 = Tool::Floor; }
                }
            }
        }
        Tool::Goal => {
            if let Some(ci) = inv_state.color_index {
                if !placed_goals.0.contains(&ci) {
                    placed_goals.0.insert(ci);
                    inv_state.last_placed_color = Some(ci);
                    despawn(&mut commands, entity);
                    spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Goal(ci), &assets, ghost_scale);
                    let next = (1..NUM_COLORS).map(|o| (ci + o) % NUM_COLORS).find(|c| !placed_goals.0.contains(c));
                    inv_state.color_index = next;
                    if next.is_none() { selected_tool.0 = Tool::Floor; }
                }
            }
        }
        Tool::Turn | Tool::TurnBut => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                inv_state.last_placed_color = Some(ci);
                let tile = if selected_tool.0 == Tool::Turn { TileKind::Turn(ci, dir) } else { TileKind::TurnBut(ci, dir) };
                despawn(&mut commands, entity);
                spawn_tile_at_scale(&mut commands, col, row, board_size.0, tile, &assets, ghost_scale);
            }
        }
        Tool::Teleport => {
            if let Some(num) = inv_state.color_index {
                if placed_teleports.0[num] < 2 {
                    placed_teleports.0[num] += 1;
                    despawn(&mut commands, entity);
                    spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Teleport(num), &assets, ghost_scale);
                    let next = (1..NUM_TELEPORTS).map(|o| (num + o) % NUM_TELEPORTS).find(|n| placed_teleports.0[*n] < 2);
                    inv_state.color_index = next;
                    if next.is_none() { selected_tool.0 = Tool::Floor; }
                }
            }
        }
        Tool::Bounce | Tool::BounceBut => {
            if let Some(ci) = inv_state.color_index {
                inv_state.last_placed_color = Some(ci);
                let tile = if selected_tool.0 == Tool::Bounce { TileKind::Bounce(ci) } else { TileKind::BounceBut(ci) };
                despawn(&mut commands, entity);
                spawn_tile_at_scale(&mut commands, col, row, board_size.0, tile, &assets, ghost_scale);
            }
        }
        Tool::Delete => {
            if let TileKind::Source(ci, _) = kind {
                if inv_state.level >= 2 { inv_state.color_index = Some(*ci); }
            }
            if let TileKind::Goal(ci) = kind {
                placed_goals.0.remove(ci);
                if inv_state.level >= 2 { inv_state.color_index = Some(*ci); }
            }
            if let TileKind::Teleport(num) = kind {
                placed_teleports.0[*num] = placed_teleports.0[*num].saturating_sub(1);
                if inv_state.level >= 2 { inv_state.color_index = Some(*num); }
            }
            despawn(&mut commands, entity);
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
    }
}
