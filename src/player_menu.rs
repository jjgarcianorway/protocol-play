// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu for the standalone bot puzzle game (player mode).
//! Clean, minimal aesthetic — looks like a polished puzzle app.

use bevy::prelude::*;
use crate::ui_helpers::gf;
use crate::types::GameFont;
use crate::i18n::Translations;

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerPhase {
    #[default]
    MainMenu,
    Playing,
}

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct MenuContinueBtn;

#[derive(Component)]
pub struct MenuNewGameBtn;

// ─── Setup ───────────────────────────────────────────────────────────────────

/// Startup: spawn the menu UI.
pub fn enter_menu(
    mut commands: Commands,
    font: Res<GameFont>,
    t: Res<Translations>,
) {
    let f = &font.0;
    let has_progress = check_has_progress();

    commands.spawn((
        MenuRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.06, 0.06, 0.10)),
    )).with_children(|root| {
        // Title
        root.spawn((
            Text::new("protocol play"),
            TextFont { font: f.clone(), font_size: 42.0, ..default() },
            TextColor(Color::srgba(0.92, 0.93, 0.96, 1.0)),
        ));

        // Subtitle: "puzzle"
        root.spawn((
            Text::new("puzzle"),
            TextFont { font: f.clone(), font_size: 18.0, ..default() },
            TextColor(Color::srgba(0.45, 0.50, 0.62, 1.0)),
            Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
        ));

        // Tagline
        root.spawn((
            Text::new(t.ui_or("tagline", "every connection matters")),
            TextFont { font: f.clone(), font_size: 13.0, ..default() },
            TextColor(Color::srgba(0.35, 0.38, 0.48, 1.0)),
            Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
        ));

        // Spacer
        root.spawn(Node { height: Val::Px(44.0), ..default() });

        // Continue button (only if there's saved progress)
        if has_progress {
            root.spawn((
                Button, MenuContinueBtn,
                Node {
                    padding: UiRect::axes(Val::Px(52.0), Val::Px(14.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    margin: UiRect::bottom(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.22, 0.42, 0.68, 0.90)),
            )).with_child((
                Text::new(t.ui_or("continue", "Continue")),
                TextFont { font: f.clone(), font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
        }

        // New Game button
        let new_game_bg = if has_progress {
            Color::srgba(0.18, 0.20, 0.26, 0.70) // subtle if Continue exists
        } else {
            Color::srgba(0.22, 0.42, 0.68, 0.90) // primary if no progress
        };
        root.spawn((
            Button, MenuNewGameBtn,
            Node {
                padding: UiRect::axes(Val::Px(42.0), Val::Px(11.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(new_game_bg),
        )).with_child((
            Text::new(t.ui_or("new_game", "New Game")),
            TextFont { font: f.clone(), font_size: 16.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, if has_progress { 0.70 } else { 1.0 })),
        ));

        // Version
        root.spawn((
            Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
            TextFont { font: f.clone(), font_size: 11.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            Node { margin: UiRect::top(Val::Px(40.0)), ..default() },
        ));
    });
}

// ─── Update ──────────────────────────────────────────────────────────────────

/// Handle menu button clicks.
pub fn menu_interaction(
    continue_q: Query<&Interaction, (With<MenuContinueBtn>, Changed<Interaction>)>,
    new_game_q: Query<&Interaction, (With<MenuNewGameBtn>, Changed<Interaction>)>,
    mut next: ResMut<NextState<PlayerPhase>>,
) {
    if continue_q.iter().any(|i| *i == Interaction::Pressed) {
        next.set(PlayerPhase::Playing);
    }
    if new_game_q.iter().any(|i| *i == Interaction::Pressed) {
        // TODO: reset progress / new seed
        next.set(PlayerPhase::Playing);
    }
}

/// OnExit(MainMenu): despawn menu UI.
pub fn exit_menu(
    mut commands: Commands,
    q: Query<Entity, With<MenuRoot>>,
) {
    for e in q.iter() { commands.entity(e).despawn(); }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Check if there's any saved progress (at least one completed level).
fn check_has_progress() -> bool {
    let dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    std::fs::read_dir(&dir).ok()
        .map(|entries| entries
            .filter_map(|e| e.ok())
            .any(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.ends_with(".progress.json")
            }))
        .unwrap_or(false)
}
