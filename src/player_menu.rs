// SPDX-License-Identifier: GPL-3.0-or-later
//! Main menu — minimal floating typography over the 3D board.
//! Design: quiet confidence, the board is the hero.

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use crate::types::GameFont;
use crate::i18n::Translations;
use crate::ui_theme::{self, palette, typo, spacing};
use crate::save_state::{exe_dir, load_game_state, save_game_state};
use crate::player_profiles::{self, ProfileState, ProfileSlotBtn};

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

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn enter_menu(
    mut commands: Commands, font: Res<GameFont>, t: Res<Translations>,
    profile: Res<ProfileState>, settings: Res<crate::player_settings::PlayerSettings>,
) {
    bevy::log::info!("Menu language: {}", t.language);
    let f = &font.0;
    let has_save = check_has_progress();

    // Root: full screen, no background — the 3D board IS the background.
    commands.spawn((MenuRoot, Node {
        width: Val::Percent(100.0), height: Val::Percent(100.0), ..default()
    })).with_children(|root| {
        // ── Left panel: clean edge, no gradient hack ──
        root.spawn((Node {
            width: Val::Percent(34.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column, align_items: AlignItems::Start,
            padding: UiRect { left: Val::Px(48.0), right: Val::Px(40.0),
                top: Val::Vh(6.0), bottom: Val::Vh(4.0) },
            ..default()
        }, BackgroundColor(palette::PANEL_MENU),
        )).with_children(|panel| {
            // Top spacer — pushes content slightly above optical center
            panel.spawn(Node { flex_grow: 1.0, ..default() });

            // ── Title: reads as a single mark ──
            // "protocol play" + "puzzle" as a tight unit
            ui_theme::spawn_heading(panel, f, "protocol play", typo::H1, palette::TEXT_MAIN);
            ui_theme::spawn_label(panel, f, "puzzle", typo::H2, palette::TEXT_SUB, 4.0);
            // Tagline: readable against moving background
            panel.spawn((
                Node { margin: UiRect::top(Val::Px(16.0)),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(spacing::RADIUS_SM)),
                    ..default() },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.25)),
            )).with_child((
                Text::new(t.ui_or("tagline", "every connection matters")),
                TextFont { font: f.clone(), font_size: typo::CAPTION, ..default() },
                TextColor(palette::TEXT_WHISPER),
            ));

            // ── Buttons ──
            panel.spawn(Node { height: Val::Px(48.0), ..default() });

            if has_save {
                ui_theme::spawn_button(panel, f, MenuContinueBtn,
                    &t.ui_or("continue", "Continue"));
                panel.spawn(Node { height: Val::Px(12.0), ..default() });
                ui_theme::spawn_button_outline(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            } else {
                ui_theme::spawn_button(panel, f, MenuNewGameBtn,
                    &t.ui_or("new_game", "New Game"));
            }

            // ── Profile selector ──
            player_profiles::spawn_profile_selector(panel, f, profile.active_slot, &t);

            // ── Seed display ──
            panel.spawn((
                Node { margin: UiRect::top(Val::Px(8.0)), ..default() },
            )).with_child((
                Text::new(format!(
                    "{}: {:012X}",
                    t.ui_or("seed", "Seed"),
                    settings.campaign_seed
                )),
                TextFont { font: f.clone(), font_size: typo::MICRO, ..default() },
                TextColor(palette::TEXT_DIM),
            ));

            // Bottom spacer (1.5× top for optical center)
            panel.spawn(Node { flex_grow: 1.5, ..default() });

            // ── Footer: Settings · Quit ──
            panel.spawn(Node {
                flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0),
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)), ..default()
            }).with_children(|row| {
                ui_theme::spawn_link(row, f, MenuSettingsBtn,
                    &t.ui_or("settings", "Settings"), palette::LINK_TEXT);
                // Separator dot
                row.spawn((
                    Text::new("·"),
                    TextFont { font: f.clone(), font_size: typo::SMALL, ..default() },
                    TextColor(palette::TEXT_SEPARATOR),
                ));
                ui_theme::spawn_link(row, f, MenuQuitBtn,
                    &t.ui_or("quit", "Quit"), palette::LINK_DIM_TEXT);
            });

            // Version
            panel.spawn((
                Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
                TextFont { font: f.clone(), font_size: typo::MICRO, ..default() },
                TextColor(palette::TEXT_DIM),
            ));
        });
    });
}

// ─── Update ──────────────────────────────────────────────────────────────────

/// Handle button clicks — triggers fade-to-black transition.
pub fn menu_interaction(
    continue_q: Query<&Interaction, (With<MenuContinueBtn>, Changed<Interaction>)>,
    new_game_q: Query<&Interaction, (With<MenuNewGameBtn>, Changed<Interaction>)>,
    quit_q: Query<&Interaction, (With<MenuQuitBtn>, Changed<Interaction>)>,
    settings_q: Query<&Interaction, (With<MenuSettingsBtn>, Changed<Interaction>)>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    mut settings_req: ResMut<crate::player_settings::SettingsOpenRequest>,
    existing_fade: Query<Entity, With<MenuFadeOverlay>>,
    mut player_settings: ResMut<crate::player_settings::PlayerSettings>,
) {
    if !existing_fade.is_empty() { return; }
    let wants_continue = continue_q.iter().any(|i| *i == Interaction::Pressed);
    let wants_new_game = new_game_q.iter().any(|i| *i == Interaction::Pressed);
    if wants_new_game {
        // Generate new random seed for this new game
        player_settings.campaign_seed = rand::random::<u64>();
        crate::player_settings::save_player_settings(&player_settings);
        reset_progress();
    }
    if wants_continue || wants_new_game { spawn_fade(&mut commands); }
    if quit_q.iter().any(|i| *i == Interaction::Pressed) { exit.write(AppExit::Success); }
    if settings_q.iter().any(|i| *i == Interaction::Pressed) { settings_req.0 = true; }
}

/// Keyboard: Enter/Space to play (with fade), Escape to quit.
pub fn menu_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    existing_fade: Query<Entity, With<MenuFadeOverlay>>,
) {
    if !existing_fade.is_empty() { return; }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        spawn_fade(&mut commands);
    }
    if keys.just_pressed(KeyCode::Escape) { exit.write(AppExit::Success); }
}

/// Detect when fade-to-black completes → transition to Playing.
pub fn menu_fade_transition(
    fade_q: Query<&BackgroundColor, With<MenuFadeOverlay>>,
    mut next: ResMut<NextState<PlayerPhase>>,
) {
    for bg in fade_q.iter() {
        if bg.0.to_srgba().alpha > 0.95 { next.set(PlayerPhase::Playing); }
    }
}

pub fn exit_menu(mut commands: Commands,
    q: Query<Entity, With<MenuRoot>>,
    fade_q: Query<Entity, With<MenuFadeOverlay>>,
) {
    for e in q.iter() { commands.entity(e).despawn(); }
    for e in fade_q.iter() { commands.entity(e).despawn(); }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Spawn the fade-to-black overlay for smooth menu→game transition.
fn spawn_fade(commands: &mut Commands) {
    commands.spawn((
        MenuFadeOverlay,
        Node { position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        BackgroundColor(palette::FADE_BLACK),
        GlobalZIndex(500),
        crate::types::UiBgFade { target: 1.0, despawn_at_zero: false },
    ));
}

/// Request to rebuild the main menu (e.g. after profile switch).
#[derive(Resource, Default)]
pub struct MenuRebuildRequest(pub bool);

/// Handle profile slot button clicks — switch profiles and request menu rebuild.
pub fn profile_slot_interaction(
    slot_q: Query<(&ProfileSlotBtn, &Interaction), Changed<Interaction>>,
    mut profile: ResMut<ProfileState>,
    mut rebuild: ResMut<MenuRebuildRequest>,
) {
    for (btn, interaction) in slot_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        if btn.0 == profile.active_slot { continue; }
        let old = profile.active_slot;
        let new = btn.0;
        player_profiles::switch_profile(old, new);
        profile.active_slot = new;
        rebuild.0 = true;
        return;
    }
}

/// Rebuild the menu when requested (after profile switch).
pub fn rebuild_menu_on_request(
    mut rebuild: ResMut<MenuRebuildRequest>,
    menu_root: Query<Entity, With<MenuRoot>>,
    mut commands: Commands,
    font: Res<GameFont>,
    t: Res<Translations>,
    profile: Res<ProfileState>,
    settings: Res<crate::player_settings::PlayerSettings>,
) {
    if !rebuild.0 { return; }
    rebuild.0 = false;
    for e in menu_root.iter() { commands.entity(e).despawn(); }
    enter_menu(commands, font, t, profile, settings);
}

/// Reset all progress: delete .progress.json files and set bot_level to 0.
fn reset_progress() {
    let dir = exe_dir();
    // Delete all progress files
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.file_name().to_string_lossy().ends_with(".progress.json") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    // Reset bot_level in game state
    let mut state = load_game_state();
    state.bot_level = 0;
    save_game_state(&state);
}

fn check_has_progress() -> bool {
    let dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    std::fs::read_dir(&dir).ok()
        .map(|entries| entries.filter_map(|e| e.ok())
            .any(|e| e.file_name().to_string_lossy().ends_with(".progress.json")))
        .unwrap_or(false)
}
