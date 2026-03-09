// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;

pub fn mark_button_interaction(
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<MarkButton>, Changed<Interaction>)>,
    mut border_q: Query<&mut BorderColor, With<MarkButton>>,
) {
    if !matches!(*play_mode, PlayMode::Marking) {
        if let Ok(mut b) = border_q.get_single_mut() {
            if b.0 != border_unsel() { *b = BorderColor(border_unsel()); }
        }
    }
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let (new_mode, color) = match *play_mode {
            PlayMode::Editing => (PlayMode::Marking, border_sel()),
            PlayMode::Marking => (PlayMode::Editing, border_unsel()),
            _ => continue,
        };
        *play_mode = new_mode;
        if let Ok(mut b) = border_q.get_single_mut() { *b = BorderColor(color); }
    }
}

pub fn handle_mark_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    play_mode: Res<PlayMode>,
    tiles: Query<(Entity, &TileCoord, &TileKind, Option<&InventoryMarker>, Option<&Children>),
        (With<Tile>, Without<DespawnAtZeroScale>)>,
    marker_visuals: Query<Entity, With<InventoryMarkerVisual>>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
) {
    if *play_mode != PlayMode::Marking { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else { return };
    let Some((entity, _, kind, has_marker, children)) = tiles.iter()
        .find(|(_, c, _, _, _)| c.col == col && c.row == row) else { return };
    if matches!(kind, TileKind::Empty | TileKind::Floor) { return; }

    if has_marker.is_some() {
        commands.entity(entity).remove::<InventoryMarker>();
        if let Some(ch) = children {
            for &c in ch.iter() { if marker_visuals.get(c).is_ok() { commands.entity(c).despawn(); } }
        }
    } else { add_marker(&mut commands, entity, &assets); }
}

// === Test mode helpers ===
fn tilekind_to_icon(kind: &TileKind, i: &InventoryIcons) -> Option<Handle<Image>> {
    match kind {
        TileKind::Source(ci, d) => Some(i.source_color_dir(*ci, *d)),
        TileKind::Goal(ci) => Some(i.goal_color(*ci)),
        TileKind::Turn(ci, d) => Some(i.turn_color_dir(*ci, *d)),
        TileKind::TurnBut(ci, d) => Some(i.turnbut_color_dir(*ci, *d)),
        TileKind::Teleport(n) => Some(i.teleport_num(*n)),
        TileKind::Bounce(ci) => Some(i.bounce_color(*ci)),
        TileKind::BounceBut(ci) => Some(i.bouncebot_color(*ci)),
        TileKind::Door(o) => Some(if *o { i.door_open.clone() } else { i.door_closed.clone() }),
        TileKind::Switch => Some(i.switch.clone()),
        TileKind::Painter(ci) => Some(i.painter_color(*ci)),
        TileKind::Arrow(ci, d) => Some(i.arrow_color_dir(*ci, *d)),
        TileKind::ArrowBut(ci, d) => Some(i.arrowbut_color_dir(*ci, *d)),
        _ => None,
    }
}

fn tile_sort_key(k: &TileKind) -> (u8, usize, u8) {
    match k {
        TileKind::Floor => (0, 0, 0), TileKind::Source(c, d) => (1, *c, d.index() as u8),
        TileKind::Goal(c) => (2, *c, 0), TileKind::Turn(c, d) => (3, *c, d.index() as u8),
        TileKind::TurnBut(c, d) => (4, *c, d.index() as u8), TileKind::Teleport(n) => (5, *n, 0),
        TileKind::Bounce(c) => (6, *c, 0), TileKind::BounceBut(c) => (7, *c, 0),
        TileKind::Door(o) => (8, if *o { 0 } else { 1 }, 0),
        TileKind::Switch => (9, 0, 0), TileKind::Painter(c) => (10, *c, 0),
        TileKind::Arrow(c, d) => (11, *c, d.index() as u8), TileKind::ArrowBut(c, d) => (12, *c, d.index() as u8),
        TileKind::Empty => (13, 0, 0),
    }
}

fn group_tiles(tiles: impl Iterator<Item = TileKind>) -> Vec<(TileKind, u8)> {
    let mut g: Vec<(TileKind, u8)> = Vec::new();
    for kind in tiles {
        if let Some(e) = g.iter_mut().find(|(k, _)| *k == kind) { e.1 += 1; }
        else { g.push((kind, 1)); }
    }
    g.sort_by(|(a, _), (b, _)| tile_sort_key(a).cmp(&tile_sort_key(b)));
    g
}

fn set_tool_from_kind(k: TileKind, tool: &mut ResMut<SelectedTool>, inv: &mut ResMut<InventoryState>) {
    let (t, dir, ci) = match k {
        TileKind::Source(c, d) => (Tool::Source, Some(d), Some(c)),
        TileKind::Goal(c) => (Tool::Goal, None, Some(c)),
        TileKind::Turn(c, d) => (Tool::Turn, Some(d), Some(c)),
        TileKind::TurnBut(c, d) => (Tool::TurnBut, Some(d), Some(c)),
        TileKind::Teleport(n) => (Tool::Teleport, None, Some(n)),
        TileKind::Bounce(c) => (Tool::Bounce, None, Some(c)),
        TileKind::BounceBut(c) => (Tool::BounceBut, None, Some(c)),
        TileKind::Door(o) => (Tool::Door, None, Some(if o { 0 } else { 1 })),
        TileKind::Switch => (Tool::Switch, None, None),
        TileKind::Painter(c) => (Tool::Painter, None, Some(c)),
        TileKind::Arrow(c, d) => (Tool::Arrow, Some(d), Some(c)),
        TileKind::ArrowBut(c, d) => (Tool::ArrowBut, Some(d), Some(c)),
        _ => return,
    };
    tool.0 = t; inv.direction = dir; inv.color_index = ci;
}

fn add_marker(commands: &mut Commands, entity: Entity, assets: &GameAssets) {
    commands.entity(entity).insert(InventoryMarker).with_children(|parent| {
        parent.spawn((
            Mesh3d(assets.marker_mesh.clone()), MeshMaterial3d(assets.marker_material.clone()),
            Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0)),
            InventoryMarkerVisual,
        ));
    });
}

// === Test mode enter/exit ===
pub fn test_button_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<TestButton>, Changed<Interaction>)>,
    tiles: Query<(Entity, &TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut saved_board: ResMut<SavedBoardState>,
    mut saved_test: ResMut<SavedTestState>,
    mut test_inv: ResMut<TestInventory>,
    placed_teleports: Res<PlacedTeleports>,
    inv_state: Res<InventoryState>,
    selected_tool: Res<SelectedTool>,
    icons: Res<InventoryIcons>,
    inv_container: Query<Entity, With<InventoryContainer>>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if *play_mode != PlayMode::Editing && *play_mode != PlayMode::Marking { continue; }

        saved_board.tiles.clear();
        for (_, c, k, m) in &tiles { saved_board.tiles.push((c.col, c.row, *k, m.is_some())); }
        saved_board.placed_teleports = placed_teleports.0;
        saved_board.inv_state = inv_state.clone();
        saved_board.selected_tool = selected_tool.0;
        let marked = tiles.iter().filter(|(_, _, _, m)| m.is_some()).map(|(_, _, k, _)| *k);
        test_inv.items = group_tiles(marked);
        test_inv.selected = None; test_inv.remove_mode = false;
        saved_test.tiles.clear(); saved_test.inventory = test_inv.items.clone();
        for (_, c, k, m) in &tiles {
            saved_test.tiles.push((c.col, c.row, if m.is_some() { TileKind::Empty } else { *k }));
        }
        for (e, _, _, _) in &tiles { commands.entity(e).despawn_recursive(); }
        for &(col, row, kind) in &saved_test.tiles {
            spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        }
        if let Ok(c) = inv_container.get_single() { commands.entity(c).insert(UiBottomAnim { target: INV_SLIDE_HIDE, despawn_at_target: false }); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, true);
        spawn_test_banner(&mut commands);
        *play_mode = PlayMode::TestEditing;
    }
}

fn spawn_test_banner(commands: &mut Commands) {
    commands.spawn((
        Node { position_type: PositionType::Absolute, top: Val::Px(BANNER_SLIDE_HIDE), width: Val::Percent(100.0),
            height: Val::Px(BANNER_HEIGHT), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, ..default() },
        BackgroundColor(rgba(TEST_BANNER_BG)),
        TestModeBanner, UiTopAnim { target: 0.0, despawn_at_target: false },
    )).with_child((Text::new("TEST MODE"), TextFont { font_size: DIALOG_TITLE_FONT, ..default() },
        TextColor(rgb(TEST_BANNER_TEXT))));
}

fn spawn_test_inventory(commands: &mut Commands, test_inv: &TestInventory, icons: &InventoryIcons, animate: bool) {
    let (tf, tc) = (TextFont { font_size: LABEL_FONT, ..default() }, TextColor(Color::WHITE));
    let btn_pad = text_btn_node();
    let start_bottom = if animate { INV_SLIDE_HIDE } else { INV_SLIDE_SHOW };
    let mut ec = commands.spawn((
        Node { position_type: PositionType::Absolute, bottom: Val::Px(start_bottom),
            width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
        TestInventoryContainer,
    ));
    if animate { ec.insert(UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false }); }
    ec.with_children(|parent| {
        parent.spawn((
            Node { flex_direction: FlexDirection::Row, padding: UiRect::all(Val::Vw(INVENTORY_PAD_VW)),
                column_gap: Val::Vw(INVENTORY_GAP_VW), align_items: AlignItems::Center, ..default() },
            BackgroundColor(rgba(TEST_INVENTORY_BG)),
        )).with_children(|c| {
            let mut rb = btn_pad.clone(); rb.margin = UiRect::right(Val::Px(BTN_SIDE_MARGIN));
            c.spawn((Button, ResetTestButton, rb, BackgroundColor(btn_bg())))
                .with_child((Text::new("Reset"), tf.clone(), tc));
            let sn = slot_node();
            for (i, (kind, count)) in test_inv.items.iter().enumerate() {
                let Some(icon) = tilekind_to_icon(kind, icons) else { continue };
                let sel = !test_inv.remove_mode && test_inv.selected == Some(i);
                c.spawn((Button, TestInventorySlot(i), sn.clone(),
                    BackgroundColor(slot_bg()), border_for(sel),
                )).with_children(|slot| {
                    slot.spawn((icon_node(), ImageNode::new(icon)));
                    let cc = if *count > 0 { rgb(COUNT_AVAIL_COLOR) } else { rgb(COUNT_EMPTY_COLOR) };
                    slot.spawn((Text::new(format!("x{count}")),
                        TextFont { font_size: COUNT_FONT, ..default() }, TextColor(cc)));
                });
            }
            // Remove tool (pick up tiles)
            c.spawn((Button, TestInventorySlot(usize::MAX), sn,
                BackgroundColor(slot_bg()), border_for(test_inv.remove_mode),
            )).with_child((icon_node(), ImageNode::new(icons.delete.clone())));
            let mut sb = btn_pad; sb.margin = UiRect::left(Val::Px(BTN_SIDE_MARGIN));
            c.spawn((Button, StopTestButton, sb, BackgroundColor(rgb(STOP_TEST_BTN_BG))))
                .with_child((Text::new("Stop Test"), tf, tc));
        });
    });
}

pub fn stop_test_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<StopTestButton>, Changed<Interaction>)>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    saved_board: Res<SavedBoardState>,
    mut placed_teleports: ResMut<PlacedTeleports>,
    mut inv_state: ResMut<InventoryState>,
    mut selected_tool: ResMut<SelectedTool>,
    inv_container: Query<Entity, With<InventoryContainer>>,
    test_container: Query<Entity, With<TestInventoryContainer>>,
    mut sim_result: ResMut<crate::simulation::SimulationResult>,
    banner: Query<Entity, With<TestModeBanner>>,
) {
    let te = *play_mode == PlayMode::TestEditing;
    let success_exit = sim_result.test_success_exit && te;
    let btn = interaction_query.iter().any(|i| *i == Interaction::Pressed) && te;
    if !success_exit && !btn { return; }
    sim_result.test_success_exit = false;
    for entity in &tiles { commands.entity(entity).despawn_recursive(); }
    for &(col, row, kind, marked) in &saved_board.tiles {
        let e = spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        if marked { add_marker(&mut commands, e, &assets); }
    }
    placed_teleports.0 = saved_board.placed_teleports;
    *inv_state = saved_board.inv_state.clone();
    selected_tool.0 = saved_board.selected_tool;
    if let Ok(c) = inv_container.get_single() { commands.entity(c).insert(UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false }); }
    for e in &test_container { commands.entity(e).insert(UiBottomAnim { target: INV_SLIDE_HIDE, despawn_at_target: true }); }
    for e in &banner { commands.entity(e).insert(UiTopAnim { target: BANNER_SLIDE_HIDE, despawn_at_target: true }); }
    *play_mode = PlayMode::Editing;
}

pub fn reset_test_interaction(
    mut commands: Commands,
    play_mode: Res<PlayMode>,
    interaction_query: Query<&Interaction, (With<ResetTestButton>, Changed<Interaction>)>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    saved_test: Res<SavedTestState>,
    mut test_inv: ResMut<TestInventory>,
    icons: Res<InventoryIcons>,
    test_container: Query<Entity, With<TestInventoryContainer>>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if *play_mode != PlayMode::TestEditing { continue; }
        for entity in &tiles { commands.entity(entity).despawn_recursive(); }
        for &(col, row, kind) in &saved_test.tiles {
            spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        }
        test_inv.items = saved_test.inventory.clone();
        test_inv.selected = None;
        test_inv.remove_mode = false;
        for e in &test_container { commands.entity(e).despawn_recursive(); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, false);
    }
}

pub fn sync_editor_buttons_visibility(
    play_mode: Res<PlayMode>,
    mut mark_q: Query<&mut Visibility, With<MarkButton>>,
    mut test_q: Query<&mut Visibility, (With<TestButton>, Without<MarkButton>)>,
    mut board_q: Query<&mut Visibility, (With<BoardButton>, Without<MarkButton>, Without<TestButton>)>,
    mut save_q: Query<&mut Visibility, (With<SaveButton>, Without<MarkButton>, Without<TestButton>, Without<BoardButton>)>,
    mut load_q: Query<&mut Visibility, (With<LoadButton>, Without<MarkButton>, Without<TestButton>, Without<BoardButton>, Without<SaveButton>)>,
) {
    let visible = matches!(*play_mode, PlayMode::Editing | PlayMode::Marking);
    let vis = if visible { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut mark_q { *v = vis; }
    for mut v in &mut test_q { *v = vis; }
    for mut v in &mut board_q { *v = vis; }
    for mut v in &mut save_q { *v = vis; }
    for mut v in &mut load_q { *v = vis; }
}

// === Test mode tile placement ===
pub fn test_inventory_interaction(
    mut test_inv: ResMut<TestInventory>,
    slots: Query<(&Interaction, &TestInventorySlot), Changed<Interaction>>,
    mut border_q: Query<(&mut BorderColor, &TestInventorySlot)>,
    play_mode: Res<PlayMode>,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
) {
    if *play_mode != PlayMode::TestEditing { return; }
    let mut clicked = None;
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed { clicked = Some(slot.0); }
    }
    let Some(idx) = clicked else { return };
    if idx == usize::MAX {
        test_inv.remove_mode = !test_inv.remove_mode;
        if test_inv.remove_mode { test_inv.selected = None; selected_tool.0 = Tool::Delete; }
    } else if idx < test_inv.items.len() && test_inv.items[idx].1 > 0 {
        test_inv.remove_mode = false; test_inv.selected = Some(idx);
        set_tool_from_kind(test_inv.items[idx].0, &mut selected_tool, &mut inv_state);
    } else { return; }
    for (mut border, slot) in &mut border_q {
        let sel = (test_inv.remove_mode && slot.0 == usize::MAX)
            || (!test_inv.remove_mode && test_inv.selected == Some(slot.0));
        *border = border_for(sel);
    }
}

pub fn handle_test_tile_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    play_mode: Res<PlayMode>,
    mut test_inv: ResMut<TestInventory>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
    icons: Res<InventoryIcons>,
    test_container: Query<Entity, With<TestInventoryContainer>>,
    ghost_q: Query<&Transform, With<GhostPreview>>,
) {
    if *play_mode != PlayMode::TestEditing { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else { return };
    let Some((entity, _, kind)) = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row) else { return };

    if test_inv.remove_mode {
        // Remove mode: pick up non-empty/non-floor tiles back to inventory
        if matches!(kind, TileKind::Empty | TileKind::Floor) { return; }
        if let Some(entry) = test_inv.items.iter_mut().find(|(k, _)| *k == *kind) {
            entry.1 += 1;
        } else {
            test_inv.items.push((*kind, 1));
            test_inv.items.sort_by(|(a, _), (b, _)| tile_sort_key(a).cmp(&tile_sort_key(b)));
        }
        commands.entity(entity).despawn_recursive();
        spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        for e in &test_container { commands.entity(e).despawn_recursive(); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, false);
    } else if let Some(idx) = test_inv.selected {
        if idx >= test_inv.items.len() { return; }
        let (tile_kind, count) = test_inv.items[idx];
        if count == 0 || !matches!(kind, TileKind::Empty) { return; }
        let ghost_scale = ghost_q.get_single().map(|t| t.scale).unwrap_or(Vec3::ZERO);
        commands.entity(entity).despawn_recursive();
        crate::board::spawn_tile_at_scale(&mut commands, col, row, board_size.0, tile_kind, &assets, ghost_scale);
        test_inv.items[idx].1 -= 1;
        if test_inv.items[idx].1 == 0 {
            test_inv.items.remove(idx);
            if test_inv.items.is_empty() { test_inv.selected = None; }
            else { test_inv.selected = Some(idx.min(test_inv.items.len() - 1)); }
        }
        for e in &test_container { commands.entity(e).despawn_recursive(); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, false);
    }
}
