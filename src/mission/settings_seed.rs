// SPDX-License-Identifier: GPL-3.0-or-later

//! Custom seed input — UI spawn helpers and keyboard/button systems.
//! Called from `settings.rs` (Game tab) and registered in `mod.rs`.

use bevy::prelude::*;
use super::constants::*;
use super::settings::*;
use super::types::MissionFont;
use crate::save_state::{reset_for_custom_seed, save_game_state, GameState};

// ── Resources ──────────────────────────────────────────────────────────

/// Tracks the text being typed into the custom seed field.
#[derive(Resource)]
pub struct SeedInputState {
    pub text: String,
    pub active: bool,
}

impl Default for SeedInputState {
    fn default() -> Self {
        Self { text: String::new(), active: false }
    }
}

// ── Components ─────────────────────────────────────────────────────────

/// The text input field background (clickable to activate).
#[derive(Component)]
pub struct SeedInputField;

/// The text display inside the input field.
#[derive(Component)]
pub struct SeedInputText;

/// The "Apply Seed" button.
#[derive(Component)]
pub struct SeedApplyBtn;

/// Status text shown below the input (errors/success).
#[derive(Component)]
pub struct SeedStatusText;

// ── Spawn helpers (called from settings.rs spawn_game_content) ─────────

/// Spawn the custom seed input section inside the Game tab.
pub fn spawn_seed_input(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
) {
    // Section label
    parent.spawn((
        Text::new("Custom Seed"),
        TextFont { font: font.clone(), font_size: SETTINGS_BODY_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_BODY_COLOR.0, SETTINGS_BODY_COLOR.1, SETTINGS_BODY_COLOR.2,
        )),
    ));

    // Row: input field + apply button
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.0),
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|row| {
        // Text input field
        row.spawn((
            Button, SeedInputField,
            Node {
                width: Val::Px(260.0),
                height: Val::Px(34.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(SETTINGS_BTN_CORNER)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.09, 0.14, 0.95)),
            BorderColor::all(Color::srgba(
                SETTINGS_BTN_BORDER.0, SETTINGS_BTN_BORDER.1,
                SETTINGS_BTN_BORDER.2, SETTINGS_BTN_BORDER.3,
            )),
        )).with_child((
            SeedInputText,
            Text::new("Click to type a hex seed..."),
            TextFont { font: font.clone(), font_size: 13.0, ..default() },
            TextColor(Color::srgba(0.45, 0.5, 0.6, 0.7)),
        ));

        // Apply button
        row.spawn((
            Button, SeedApplyBtn,
            Node {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(SETTINGS_BTN_CORNER)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                SETTINGS_BTN_BG.0, SETTINGS_BTN_BG.1,
                SETTINGS_BTN_BG.2, SETTINGS_BTN_BG.3,
            )),
            BorderColor::all(Color::srgba(
                SETTINGS_BTN_BORDER.0, SETTINGS_BTN_BORDER.1,
                SETTINGS_BTN_BORDER.2, SETTINGS_BTN_BORDER.3,
            )),
        )).with_child((
            Text::new("Apply Seed"),
            TextFont { font: font.clone(), font_size: SETTINGS_BTN_FONT, ..default() },
            TextColor(Color::srgb(
                SETTINGS_BTN_COLOR.0, SETTINGS_BTN_COLOR.1, SETTINGS_BTN_COLOR.2,
            )),
        ));
    });

    // Status text (initially empty, updated by systems)
    parent.spawn((
        SeedStatusText,
        Text::new(""),
        TextFont { font: font.clone(), font_size: SETTINGS_NOTE_FONT, ..default() },
        TextColor(Color::srgba(0.45, 0.5, 0.6, 0.0)),
        Node { margin: UiRect::top(Val::Px(2.0)), ..default() },
    ));

    // Share note
    parent.spawn((
        Text::new("Share your seed with friends to play the same world"),
        TextFont { font: font.clone(), font_size: SETTINGS_NOTE_FONT, ..default() },
        TextColor(Color::srgb(
            SETTINGS_NOTE_COLOR.0, SETTINGS_NOTE_COLOR.1, SETTINGS_NOTE_COLOR.2,
        )),
        Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
    ));
}

// ── Systems ────────────────────────────────────────────────────────────

/// Activate/deactivate the seed input field on click.
pub fn seed_input_click(
    query: Query<&Interaction, (Changed<Interaction>, With<SeedInputField>)>,
    mut state: ResMut<SeedInputState>,
    mut border_q: Query<&mut BorderColor, With<SeedInputField>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            state.active = true;
            for mut border in border_q.iter_mut() {
                *border = BorderColor::all(Color::srgba(0.4, 0.6, 0.9, 0.9));
            }
        }
    }
}

/// Deactivate when clicking elsewhere (overlay bg click or tab switch).
pub fn seed_input_deactivate(
    overlay_q: Query<&Interaction, (Changed<Interaction>, With<SettingsOverlay>)>,
    mut state: ResMut<SeedInputState>,
    mut border_q: Query<&mut BorderColor, With<SeedInputField>>,
) {
    for interaction in overlay_q.iter() {
        if *interaction == Interaction::Pressed {
            state.active = false;
            for mut border in border_q.iter_mut() {
                *border = BorderColor::all(Color::srgba(
                    SETTINGS_BTN_BORDER.0, SETTINGS_BTN_BORDER.1,
                    SETTINGS_BTN_BORDER.2, SETTINGS_BTN_BORDER.3,
                ));
            }
        }
    }
}

/// Capture keyboard input when the seed field is active.
pub fn seed_keyboard_input(
    mut state: ResMut<SeedInputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<SeedInputText>>,
) {
    if !state.active { return; }

    let mut changed = false;

    // Backspace
    if keys.just_pressed(KeyCode::Backspace) {
        state.text.pop();
        changed = true;
    }

    // Hex characters: 0-9, A-F
    const HEX_KEYS: &[(KeyCode, char)] = &[
        (KeyCode::Digit0, '0'), (KeyCode::Digit1, '1'),
        (KeyCode::Digit2, '2'), (KeyCode::Digit3, '3'),
        (KeyCode::Digit4, '4'), (KeyCode::Digit5, '5'),
        (KeyCode::Digit6, '6'), (KeyCode::Digit7, '7'),
        (KeyCode::Digit8, '8'), (KeyCode::Digit9, '9'),
        (KeyCode::KeyA, 'A'), (KeyCode::KeyB, 'B'),
        (KeyCode::KeyC, 'C'), (KeyCode::KeyD, 'D'),
        (KeyCode::KeyE, 'E'), (KeyCode::KeyF, 'F'),
    ];

    for &(key, ch) in HEX_KEYS {
        if keys.just_pressed(key) && state.text.len() < 16 {
            state.text.push(ch);
            changed = true;
        }
    }

    // Escape — deactivate
    if keys.just_pressed(KeyCode::Escape) {
        state.active = false;
        // Don't return — let the toggle_settings handle the overlay
    }

    if changed {
        for (mut text, mut color) in text_q.iter_mut() {
            if state.text.is_empty() {
                **text = "Click to type a hex seed...".into();
                *color = TextColor(Color::srgba(0.45, 0.5, 0.6, 0.7));
            } else {
                **text = state.text.clone().into();
                *color = TextColor(Color::srgb(0.85, 0.88, 0.95));
            }
        }
    }
}

/// Handle Apply Seed button click.
pub fn seed_apply_click(
    query: Query<&Interaction, (Changed<Interaction>, With<SeedApplyBtn>)>,
    mut state: ResMut<SeedInputState>,
    mut gs: ResMut<GameState>,
    mut settings: ResMut<SettingsOpen>,
    mut status_q: Query<(&mut Text, &mut TextColor), With<SeedStatusText>>,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    mut commands: Commands,
    _font: Res<MissionFont>,
) {
    // Don't process if confirm dialog is open
    if !confirm_q.is_empty() { return; }

    for interaction in query.iter() {
        if *interaction != Interaction::Pressed { continue; }

        if state.text.is_empty() {
            for (mut text, mut color) in status_q.iter_mut() {
                **text = "Enter a hex seed first".into();
                *color = TextColor(Color::srgba(0.9, 0.5, 0.3, 1.0));
            }
            return;
        }

        match u64::from_str_radix(&state.text, 16) {
            Ok(seed) if seed != 0 => {
                // Apply the custom seed — triggers a new world
                reset_for_custom_seed(&mut gs, seed);
                save_game_state(&gs);
                state.text.clear();
                state.active = false;
                // Rebuild overlay to show new seed
                let overlay_q =
                    commands.spawn_empty().id(); // placeholder — we close settings
                let _ = overlay_q; // suppress warning
                settings.open = false;
            }
            Ok(_) => {
                for (mut text, mut color) in status_q.iter_mut() {
                    **text = "Seed cannot be zero".into();
                    *color = TextColor(Color::srgba(0.9, 0.5, 0.3, 1.0));
                }
            }
            Err(_) => {
                for (mut text, mut color) in status_q.iter_mut() {
                    **text = "Invalid hex value".into();
                    *color = TextColor(Color::srgba(0.9, 0.5, 0.3, 1.0));
                }
            }
        }
    }
}

/// Hover effect for the Apply Seed button.
pub fn seed_apply_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SeedApplyBtn>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(
                    SETTINGS_BTN_HOVER_BG.0, SETTINGS_BTN_HOVER_BG.1,
                    SETTINGS_BTN_HOVER_BG.2, SETTINGS_BTN_HOVER_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    SETTINGS_BTN_HOVER_BORDER.0, SETTINGS_BTN_HOVER_BORDER.1,
                    SETTINGS_BTN_HOVER_BORDER.2, SETTINGS_BTN_HOVER_BORDER.3,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(
                    SETTINGS_BTN_BG.0, SETTINGS_BTN_BG.1,
                    SETTINGS_BTN_BG.2, SETTINGS_BTN_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    SETTINGS_BTN_BORDER.0, SETTINGS_BTN_BORDER.1,
                    SETTINGS_BTN_BORDER.2, SETTINGS_BTN_BORDER.3,
                ));
            }
        }
    }
}
