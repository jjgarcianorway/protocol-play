// SPDX-License-Identifier: GPL-3.0-or-later
use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;

pub fn mark_button_interaction(
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<MarkButton>, Changed<Interaction>)>,
    mut mark_q: Query<(&mut BorderColor, &mut BackgroundColor), With<MarkButton>>,
) {
    if !matches!(*play_mode, PlayMode::Marking) {
        if let Ok((mut b, mut bg)) = mark_q.get_single_mut() { if b.0 != border_unsel() { *b = BorderColor(border_unsel()); bg.0 = btn_bg(); } }
    }
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let (nm, bc, bgc) = match *play_mode {
            PlayMode::Editing => (PlayMode::Marking, border_sel(), rgb(MARK_ACTIVE_BG)),
            PlayMode::Marking => (PlayMode::Editing, border_unsel(), btn_bg()),
            _ => continue,
        };
        *play_mode = nm;
        if let Ok((mut b, mut bg)) = mark_q.get_single_mut() { *b = BorderColor(bc); bg.0 = bgc; }
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
    if matches!(kind, TileKind::Empty | TileKind::Floor | TileKind::Source(_, _) | TileKind::Goal(_)) { return; }

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
        TileKind::Teleport(ci, _) => Some(i.teleport_color(*ci)),
        TileKind::TeleportBut(ci, _) => Some(i.teleportbut_color(*ci)),
        TileKind::Bounce(ci) => Some(i.bounce_color(*ci)),
        TileKind::BounceBut(ci) => Some(i.bouncebot_color(*ci)),
        TileKind::Door(o) => Some(if *o { i.door_open.clone() } else { i.door_closed.clone() }),
        TileKind::Switch => Some(i.switch_color(NUM_COLORS)),
        TileKind::ColorSwitch(ci) => Some(i.switch_color(*ci)),
        TileKind::ColorSwitchBut(ci) => Some(i.switchbut_color(*ci)),
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
        TileKind::TurnBut(c, d) => (4, *c, d.index() as u8),
        TileKind::Teleport(c, n) => (5, c * 10 + *n, 0), TileKind::TeleportBut(c, n) => (6, c * 10 + *n, 0),
        TileKind::Bounce(c) => (7, *c, 0), TileKind::BounceBut(c) => (8, *c, 0),
        TileKind::Door(o) => (9, if *o { 0 } else { 1 }, 0),
        TileKind::Switch => (10, 0, 0), TileKind::ColorSwitch(c) => (11, *c, 0),
        TileKind::ColorSwitchBut(c) => (12, *c, 0), TileKind::Painter(c) => (13, *c, 0),
        TileKind::Arrow(c, d) => (14, *c, d.index() as u8), TileKind::ArrowBut(c, d) => (15, *c, d.index() as u8),
        TileKind::Empty => (16, 0, 0),
    }
}

pub fn group_tiles(tiles: impl Iterator<Item = TileKind>) -> Vec<(TileKind, u8)> {
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
        TileKind::Teleport(c, _) => (Tool::Teleport, None, Some(c)),
        TileKind::TeleportBut(c, _) => (Tool::TeleportBut, None, Some(c)),
        TileKind::Bounce(c) => (Tool::Bounce, None, Some(c)), TileKind::BounceBut(c) => (Tool::BounceBut, None, Some(c)),
        TileKind::Door(o) => (Tool::Door, None, Some(if o { 0 } else { 1 })), TileKind::Switch => (Tool::Switch, None, None),
        TileKind::ColorSwitch(c) => (Tool::ColorSwitch, None, Some(c)), TileKind::ColorSwitchBut(c) => (Tool::ColorSwitchBut, None, Some(c)),
        TileKind::Painter(c) => (Tool::Painter, None, Some(c)),
        TileKind::Arrow(c, d) => (Tool::Arrow, Some(d), Some(c)), TileKind::ArrowBut(c, d) => (Tool::ArrowBut, Some(d), Some(c)),
        _ => return,
    }; tool.0 = t; inv.direction = dir; inv.color_index = ci;
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
    inv_state: Res<InventoryState>,
    selected_tool: Res<SelectedTool>,
    icons: Res<InventoryIcons>,
    font: Res<GameFont>,
    inv_container: Query<Entity, With<InventoryContainer>>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if *play_mode != PlayMode::Editing && *play_mode != PlayMode::Marking { continue; }

        saved_board.tiles.clear();
        for (_, c, k, m) in &tiles { saved_board.tiles.push((c.col, c.row, *k, m.is_some())); }
        saved_board.inv_state = inv_state.clone(); saved_board.selected_tool = selected_tool.0;
        let marked = tiles.iter().filter(|(_, _, _, m)| m.is_some()).map(|(_, _, k, _)| *k);
        test_inv.items = group_tiles(marked);
        test_inv.selected = None; test_inv.remove_mode = false;
        saved_test.tiles.clear(); saved_test.inventory = test_inv.items.clone();
        for (_, c, k, m) in &tiles {
            if m.is_some() { saved_test.tiles.push((c.col, c.row, TileKind::Empty)); }
            else if !matches!(k, TileKind::Empty) { saved_test.tiles.push((c.col, c.row, *k)); }
        }
        for (e, _, _, _) in &tiles { commands.entity(e).despawn_recursive(); }
        for &(col, row, kind) in &saved_test.tiles {
            spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        }
        spawn_test_inventory(&mut commands, &test_inv, &icons, true, &font.0);
        spawn_test_banner(&mut commands, &font.0);
        spawn_test_buttons(&mut commands, &font.0);
        if let Ok(c) = inv_container.get_single() { commands.entity(c).insert(UiBottomAnim { target: INV_SLIDE_HIDE, despawn_at_target: false }); }
        *play_mode = PlayMode::TestEditing;
    }
}

fn spawn_test_banner(commands: &mut Commands, f: &Handle<Font>) {
    commands.spawn((Node { position_type: PositionType::Absolute, top: Val::Px(BANNER_SLIDE_HIDE), width: Val::Percent(100.0),
        height: Val::Px(BANNER_HEIGHT), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        BackgroundColor(rgba(TEST_BANNER_BG)), TestModeBanner,
        UiTopAnim { target: TOP_SLIDE_SHOW + PLAY_BTN_SIZE + 6.0, despawn_at_target: false },
    )).with_child((Text::new("TEST MODE"), gf(DIALOG_TITLE_FONT, f), TextColor(rgb(TEST_BANNER_TEXT))));
}

fn spawn_test_buttons(commands: &mut Commands, f: &Handle<Font>) {
    let (tf, tc, br) = (gf(LABEL_FONT, f), TextColor(Color::WHITE), BorderRadius::all(Val::Px(UI_CORNER_RADIUS)));
    let btn = text_btn_node(); let mut rb = btn.clone(); rb.margin = UiRect::right(Val::Px(BTN_SIDE_MARGIN));
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(-50.0),
        flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0), ..default() },
        UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false }, TestTopButtons,
    )).with_children(|p| {
        p.spawn((Button, ResetTestButton, rb, BackgroundColor(btn_bg()), br)).with_child((Text::new("Reset"), tf.clone(), tc));
        p.spawn((Button, SaveButton, btn.clone(), BackgroundColor(btn_bg()), br)).with_child((Text::new("Save"), tf.clone(), tc));
        p.spawn((Button, StopTestButton, btn, BackgroundColor(rgb(STOP_TEST_BTN_BG)), br)).with_child((Text::new("Stop Test"), tf, tc));
    });
}

pub fn spawn_test_inventory(commands: &mut Commands, test_inv: &TestInventory, icons: &InventoryIcons, animate: bool, f: &Handle<Font>) {
    let start_bottom = if animate { INV_SLIDE_HIDE } else { INV_SLIDE_SHOW };
    let mut ec = commands.spawn((Node { position_type: PositionType::Absolute, bottom: Val::Px(start_bottom),
        width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() }, TestInventoryContainer));
    if animate { ec.insert(UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false }); }
    ec.with_children(|parent| {
        parent.spawn(Node { position_type: PositionType::Absolute, top: Val::Px(-22.0),
            width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new(""), gf(STATUS_FONT, f),
                TextColor(Color::srgba(TOOLTIP_COLOR.0, TOOLTIP_COLOR.1, TOOLTIP_COLOR.2, 0.0)), StatusBarText));
        let br = BorderRadius::all(Val::Px(UI_CORNER_RADIUS));
        parent.spawn((Node { flex_direction: FlexDirection::Row, padding: UiRect::all(Val::Vw(INVENTORY_PAD_VW)),
            column_gap: Val::Vw(INVENTORY_GAP_VW), align_items: AlignItems::Center, ..default() },
            BackgroundColor(rgba(TEST_INVENTORY_BG)), br,
        )).with_children(|c| {
            let sn = slot_node();
            for (i, (kind, count)) in test_inv.items.iter().enumerate() {
                let Some(icon) = tilekind_to_icon(kind, icons) else { continue };
                let sel = !test_inv.remove_mode && test_inv.selected == Some(i);
                c.spawn((Button, TestInventorySlot(i), border_for(sel), sn.clone(),
                    BackgroundColor(slot_bg()), br)).with_children(|w| {
                    w.spawn((icon_node(), ImageNode::new(icon)));
                    let cc = if *count > 0 { rgb(COUNT_AVAIL_COLOR) } else { rgb(COUNT_EMPTY_COLOR) };
                    w.spawn(Node { width: Val::Percent(100.0), justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute, bottom: Val::Px(2.0), ..default() })
                        .with_child((Text::new(format!("{count}")), gf(COUNT_FONT, f), TextColor(cc)));
                });
            }
            c.spawn((Button, TestInventorySlot(usize::MAX), border_for(test_inv.remove_mode),
                sn, BackgroundColor(Color::NONE), BorderColor(Color::NONE), br,
            )).with_child((icon_node(), ImageNode::new(icons.delete.clone())));
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
    mut inv_state: ResMut<InventoryState>,
    mut selected_tool: ResMut<SelectedTool>,
    inv_container: Query<Entity, With<InventoryContainer>>,
    test_container: Query<Entity, With<TestInventoryContainer>>,
    mut sim_result: ResMut<crate::simulation::SimulationResult>,
    banner: Query<Entity, With<TestModeBanner>>,
    top_btns: Query<Entity, With<TestTopButtons>>,
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
    *inv_state = saved_board.inv_state.clone();
    selected_tool.0 = saved_board.selected_tool;
    if let Ok(c) = inv_container.get_single() { commands.entity(c).insert(UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false }); }
    for e in &test_container { commands.entity(e).insert(UiBottomAnim { target: INV_SLIDE_HIDE, despawn_at_target: true }); }
    for e in &banner { commands.entity(e).insert(UiTopAnim { target: BANNER_SLIDE_HIDE, despawn_at_target: true }); }
    for e in &top_btns { commands.entity(e).insert(UiTopAnim { target: BANNER_SLIDE_HIDE, despawn_at_target: true }); }
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
    font: Res<GameFont>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if *play_mode != PlayMode::TestEditing { continue; }
        for entity in &tiles { commands.entity(entity).despawn_recursive(); }
        for &(col, row, kind) in &saved_test.tiles {
            spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        }
        test_inv.items = saved_test.inventory.clone();
        test_inv.selected = None; test_inv.remove_mode = false;
        for e in &test_container { commands.entity(e).despawn_recursive(); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, false, &font.0);
    }
}

pub fn sync_editor_buttons_visibility(
    play_mode: Res<PlayMode>, mut bar: Query<&mut Visibility, With<TopControlsBar>>,
    mut buttons: Query<&mut BackgroundColor, Or<(With<MarkButton>, With<TestButton>, With<BoardButton>, With<LoadButton>)>>,
) {
    if !play_mode.is_changed() { return; }
    let test = matches!(*play_mode, PlayMode::TestEditing | PlayMode::TestPlaying);
    for mut v in &mut bar { *v = if test { Visibility::Hidden } else { Visibility::Inherited }; }
    let alpha = if *play_mode == PlayMode::Playing { DISABLED_BTN_ALPHA } else { 1.0 };
    for mut bg in &mut buttons { bg.0.set_alpha(alpha); }
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
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
    icons: Res<InventoryIcons>,
    test_container: Query<Entity, With<TestInventoryContainer>>,
    mut ghost_cell: ResMut<GhostCell>,
    saved_test: Res<SavedTestState>,
    font: Res<GameFont>,
) {
    if *play_mode != PlayMode::TestEditing { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else {
        if test_inv.selected.is_some() || test_inv.remove_mode {
            test_inv.selected = None; test_inv.remove_mode = false; selected_tool.0 = Tool::Floor;
            for e in &test_container { commands.entity(e).despawn_recursive(); }
            spawn_test_inventory(&mut commands, &test_inv, &icons, false, &font.0);
        } return;
    };
    let Some((entity, _, kind)) = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row) else { return };

    let mut changed = false;
    if test_inv.remove_mode {
        if matches!(kind, TileKind::Empty | TileKind::Floor) { return; }
        if !saved_test.tiles.iter().any(|&(c, r, k)| c == col && r == row && matches!(k, TileKind::Empty)) { return; }
        if let Some(e) = test_inv.items.iter_mut().find(|(k, _)| *k == *kind) { e.1 += 1; }
        else { test_inv.items.push((*kind, 1)); test_inv.items.sort_by(|(a, _), (b, _)| tile_sort_key(a).cmp(&tile_sort_key(b))); }
        commands.entity(entity).despawn_recursive();
        spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        changed = true;
    } else if let Some(idx) = test_inv.selected {
        if idx >= test_inv.items.len() { return; }
        let (tile_kind, count) = test_inv.items[idx];
        if count == 0 || !matches!(kind, TileKind::Empty) { return; }
        commands.entity(entity).despawn_recursive();
        crate::board::spawn_tile_at_scale(&mut commands, col, row, board_size.0, tile_kind, &assets, Vec3::ZERO);
        test_inv.items[idx].1 -= 1;
        if test_inv.items[idx].1 == 0 {
            test_inv.items.remove(idx);
            if test_inv.items.is_empty() {
                test_inv.selected = None; test_inv.remove_mode = true;
                selected_tool.0 = Tool::Delete;
            } else {
                let new_idx = idx.min(test_inv.items.len() - 1);
                test_inv.selected = Some(new_idx);
                set_tool_from_kind(test_inv.items[new_idx].0, &mut selected_tool, &mut inv_state);
            }
        }
        changed = true;
    }
    if changed {
        ghost_cell.last_placed = Some((col, row));
        for e in &test_container { commands.entity(e).despawn_recursive(); }
        spawn_test_inventory(&mut commands, &test_inv, &icons, false, &font.0);
    }
}
