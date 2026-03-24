// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu — left panel with controls, 3D board visible on the right.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::types::GameFont;
use crate::i18n::Translations;
use crate::ui_theme::{self, palette, typo, spacing};

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerPhase { #[default] MainMenu, Playing }

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)] pub struct MenuRoot;
#[derive(Component)] pub struct MenuContinueBtn;
#[derive(Component)] pub struct MenuNewGameBtn;
#[derive(Component)] pub struct MenuQuitBtn;
#[derive(Component)] pub struct MenuSettingsBtn;

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn enter_menu(mut commands: Commands, font: Res<GameFont>, t: Res<Translations>) {
    let f = &font.0;
    let has_save = check_has_progress();

    commands.spawn((MenuRoot, Node {
        width: Val::Percent(100.0), height: Val::Percent(100.0), ..default()
    })).with_children(|root| {
        // ── Left panel ──
        root.spawn((Node {
            width: Val::Percent(42.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
            padding: UiRect { left: Val::Px(40.0), right: Val::Px(40.0),
                top: Val::Vh(6.0), bottom: Val::Vh(4.0) },
            ..default()
        }, BackgroundColor(palette::PANEL),
        )).with_children(|panel| {
            panel.spawn(Node { flex_grow: 1.0, ..default() });

            // Title block
            ui_theme::spawn_heading(panel, f, "protocol play", typo::H1, palette::TEXT_MAIN);
            ui_theme::spawn_label(panel, f, "puzzle", typo::H2, palette::TEXT_SUB, 2.0);
            ui_theme::spawn_label(panel, f,
                &t.ui_or("tagline", "every connection matters"),
                typo::CAPTION, palette::TEXT_MUTED, 12.0);

            // Buttons
            panel.spawn(Node { height: Val::Px(44.0), ..default() });

            if has_save {
                ui_theme::spawn_button(panel, f, MenuContinueBtn,
                    &t.ui_or("continue", "Continue"));
                panel.spawn(Node { height: Val::Px(10.0), ..default() });
                ui_theme::spawn_button_secondary(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            } else {
                ui_theme::spawn_button(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            }

            panel.spawn(Node { flex_grow: 1.2, ..default() });

            // Footer
            panel.spawn(Node {
                flex_direction: FlexDirection::Row, column_gap: Val::Px(32.0),
                justify_content: JustifyContent::Center, width: Val::Percent(100.0),
                margin: UiRect::bottom(Val::Px(14.0)), ..default()
            }).with_children(|row| {
                ui_theme::spawn_link(row, f, MenuSettingsBtn,
                    &t.ui_or("settings", "Settings"), palette::LINK_TEXT);
                ui_theme::spawn_link(row, f, MenuQuitBtn,
                    &t.ui_or("quit", "Quit"), palette::LINK_DIM_TEXT);
            });
            panel.spawn((
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
                TextFont { font: f.clone(), font_size: typo::MICRO, ..default() },
                TextColor(palette::TEXT_DIM),
            ));
        });

        // ── Right side: subtle overlay so board shows through ──
        root.spawn((Node {
            width: Val::Percent(58.0), height: Val::Percent(100.0), ..default()
        }, BackgroundColor(palette::OVERLAY)));
    });
}

// ─── Update ──────────────────────────────────────────────────────────────────

pub fn menu_interaction(
    continue_q: Query<&Interaction, (With<MenuContinueBtn>, Changed<Interaction>)>,
    new_game_q: Query<&Interaction, (With<MenuNewGameBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<MenuQuitBtn>, Changed<Interaction>)>,
    settings_q: Query<&Interaction, (With<MenuSettingsBtn>, Changed<Interaction>)>,
    mut next: ResMut<NextState<PlayerPhase>>,
    mut exit: MessageWriter<AppExit>,
    mut settings_req: ResMut<crate::player_settings::SettingsOpenRequest>,
) {
    if continue_q.iter().any(|i| *i == Interaction::Pressed) { next.set(PlayerPhase::Playing); }
    if new_game_q.iter().any(|i| *i == Interaction::Pressed) { next.set(PlayerPhase::Playing); }
    if quit_q.iter().any(|i| *i == Interaction::Pressed) { exit.write(AppExit::Success); }
    if settings_q.iter().any(|i| *i == Interaction::Pressed) { settings_req.0 = true; }
}

pub fn exit_menu(mut commands: Commands, q: Query<Entity, With<MenuRoot>>) {
    for e in q.iter() { commands.entity(e).despawn(); }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn check_has_progress() -> bool {
    let dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    std::fs::read_dir(&dir).ok()
        .map(|entries| entries.filter_map(|e| e.ok())
            .any(|e| e.file_name().to_string_lossy().ends_with(".progress.json")))
        .unwrap_or(false)
}
