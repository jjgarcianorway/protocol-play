// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::level_io::levels_dir;
use std::collections::HashSet;
use std::fs;

// === Validation ===
fn check_unpaired_teleports(tiles: impl Iterator<Item = TileKind>) -> Vec<String> {
    let mut tp_counts: std::collections::HashMap<(u8, usize, usize), u8> = std::collections::HashMap::new();
    for kind in tiles {
        match kind {
            TileKind::Teleport(c, n) => *tp_counts.entry((0, c, n)).or_default() += 1,
            TileKind::TeleportBut(c, n) => *tp_counts.entry((1, c, n)).or_default() += 1,
            _ => {}
        }
    }
    let mut errors = Vec::new();
    for ((t, _c, n), count) in &tp_counts {
        if *count == 1 {
            let name = if *t == 0 { "Teleport" } else { "TeleportBut" };
            errors.push(format!("{name} {} needs a pair", n + 1));
        }
    }
    errors
}

fn validate_kinds(kinds: impl Iterator<Item = TileKind>, validated: bool) -> Vec<String> {
    let mut errors = Vec::new();
    let (mut sources, mut goals, mut painters) = (HashSet::new(), HashSet::new(), HashSet::new());
    let mut all: Vec<TileKind> = Vec::new();
    for kind in kinds {
        match kind {
            TileKind::Source(ci, _) => { sources.insert(ci); }
            TileKind::Goal(ci) => { goals.insert(ci); }
            TileKind::Painter(ci) => { painters.insert(ci); }
            _ => {}
        }
        all.push(kind);
    }
    if sources.is_empty() { errors.push("No source tiles on the board".into()); }
    if goals.is_empty() { errors.push("No goal tiles on the board".into()); }
    for &ci in &goals {
        if !sources.contains(&ci) && !painters.contains(&ci) {
            errors.push(format!("{} goal has no matching source or painter", COLOR_NAMES[ci]));
        }
    }
    errors.extend(check_unpaired_teleports(all.into_iter()));
    if !validated { errors.push("Level has not been tested successfully".into()); }
    errors
}

// === Save button ===
pub fn save_button_interaction(
    interaction_query: Query<&Interaction, (With<SaveButton>, Changed<Interaction>)>,
    play_mode: Res<PlayMode>,
    mut commands: Commands,
    existing_dialog: Query<Entity, Or<(With<SaveDialog>, With<LoadDialog>, With<ValidationErrorDialog>, With<OverwriteDialog>, With<DeleteLevelDialog>)>>,
    tiles: Query<(&TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    validated: Res<LevelValidated>,
    saved_board: Res<SavedBoardState>,
    font: Res<GameFont>,
    loaded_name: Res<LoadedLevelName>,
) {
    let in_test = matches!(*play_mode, PlayMode::TestEditing | PlayMode::TestPlaying);
    if *play_mode != PlayMode::Editing && !in_test { return; }
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        if !existing_dialog.is_empty() { return; }
        let errors = if in_test {
            validate_kinds(saved_board.tiles.iter().map(|(_, _, k, _)| *k), validated.0)
        } else {
            validate_kinds(tiles.iter().map(|(_, k, _)| *k), validated.0)
        };
        if errors.is_empty() {
            spawn_save_dialog(&mut commands, &font.0, loaded_name.0.as_deref());
        } else {
            spawn_validation_error(&mut commands, &errors, &font.0);
        }
    }
}

fn spawn_save_dialog(commands: &mut Commands, f: &Handle<Font>, prefill: Option<&str>) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let tc = TextColor(Color::WHITE);
    let itc = TextColor(rgb(DIALOG_INPUT_TEXT));
    spawn_dialog(commands, SaveDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new("Save Level"), tf.clone(), tc));
        panel.spawn((
            Node { width: Val::Px(DIALOG_INPUT_WIDTH), height: Val::Px(DIALOG_INPUT_HEIGHT), padding: UiRect::all(Val::Px(DIALOG_INPUT_PAD)),
                justify_content: JustifyContent::FlexStart, align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(DIALOG_INPUT_BORDER_PX)), ..default() },
            BackgroundColor(rgb(DIALOG_INPUT_BG)), BorderColor::all(rgb(DIALOG_INPUT_BORDER)), SaveDialogInput,
        )).with_children(|row| {
            row.spawn((Text::new(prefill.unwrap_or("")), gf(DIALOG_BODY_FONT, f), itc));
            row.spawn((Text::new("|"), gf(DIALOG_BODY_FONT, f), itc, SaveDialogCursor));
        });
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
        for err in errors { panel.spawn((Text::new(format!("• {err}")), bf.clone(), tc)); }
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

// === Cursor blink ===
pub fn blink_save_cursor(
    time: Res<Time>, mut timer: ResMut<CursorBlinkTimer>,
    mut cursor_q: Query<&mut TextColor, With<SaveDialogCursor>>,
    dialog: Query<(), With<SaveDialog>>,
) {
    if dialog.is_empty() { timer.0 = 0.0; return; }
    timer.0 += time.delta_secs();
    let visible = (timer.0 * CURSOR_BLINK_RATE * std::f32::consts::PI).sin() > 0.0;
    let c = if visible { rgb(DIALOG_INPUT_TEXT) } else { Color::NONE };
    for mut tc in &mut cursor_q { tc.0 = c; }
}

// === Text input ===
pub fn save_dialog_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input_q: Query<&Children, With<SaveDialogInput>>,
    mut text_writer: Query<&mut Text, Without<SaveDialogCursor>>,
    dialog: Query<(Entity, Option<&UiBgFade>), With<SaveDialog>>,
    mut timer: ResMut<CursorBlinkTimer>,
) {
    if dialog.is_empty() { return; }
    if dialog.iter().any(|(_, f)| f.is_some_and(|f| f.target < 0.01)) { return; }
    let Ok(children) = input_q.single_mut() else { return };
    let Some(child) = children.iter().next() else { return };
    let Ok(mut text) = text_writer.get_mut(child) else { return };
    if keys.just_pressed(KeyCode::Backspace) { text.0.pop(); timer.0 = 0.0; return; }
    for key in keys.get_just_pressed() {
        let c = match key {
            KeyCode::KeyA => 'a', KeyCode::KeyB => 'b', KeyCode::KeyC => 'c', KeyCode::KeyD => 'd',
            KeyCode::KeyE => 'e', KeyCode::KeyF => 'f', KeyCode::KeyG => 'g', KeyCode::KeyH => 'h',
            KeyCode::KeyI => 'i', KeyCode::KeyJ => 'j', KeyCode::KeyK => 'k', KeyCode::KeyL => 'l',
            KeyCode::KeyM => 'm', KeyCode::KeyN => 'n', KeyCode::KeyO => 'o', KeyCode::KeyP => 'p',
            KeyCode::KeyQ => 'q', KeyCode::KeyR => 'r', KeyCode::KeyS => 's', KeyCode::KeyT => 't',
            KeyCode::KeyU => 'u', KeyCode::KeyV => 'v', KeyCode::KeyW => 'w', KeyCode::KeyX => 'x',
            KeyCode::KeyY => 'y', KeyCode::KeyZ => 'z',
            KeyCode::Digit0 => '0', KeyCode::Digit1 => '1', KeyCode::Digit2 => '2', KeyCode::Digit3 => '3',
            KeyCode::Digit4 => '4', KeyCode::Digit5 => '5', KeyCode::Digit6 => '6', KeyCode::Digit7 => '7',
            KeyCode::Digit8 => '8', KeyCode::Digit9 => '9',
            KeyCode::Space => ' ', KeyCode::Minus => '-',
            _ => continue,
        };
        if text.0.len() < MAX_LEVEL_NAME {
            let c = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                c.to_uppercase().next().unwrap_or(c)
            } else { c };
            text.0.push(c); timer.0 = 0.0;
        }
    }
}

// === Save confirm / overwrite ===
pub fn save_dialog_buttons(
    mut commands: Commands,
    confirm_q: Query<&Interaction, (With<SaveDialogConfirm>, Changed<Interaction>)>,
    cancel_q: Query<&Interaction, (With<SaveDialogCancel>, Changed<Interaction>)>,
    dialog: Query<Entity, With<SaveDialog>>,
    input_q: Query<&Children, With<SaveDialogInput>>,
    text_q: Query<&Text, Without<SaveDialogCursor>>,
    tiles: Query<(&TileCoord, &TileKind, Option<&InventoryMarker>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    board_size: Res<BoardSize>,
    keys: Res<ButtonInput<KeyCode>>,
    play_mode: Res<PlayMode>,
    saved_board: Res<SavedBoardState>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
    mut loaded_name: ResMut<LoadedLevelName>,
    mut pending: ResMut<PendingSave>,
    font: Res<GameFont>,
) {
    let cancel = cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    let confirm = confirm_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Enter);
    if cancel { suppress_ghost(&hovered, &mut ghost_cell); fade_out(&mut commands, &dialog); return; }
    if !confirm { return; }
    let Ok(children) = input_q.single() else { return };
    let Some(child) = children.iter().next() else { return };
    let Ok(text) = text_q.get(child) else { return };
    let raw_name = sanitize_filename(&text.0);
    if raw_name.is_empty() { return; }
    let in_test = matches!(*play_mode, PlayMode::TestEditing | PlayMode::TestPlaying);
    let tile_data = if in_test { saved_board.tiles.clone() }
        else { tiles.iter().map(|(c, k, m)| (c.col, c.row, *k, m.is_some())).collect() };
    let solution: Vec<(u32, u32, TileKind)> = tile_data.iter()
        .filter(|(_, _, _, marked)| *marked).map(|(c, r, k, _)| (*c, *r, *k)).collect();
    let bs = board_size.0;
    let name = format!("{bs}x{bs}_{raw_name}");
    let level = LevelData { name: raw_name.clone(), board_size: bs, tiles: tile_data, solution };
    let path = levels_dir().join(format!("{name}.json"));
    loaded_name.0 = Some(name.clone());
    if path.exists() {
        pending.0 = Some((name.clone(), level));
        suppress_ghost(&hovered, &mut ghost_cell);
        fade_out(&mut commands, &dialog);
        spawn_overwrite_dialog(&mut commands, &name, &font.0);
    } else {
        if let Ok(json) = serde_json::to_string_pretty(&level) { let _ = fs::write(&path, json); }
        suppress_ghost(&hovered, &mut ghost_cell);
        fade_out::<SaveDialog>(&mut commands, &dialog);
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else { '_' })
        .collect::<String>().trim().to_string()
}

fn spawn_overwrite_dialog(commands: &mut Commands, name: &str, f: &Handle<Font>) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let tc = TextColor(Color::WHITE);
    spawn_dialog(commands, OverwriteDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new(format!("Overwrite \"{name}\"?")), tf.clone(), tc));
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(DIALOG_BTN_GAP), ..default() })
            .with_children(|row| {
                row.spawn((Button, OverwriteConfirm, dialog_btn_node(), BackgroundColor(rgb(CONFIRM_BTN_BG))))
                    .with_child((Text::new("Overwrite"), tf.clone(), tc));
                row.spawn((Button, OverwriteCancel, dialog_btn_node(), BackgroundColor(btn_bg())))
                    .with_child((Text::new("Cancel"), tf, tc));
            });
    });
}

pub fn overwrite_dialog_buttons(
    mut commands: Commands,
    confirm_q: Query<&Interaction, (With<OverwriteConfirm>, Changed<Interaction>)>,
    cancel_q: Query<&Interaction, (With<OverwriteCancel>, Changed<Interaction>)>,
    dialog: Query<Entity, With<OverwriteDialog>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut pending: ResMut<PendingSave>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    if dialog.is_empty() { return; }
    let cancel = cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    let confirm = confirm_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Enter);
    if cancel { pending.0 = None; suppress_ghost(&hovered, &mut ghost_cell); fade_out(&mut commands, &dialog); return; }
    if !confirm { return; }
    if let Some((name, level)) = pending.0.take() {
        let path = levels_dir().join(format!("{name}.json"));
        if let Ok(json) = serde_json::to_string_pretty(&level) { let _ = fs::write(&path, json); }
    }
    suppress_ghost(&hovered, &mut ghost_cell);
    fade_out::<OverwriteDialog>(&mut commands, &dialog);
}
