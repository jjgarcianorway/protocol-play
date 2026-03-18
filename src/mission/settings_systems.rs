// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings overlay systems — input handling, fade, hover, and interactions.

use bevy::prelude::*;
use super::constants::*;
use super::settings::*;
use super::types::MissionFont;
use crate::i18n::AVAILABLE_LANGUAGES;
use crate::save_state::{save_game_state, GameState};

// ── Core systems ───────────────────────────────────────────────────────

/// Toggle settings on ESC; despawn when fade completes.
pub fn toggle_settings(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<SettingsOpen>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if !confirm_q.is_empty() {
            for e in confirm_q.iter() { commands.entity(e).despawn(); }
            return;
        }
        if settings.open {
            settings.open = false;
        }
    }
    if !settings.open && settings.fade <= 0.0 {
        for entity in overlay_q.iter() {
            commands.entity(entity).despawn();
        }
    }
}

/// Animate fade in/out; spawn overlay on open if absent.
pub fn animate_settings_fade(
    time: Res<Time>,
    mut settings: ResMut<SettingsOpen>,
    mut overlay_q: Query<&mut BackgroundColor, With<SettingsOverlay>>,
    font: Res<MissionFont>,
    gs: Res<GameState>,
    tab: Res<ActiveSettingsTab>,
    existing: Query<Entity, With<SettingsOverlay>>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    if settings.open {
        if existing.is_empty() {
            spawn_overlay(&mut commands, &font.0, tab.0, &gs);
        }
        settings.fade = (settings.fade + dt / SETTINGS_FADE_DURATION).min(1.0);
    } else {
        settings.fade = (settings.fade - dt / SETTINGS_FADE_DURATION).max(0.0);
    }

    for mut bg in overlay_q.iter_mut() {
        let a = SETTINGS_OVERLAY_BG.3 * settings.fade;
        *bg = BackgroundColor(Color::srgba(
            SETTINGS_OVERLAY_BG.0, SETTINGS_OVERLAY_BG.1, SETTINGS_OVERLAY_BG.2, a,
        ));
    }
}

/// Dismiss when clicking the overlay background (outside the panel).
pub fn dismiss_on_bg_click(
    overlay_q: Query<&Interaction, (Changed<Interaction>, With<SettingsOverlay>)>,
    mut settings: ResMut<SettingsOpen>,
) {
    for interaction in overlay_q.iter() {
        if *interaction == Interaction::Pressed {
            settings.open = false;
        }
    }
}

// ── Tab systems ────────────────────────────────────────────────────────

/// Switch tabs — rebuild the overlay with the new tab.
pub fn tab_click(
    query: Query<(&Interaction, &SettingsTabBtn), Changed<Interaction>>,
    mut tab: ResMut<ActiveSettingsTab>,
    mut commands: Commands,
    font: Res<MissionFont>,
    gs: Res<GameState>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && btn.0 != tab.0 {
            tab.0 = btn.0;
            for e in overlay_q.iter() { commands.entity(e).despawn(); }
            spawn_overlay(&mut commands, &font.0, tab.0, &gs);
        }
    }
}

/// Tab hover effects.
pub fn tab_hover(
    mut query: Query<
        (&Interaction, &SettingsTabBtn, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    tab: Res<ActiveSettingsTab>,
) {
    for (interaction, btn, mut bg) in query.iter_mut() {
        let is_active = btn.0 == tab.0;
        match interaction {
            Interaction::Hovered if !is_active => {
                *bg = BackgroundColor(Color::srgba(
                    SETTINGS_TAB_ACTIVE_BG.0, SETTINGS_TAB_ACTIVE_BG.1,
                    SETTINGS_TAB_ACTIVE_BG.2, 0.6,
                ));
            }
            Interaction::None if !is_active => {
                *bg = BackgroundColor(Color::srgba(
                    SETTINGS_TAB_BG.0, SETTINGS_TAB_BG.1,
                    SETTINGS_TAB_BG.2, SETTINGS_TAB_BG.3,
                ));
            }
            _ => {}
        }
    }
}

// ── Language systems ───────────────────────────────────────────────────

/// Handle language selection — update GameState and rebuild overlay.
pub fn language_click(
    query: Query<(&Interaction, &SettingsLangBtn), Changed<Interaction>>,
    mut gs: ResMut<GameState>,
    mut commands: Commands,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    font: Res<MissionFont>,
    tab: Res<ActiveSettingsTab>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(&(code, _)) = AVAILABLE_LANGUAGES.get(btn.0) {
                if gs.language != code {
                    gs.language = code.to_string();
                    save_game_state(&gs);
                    for e in overlay_q.iter() { commands.entity(e).despawn(); }
                    spawn_overlay(&mut commands, &font.0, tab.0, &gs);
                }
            }
        }
    }
}

/// Language button hover.
pub fn lang_btn_hover(
    mut query: Query<
        (&Interaction, &SettingsLangBtn, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    gs: Res<GameState>,
) {
    for (interaction, btn, mut bg, mut border) in query.iter_mut() {
        let code = AVAILABLE_LANGUAGES.get(btn.0).map(|l| l.0).unwrap_or("");
        if gs.language == code { continue; }
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

// ── Reset systems ──────────────────────────────────────────────────────

/// Reset button click — spawn confirmation dialog.
pub fn reset_click(
    query: Query<&Interaction, (Changed<Interaction>, With<SettingsResetBtn>)>,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    mut commands: Commands,
    font: Res<MissionFont>,
) {
    if !confirm_q.is_empty() { return; }
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            spawn_confirm_dialog(&mut commands, &font.0);
        }
    }
}

/// Handle confirm/cancel on the reset confirmation dialog.
pub fn confirm_reset_click(
    confirm_q: Query<&Interaction, (Changed<Interaction>, With<SettingsConfirmReset>)>,
    cancel_q: Query<&Interaction, (Changed<Interaction>, With<SettingsCancelReset>)>,
    dialog_q: Query<Entity, With<ConfirmDialog>>,
    mut commands: Commands,
    mut gs: ResMut<GameState>,
    mut settings: ResMut<SettingsOpen>,
) {
    for interaction in cancel_q.iter() {
        if *interaction == Interaction::Pressed {
            for e in dialog_q.iter() { commands.entity(e).despawn(); }
        }
    }
    for interaction in confirm_q.iter() {
        if *interaction == Interaction::Pressed {
            crate::save_state::reset_for_new_game(&mut gs);
            save_game_state(&gs);
            settings.open = false;
            for e in dialog_q.iter() { commands.entity(e).despawn(); }
        }
    }
}

/// Reset button hover.
pub fn reset_btn_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SettingsResetBtn>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(0.35, 0.10, 0.10, 0.95));
                *border = BorderColor::all(Color::srgba(0.9, 0.3, 0.3, 0.8));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(
                    SETTINGS_DANGER_BG.0, SETTINGS_DANGER_BG.1,
                    SETTINGS_DANGER_BG.2, SETTINGS_DANGER_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    SETTINGS_DANGER_BORDER.0, SETTINGS_DANGER_BORDER.1,
                    SETTINGS_DANGER_BORDER.2, SETTINGS_DANGER_BORDER.3,
                ));
            }
        }
    }
}
