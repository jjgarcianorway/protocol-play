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
pub struct MenuPlayBtn;

// ─── Systems ─────────────────────────────────────────────────────────────────

/// OnEnter(MainMenu): spawn the menu UI.
pub fn enter_menu(
    mut commands: Commands,
    font: Res<GameFont>,
    t: Res<Translations>,
) {
    let f = &font.0;

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

        // Subtitle
        root.spawn((
            Text::new("repairing"),
            TextFont { font: f.clone(), font_size: 18.0, ..default() },
            TextColor(Color::srgba(0.45, 0.50, 0.62, 1.0)),
            Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
        ));

        // Spacer
        root.spawn(Node { height: Val::Px(48.0), ..default() });

        // Play button
        root.spawn((
            Button, MenuPlayBtn,
            Node {
                padding: UiRect::axes(Val::Px(52.0), Val::Px(14.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.22, 0.42, 0.68, 0.90)),
        )).with_child((
            Text::new(t.ui_or("play", "Play")),
            TextFont { font: f.clone(), font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
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

/// Handle Play button click.
pub fn menu_interaction(
    btn_q: Query<&Interaction, (With<MenuPlayBtn>, Changed<Interaction>)>,
    mut next: ResMut<NextState<PlayerPhase>>,
) {
    if btn_q.iter().any(|i| *i == Interaction::Pressed) {
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
