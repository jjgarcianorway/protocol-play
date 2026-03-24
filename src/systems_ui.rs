// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(dead_code)]

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::{gf, rgb, btn_bg, dialog_panel_node, dialog_btn_node};
use crate::simulation::SimulationOverlay;

pub fn sync_ui_play_mode(
    mut commands: Commands,
    play_mode: Res<PlayMode>,
    mut inv: Query<&mut UiBottomAnim, With<InventoryContainer>>,
    test_inv: Query<Entity, With<TestInventoryContainer>>,
    top_bar: Query<Entity, With<TopControlsBar>>,
    test_top: Query<Entity, With<TestTopButtons>>,
) {
    if !play_mode.is_changed() { return; }
    let playing = matches!(*play_mode, PlayMode::Playing | PlayMode::TestPlaying);
    // Editor inventory
    match *play_mode {
        PlayMode::Playing => { for mut a in &mut inv { a.target = INV_SLIDE_HIDE; } }
        PlayMode::Editing => { for mut a in &mut inv { a.target = INV_SLIDE_SHOW; } }
        _ => {}
    }
    // Test inventory
    if playing {
        for e in &test_inv {
            commands.entity(e).insert(UiBottomAnim { target: INV_SLIDE_HIDE, despawn_at_target: false });
        }
    } else if *play_mode == PlayMode::TestEditing {
        for e in &test_inv {
            commands.entity(e).insert(UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false });
        }
    }
    // Top bar (editor + player nav)
    for e in top_bar.iter().chain(test_top.iter()) {
        if playing {
            commands.entity(e).insert(UiTopAnim { target: TOP_SLIDE_HIDE, despawn_at_target: false });
        } else {
            commands.entity(e).insert(UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false });
        }
    }
}

pub fn escape_to_quit(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    existing: Query<Entity, With<QuitDialog>>,
    other_dialogs: Query<Entity, Or<(
        With<SaveDialog>, With<LoadDialog>, With<ValidationErrorDialog>,
        With<OverwriteDialog>, With<DeleteLevelDialog>, With<SimulationOverlay>,
    )>>,
    font: Res<GameFont>,
) {
    if !keys.just_pressed(KeyCode::Escape) { return; }
    if !existing.is_empty() || !other_dialogs.is_empty() { return; }
    let tf = gf(DIALOG_TITLE_FONT, &font.0);
    let tc = TextColor(Color::WHITE);
    crate::ui_helpers::spawn_dialog(&mut commands, QuitDialog, dialog_panel_node(DIALOG_ROW_GAP), |panel| {
        panel.spawn((Text::new("Quit game?"), tf.clone(), tc));
        panel.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(DIALOG_BTN_GAP), ..default() })
            .with_children(|row| {
                row.spawn((Button, QuitConfirm, dialog_btn_node(),
                    BackgroundColor(rgb(STOP_TEST_BTN_BG))))
                    .with_child((Text::new("Quit"), tf.clone(), tc));
                row.spawn((Button, QuitCancel, dialog_btn_node(), BackgroundColor(btn_bg())))
                    .with_child((Text::new("Cancel"), tf, tc));
            });
    });
}

pub fn quit_dialog_buttons(
    mut commands: Commands,
    confirm_q: Query<&Interaction, (With<QuitConfirm>, Changed<Interaction>)>,
    cancel_q: Query<&Interaction, (With<QuitCancel>, Changed<Interaction>)>,
    dialog: Query<Entity, With<QuitDialog>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if dialog.is_empty() { return; }
    if cancel_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Escape) {
        crate::ui_helpers::fade_out(&mut commands, &dialog);
        return;
    }
    if confirm_q.iter().any(|i| *i == Interaction::Pressed) || keys.just_pressed(KeyCode::Enter) {
        std::process::exit(0);
    }
}
