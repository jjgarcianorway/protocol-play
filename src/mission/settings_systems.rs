// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings overlay systems — input handling, fade, hover, and interactions.

use bevy::prelude::*;
use super::constants::*;
use super::settings::*;
use super::types::{MissionFont, GameScene};
use super::dashboard::DashboardSettingsBtn;
use crate::i18n::AVAILABLE_LANGUAGES;
use crate::save_state::{save_game_state, GameState};

// ── Core systems ───────────────────────────────────────────────────────

/// Toggle settings on ESC (Dashboard only); despawn when fade completes.
pub fn toggle_settings(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<SettingsOpen>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    confirm_q: Query<Entity, With<ConfirmDialog>>,
    mut commands: Commands,
    scene: Option<Res<State<GameScene>>>,
) {
    let on_dashboard = scene.map_or(true, |s| *s.get() == GameScene::Dashboard);
    if keys.just_pressed(KeyCode::Escape) && on_dashboard {
        if !confirm_q.is_empty() {
            for e in confirm_q.iter() { commands.entity(e).despawn(); }
            return;
        }
        if settings.open {
            settings.open = false;
        } else {
            settings.open = true;
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

/// Dashboard Settings button click — opens settings overlay.
pub fn dashboard_settings_btn_click(
    query: Query<&Interaction, (Changed<Interaction>, With<DashboardSettingsBtn>)>,
    mut settings: ResMut<SettingsOpen>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            settings.open = true;
        }
    }
}

// ── Navigation systems ─────────────────────────────────────────────────

/// Back to Main Menu button — return to main menu without resetting progress.
pub fn main_menu_btn_click(
    query: Query<&Interaction, (Changed<Interaction>, With<SettingsMainMenuBtn>)>,
    mut settings: ResMut<SettingsOpen>,
    mut next_phase: ResMut<NextState<super::types::AppPhase>>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    mut commands: Commands,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            settings.open = false;
            for e in overlay_q.iter() { commands.entity(e).despawn(); }
            next_phase.set(super::types::AppPhase::MainMenu);
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
    mut ship: ResMut<super::types::ShipStatus>,
    mut qs: ResMut<super::questions::QuestionState>,
    mut ds: ResMut<super::dialog_types::DialogState>,
    mut next_phase: ResMut<NextState<super::types::AppPhase>>,
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
            // Reload ShipStatus so dashboard reflects fresh state immediately
            ship.power = gs.power;
            ship.life_support = gs.life_support;
            ship.cryo = gs.cryo;
            ship.shields = gs.shields;
            ship.repair = gs.repair;
            ship.crystals = gs.total_crystals();
            ship.crew_count = gs.crew_count;
            ship.day = gs.day;
            ship.distance_au = gs.distance_au;
            ship.bot_level = gs.bot_level;
            // Reset dialog/question checks so intro fires fresh
            super::questions::reset_question_check(&mut qs);
            super::dialog_system::reset_dialog_check(&mut ds);
            settings.open = false;
            for e in dialog_q.iter() { commands.entity(e).despawn(); }
            // Return to main menu so player starts clean
            next_phase.set(super::types::AppPhase::MainMenu);
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
