// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu for the standalone bot puzzle game (player mode).
//! Polished layout with a semi-transparent overlay over the 3D board background.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::ui_helpers::gf;
use crate::types::GameFont;
use crate::i18n::Translations;
use crate::constants::*;

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

#[derive(Component)]
pub struct MenuQuitBtn;

#[derive(Component)]
pub struct MenuSettingsBtn;

/// Marker for buttons that should get hover effects.
#[derive(Component)]
pub struct MenuHoverable {
    pub normal: Color,
    pub hovered: Color,
}

// ─── Setup ───────────────────────────────────────────────────────────────────

/// Startup: spawn the menu UI.
pub fn enter_menu(
    mut commands: Commands,
    font: Res<GameFont>,
    t: Res<Translations>,
) {
    let f = &font.0;
    let has_progress = check_has_progress();

    // Root: full-screen semi-transparent overlay (the 3D board IS the background)
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
        BackgroundColor(Color::srgba(MENU_BG.0, MENU_BG.1, MENU_BG.2, MENU_OVERLAY_ALPHA)),
    )).with_children(|root| {
        // Title
        root.spawn((
            Text::new("protocol play"),
            TextFont { font: f.clone(), font_size: MENU_TITLE_SIZE, ..default() },
            TextColor(Color::srgba(0.92, 0.93, 0.96, 1.0)),
        ));

        // Subtitle: "puzzle"
        root.spawn((
            Text::new("puzzle"),
            TextFont { font: f.clone(), font_size: MENU_SUBTITLE_SIZE, ..default() },
            TextColor(Color::srgba(0.45, 0.50, 0.62, 1.0)),
            Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
        ));

        // Tagline
        root.spawn((
            Text::new(t.ui_or("tagline", "every connection matters")),
            TextFont { font: f.clone(), font_size: MENU_TAGLINE_SIZE, ..default() },
            TextColor(Color::srgba(0.35, 0.38, 0.48, 0.80)),
            Node { margin: UiRect::top(Val::Px(16.0)), ..default() },
        ));

        // Spacer
        root.spawn(Node { height: Val::Px(44.0), ..default() });

        // Continue button (only if there's saved progress)
        if has_progress {
            let normal = Color::srgba(MENU_BTN_PRIMARY.0, MENU_BTN_PRIMARY.1, MENU_BTN_PRIMARY.2, MENU_BTN_PRIMARY.3);
            let hovered = Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3);
            root.spawn((
                Button, MenuContinueBtn,
                MenuHoverable { normal, hovered },
                Node {
                    padding: UiRect::axes(Val::Px(56.0), Val::Px(14.0)),
                    border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)),
                    margin: UiRect::bottom(Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(normal),
            )).with_child((
                Text::new(t.ui_or("continue", "Continue")),
                TextFont { font: f.clone(), font_size: MENU_BTN_FONT, ..default() },
                TextColor(Color::WHITE),
            ));
        }

        // New Game button
        let (ng_normal, ng_hovered, ng_alpha) = if has_progress {
            (
                Color::srgba(MENU_BTN_SECONDARY.0, MENU_BTN_SECONDARY.1, MENU_BTN_SECONDARY.2, MENU_BTN_SECONDARY.3),
                Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3 * 0.7),
                0.70,
            )
        } else {
            (
                Color::srgba(MENU_BTN_PRIMARY.0, MENU_BTN_PRIMARY.1, MENU_BTN_PRIMARY.2, MENU_BTN_PRIMARY.3),
                Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3),
                1.0,
            )
        };
        root.spawn((
            Button, MenuNewGameBtn,
            MenuHoverable { normal: ng_normal, hovered: ng_hovered },
            Node {
                padding: UiRect::axes(Val::Px(44.0), Val::Px(11.0)),
                border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)),
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(ng_normal),
        )).with_child((
            Text::new(t.ui_or("new_game", "New Game")),
            TextFont { font: f.clone(), font_size: MENU_BTN_SMALL_FONT, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, ng_alpha)),
        ));

        // Spacer before bottom area
        root.spawn(Node { flex_grow: 1.0, ..default() });

        // Bottom area: settings + quit
        root.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(30.0)),
            row_gap: Val::Px(8.0),
            ..default()
        }).with_children(|bottom| {
            // Settings button (text-only, small)
            let settings_normal = Color::NONE;
            let settings_hover = Color::srgba(1.0, 1.0, 1.0, 0.08);
            bottom.spawn((
                Button, MenuSettingsBtn,
                MenuHoverable { normal: settings_normal, hovered: settings_hover },
                Node {
                    padding: UiRect::axes(Val::Px(20.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)),
                    ..default()
                },
                BackgroundColor(settings_normal),
            )).with_child((
                Text::new(t.ui_or("settings", "Settings")),
                TextFont { font: f.clone(), font_size: MENU_BTN_SMALL_FONT, ..default() },
                TextColor(Color::srgba(0.6, 0.65, 0.75, 0.80)),
            ));

            // Quit button (text-only, small)
            let quit_normal = Color::NONE;
            let quit_hover = Color::srgba(1.0, 1.0, 1.0, 0.08);
            bottom.spawn((
                Button, MenuQuitBtn,
                MenuHoverable { normal: quit_normal, hovered: quit_hover },
                Node {
                    padding: UiRect::axes(Val::Px(20.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)),
                    ..default()
                },
                BackgroundColor(quit_normal),
            )).with_child((
                Text::new(t.ui_or("quit", "Quit")),
                TextFont { font: f.clone(), font_size: MENU_BTN_SMALL_FONT, ..default() },
                TextColor(Color::srgba(0.5, 0.5, 0.55, 0.60)),
            ));

            // Version
            bottom.spawn((
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
                TextFont { font: f.clone(), font_size: VERSION_FONT, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.25)),
                Node { margin: UiRect::top(Val::Px(8.0)), ..default() },
            ));
        });
    });
}

// ─── Update ──────────────────────────────────────────────────────────────────

/// Handle menu button clicks.
pub fn menu_interaction(
    continue_q: Query<&Interaction, (With<MenuContinueBtn>, Changed<Interaction>)>,
    new_game_q: Query<&Interaction, (With<MenuNewGameBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<MenuQuitBtn>, Changed<Interaction>)>,
    settings_q: Query<&Interaction, (With<MenuSettingsBtn>, Changed<Interaction>)>,
    mut next: ResMut<NextState<PlayerPhase>>,
    mut exit: MessageWriter<AppExit>,
    mut settings_req: ResMut<crate::player_settings::SettingsOpenRequest>,
) {
    if continue_q.iter().any(|i| *i == Interaction::Pressed) {
        next.set(PlayerPhase::Playing);
    }
    if new_game_q.iter().any(|i| *i == Interaction::Pressed) {
        // TODO: reset progress / new seed
        next.set(PlayerPhase::Playing);
    }
    if quit_q.iter().any(|i| *i == Interaction::Pressed) {
        exit.write(AppExit::Success);
    }
    if settings_q.iter().any(|i| *i == Interaction::Pressed) {
        settings_req.0 = true;
    }
}

/// Hover effect: change BackgroundColor on Interaction::Hovered for MenuHoverable buttons.
pub fn menu_hover(
    mut q: Query<(&Interaction, &MenuHoverable, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, hoverable, mut bg) in q.iter_mut() {
        match interaction {
            Interaction::Hovered => { bg.0 = hoverable.hovered; }
            Interaction::None => { bg.0 = hoverable.normal; }
            Interaction::Pressed => {} // keep hover color while pressing
        }
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
