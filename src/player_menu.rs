// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu — left panel with controls, right side shows the 3D board.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::types::GameFont;
use crate::i18n::Translations;
use crate::constants::*;

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerPhase { #[default] MainMenu, Playing }

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)] pub struct MenuRoot;
#[derive(Component)] pub struct MenuContinueBtn;
#[derive(Component)] pub struct MenuNewGameBtn;
#[derive(Component)] pub struct MenuQuitBtn;
#[derive(Component)] pub struct MenuSettingsBtn;
#[derive(Component)] pub struct MenuHoverable { pub normal: Color, pub hovered: Color }

// ─── Colors ──────────────────────────────────────────────────────────────────

fn primary() -> Color { Color::srgba(MENU_BTN_PRIMARY.0, MENU_BTN_PRIMARY.1, MENU_BTN_PRIMARY.2, MENU_BTN_PRIMARY.3) }
fn secondary() -> Color { Color::srgba(MENU_BTN_SECONDARY.0, MENU_BTN_SECONDARY.1, MENU_BTN_SECONDARY.2, MENU_BTN_SECONDARY.3) }
fn hover() -> Color { Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3) }
fn hover_dim() -> Color { Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, 0.55) }
fn link_color() -> Color { Color::srgba(0.65, 0.72, 0.80, 0.90) }
fn link_dim() -> Color { Color::srgba(0.50, 0.55, 0.62, 0.70) }
fn link_hover() -> Color { Color::srgba(1.0, 1.0, 1.0, 0.08) }
const PANEL_BG: Color = Color::srgba(0.04, 0.05, 0.09, 0.82);

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn enter_menu(mut commands: Commands, font: Res<GameFont>, t: Res<Translations>) {
    let f = &font.0;
    let has_save = check_has_progress();

    // Root: full screen, no background (3D board shows through on the right)
    commands.spawn((MenuRoot, Node {
        width: Val::Percent(100.0), height: Val::Percent(100.0),
        ..default()
    })).with_children(|root| {
        // ── Left panel: dark frosted panel with menu content ──
        root.spawn((Node {
            width: Val::Percent(42.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
            padding: UiRect { left: Val::Px(40.0), right: Val::Px(40.0),
                top: Val::Vh(6.0), bottom: Val::Vh(4.0) },
            ..default()
        }, BackgroundColor(PANEL_BG),
        )).with_children(|panel| {
            // Top spacer
            panel.spawn(Node { flex_grow: 1.0, ..default() });

            // Title
            panel.spawn((
                Text::new("protocol play"),
                TextFont { font: f.clone(), font_size: MENU_TITLE_SIZE, ..default() },
                TextColor(Color::WHITE),
            ));
            panel.spawn((
                Text::new("puzzle"),
                TextFont { font: f.clone(), font_size: MENU_SUBTITLE_SIZE, ..default() },
                TextColor(Color::srgba(0.55, 0.80, 0.72, 0.90)),
                Node { margin: UiRect::top(Val::Px(2.0)), ..default() },
            ));
            panel.spawn((
                Text::new(t.ui_or("tagline", "every connection matters")),
                TextFont { font: f.clone(), font_size: MENU_TAGLINE_SIZE, ..default() },
                TextColor(Color::srgba(0.55, 0.60, 0.68, 0.65)),
                Node { margin: UiRect::top(Val::Px(12.0)), ..default() },
            ));

            // Button area
            panel.spawn(Node { height: Val::Px(44.0), ..default() });

            if has_save {
                spawn_btn(panel, f, MenuContinueBtn, &t.ui_or("continue", "Continue"),
                    MENU_BTN_FONT, primary(), hover(),
                    UiRect::axes(Val::Px(64.0), Val::Px(16.0)), Val::Px(10.0));
                spawn_btn(panel, f, MenuNewGameBtn, &t.ui_or("new_game", "New Game"),
                    MENU_BTN_SMALL_FONT, secondary(), hover_dim(),
                    UiRect::axes(Val::Px(48.0), Val::Px(12.0)), Val::Px(6.0));
            } else {
                spawn_btn(panel, f, MenuNewGameBtn, &t.ui_or("new_game", "New Game"),
                    MENU_BTN_FONT, primary(), hover(),
                    UiRect::axes(Val::Px(64.0), Val::Px(16.0)), Val::Px(0.0));
            }

            // Bottom spacer
            panel.spawn(Node { flex_grow: 1.2, ..default() });

            // Footer
            panel.spawn(Node {
                flex_direction: FlexDirection::Row, column_gap: Val::Px(32.0),
                justify_content: JustifyContent::Center, width: Val::Percent(100.0),
                margin: UiRect::bottom(Val::Px(14.0)), ..default()
            }).with_children(|row| {
                spawn_link(row, f, MenuSettingsBtn, &t.ui_or("settings", "Settings"), link_color());
                spawn_link(row, f, MenuQuitBtn, &t.ui_or("quit", "Quit"), link_dim());
            });
            panel.spawn((
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
                TextFont { font: f.clone(), font_size: VERSION_FONT, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.18)),
            ));
        });

        // ── Right side: thin gradient overlay to soften the 3D board edge ──
        root.spawn((Node {
            width: Val::Percent(58.0), height: Val::Percent(100.0),
            ..default()
        }, BackgroundColor(Color::srgba(MENU_BG.0, MENU_BG.1, MENU_BG.2, 0.15)),
        ));
    });
}

fn spawn_btn(p: &mut ChildSpawnerCommands<'_>, f: &Handle<Font>, marker: impl Component,
    label: &str, size: f32, normal: Color, hovered: Color, padding: UiRect, mb: Val,
) {
    p.spawn((Button, marker, MenuHoverable { normal, hovered },
        Node { padding, border_radius: BorderRadius::all(Val::Px(8.0)),
            margin: UiRect::bottom(mb), justify_content: JustifyContent::Center,
            width: Val::Percent(80.0), ..default() },
        BackgroundColor(normal),
    )).with_child((
        Text::new(label), TextFont { font: f.clone(), font_size: size, ..default() },
        TextColor(Color::WHITE),
    ));
}

fn spawn_link(p: &mut ChildSpawnerCommands<'_>, f: &Handle<Font>, marker: impl Component,
    label: &str, color: Color,
) {
    p.spawn((Button, marker, MenuHoverable { normal: Color::NONE, hovered: link_hover() },
        Node { padding: UiRect::axes(Val::Px(18.0), Val::Px(7.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
        BackgroundColor(Color::NONE),
    )).with_child((
        Text::new(label), TextFont { font: f.clone(), font_size: MENU_BTN_SMALL_FONT, ..default() },
        TextColor(color),
    ));
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

pub fn menu_hover(
    mut q: Query<(&Interaction, &MenuHoverable, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, h, mut bg) in q.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => { bg.0 = h.hovered; }
            Interaction::None => { bg.0 = h.normal; }
        }
    }
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
