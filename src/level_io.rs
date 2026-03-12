// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

const LEVELS_DIR: &str = "levels";

fn levels_dir() -> PathBuf {
    let dir = PathBuf::from(LEVELS_DIR);
    if !dir.exists() { let _ = fs::create_dir_all(&dir); }
    dir
}

fn sanitize_filename(name: &str) -> String {
    name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else { '_' })
        .collect::<String>().trim().to_string()
}

// === Validation ===
fn validate_level(
    tiles: &Query<(&TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    placed_teleports: &PlacedTeleports, validated: &LevelValidated,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut sources: HashSet<usize> = HashSet::new();
    let mut goals: HashSet<usize> = HashSet::new();
    let mut painters: HashSet<usize> = HashSet::new();
    for (_, kind, _) in tiles.iter() {
        match kind {
            TileKind::Source(ci, _) => { sources.insert(*ci); }
            TileKind::Goal(ci) => { goals.insert(*ci); }
            TileKind::Painter(ci) => { painters.insert(*ci); }
            _ => {}
        }
    }
    if sources.is_empty() { errors.push("No source tiles on the board".into()); }
    if goals.is_empty() { errors.push("No goal tiles on the board".into()); }
    for &ci in &goals {
        if !sources.contains(&ci) && !painters.contains(&ci) {
            errors.push(format!("{} goal has no matching source or painter", COLOR_NAMES[ci]));
        }
    }
    for i in 0..NUM_TELEPORTS {
        if placed_teleports.0[i] == 1 {
            errors.push(format!("Teleport {} needs a pair", i + 1));
        }
    }
    if !validated.0 { errors.push("Level has not been tested successfully".into()); }
    errors
}

// === Save ===
fn validate_saved(saved: &SavedBoardState, validated: &LevelValidated) -> Vec<String> {
    let mut errors = Vec::new();
    let (mut sources, mut goals, mut painters) = (HashSet::new(), HashSet::new(), HashSet::new());
    let mut tp = [0u8; 10];
    for (_, _, kind, _) in &saved.tiles {
        match kind {
            TileKind::Source(ci, _) => { sources.insert(*ci); }
            TileKind::Goal(ci) => { goals.insert(*ci); }
            TileKind::Painter(ci) => { painters.insert(*ci); }
            TileKind::Teleport(n) => { tp[*n] += 1; }
            _ => {}
        }
    }
    if sources.is_empty() { errors.push("No source tiles on the board".into()); }
    if goals.is_empty() { errors.push("No goal tiles on the board".into()); }
    for &ci in &goals {
        if !sources.contains(&ci) && !painters.contains(&ci) {
            errors.push(format!("{} goal has no matching source or painter", COLOR_NAMES[ci]));
        }
    }
    for i in 0..NUM_TELEPORTS { if tp[i] == 1 { errors.push(format!("Teleport {} needs a pair", i + 1)); } }
    if !validated.0 { errors.push("Level has not been tested successfully".into()); }
    errors
}

pub fn save_button_interaction(
    interaction_query: Query<&Interaction, (With<SaveButton>, Changed<Interaction>)>,
    play_mode: Res<PlayMode>,
    mut commands: Commands,
    existing_dialog: Query<Entity, Or<(With<SaveDialog>, With<LoadDialog>, With<ValidationErrorDialog>)>>,
    tiles: Query<(&TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    placed_teleports: Res<PlacedTeleports>,
    validated: Res<LevelValidated>,
    saved_board: Res<SavedBoardState>,
    font: Res<GameFont>,
) {
    let in_test = matches!(*play_mode, PlayMode::TestEditing | PlayMode::TestPlaying);
    if *play_mode != PlayMode::Editing && !in_test { return; }
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if !existing_dialog.is_empty() { return; }
        let errors = if in_test {
            validate_saved(&saved_board, &validated)
        } else {
            validate_level(&tiles, &placed_teleports, &validated)
        };
        if errors.is_empty() {
            spawn_save_dialog(&mut commands, &font.0);
        } else {
            spawn_validation_error(&mut commands, &errors, &font.0);
        }
    }
}

fn spawn_save_dialog(commands: &mut Commands, f: &Handle<Font>) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let tc = TextColor(Color::WHITE);
    spawn_dialog(commands, SaveDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new("Save Level"), tf.clone(), tc));
        panel.spawn((
            Node { width: Val::Px(DIALOG_INPUT_WIDTH), height: Val::Px(DIALOG_INPUT_HEIGHT), padding: UiRect::all(Val::Px(DIALOG_INPUT_PAD)),
                justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(DIALOG_INPUT_BORDER_PX)), ..default() },
            BackgroundColor(rgb(DIALOG_INPUT_BG)), BorderColor(rgb(DIALOG_INPUT_BORDER)), SaveDialogInput,
        )).with_child((Text::new(""), gf(DIALOG_BODY_FONT, f), TextColor(rgb(DIALOG_INPUT_TEXT))));
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(DIALOG_BTN_GAP), ..default() })
            .with_children(|row| {
                row.spawn((Button, SaveDialogConfirm, dialog_btn_node(), BackgroundColor(rgb(CONFIRM_BTN_BG))))
                    .with_child((Text::new("Save"), tf.clone(), tc));
                row.spawn((Button, SaveDialogCancel, dialog_btn_node(), BackgroundColor(btn_bg())))
                    .with_child((Text::new("Cancel"), tf, tc));
            });
    });
}

fn spawn_validation_error(commands: &mut Commands, errors: &[String], f: &Handle<Font>) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let bf = gf(DIALOG_BODY_FONT, f);
    let tc = TextColor(Color::WHITE);
    spawn_dialog(commands, ValidationErrorDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new("Cannot Save"), tf.clone(), TextColor(rgb(SIM_ERROR_COLOR))));
        for err in errors {
            panel.spawn((Text::new(format!("• {err}")), bf.clone(), tc));
        }
        panel.spawn((Button, ValidationErrorOk, dialog_btn_node(), BackgroundColor(btn_bg())))
            .with_child((Text::new("OK"), tf, tc));
    });
}

pub fn validation_error_ok(
    mut commands: Commands,
    q: Query<&Interaction, (With<ValidationErrorOk>, Changed<Interaction>)>,
    dialog: Query<Entity, With<ValidationErrorDialog>>,
    keys: Res<ButtonInput<KeyCode>>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    let ok = q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Enter);
    if !ok || dialog.is_empty() { return; }
    suppress_ghost(&hovered, &mut ghost_cell);
    fade_out::<ValidationErrorDialog>(&mut commands, &dialog);
}

pub fn save_dialog_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut text_q: Query<&Children, With<SaveDialogInput>>,
    mut text_writer: Query<&mut Text>,
    dialog: Query<(Entity, Option<&UiBgFade>), With<SaveDialog>>,
) {
    if dialog.is_empty() { return; }
    if dialog.iter().any(|(_, f)| f.is_some_and(|f| f.target < 0.01)) { return; }
    let Ok(children) = text_q.get_single_mut() else { return };
    let Some(&child) = children.iter().next() else { return };
    let Ok(mut text) = text_writer.get_mut(child) else { return };
    if keys.just_pressed(KeyCode::Backspace) { text.0.pop(); return; }
    // Map key presses to characters
    for key in keys.get_just_pressed() {
        let c = match key {
            KeyCode::KeyA => 'a', KeyCode::KeyB => 'b', KeyCode::KeyC => 'c',
            KeyCode::KeyD => 'd', KeyCode::KeyE => 'e', KeyCode::KeyF => 'f',
            KeyCode::KeyG => 'g', KeyCode::KeyH => 'h', KeyCode::KeyI => 'i',
            KeyCode::KeyJ => 'j', KeyCode::KeyK => 'k', KeyCode::KeyL => 'l',
            KeyCode::KeyM => 'm', KeyCode::KeyN => 'n', KeyCode::KeyO => 'o',
            KeyCode::KeyP => 'p', KeyCode::KeyQ => 'q', KeyCode::KeyR => 'r',
            KeyCode::KeyS => 's', KeyCode::KeyT => 't', KeyCode::KeyU => 'u',
            KeyCode::KeyV => 'v', KeyCode::KeyW => 'w', KeyCode::KeyX => 'x',
            KeyCode::KeyY => 'y', KeyCode::KeyZ => 'z',
            KeyCode::Digit0 => '0', KeyCode::Digit1 => '1', KeyCode::Digit2 => '2',
            KeyCode::Digit3 => '3', KeyCode::Digit4 => '4', KeyCode::Digit5 => '5',
            KeyCode::Digit6 => '6', KeyCode::Digit7 => '7', KeyCode::Digit8 => '8',
            KeyCode::Digit9 => '9',
            KeyCode::Space => ' ', KeyCode::Minus => '-',
            _ => continue,
        };
        if text.0.len() < MAX_LEVEL_NAME {
            let c = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                c.to_uppercase().next().unwrap_or(c)
            } else { c };
            text.0.push(c);
        }
    }
}

pub fn save_dialog_buttons(
    mut commands: Commands,
    confirm_q: Query<&Interaction, (With<SaveDialogConfirm>, Changed<Interaction>)>,
    cancel_q: Query<&Interaction, (With<SaveDialogCancel>, Changed<Interaction>)>,
    dialog: Query<Entity, With<SaveDialog>>,
    input_q: Query<&Children, With<SaveDialogInput>>,
    text_q: Query<&Text>,
    tiles: Query<(&TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    board_size: Res<BoardSize>,
    keys: Res<ButtonInput<KeyCode>>,
    play_mode: Res<PlayMode>,
    saved_board: Res<SavedBoardState>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    let cancel = cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    let confirm = confirm_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Enter);

    if cancel { suppress_ghost(&hovered, &mut ghost_cell); fade_out(&mut commands, &dialog); return; }
    if !confirm { return; }
    let Ok(children) = input_q.get_single() else { return };
    let Some(&child) = children.iter().next() else { return };
    let Ok(text) = text_q.get(child) else { return };
    let name = sanitize_filename(&text.0);
    if name.is_empty() { return; }
    let in_test = matches!(*play_mode, PlayMode::TestEditing | PlayMode::TestPlaying);
    let tile_data = if in_test {
        saved_board.tiles.clone()
    } else {
        tiles.iter().map(|(c, k, m)| (c.col, c.row, *k, m.is_some())).collect()
    };
    let solution: Vec<(u32, u32, TileKind)> = tile_data.iter()
        .filter(|(_, _, _, marked)| *marked)
        .map(|(c, r, k, _)| (*c, *r, *k))
        .collect();
    let level = LevelData { name: name.clone(), board_size: board_size.0, tiles: tile_data, solution };
    let path = levels_dir().join(format!("{name}.json"));
    if let Ok(json) = serde_json::to_string_pretty(&level) { let _ = fs::write(&path, json); }
    suppress_ghost(&hovered, &mut ghost_cell);
    fade_out::<SaveDialog>(&mut commands, &dialog);
}

// === Load ===
pub fn load_button_interaction(
    interaction_query: Query<&Interaction, (With<LoadButton>, Changed<Interaction>)>,
    play_mode: Res<PlayMode>,
    mut commands: Commands,
    existing_dialog: Query<Entity, Or<(With<SaveDialog>, With<LoadDialog>)>>,
    font: Res<GameFont>,
) {
    if *play_mode != PlayMode::Editing { return; }
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if !existing_dialog.is_empty() { return; }
        spawn_load_dialog(&mut commands, &font.0);
    }
}

fn spawn_load_dialog(commands: &mut Commands, f: &Handle<Font>) {
    let tf = gf(DIALOG_BODY_FONT, f);
    let tc = TextColor(Color::WHITE);
    let dir = levels_dir();
    let mut entries: Vec<String> = Vec::new();
    if let Ok(readdir) = fs::read_dir(&dir) {
        for entry in readdir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") { if let Some(stem) = path.file_stem() {
                entries.push(stem.to_string_lossy().to_string());
            }}
        }
    }
    entries.sort();

    let mut pn = dialog_panel_node(LOAD_DIALOG_ROW_GAP);
    pn.max_height = Val::Vh(70.0);
    spawn_dialog(commands, LoadDialog, pn, |panel| {
        panel.spawn((Text::new("Load Level"), gf(DIALOG_TITLE_FONT, f), tc));
        panel.spawn(Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(DIALOG_LIST_GAP),
            overflow: Overflow::scroll_y(), max_height: Val::Vh(DIALOG_LIST_MAX_H),
            width: Val::Px(DIALOG_LIST_WIDTH), ..default()
        }).with_children(|list| {
            if entries.is_empty() {
                list.spawn((Text::new("No saved levels"), tf.clone(), TextColor(rgb(DIALOG_EMPTY_TEXT))));
            }
            for name in &entries {
                list.spawn((Button, LoadDialogEntry(name.clone()),
                    Node { padding: UiRect::axes(Val::Px(LOAD_DIALOG_ROW_GAP), Val::Px(DIALOG_BTN_PAD.1)),
                        width: Val::Percent(100.0), ..default() },
                    BackgroundColor(btn_bg()),
                )).with_child((Text::new(name.clone()), tf.clone(), tc));
            }
        });
        let mut cn = dialog_btn_node(); cn.margin = UiRect::top(Val::Px(DIALOG_CANCEL_TOP_MARGIN));
        panel.spawn((Button, LoadDialogCancel, cn, BackgroundColor(btn_bg())))
            .with_child((Text::new("Cancel"), tf, tc));
    });
}

pub fn load_dialog_buttons(
    mut commands: Commands,
    cancel_q: Query<&Interaction, (With<LoadDialogCancel>, Changed<Interaction>)>,
    entry_q: Query<(&Interaction, &LoadDialogEntry), Changed<Interaction>>,
    dialog: Query<Entity, With<LoadDialog>>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    mut placed_teleports: ResMut<PlacedTeleports>,
    mut inv_state: ResMut<InventoryState>,
    mut selected_tool: ResMut<SelectedTool>,
    keys: Res<ButtonInput<KeyCode>>,
    mut size_text_q: Query<&mut Text, With<BoardSizeText>>,
    expansion: Query<(Entity, &Children), With<ExpansionContainer>>,
    mut validated: ResMut<LevelValidated>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    let cancel = cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    if cancel { suppress_ghost(&hovered, &mut ghost_cell); fade_out(&mut commands, &dialog); return; }
    let mut selected_file = None;
    for (interaction, entry) in &entry_q {
        if *interaction == Interaction::Pressed { selected_file = Some(entry.0.clone()); }
    }
    let Some(filename) = selected_file else { return };
    let path = levels_dir().join(format!("{filename}.json"));
    let Ok(json) = fs::read_to_string(&path) else { return };
    let Ok(level) = serde_json::from_str::<LevelData>(&json) else { return };

    // Despawn existing board
    for entity in &tiles { commands.entity(entity).despawn_recursive(); }
    board_size.0 = level.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);

    // Fill grid positions not in level data with Empty tiles
    let mut grid = std::collections::HashSet::new();
    for &(col, row, _, _) in &level.tiles { grid.insert((col, row)); }
    for row in 0..board_size.0 {
        for col in 0..board_size.0 {
            if !grid.contains(&(col, row)) {
                spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
            }
        }
    }

    // Rebuild tracking state
    placed_teleports.0 = [0; 10];
    for &(col, row, kind, is_marked) in &level.tiles {
        if col >= board_size.0 || row >= board_size.0 { continue; }
        let entity = spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
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
        if let TileKind::Teleport(n) = kind { placed_teleports.0[n] += 1; }
    }
    validated.0 = false;
    *inv_state = InventoryState::default();
    inv_state.level = 1;
    selected_tool.0 = Tool::Floor;
    suppress_ghost(&hovered, &mut ghost_cell);
    if let Ok((_, children)) = expansion.get_single() {
        for &child in children.iter() { commands.entity(child).despawn_recursive(); }
    }
    if let Ok(mut text) = size_text_q.get_single_mut() {
        text.0 = format!("{}x{}", board_size.0, board_size.0);
    }
    fade_out(&mut commands, &dialog);
}
