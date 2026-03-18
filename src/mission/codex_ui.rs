// SPDX-License-Identifier: GPL-3.0-or-later

//! Crew Manifest / Character Codex — UI spawning.

use bevy::prelude::*;
use crate::save_state::GameState;
use super::codex::*;

/// Spawn the full-screen codex overlay with scrollable character list.
pub fn spawn_codex_overlay(commands: &mut Commands, font: &Handle<Font>, gs: &GameState) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            CODEX_OVERLAY_BG.0, CODEX_OVERLAY_BG.1,
            CODEX_OVERLAY_BG.2, CODEX_OVERLAY_BG.3,
        )),
        GlobalZIndex(45),
        CodexOverlay,
        Button,
    )).with_children(|overlay| {
        // Central panel
        overlay.spawn((
            Node {
                max_width: Val::Px(CODEX_MAX_WIDTH),
                width: Val::Percent(85.0),
                max_height: Val::Percent(CODEX_MAX_HEIGHT),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(CODEX_PADDING)),
                row_gap: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(CODEX_PANEL_CORNER)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(
                CODEX_PANEL_BG.0, CODEX_PANEL_BG.1,
                CODEX_PANEL_BG.2, CODEX_PANEL_BG.3,
            )),
            BoxShadow::new(
                Color::srgba(
                    CODEX_GLOW_COLOR.0, CODEX_GLOW_COLOR.1,
                    CODEX_GLOW_COLOR.2, CODEX_GLOW_COLOR.3,
                ),
                Val::ZERO, Val::ZERO,
                Val::Px(CODEX_GLOW_SPREAD), Val::Px(CODEX_GLOW_BLUR),
            ),
            CodexPanelGlow,
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("CREW MANIFEST"),
                TextFont { font: font.clone(), font_size: CODEX_TITLE_FONT, ..default() },
                TextColor(Color::srgb(
                    CODEX_TITLE_COLOR.0, CODEX_TITLE_COLOR.1, CODEX_TITLE_COLOR.2,
                )),
            ));

            // Scrollable entry list
            panel.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                width: Val::Percent(100.0),
                overflow: Overflow::scroll_y(),
                flex_grow: 1.0,
                ..default()
            }).with_children(|list| {
                spawn_codex_entries(list, font, gs);
            });

            // Dismiss hint
            panel.spawn((
                Text::new("Press ESC or click outside to close"),
                TextFont { font: font.clone(), font_size: CODEX_HINT_FONT, ..default() },
                TextColor(Color::srgba(
                    CODEX_HINT_COLOR.0, CODEX_HINT_COLOR.1,
                    CODEX_HINT_COLOR.2, CODEX_HINT_COLOR.3,
                )),
            ));
        });
    });
}

/// Spawn individual codex entry rows inside the scrollable list.
fn spawn_codex_entries(
    list: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    let unlocked_count = CODEX_ENTRIES.iter()
        .filter(|e| gs.codex_unlocked.contains(&e.scene_id.to_string()))
        .count();
    let total = CODEX_ENTRIES.len();

    // Summary line
    list.spawn((
        Text::new(format!("{} / {} crew discovered", unlocked_count, total)),
        TextFont { font: font.clone(), font_size: CODEX_DETAIL_FONT, ..default() },
        TextColor(Color::srgba(
            CODEX_HINT_COLOR.0, CODEX_HINT_COLOR.1,
            CODEX_HINT_COLOR.2, CODEX_HINT_COLOR.3,
        )),
    ));

    for (i, entry) in CODEX_ENTRIES.iter().enumerate() {
        let unlocked = gs.codex_unlocked.contains(&entry.scene_id.to_string());
        spawn_single_entry(list, font, i, entry, unlocked);
    }
}

/// Spawn a single codex entry row.
fn spawn_single_entry(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    entry: &CodexEntry,
    unlocked: bool,
) {
    let bg_color = if unlocked {
        Color::srgba(
            CODEX_ENTRY_BG.0, CODEX_ENTRY_BG.1,
            CODEX_ENTRY_BG.2, CODEX_ENTRY_BG.3,
        )
    } else {
        Color::srgba(
            CODEX_ENTRY_BG.0, CODEX_ENTRY_BG.1,
            CODEX_ENTRY_BG.2, CODEX_ENTRY_BG.3 * 0.5,
        )
    };

    let mut row = parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(CODEX_ENTRY_PAD)),
            border_radius: BorderRadius::all(Val::Px(CODEX_ENTRY_CORNER)),
            row_gap: Val::Px(4.0),
            width: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(bg_color),
        CodexEntryRow(index),
    ));

    if unlocked {
        row.insert(BoxShadow::new(
            Color::srgba(
                CODEX_UNLOCKED_GLOW.0, CODEX_UNLOCKED_GLOW.1,
                CODEX_UNLOCKED_GLOW.2, CODEX_UNLOCKED_GLOW.3,
            ),
            Val::ZERO, Val::ZERO,
            Val::Px(3.0), Val::Px(8.0),
        ));
    }

    row.with_children(|row_children| {
        if unlocked {
            spawn_unlocked_content(row_children, font, entry);
        } else {
            spawn_locked_content(row_children, font);
        }
    });
}

/// Spawn content for an unlocked codex entry.
fn spawn_unlocked_content(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    entry: &CodexEntry,
) {
    // Top row: name + pod
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        ..default()
    }).with_children(|top| {
        top.spawn((
            Text::new(entry.name),
            TextFont { font: font.clone(), font_size: CODEX_NAME_FONT, ..default() },
            TextColor(Color::srgb(
                CODEX_UNLOCKED_NAME_COLOR.0,
                CODEX_UNLOCKED_NAME_COLOR.1,
                CODEX_UNLOCKED_NAME_COLOR.2,
            )),
        ));
        top.spawn((
            Text::new(entry.pod),
            TextFont { font: font.clone(), font_size: CODEX_DETAIL_FONT, ..default() },
            TextColor(Color::srgb(
                CODEX_UNLOCKED_DETAIL_COLOR.0,
                CODEX_UNLOCKED_DETAIL_COLOR.1,
                CODEX_UNLOCKED_DETAIL_COLOR.2,
            )),
        ));
    });

    // Role + description
    parent.spawn((
        Text::new(format!("{} — {}", entry.role, entry.description)),
        TextFont { font: font.clone(), font_size: CODEX_DETAIL_FONT, ..default() },
        TextColor(Color::srgb(
            CODEX_UNLOCKED_DETAIL_COLOR.0,
            CODEX_UNLOCKED_DETAIL_COLOR.1,
            CODEX_UNLOCKED_DETAIL_COLOR.2,
        )),
    ));
}

/// Spawn content for a locked codex entry.
fn spawn_locked_content(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        ..default()
    }).with_children(|top| {
        top.spawn((
            Text::new("???"),
            TextFont { font: font.clone(), font_size: CODEX_NAME_FONT, ..default() },
            TextColor(Color::srgba(
                CODEX_LOCKED_COLOR.0, CODEX_LOCKED_COLOR.1,
                CODEX_LOCKED_COLOR.2, CODEX_LOCKED_COLOR.3,
            )),
        ));
        top.spawn((
            Text::new("Pod ???"),
            TextFont { font: font.clone(), font_size: CODEX_DETAIL_FONT, ..default() },
            TextColor(Color::srgba(
                CODEX_LOCKED_COLOR.0, CODEX_LOCKED_COLOR.1,
                CODEX_LOCKED_COLOR.2, CODEX_LOCKED_COLOR.3,
            )),
        ));
    });

    parent.spawn((
        Text::new("Crew member not yet encountered"),
        TextFont { font: font.clone(), font_size: CODEX_DETAIL_FONT, ..default() },
        TextColor(Color::srgba(
            CODEX_LOCKED_COLOR.0, CODEX_LOCKED_COLOR.1,
            CODEX_LOCKED_COLOR.2, CODEX_LOCKED_COLOR.3,
        )),
    ));
}
