// SPDX-License-Identifier: GPL-3.0-or-later
//! Pause menu overlay for player mode.
//! ESC during gameplay shows: Resume, Settings, Main Menu, Quit.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::types::{GameFont, UiBgFade};
use crate::ui_theme::{self, palette, typo, spacing};
use crate::i18n::Translations;
use crate::player_settings::{SettingsOverlay, SettingsOpenRequest};
use crate::simulation::SimulationOverlay;

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)] pub struct PauseScreen;
#[derive(Component)] pub struct PauseResumeBtn;
#[derive(Component)] pub struct PauseSettingsBtn;
#[derive(Component)] pub struct PauseMainMenuBtn;
#[derive(Component)] pub struct PauseQuitBtn;

// ─── Handle ESC → open/close pause ──────────────────────────────────────────

pub fn handle_pause(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    pause_q: Query<Entity, With<PauseScreen>>,
    settings_q: Query<Entity, With<SettingsOverlay>>,
    sim_overlay_q: Query<Entity, With<SimulationOverlay>>,
    font: Res<GameFont>,
    t: Res<Translations>,
) {
    if !keys.just_pressed(KeyCode::Escape) { return; }
    // Don't open pause if settings overlay is open (settings handles its own ESC)
    if !settings_q.is_empty() { return; }
    // Don't open pause if simulation overlay is showing
    if !sim_overlay_q.is_empty() { return; }

    if !pause_q.is_empty() {
        // Close existing pause screen
        close_pause(&mut commands, &pause_q);
        return;
    }

    spawn_pause(&mut commands, &font.0, &t);
}

// ─── Spawn pause overlay ─────────────────────────────────────────────────────

fn spawn_pause(commands: &mut Commands, font: &Handle<Font>, t: &Translations) {
    let scrim_alpha = palette::SCRIM.to_srgba().alpha;

    commands.spawn((
        PauseScreen,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        UiBgFade { target: scrim_alpha, despawn_at_zero: true },
        GlobalZIndex(200),
    )).with_children(|root| {
        // Centered panel
        root.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(spacing::RADIUS_LG)),
                min_width: Val::Px(300.0),
                ..default()
            },
            BackgroundColor(palette::PANEL),
        )).with_children(|panel| {
            // Title: "Paused"
            ui_theme::spawn_heading(panel, font,
                t.ui_or("paused", "Paused"), typo::H2, palette::TEXT_MAIN);

            panel.spawn(Node { height: Val::Px(8.0), ..default() });

            // Resume button (primary)
            ui_theme::spawn_button(panel, font, PauseResumeBtn,
                t.ui_or("resume", "Resume"));

            // Settings button (outline)
            ui_theme::spawn_button_outline(panel, font, PauseSettingsBtn,
                t.ui_or("settings", "Settings"));

            // Main Menu button (outline)
            ui_theme::spawn_button_outline(panel, font, PauseMainMenuBtn,
                t.ui_or("main_menu", "Main Menu"));

            panel.spawn(Node { height: Val::Px(4.0), ..default() });

            // Quit link
            ui_theme::spawn_link(panel, font, PauseQuitBtn,
                t.ui_or("quit", "Quit"), palette::LINK_DIM_TEXT);
        });
    });
}

fn close_pause(commands: &mut Commands, pause_q: &Query<Entity, With<PauseScreen>>) {
    for e in pause_q.iter() {
        commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
    }
}

// ─── Button interactions ─────────────────────────────────────────────────────

pub fn pause_interactions(
    mut commands: Commands,
    resume_q: Query<&Interaction, (With<PauseResumeBtn>, Changed<Interaction>)>,
    settings_q: Query<&Interaction, (With<PauseSettingsBtn>, Changed<Interaction>)>,
    main_menu_q: Query<&Interaction, (With<PauseMainMenuBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<PauseQuitBtn>, Changed<Interaction>)>,
    pause_screen: Query<(Entity, Option<&UiBgFade>), With<PauseScreen>>,
    mut exit: MessageWriter<AppExit>,
    mut settings_req: ResMut<SettingsOpenRequest>,
    mut next_state: ResMut<NextState<crate::player_menu::PlayerPhase>>,
) {
    if pause_screen.is_empty() { return; }
    // Don't process input while fading out
    let is_fading_out = pause_screen.iter().any(|(_, fade)| {
        fade.is_some_and(|f| f.despawn_at_zero && f.target < 0.01)
    });
    if is_fading_out { return; }

    // Resume
    if resume_q.iter().any(|i| *i == Interaction::Pressed) {
        for (e, _) in pause_screen.iter() {
            commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
        }
        return;
    }

    // Settings — close pause, open settings
    if settings_q.iter().any(|i| *i == Interaction::Pressed) {
        for (e, _) in pause_screen.iter() {
            commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
        }
        settings_req.0 = true;
        return;
    }

    // Main Menu — transition back
    if main_menu_q.iter().any(|i| *i == Interaction::Pressed) {
        for (e, _) in pause_screen.iter() {
            commands.entity(e).despawn();
        }
        next_state.set(crate::player_menu::PlayerPhase::MainMenu);
        return;
    }

    // Quit
    if quit_q.iter().any(|i| *i == Interaction::Pressed) {
        exit.write(AppExit::Success);
    }
}
