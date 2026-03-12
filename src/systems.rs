// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::*;

// === Animation ===
pub fn animate_scale(
    mut query: Query<(&mut Transform, &TargetScale, Option<&GhostPreview>, Option<&TileHighlight>, Option<&GhostTrail>)>,
    time: Res<Time>,
) {
    for (mut transform, target, ghost, highlight, trail) in &mut query {
        let speed = if ghost.is_some() || highlight.is_some() || trail.is_some() { HOVER_ANIM_SPEED } else { ANIM_SPEED };
        transform.scale = transform.scale.lerp(target.0, speed * time.delta_secs());
        if transform.scale.distance(target.0) < SCALE_SNAP { transform.scale = target.0; }
    }
}

pub fn animate_node_width(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Node, &NodeWidthAnim, Option<&mut BackgroundColor>)>,
    time: Res<Time>,
) {
    for (entity, mut node, anim, bg) in &mut query {
        let current = match node.width {
            Val::Vw(w) | Val::Px(w) => w,
            _ => anim.target,
        };
        let new_w = current + (anim.target - current) * UI_ANIM_SPEED * time.delta_secs();
        if (new_w - anim.target).abs() < WIDTH_SNAP {
            if anim.despawn_at_zero && anim.target < 0.1 {
                commands.entity(entity).despawn_recursive();
            } else {
                node.width = Val::Vw(anim.target);
                if let Some(mut bg) = bg { bg.0.set_alpha(1.0); }
                commands.entity(entity).remove::<NodeWidthAnim>();
            }
        } else {
            node.width = Val::Vw(new_w);
            if anim.target > 0.1 {
                if let Some(mut bg) = bg { bg.0.set_alpha((new_w / anim.target).clamp(0.0, 1.0)); }
            }
        }
    }
}

pub fn animate_ui_slides(
    time: Res<Time>, mut commands: Commands,
    mut bq: Query<(Entity, &mut Node, &UiBottomAnim), Without<ExpHeightAnim>>,
    mut tq: Query<(Entity, &mut Node, &UiTopAnim), (Without<UiBottomAnim>, Without<ExpHeightAnim>)>,
    mut fq: Query<(Entity, &mut BackgroundColor, &UiBgFade)>,
    mut hq: Query<(Entity, &mut Node, &ExpHeightAnim), (Without<UiBottomAnim>, Without<UiTopAnim>)>,
) {
    let s = time.delta_secs() * UI_ANIM_SPEED;
    macro_rules! slide { ($q:expr, $field:ident, $comp:ty) => {
        for (e, mut n, a) in &mut $q {
            let c = match n.$field { Val::Px(v) => v, _ => continue };
            let d = a.target - c;
            if d.abs() < SLIDE_SNAP {
                n.$field = Val::Px(a.target); commands.entity(e).remove::<$comp>();
                if a.despawn_at_target { commands.entity(e).despawn_recursive(); }
            } else { n.$field = Val::Px(c + d * s); }
        }
    }}
    slide!(bq, bottom, UiBottomAnim);
    slide!(tq, top, UiTopAnim);
    for (e, mut n, a) in &mut hq {
        let c = match n.height { Val::Vw(v) => v, Val::Px(v) => { n.height = Val::Vw(0.0); v }, _ => continue };
        let d = a.target - c;
        if d.abs() < 0.05 { n.height = Val::Vw(a.target); commands.entity(e).remove::<ExpHeightAnim>(); }
        else { n.height = Val::Vw(c + d * s); }
    }
    for (e, mut bg, a) in &mut fq {
        let c = bg.0.alpha(); let d = a.target - c;
        if d.abs() < FADE_SNAP {
            bg.0.set_alpha(a.target); commands.entity(e).remove::<UiBgFade>();
            if a.despawn_at_zero && a.target < FADE_SNAP { commands.entity(e).despawn_recursive(); }
        } else { bg.0.set_alpha(c + d * s); }
    }
}

pub fn animate_border_fade(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BorderColor, &BorderFade)>,
    time: Res<Time>,
) {
    for (entity, mut border, fade) in &mut query {
        let c = border.0.to_srgba();
        let t = fade.target;
        let s = fade.speed * time.delta_secs();
        let nr = c.red + (t[0] - c.red) * s;
        let ng = c.green + (t[1] - c.green) * s;
        let nb = c.blue + (t[2] - c.blue) * s;
        let na = c.alpha + (t[3] - c.alpha) * s;
        if (nr - t[0]).abs() < 0.01 && (na - t[3]).abs() < 0.01 {
            border.0 = Color::srgba(t[0], t[1], t[2], t[3]);
            commands.entity(entity).remove::<BorderFade>();
        } else {
            border.0 = Color::srgba(nr, ng, nb, na);
        }
    }
}

pub fn cleanup_despawned(mut commands: Commands, query: Query<(Entity, &Transform), With<DespawnAtZeroScale>>) {
    for (entity, transform) in &query {
        if transform.scale.length() < DESPAWN_SCALE {
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
    ui_interactions: Query<&Interaction>,
    play_mode: Res<PlayMode>,
) {
    let can_hover = matches!(*play_mode, PlayMode::Editing | PlayMode::Marking | PlayMode::TestEditing);
    if !can_hover { hovered.0 = None; return; }
    if ui_interactions.iter().any(|i| *i != Interaction::None) { hovered.0 = None; return; }
    let Ok(window) = windows.get_single() else { hovered.0 = None; return; };
    let Ok((camera, cam_transform)) = cameras.get_single() else { hovered.0 = None; return; };
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
        (&mut Transform, &Mesh3d, &mut MeshMaterial3d<StandardMaterial>),
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
    play_mode: Res<PlayMode>,
    saved_test: Res<SavedTestState>,
) {
    // Restore previous suppressed tile
    if let Some(old_entity) = hidden_tile.0.take() {
        if let Ok(mut target) = tile_scale_q.get_mut(old_entity) { target.0 = Vec3::ONE; }
    }

    let (mut ghost_tf, mut ghost_target, mut ghost_mesh, mut ghost_mat) = ghost_q.single_mut();
    let (mut overlay_tf, overlay_mesh_3d, mut overlay_mat) = ghost_overlay_q.single_mut();
    let (mut hl_tf, mut hl_target) = highlight_q.single_mut();
    let mut show_overlay = false;

    macro_rules! hide_ghost { () => {
        ghost_target.0 = Vec3::ZERO; overlay_tf.scale = Vec3::ZERO;
        hl_target.0 = Vec3::ZERO; ghost_cell.current = None;
    }}
    // No ghost/highlight in marking mode or during play
    if matches!(*play_mode, PlayMode::Marking | PlayMode::Playing | PlayMode::TestPlaying) {
        hide_ghost!(); return;
    }
    let Some((col, row)) = hovered.0 else {
        hide_ghost!(); return;
    };
    let tile_info = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile_info else { hide_ghost!(); return; };

    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let world_x = col as f32 - offset;
    let world_z = row as f32 - offset;

    // Clear last_placed when mouse moves to a different cell
    if ghost_cell.last_placed == Some((col, row)) {
        hide_ghost!(); return;
    } else { ghost_cell.last_placed = None; }

    if ghost_cell.current != Some((col, row)) {
        // Spawn fade-out trails at the old position so animations finish smoothly
        if ghost_tf.scale.length() > DESPAWN_SCALE {
            let mut trail = commands.spawn((
                Mesh3d(ghost_mesh.0.clone()), MeshMaterial3d(ghost_mat.0.clone()),
                Transform { translation: ghost_tf.translation, rotation: ghost_tf.rotation, scale: ghost_tf.scale },
                TargetScale(Vec3::ZERO), DespawnAtZeroScale, GhostTrail,
            ));
            if overlay_tf.scale.length() > 0.5 {
                trail.with_children(|p| { p.spawn((
                    Mesh3d(overlay_mesh_3d.0.clone()), MeshMaterial3d(overlay_mat.0.clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                )); });
            }
        }
        if hl_tf.scale.length() > DESPAWN_SCALE {
            commands.spawn((
                Mesh3d(assets.highlight_mesh.clone()), MeshMaterial3d(assets.highlight_material.clone()),
                Transform { translation: hl_tf.translation, scale: hl_tf.scale, ..default() },
                TargetScale(Vec3::ZERO), DespawnAtZeroScale, GhostTrail,
            ));
        }
        ghost_tf.scale = Vec3::ZERO; hl_tf.scale = Vec3::ZERO;
        ghost_cell.current = Some((col, row));
    }

    // In test mode, check if tile was originally Empty (player-placed) vs level-designed
    let in_test = matches!(*play_mode, PlayMode::TestEditing);
    let is_original = in_test && !saved_test.tiles.iter()
        .any(|&(c, r, k)| c == col && r == row && matches!(k, TileKind::Empty));

    // Hide ghost/highlight on level-designed tiles in test mode (not interactive)
    if in_test && is_original {
        hl_target.0 = Vec3::ZERO; ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO; return;
    }
    // In test delete mode, also hide on empty tiles (nothing to remove)
    if in_test && selected_tool.0 == Tool::Delete && matches!(kind, TileKind::Empty) {
        hl_target.0 = Vec3::ZERO; ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO; return;
    }
    hl_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + HIGHLIGHT_Y_OFFSET, world_z); hl_target.0 = Vec3::ONE;

    // Compute ghost mode: Some((rotation, optional overlay material)) for tile-placing tools
    let dir_ci = || -> Option<(Direction, usize)> { Some((inv_state.direction?, inv_state.color_index?)) };
    let ci = inv_state.color_index;
    let rot_mat = |mats: &[Handle<StandardMaterial>]| dir_ci().map(|(d, ci)| (Quat::from_rotation_y(d.rotation()), Some(mats[ci].clone())));
    let id_mat = |mats: &[Handle<StandardMaterial>]| ci.map(|ci| (Quat::IDENTITY, Some(mats[ci].clone())));
    let dc = dir_ci();
    let exact = match (selected_tool.0, kind) {
        (Tool::Floor, TileKind::Floor) | (Tool::Switch, TileKind::Switch) => true,
        (Tool::Source, TileKind::Source(c, d)) | (Tool::Turn, TileKind::Turn(c, d))
        | (Tool::TurnBut, TileKind::TurnBut(c, d)) | (Tool::Arrow, TileKind::Arrow(c, d))
        | (Tool::ArrowBut, TileKind::ArrowBut(c, d)) => dc == Some((*d, *c)),
        (Tool::Goal, TileKind::Goal(c))
        | (Tool::Bounce, TileKind::Bounce(c)) | (Tool::BounceBut, TileKind::BounceBut(c))
        | (Tool::ColorSwitch, TileKind::ColorSwitch(c)) | (Tool::ColorSwitchBut, TileKind::ColorSwitchBut(c))
        | (Tool::Painter, TileKind::Painter(c)) => ci == Some(*c),
        (Tool::Door, TileKind::Door(o)) => ci == Some(if *o { 0 } else { 1 }),
        _ => false,
    };
    let ghost_mode = if exact { None } else { match selected_tool.0 {
        Tool::Floor if !matches!(kind, TileKind::Floor) => Some((Quat::IDENTITY, None)),
        Tool::Source => rot_mat(&assets.ghost_symbol_materials),
        Tool::Goal => id_mat(&assets.ghost_goal_materials),
        Tool::Turn => rot_mat(&assets.ghost_turn_materials),
        Tool::TurnBut => rot_mat(&assets.ghost_turnbut_materials),
        Tool::Arrow => rot_mat(&assets.ghost_arrow_materials),
        Tool::ArrowBut => rot_mat(&assets.ghost_arrowbut_materials),
        Tool::Teleport => ci.and_then(|ci| {
            next_teleport_number(tiles.iter().map(|(_, _, k)| *k), false, ci)
                .map(|n| (Quat::IDENTITY, Some(assets.ghost_teleport_materials[n * NUM_TELEPORT_COLORS + ci].clone())))
        }),
        Tool::TeleportBut => ci.and_then(|ci| {
            next_teleport_number(tiles.iter().map(|(_, _, k)| *k), true, ci)
                .map(|n| (Quat::IDENTITY, Some(assets.ghost_teleportbut_materials[n * NUM_COLORS + ci].clone())))
        }),
        Tool::Bounce => id_mat(&assets.ghost_bounce_materials),
        Tool::BounceBut => id_mat(&assets.ghost_bouncebot_materials),
        Tool::Door => ci.map(|s| (Quat::IDENTITY,
            Some(if s == 0 { assets.ghost_door_open_material.clone() } else { assets.ghost_door_closed_material.clone() }))),
        Tool::Switch => Some((Quat::IDENTITY, Some(assets.ghost_switch_material.clone()))),
        Tool::ColorSwitch => id_mat(&assets.ghost_colorswitch_materials),
        Tool::ColorSwitchBut => id_mat(&assets.ghost_colorswitchbut_materials),
        Tool::Painter => id_mat(&assets.ghost_painter_materials),
        _ => None,
    }};

    if let Some((rotation, overlay_mat_opt)) = ghost_mode {
        ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
        ghost_tf.rotation = rotation;
        *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
        *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
        if let Some(mat) = overlay_mat_opt { *overlay_mat = MeshMaterial3d(mat); show_overlay = true; }
        ghost_target.0 = Vec3::ONE;
        if let Ok(mut target) = tile_scale_q.get_mut(entity) { target.0 = Vec3::ZERO; hidden_tile.0 = Some(entity); }
    } else if selected_tool.0 == Tool::Delete && !matches!(kind, TileKind::Empty) {
        ghost_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + DELETE_OVERLAY_OFFSET, world_z);
        ghost_tf.rotation = Quat::IDENTITY;
        *ghost_mesh = Mesh3d(assets.ghost_delete_mesh.clone());
        *ghost_mat = MeshMaterial3d(assets.ghost_delete_material.clone());
        ghost_target.0 = Vec3::ONE;
    } else { ghost_target.0 = Vec3::ZERO; }
    overlay_tf.scale = if show_overlay { Vec3::ONE } else { Vec3::ZERO };
}

// === Tile Placement ===
pub fn handle_tile_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    selected_tool: Res<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction>,
    play_mode: Res<PlayMode>,
    ghost_q: Query<&Transform, With<GhostPreview>>,
    mut validated: ResMut<LevelValidated>,
    mut ghost_cell: ResMut<GhostCell>,
) {
    if *play_mode != PlayMode::Editing || !mouse.just_pressed(MouseButton::Left) { return; }
    if ui_interactions.iter().any(|i| *i != Interaction::None) { return; }
    let Some((col, row)) = hovered.0 else { return; };
    let Some((entity, _, kind)) = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row) else { return; };
    let ghost_scale = ghost_q.single().scale;
    let same = matches!((selected_tool.0, kind), (Tool::Floor, TileKind::Floor) | (Tool::Switch, TileKind::Switch) | (Tool::Delete, TileKind::Empty))
        || matches!((selected_tool.0, kind, inv_state.color_index), (Tool::ColorSwitch, TileKind::ColorSwitch(ci), Some(sel)) if *ci == sel)
        || matches!((selected_tool.0, kind, inv_state.color_index), (Tool::ColorSwitchBut, TileKind::ColorSwitchBut(ci), Some(sel)) if *ci == sel);
    if same { return; }
    validated.0 = false;
    macro_rules! dp { () => { commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale)); } }
    macro_rules! place { ($k:expr) => { dp!(); spawn_tile_at_scale(&mut commands, col, row, board_size.0, $k, &assets, ghost_scale); } }
    match selected_tool.0 {
        Tool::Floor => { place!(TileKind::Floor); }
        Tool::Source => { if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
            inv_state.last_placed_color = Some(ci); place!(TileKind::Source(ci, dir));
        }}
        Tool::Goal => { if let Some(ci) = inv_state.color_index {
            inv_state.last_placed_color = Some(ci); place!(TileKind::Goal(ci));
        }}
        Tool::Turn | Tool::TurnBut | Tool::Arrow | Tool::ArrowBut => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                inv_state.last_placed_color = Some(ci);
                let tile = match selected_tool.0 { Tool::Turn => TileKind::Turn(ci, dir), Tool::TurnBut => TileKind::TurnBut(ci, dir), Tool::Arrow => TileKind::Arrow(ci, dir), _ => TileKind::ArrowBut(ci, dir) };
                place!(tile);
            }
        }
        Tool::Teleport | Tool::TeleportBut => { if let Some(ci) = inv_state.color_index {
            let is_but = selected_tool.0 == Tool::TeleportBut;
            if let Some(num) = next_teleport_number(tiles.iter().map(|(_, _, k)| *k), is_but, ci) {
                inv_state.last_placed_color = Some(ci);
                place!(if is_but { TileKind::TeleportBut(ci, num) } else { TileKind::Teleport(ci, num) });
            }
        }}
        Tool::Bounce | Tool::BounceBut => { if let Some(ci) = inv_state.color_index {
            inv_state.last_placed_color = Some(ci);
            place!(if selected_tool.0 == Tool::Bounce { TileKind::Bounce(ci) } else { TileKind::BounceBut(ci) });
        }}
        Tool::Door => { if let Some(s) = inv_state.color_index { place!(TileKind::Door(s == 0)); } }
        Tool::Switch => { place!(TileKind::Switch); }
        Tool::ColorSwitch | Tool::ColorSwitchBut => { if let Some(ci) = inv_state.color_index {
            inv_state.last_placed_color = Some(ci);
            place!(if selected_tool.0 == Tool::ColorSwitch { TileKind::ColorSwitch(ci) } else { TileKind::ColorSwitchBut(ci) });
        }}
        Tool::Painter => { if let Some(ci) = inv_state.color_index {
            inv_state.last_placed_color = Some(ci); place!(TileKind::Painter(ci));
        }}
        Tool::Delete => {
            if let TileKind::Source(ci, _) | TileKind::Goal(ci) = kind { if inv_state.level >= 2 { inv_state.color_index = Some(*ci); } }
            if let TileKind::Teleport(ci, _) | TileKind::TeleportBut(ci, _) = kind { if inv_state.level >= 2 { inv_state.color_index = Some(*ci); } }
            dp!(); spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
    }
    ghost_cell.last_placed = Some((col, row));
}

pub fn sync_inventory_play_mode(
    play_mode: Res<PlayMode>,
    mut inv: Query<&mut UiBottomAnim, With<InventoryContainer>>,
) {
    if !play_mode.is_changed() { return; }
    let target = match *play_mode {
        PlayMode::Playing => INV_SLIDE_HIDE,
        PlayMode::Editing => INV_SLIDE_SHOW,
        _ => return,
    };
    for mut anim in &mut inv { anim.target = target; }
}
