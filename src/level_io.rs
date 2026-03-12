// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;
use std::fs;
use std::path::PathBuf;

const LEVELS_DIR: &str = "levels";

pub fn levels_dir() -> PathBuf {
    let dir = PathBuf::from(LEVELS_DIR);
    if !dir.exists() { let _ = fs::create_dir_all(&dir); }
    dir
}

// === Load ===
pub fn load_button_interaction(
    interaction_query: Query<&Interaction, (With<LoadButton>, Changed<Interaction>)>,
    play_mode: Res<PlayMode>,
    mut commands: Commands,
    existing_dialog: Query<Entity, Or<(With<SaveDialog>, With<LoadDialog>, With<OverwriteDialog>)>>,
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
    let n_entries = entries.len();

    let mut pn = dialog_panel_node(LOAD_DIALOG_ROW_GAP);
    pn.max_height = Val::Vh(70.0);
    spawn_dialog(commands, LoadDialog, pn, |panel| {
        panel.spawn((Text::new("Load Level"), gf(DIALOG_TITLE_FONT, f), tc));
        // Row: list + scrollbar
        panel.spawn(Node { flex_direction: FlexDirection::Row, width: Val::Px(DIALOG_LIST_WIDTH),
            max_height: Val::Vh(DIALOG_LIST_MAX_H), ..default()
        }).with_children(|row| {
            // Scrollable list
            row.spawn((Node { flex_direction: FlexDirection::Column, row_gap: Val::Px(DIALOG_LIST_GAP),
                overflow: Overflow::scroll_y(), max_height: Val::Vh(DIALOG_LIST_MAX_H),
                flex_grow: 1.0, ..default() }, LoadDialogList,
            )).with_children(|list| {
                if entries.is_empty() {
                    list.spawn((Text::new("No saved levels"), tf.clone(), TextColor(rgb(DIALOG_EMPTY_TEXT))));
                }
                for name in &entries {
                    list.spawn(Node { flex_direction: FlexDirection::Row, width: Val::Percent(100.0),
                        align_items: AlignItems::Center, ..default() })
                    .with_children(|row| {
                        row.spawn((Button, LoadDialogEntry(name.clone()),
                            Node { padding: UiRect::axes(Val::Px(LOAD_DIALOG_ROW_GAP), Val::Px(DIALOG_BTN_PAD.1)),
                                flex_grow: 1.0, ..default() },
                            BackgroundColor(btn_bg()),
                        )).with_child((Text::new(name.clone()), tf.clone(), tc));
                        row.spawn((Button, DeleteLevelButton(name.clone()),
                            Node { width: Val::Px(28.0), height: Val::Px(28.0),
                                justify_content: JustifyContent::Center, align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                            BackgroundColor(rgb(DELETE_BTN_COLOR)),
                        )).with_child((Text::new("×"), gf(DIALOG_BODY_FONT, f), tc));
                    });
                }
            });
            // Scrollbar track (only if entries overflow)
            if n_entries > 6 {
                row.spawn((Node { width: Val::Px(SCROLLBAR_WIDTH), height: Val::Percent(100.0),
                    margin: UiRect::left(Val::Px(4.0)), ..default() },
                    BackgroundColor(rgba(SCROLLBAR_TRACK_COLOR)), Interaction::default(),
                )).with_children(|track| {
                    track.spawn((Node { width: Val::Px(SCROLLBAR_WIDTH), height: Val::Px(SCROLLBAR_MIN_H),
                        position_type: PositionType::Absolute, top: Val::Px(0.0),
                        border_radius: BorderRadius::all(Val::Px(SCROLLBAR_WIDTH / 2.0)), ..default() },
                        BackgroundColor(rgba(SCROLLBAR_COLOR)), ScrollbarThumb,
                    ));
                });
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
    mut inv_state: ResMut<InventoryState>,
    mut selected_tool: ResMut<SelectedTool>,
    keys: Res<ButtonInput<KeyCode>>,
    mut size_text_q: Query<&mut Text, With<BoardSizeText>>,
    expansion: Query<(Entity, &Children), With<ExpansionContainer>>,
    mut validated: ResMut<LevelValidated>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
    mut loaded_name: ResMut<LoadedLevelName>,
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

    loaded_name.0 = Some(level.name.clone());

    for entity in &tiles { commands.entity(entity).despawn(); }
    board_size.0 = level.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);

    let mut grid = std::collections::HashSet::new();
    for &(col, row, _, _) in &level.tiles { grid.insert((col, row)); }
    for row in 0..board_size.0 { for col in 0..board_size.0 {
        if !grid.contains(&(col, row)) { spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets); }
    }}
    for &(col, row, kind, is_marked) in &level.tiles {
        if col >= board_size.0 || row >= board_size.0 { continue; }
        let entity = spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
        if is_marked {
            commands.entity(entity).insert(InventoryMarker).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.marker_mesh.clone()), MeshMaterial3d(assets.marker_material.clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0)),
                    InventoryMarkerVisual,
                ));
            });
        }
    }
    validated.0 = false;
    *inv_state = InventoryState::default(); inv_state.level = 1;
    selected_tool.0 = Tool::Floor;
    suppress_ghost(&hovered, &mut ghost_cell);
    if let Ok((_, children)) = expansion.single() {
        for child in children.iter() { commands.entity(child).despawn(); }
    }
    if let Ok(mut text) = size_text_q.single_mut() {
        text.0 = format!("{}x{}", board_size.0, board_size.0);
    }
    fade_out(&mut commands, &dialog);
}

// === Scrollbar ===
pub fn update_load_scrollbar(
    list_q: Query<(&ComputedNode, &ScrollPosition, &Children), With<LoadDialogList>>,
    mut thumb_q: Query<&mut Node, With<ScrollbarThumb>>,
    child_cn: Query<&ComputedNode, Without<LoadDialogList>>,
) {
    let Ok((cn, scroll, children)) = list_q.single() else { return };
    let Ok(mut thumb) = thumb_q.single_mut() else { return };
    let visible_h = cn.size().y;
    if visible_h < 1.0 { return; }
    let total_h: f32 = children.iter()
        .filter_map(|c| child_cn.get(c).ok()).map(|c| c.size().y).sum::<f32>();
    if total_h <= visible_h { thumb.height = Val::Px(0.0); return; }
    let ratio = (visible_h / total_h).clamp(0.05, 1.0);
    let thumb_h = (visible_h * ratio).max(SCROLLBAR_MIN_H);
    let max_scroll = total_h - visible_h;
    let scroll_ratio = (scroll.y / max_scroll).clamp(0.0, 1.0);
    let track_space = visible_h - thumb_h;
    thumb.height = Val::Px(thumb_h);
    thumb.top = Val::Px(scroll_ratio * track_space);
}

pub fn scrollbar_drag(
    windows: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag: ResMut<ScrollbarDrag>,
    thumb_q: Query<(&GlobalTransform, &ComputedNode), With<ScrollbarThumb>>,
    mut list_q: Query<(&ComputedNode, &mut ScrollPosition, &Children), With<LoadDialogList>>,
    child_cn: Query<&ComputedNode, Without<LoadDialogList>>,
    dialog: Query<(), With<LoadDialog>>,
) -> Result {
    if dialog.is_empty() { drag.0 = None; return Ok(()); }
    let window = windows.single()?;
    let Some(cursor) = window.cursor_position() else { drag.0 = None; return Ok(()); };
    if mouse.just_released(MouseButton::Left) { drag.0 = None; return Ok(()); }

    let Ok((thumb_gt, thumb_cn)) = thumb_q.single() else { return Ok(()); };
    let Ok((list_cn, mut scroll, children)) = list_q.single_mut() else { return Ok(()); };

    let visible_h = list_cn.size().y;
    let total_h: f32 = children.iter()
        .filter_map(|c| child_cn.get(c).ok()).map(|c| c.size().y).sum::<f32>();
    if total_h <= visible_h { return Ok(()); }
    let max_scroll = total_h - visible_h;
    let thumb_h = thumb_cn.size().y;
    let track_space = visible_h - thumb_h;
    if track_space < 1.0 { return Ok(()); }

    // Start drag when clicking on thumb
    if mouse.just_pressed(MouseButton::Left) {
        let thumb_top = thumb_gt.translation().y - thumb_h / 2.0;
        let thumb_bot = thumb_top + thumb_h;
        let thumb_left = thumb_gt.translation().x - thumb_cn.size().x / 2.0;
        let thumb_right = thumb_left + thumb_cn.size().x + 8.0; // generous hit area
        if cursor.x >= thumb_left && cursor.x <= thumb_right && cursor.y >= thumb_top && cursor.y <= thumb_bot {
            drag.0 = Some(cursor.y - thumb_top);
        }
    }

    // Drag in progress
    if let Some(offset) = drag.0 {
        // Track starts at list top
        let list_top = thumb_gt.translation().y - thumb_h / 2.0
            - (scroll.y / max_scroll).clamp(0.0, 1.0) * track_space;
        let rel = ((cursor.y - offset - list_top) / track_space).clamp(0.0, 1.0);
        scroll.y = rel * max_scroll;
    }
    Ok(())
}

// === Delete level ===
pub fn delete_level_button_interaction(
    mut commands: Commands,
    btn_q: Query<(&Interaction, &DeleteLevelButton), Changed<Interaction>>,
    existing: Query<Entity, With<DeleteLevelDialog>>,
    font: Res<GameFont>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    if !existing.is_empty() { return; }
    for (interaction, btn) in &btn_q {
        if *interaction == Interaction::Pressed {
            suppress_ghost(&hovered, &mut ghost_cell);
            spawn_delete_level_dialog(&mut commands, &btn.0, &font.0);
        }
    }
}

fn spawn_delete_level_dialog(commands: &mut Commands, name: &str, f: &Handle<Font>) {
    let tf = gf(DIALOG_TITLE_FONT, f);
    let tc = TextColor(Color::WHITE);
    spawn_dialog(commands, DeleteLevelDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new(format!("Delete \"{name}\"?")), tf.clone(), tc));
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(DIALOG_BTN_GAP), ..default() })
            .with_children(|row| {
                row.spawn((Button, DeleteLevelConfirm(name.to_string()), dialog_btn_node(),
                    BackgroundColor(rgb(STOP_TEST_BTN_BG))))
                    .with_child((Text::new("Delete"), tf.clone(), tc));
                row.spawn((Button, DeleteLevelCancel, dialog_btn_node(), BackgroundColor(btn_bg())))
                    .with_child((Text::new("Cancel"), tf, tc));
            });
    });
}

pub fn delete_level_dialog_buttons(
    mut commands: Commands,
    confirm_q: Query<(&Interaction, &DeleteLevelConfirm), Changed<Interaction>>,
    cancel_q: Query<&Interaction, (With<DeleteLevelCancel>, Changed<Interaction>)>,
    dialog: Query<Entity, With<DeleteLevelDialog>>,
    entries: Query<(&LoadDialogEntry, &ChildOf)>,
    keys: Res<ButtonInput<KeyCode>>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    if dialog.is_empty() { return; }
    let cancel = cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape);
    if cancel { suppress_ghost(&hovered, &mut ghost_cell); fade_out(&mut commands, &dialog); return; }
    let mut to_delete = None;
    for (interaction, confirm) in &confirm_q {
        if *interaction == Interaction::Pressed { to_delete = Some(confirm.0.clone()); }
    }
    if to_delete.is_none() && keys.just_pressed(KeyCode::Enter) {
        if let Some((_, confirm)) = confirm_q.iter().next() {
            to_delete = Some(confirm.0.clone());
        }
    }
    let Some(name) = to_delete else { return };
    let path = levels_dir().join(format!("{name}.json"));
    let _ = fs::remove_file(&path);
    for (entry, child_of) in &entries {
        if entry.0 == name { commands.entity(child_of.0).despawn(); break; }
    }
    suppress_ghost(&hovered, &mut ghost_cell);
    fade_out(&mut commands, &dialog);
}

// === Entry hover highlighting ===
pub fn load_entry_hover(
    mut entry_q: Query<(&Interaction, &mut BackgroundColor), (With<LoadDialogEntry>, Changed<Interaction>)>,
    mut del_q: Query<(&Interaction, &mut BackgroundColor), (With<DeleteLevelButton>, Changed<Interaction>, Without<LoadDialogEntry>)>,
) {
    for (interaction, mut bg) in &mut entry_q {
        bg.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed => rgb(LOAD_ENTRY_HOVER_BG),
            _ => btn_bg(),
        };
    }
    for (interaction, mut bg) in &mut del_q {
        bg.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed => rgb(DELETE_BTN_HOVER),
            _ => rgb(DELETE_BTN_COLOR),
        };
    }
}
