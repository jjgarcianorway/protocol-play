// SPDX-License-Identifier: GPL-3.0-or-later
//! Persistent player preferences and the settings overlay UI.

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::types::GameFont;
use crate::save_state::exe_dir;
use crate::i18n::{Translations, load_translations};

// ─── Data ─────────────────────────────────────────────────────────────────────

/// Persistent player preferences, stored as `player_settings.json`.
#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct PlayerSettings {
    /// When false: no Anna commentary. Pure puzzle mode.
    pub anna_enabled: bool,
    /// Language code: "en" or "es".
    pub language: String,
    /// Simulation playback speed multiplier: 1.0 / 2.0 / 4.0.
    #[serde(default = "default_sim_speed")]
    pub sim_speed: f32,
}
fn default_sim_speed() -> f32 { 1.0 }

impl Default for PlayerSettings {
    fn default() -> Self { Self { anna_enabled: true, language: "en".to_string(), sim_speed: 1.0 } }
}

pub fn load_player_settings() -> PlayerSettings {
    let path = exe_dir().join("player_settings.json");
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_player_settings(s: &PlayerSettings) {
    let path = exe_dir().join("player_settings.json");
    if let Ok(json) = serde_json::to_string_pretty(s) { let _ = std::fs::write(path, json); }
}

// ─── Components ───────────────────────────────────────────────────────────────

#[derive(Component)] pub struct SettingsOverlay;
#[derive(Component)] pub struct SettingsCloseBtn;
#[derive(Component)] pub struct AnnaToggleBtn;
#[derive(Component)] pub struct LangBtn(pub String);
#[derive(Component)] pub struct SimSpeedBtn(pub f32);
/// Settings link in the main menu.
#[derive(Component)] pub struct MenuSettingsBtn;

// ─── Open request ─────────────────────────────────────────────────────────────

/// Set to true to open the settings overlay from any screen.
#[derive(Resource, Default)] pub struct SettingsOpenRequest(pub bool);

// ─── Overlay spawn ────────────────────────────────────────────────────────────

pub fn spawn_settings_overlay(
    commands: &mut Commands,
    font: &Handle<Font>,
    s: &PlayerSettings,
    t: &Translations,
) {
    let tf = |sz: f32| TextFont { font: font.clone(), font_size: sz, ..default() };
    let (anna_label, anna_bg) = if s.anna_enabled {
        (t.ui_or("anna_on", "ON"),  Color::srgba(0.25, 0.68, 0.42, 1.0))
    } else {
        (t.ui_or("anna_off", "OFF"), Color::srgba(0.40, 0.40, 0.45, 1.0))
    };
    let lang_active_bg   = Color::srgba(0.25, 0.45, 0.70, 0.90);
    let lang_inactive_bg = Color::srgba(0.18, 0.20, 0.26, 0.70);

    commands.spawn((
        SettingsOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.72)),
        GlobalZIndex(300),
    )).with_children(|root| {
        root.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(20.0),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.07, 0.08, 0.12, 0.98)),
        )).with_children(|p| {

            // ── Title ──
            p.spawn((Text::new(t.ui_or("settings", "Settings")),
                tf(21.0), TextColor(Color::WHITE)));

            // ── Anna row ──
            setting_row(p, &tf, t.ui_or("anna_commentary", "Anna's commentary"), |row| {
                row.spawn((
                    Button, AnnaToggleBtn,
                    Node { padding: UiRect::axes(Val::Px(18.0), Val::Px(7.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                    BackgroundColor(anna_bg),
                )).with_child((Text::new(anna_label), tf(13.0), TextColor(Color::WHITE)));
            });

            // Anna description
            p.spawn((
                Text::new(t.ui_or("anna_desc",
                    "Gamification history, psychology, and real-world use.\nTurn off for pure puzzle mode.")),
                tf(11.5), TextColor(Color::srgba(0.45, 0.48, 0.56, 1.0)),
                Node { max_width: Val::Px(310.0), ..default() },
            ));

            // ── Simulation speed row ──
            let speed_active_bg   = Color::srgba(0.25, 0.45, 0.70, 0.90);
            let speed_inactive_bg = Color::srgba(0.18, 0.20, 0.26, 0.70);
            setting_row(p, &tf, t.ui_or("sim_speed", "Simulation Speed"), |row| {
                for (val, _key, fallback) in &[
                    (1.0_f32, "speed_1x", "1×"),
                    (2.0_f32, "speed_2x", "2×"),
                    (4.0_f32, "speed_4x", "4×"),
                ] {
                    let active = (s.sim_speed - val).abs() < 0.05;
                    row.spawn((
                        Button, SimSpeedBtn(*val),
                        Node { padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                        BackgroundColor(if active { speed_active_bg } else { speed_inactive_bg }),
                    )).with_child((Text::new(*fallback), tf(13.0), TextColor(Color::WHITE)));
                }
            });

            // ── Language row ──
            setting_row(p, &tf, t.ui_or("language", "Language"), |row| {
                for (code, label) in &[("en", "English"), ("es", "Español")] {
                    let active = s.language == *code;
                    row.spawn((
                        Button, LangBtn(code.to_string()),
                        Node { padding: UiRect::axes(Val::Px(14.0), Val::Px(7.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                        BackgroundColor(if active { lang_active_bg } else { lang_inactive_bg }),
                    )).with_child((Text::new(*label), tf(13.0), TextColor(Color::WHITE)));
                }
            });

            // ── Done ──
            p.spawn((
                Button, SettingsCloseBtn,
                Node { padding: UiRect::axes(Val::Px(44.0), Val::Px(10.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    margin: UiRect::top(Val::Px(8.0)), ..default() },
                BackgroundColor(Color::srgba(0.22, 0.40, 0.65, 0.9)),
            )).with_child((Text::new(t.ui_or("done", "Done")),
                tf(14.0), TextColor(Color::WHITE)));
        });
    });
}

/// Spawn a two-column label + controls row.
fn setting_row(
    parent: &mut ChildSpawnerCommands,
    tf: &impl Fn(f32) -> TextFont,
    label: &str,
    build_controls: impl FnOnce(&mut ChildSpawnerCommands),
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Px(310.0), column_gap: Val::Px(16.0), ..default()
    }).with_children(|row| {
        row.spawn((Text::new(label), tf(14.0),
            TextColor(Color::srgba(0.72, 0.75, 0.82, 1.0))));
        row.spawn(Node {
            flex_direction: FlexDirection::Row, column_gap: Val::Px(6.0), ..default()
        }).with_children(|btns| {
            build_controls(btns);
        });
    });
}

// ─── Systems ──────────────────────────────────────────────────────────────────

/// Open overlay when SettingsOpenRequest is set or the MenuSettingsBtn is pressed.
pub fn settings_request(
    mut req: ResMut<SettingsOpenRequest>,
    menu_btn_q: Query<&Interaction, (With<MenuSettingsBtn>, Changed<Interaction>)>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    mut commands: Commands,
    font: Res<GameFont>,
    settings: Res<PlayerSettings>,
    translations: Res<Translations>,
) {
    let via_btn = menu_btn_q.iter().any(|i| *i == Interaction::Pressed);
    if via_btn { req.0 = true; }
    if !req.0 || !overlay_q.is_empty() { return; }
    req.0 = false;
    spawn_settings_overlay(&mut commands, &font.0, &settings, &translations);
}

/// Handle interactions inside the settings overlay.
pub fn settings_overlay_input(
    mut commands: Commands,
    close_q:  Query<&Interaction, (With<SettingsCloseBtn>, Changed<Interaction>)>,
    anna_q:   Query<(Entity, &Interaction, &Children), (With<AnnaToggleBtn>, Changed<Interaction>)>,
    lang_q:   Query<(&LangBtn, &Interaction), Changed<Interaction>>,
    mut speed_q: Query<(Entity, &SimSpeedBtn, &Interaction, &mut BackgroundColor)>,
    overlay_q: Query<Entity, With<SettingsOverlay>>,
    mut text_q: Query<&mut Text>,
    mut bg_q:   Query<&mut BackgroundColor, Without<SimSpeedBtn>>,
    mut settings: ResMut<PlayerSettings>,
    mut translations: ResMut<Translations>,
    mut req: ResMut<SettingsOpenRequest>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if overlay_q.is_empty() { return; }

    // Anna toggle — update in place
    for (btn_ent, interaction, children) in anna_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        settings.anna_enabled = !settings.anna_enabled;
        save_player_settings(&settings);
        let (label, color) = if settings.anna_enabled {
            (translations.ui_or("anna_on", "ON"),  Color::srgba(0.25, 0.68, 0.42, 1.0))
        } else {
            (translations.ui_or("anna_off", "OFF"), Color::srgba(0.40, 0.40, 0.45, 1.0))
        };
        for child in children.iter() {
            if let Ok(mut t) = text_q.get_mut(child) { **t = label.to_string(); }
        }
        if let Ok(mut bg) = bg_q.get_mut(btn_ent) { bg.0 = color; }
    }

    // Simulation speed toggle
    let speed_active   = Color::srgba(0.25, 0.45, 0.70, 0.90);
    let speed_inactive = Color::srgba(0.18, 0.20, 0.26, 0.70);
    let pressed_speed = speed_q.iter().find_map(|(_, sb, i, _)| {
        if *i == Interaction::Pressed && (settings.sim_speed - sb.0).abs() > 0.05 { Some(sb.0) } else { None }
    });
    if let Some(new_speed) = pressed_speed {
        settings.sim_speed = new_speed;
        save_player_settings(&settings);
        for (_, sb, _, mut bg) in speed_q.iter_mut() {
            let active = (settings.sim_speed - sb.0).abs() < 0.05;
            bg.0 = if active { speed_active } else { speed_inactive };
        }
    }

    // Language toggle — close + reopen with new language
    for (lang_btn, interaction) in lang_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        if settings.language == lang_btn.0 { continue; }
        settings.language = lang_btn.0.clone();
        save_player_settings(&settings);
        *translations = load_translations(&settings.language);
        for e in overlay_q.iter() { commands.entity(e).despawn(); }
        req.0 = true;
        return;
    }

    // Close
    let close = close_q.iter().any(|i| *i == Interaction::Pressed)
        || keys.just_pressed(KeyCode::Escape);
    if close {
        for e in overlay_q.iter() { commands.entity(e).despawn(); }
    }
}
