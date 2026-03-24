// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu — floating typography over the 3D board. Minimal, elegant.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::types::GameFont;
use crate::i18n::Translations;
use crate::ui_theme::{self, palette, typo, spacing, Hoverable};

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerPhase { #[default] MainMenu, Playing }

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)] pub struct MenuRoot;
#[derive(Component)] pub struct MenuContinueBtn;
#[derive(Component)] pub struct MenuNewGameBtn;
#[derive(Component)] pub struct MenuQuitBtn;
#[derive(Component)] pub struct MenuSettingsBtn;
#[derive(Component)] pub struct MenuFadeOverlay;
#[derive(Component)] pub struct MenuPrimaryPulse(f32);

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn enter_menu(mut commands: Commands, font: Res<GameFont>, t: Res<Translations>) {
    let f = &font.0;
    let has_save = check_has_progress();

    // Root: full screen, no background — the 3D board IS the background
    commands.spawn((MenuRoot, Node {
        width: Val::Percent(100.0), height: Val::Percent(100.0), ..default()
    })).with_children(|root| {
        // ── Left panel: gradient-edged, narrower, semi-transparent ──
        root.spawn((Node {
            width: Val::Percent(36.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, align_items: AlignItems::Start,
            padding: UiRect { left: Val::Px(48.0), right: Val::Px(32.0),
                top: Val::Vh(6.0), bottom: Val::Vh(4.0) },
            ..default()
        }, BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.72)),
        )).with_children(|panel| {
            // Top spacer
            panel.spawn(Node { flex_grow: 1.0, ..default() });

            // ── Title: tracked, prominent ──
            panel.spawn((
                Text::new("PROTOCOL PLAY"),
                TextFont { font: f.clone(), font_size: typo::H1, ..default() },
                TextColor(palette::TEXT_MAIN),
                Node { margin: UiRect::bottom(Val::Px(0.0)), ..default() },
            ));
            panel.spawn((
                Text::new("puzzle"),
                TextFont { font: f.clone(), font_size: 28.0, ..default() },
                TextColor(palette::TEXT_SUB),
                Node { margin: UiRect::top(Val::Px(0.0)), ..default() },
            ));

            // Tagline — whispered
            panel.spawn((
                Text::new(t.ui_or("tagline", "every connection matters")),
                TextFont { font: f.clone(), font_size: typo::CAPTION, ..default() },
                TextColor(Color::srgba(0.45, 0.52, 0.58, 0.55)),
                Node { margin: UiRect::top(Val::Px(10.0)), ..default() },
            ));

            // ── Button area ──
            panel.spawn(Node { height: Val::Px(48.0), ..default() });

            if has_save {
                // Continue: primary with subtle glow
                spawn_primary_btn(panel, f, MenuContinueBtn,
                    &t.ui_or("continue", "Continue"));
                panel.spawn(Node { height: Val::Px(12.0), ..default() });
                // New Game: outline style, understated
                spawn_outline_btn(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            } else {
                spawn_primary_btn(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            }

            // ── Bottom spacer ──
            panel.spawn(Node { flex_grow: 1.5, ..default() });

            // ── Footer: left-aligned ──
            panel.spawn(Node {
                flex_direction: FlexDirection::Row, column_gap: Val::Px(24.0),
                margin: UiRect::bottom(Val::Px(10.0)), ..default()
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

        // ── Soft vignette edge (not a flat overlay — just darkens the left edge) ──
        root.spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(34.0), top: Val::Px(0.0),
            width: Val::Percent(8.0), height: Val::Percent(100.0),
            ..default()
        }, BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.35)),
        ));
    });
}

/// Primary button: filled with subtle shadow glow.
fn spawn_primary_btn(
    parent: &mut ChildSpawnerCommands, f: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    parent.spawn((
        Button, marker,
        Hoverable { normal: palette::PRIMARY, hovered: palette::PRIMARY_HOVER },
        MenuPrimaryPulse(0.0),
        Node {
            padding: UiRect::axes(Val::Px(56.0), Val::Px(16.0)),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
            justify_content: JustifyContent::Center,
            min_width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(palette::PRIMARY),
        BoxShadow::new(
            Color::srgba(0.18, 0.54, 0.48, 0.35),
            Val::ZERO, Val::Px(4.0), Val::Px(8.0), Val::Px(16.0),
        ),
    )).with_child((
        Text::new(label),
        TextFont { font: f.clone(), font_size: typo::BODY, ..default() },
        TextColor(Color::WHITE),
    ));
}

/// Secondary button: outline only, transparent fill.
fn spawn_outline_btn(
    parent: &mut ChildSpawnerCommands, f: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    let normal = Color::srgba(1.0, 1.0, 1.0, 0.04);
    let hovered = Color::srgba(1.0, 1.0, 1.0, 0.10);
    parent.spawn((
        Button, marker,
        Hoverable { normal, hovered },
        Node {
            padding: UiRect::axes(Val::Px(44.0), Val::Px(12.0)),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
            border: UiRect::all(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            min_width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(normal),
        BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.15)),
    )).with_child((
        Text::new(label),
        TextFont { font: f.clone(), font_size: typo::SMALL, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.70)),
    ));
}

// ─── Update ──────────────────────────────────────────────────────────────────

pub fn menu_interaction(
    continue_q: Query<&Interaction, (With<MenuContinueBtn>, Changed<Interaction>)>,
    new_game_q: Query<&Interaction, (With<MenuNewGameBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<MenuQuitBtn>, Changed<Interaction>)>,
    settings_q: Query<&Interaction, (With<MenuSettingsBtn>, Changed<Interaction>)>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    mut settings_req: ResMut<crate::player_settings::SettingsOpenRequest>,
    existing_fade: Query<Entity, With<MenuFadeOverlay>>,
) {
    if !existing_fade.is_empty() { return; } // transition in progress

    let target = if continue_q.iter().any(|i| *i == Interaction::Pressed) {
        Some(PlayerPhase::Playing)
    } else if new_game_q.iter().any(|i| *i == Interaction::Pressed) {
        Some(PlayerPhase::Playing)
    } else { None };

    if let Some(_phase) = target {
        // Spawn fade-to-black overlay for smooth transition
        commands.spawn((
            MenuFadeOverlay,
            Node { position_type: PositionType::Absolute,
                width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            GlobalZIndex(500),
            crate::types::UiBgFade { target: 1.0, despawn_at_zero: false },
        ));
    }
    if quit_q.iter().any(|i| *i == Interaction::Pressed) { exit.write(AppExit::Success); }
    if settings_q.iter().any(|i| *i == Interaction::Pressed) { settings_req.0 = true; }
}

/// Detect when fade-to-black completes, then transition state.
pub fn menu_fade_transition(
    fade_q: Query<&BackgroundColor, With<MenuFadeOverlay>>,
    mut next: ResMut<NextState<PlayerPhase>>,
) {
    for bg in fade_q.iter() {
        if bg.0.to_srgba().alpha > 0.95 {
            next.set(PlayerPhase::Playing);
        }
    }
}

/// Subtle pulse on the primary button shadow.
pub fn menu_primary_pulse(
    time: Res<Time>,
    mut q: Query<(&mut MenuPrimaryPulse, &mut BoxShadow)>,
) {
    for (mut pulse, mut shadow) in q.iter_mut() {
        pulse.0 += time.delta_secs() * 1.5;
        let glow = 0.25 + (pulse.0.sin() * 0.5 + 0.5) * 0.15;
        if let Some(s) = shadow.0.first_mut() {
            s.color = Color::srgba(0.18, 0.54, 0.48, glow);
        }
    }
}

/// Keyboard: Enter to continue/play, Escape to quit.
pub fn menu_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut next: ResMut<NextState<PlayerPhase>>,
    mut exit: MessageWriter<AppExit>,
    existing_fade: Query<Entity, With<MenuFadeOverlay>>,
) {
    if !existing_fade.is_empty() { return; }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        next.set(PlayerPhase::Playing);
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

pub fn exit_menu(mut commands: Commands, q: Query<Entity, With<MenuRoot>>,
    fade_q: Query<Entity, With<MenuFadeOverlay>>,
) {
    for e in q.iter() { commands.entity(e).despawn(); }
    for e in fade_q.iter() { commands.entity(e).despawn(); }
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
