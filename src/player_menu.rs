// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu — clean, modern puzzle game aesthetic.

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

#[derive(Component)] pub struct MenuRoot;
#[derive(Component)] pub struct MenuContinueBtn;
#[derive(Component)] pub struct MenuNewGameBtn;
#[derive(Component)] pub struct MenuQuitBtn;
#[derive(Component)] pub struct MenuSettingsBtn;

#[derive(Component)]
pub struct MenuHoverable { pub normal: Color, pub hovered: Color }

// ─── Colors ──────────────────────────────────────────────────────────────────

fn btn_primary() -> Color { Color::srgba(MENU_BTN_PRIMARY.0, MENU_BTN_PRIMARY.1, MENU_BTN_PRIMARY.2, MENU_BTN_PRIMARY.3) }
fn btn_secondary() -> Color { Color::srgba(MENU_BTN_SECONDARY.0, MENU_BTN_SECONDARY.1, MENU_BTN_SECONDARY.2, MENU_BTN_SECONDARY.3) }
fn btn_hover() -> Color { Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3) }
fn btn_hover_dim() -> Color { Color::srgba(MENU_BTN_HOVER.0, MENU_BTN_HOVER.1, MENU_BTN_HOVER.2, MENU_BTN_HOVER.3 * 0.6) }
fn text_link() -> Color { Color::srgba(0.65, 0.72, 0.80, 0.90) }
fn text_link_dim() -> Color { Color::srgba(0.50, 0.55, 0.62, 0.70) }
fn link_hover() -> Color { Color::srgba(1.0, 1.0, 1.0, 0.06) }

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn enter_menu(mut commands: Commands, font: Res<GameFont>, t: Res<Translations>) {
    let f = &font.0;
    let has_save = check_has_progress();

    // Full-screen overlay — semi-transparent so 3D board shows through
    commands.spawn((MenuRoot, Node {
        width: Val::Percent(100.0), height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
        padding: UiRect::vertical(Val::Vh(8.0)),
        ..default()
    }, BackgroundColor(Color::srgba(MENU_BG.0, MENU_BG.1, MENU_BG.2, MENU_OVERLAY_ALPHA)),
    )).with_children(|root| {
        // ── Top spacer (pushes content toward center) ──
        root.spawn(Node { flex_grow: 1.0, ..default() });

        // ── Title block ──
        root.spawn((
            Text::new("protocol play"),
            TextFont { font: f.clone(), font_size: MENU_TITLE_SIZE, ..default() },
            TextColor(Color::WHITE),
        ));
        root.spawn((
            Text::new("puzzle"),
            TextFont { font: f.clone(), font_size: MENU_SUBTITLE_SIZE, ..default() },
            TextColor(Color::srgba(0.55, 0.75, 0.70, 0.90)),
            Node { margin: UiRect::top(Val::Px(2.0)), ..default() },
        ));
        root.spawn((
            Text::new(t.ui_or("tagline", "every connection matters")),
            TextFont { font: f.clone(), font_size: MENU_TAGLINE_SIZE, ..default() },
            TextColor(Color::srgba(0.50, 0.58, 0.65, 0.70)),
            Node { margin: UiRect::top(Val::Px(14.0)), ..default() },
        ));

        // ── Button area ──
        root.spawn(Node { height: Val::Px(50.0), ..default() });

        if has_save {
            spawn_btn(root, f, MenuContinueBtn, &t.ui_or("continue", "Continue"),
                MENU_BTN_FONT, btn_primary(), btn_hover(),
                UiRect::axes(Val::Px(64.0), Val::Px(16.0)), Val::Px(10.0));
            spawn_btn(root, f, MenuNewGameBtn, &t.ui_or("new_game", "New Game"),
                MENU_BTN_SMALL_FONT, btn_secondary(), btn_hover_dim(),
                UiRect::axes(Val::Px(48.0), Val::Px(12.0)), Val::Px(6.0));
        } else {
            spawn_btn(root, f, MenuNewGameBtn, &t.ui_or("new_game", "New Game"),
                MENU_BTN_FONT, btn_primary(), btn_hover(),
                UiRect::axes(Val::Px(64.0), Val::Px(16.0)), Val::Px(0.0));
        }

        // ── Bottom spacer ──
        root.spawn(Node { flex_grow: 1.5, ..default() });

        // ── Footer: Settings · Quit · Version ──
        root.spawn(Node {
            flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
            row_gap: Val::Px(6.0), margin: UiRect::bottom(Val::Px(10.0)), ..default()
        }).with_children(|footer| {
            // Settings + Quit in a row
            footer.spawn(Node {
                flex_direction: FlexDirection::Row, column_gap: Val::Px(24.0), ..default()
            }).with_children(|row| {
                spawn_link(row, f, MenuSettingsBtn, &t.ui_or("settings", "Settings"), text_link());
                spawn_link(row, f, MenuQuitBtn, &t.ui_or("quit", "Quit"), text_link_dim());
            });
            // Version
            footer.spawn((
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
                TextFont { font: f.clone(), font_size: VERSION_FONT, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.20)),
                Node { margin: UiRect::top(Val::Px(6.0)), ..default() },
            ));
        });
    });
}

fn spawn_btn(parent: &mut ChildSpawnerCommands<'_>, f: &Handle<Font>, marker: impl Component,
    label: &str, font_size: f32, normal: Color, hovered: Color, padding: UiRect, margin_bot: Val,
) {
    parent.spawn((
        Button, marker, MenuHoverable { normal, hovered },
        Node { padding, border_radius: BorderRadius::all(Val::Px(8.0)),
            margin: UiRect::bottom(margin_bot), justify_content: JustifyContent::Center, ..default() },
        BackgroundColor(normal),
    )).with_child((
        Text::new(label), TextFont { font: f.clone(), font_size, ..default() }, TextColor(Color::WHITE),
    ));
}

fn spawn_link(parent: &mut ChildSpawnerCommands<'_>, f: &Handle<Font>, marker: impl Component,
    label: &str, color: Color,
) {
    parent.spawn((
        Button, marker,
        MenuHoverable { normal: Color::NONE, hovered: link_hover() },
        Node { padding: UiRect::axes(Val::Px(16.0), Val::Px(5.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
        BackgroundColor(Color::NONE),
    )).with_child((
        Text::new(label), TextFont { font: f.clone(), font_size: 14.0, ..default() }, TextColor(color),
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
