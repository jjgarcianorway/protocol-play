// SPDX-License-Identifier: GPL-3.0-or-later
//! Persistent player preferences and the settings overlay UI.

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ui_theme::palette;
use crate::types::{GameFont, UiBgFade};
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
    /// Fullscreen mode (borderless).
    #[serde(default = "default_true")]
    pub fullscreen: bool,
    /// Post-processing bloom effect.
    #[serde(default = "default_true")]
    pub bloom_enabled: bool,
    /// Master volume: 0.3 (low), 0.7 (medium), 1.0 (high).
    #[serde(default = "default_volume")]
    pub master_volume: f32,
    /// Sound effects enabled.
    #[serde(default = "default_true")]
    pub sfx_enabled: bool,
    /// Campaign seed — identifies this "world" (cosmetic for now).
    #[serde(default = "default_campaign_seed")]
    pub campaign_seed: u64,
}
fn default_sim_speed() -> f32 { 1.0 }
fn default_true() -> bool { true }
fn default_volume() -> f32 { 0.7 }
fn default_campaign_seed() -> u64 { rand::random::<u64>() }

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            anna_enabled: true, language: "en".to_string(), sim_speed: 1.0,
            fullscreen: true, bloom_enabled: true,
            master_volume: 0.7, sfx_enabled: true,
            campaign_seed: rand::random::<u64>(),
        }
    }
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
#[derive(Component)] pub struct FullscreenToggleBtn;
#[derive(Component)] pub struct BloomToggleBtn;
#[derive(Component)] pub struct VolumeBtn(pub f32);
#[derive(Component)] pub struct SfxToggleBtn;
/// Settings link in the main menu.
#[derive(Component)] pub struct MenuSettingsBtn;

// ─── Open request ─────────────────────────────────────────────────────────────

/// Set to true to open the settings overlay from any screen.
#[derive(Resource, Default)] pub struct SettingsOpenRequest(pub bool);

/// Timer that delays settings reopen after a language switch (lets the fade-out finish).
#[derive(Resource, Default)] pub struct SettingsReopenTimer(pub Option<f32>);

/// Tick the reopen timer; when it expires, request the overlay to reopen.
pub fn settings_reopen_tick(
    time: Res<Time>,
    mut timer: ResMut<SettingsReopenTimer>,
    mut req: ResMut<SettingsOpenRequest>,
) {
    if let Some(ref mut t) = timer.0 {
        *t -= time.delta_secs();
        if *t <= 0.0 {
            timer.0 = None;
            req.0 = true;
        }
    }
}

// ─── Overlay spawn ────────────────────────────────────────────────────────────

pub fn spawn_settings_overlay(commands: &mut Commands, font: &Handle<Font>,
    s: &PlayerSettings, t: &Translations,
) {
    let tf = |sz: f32| TextFont { font: font.clone(), font_size: sz, ..default() };
    let (anna_label, anna_bg) = if s.anna_enabled {
        (t.ui_or("anna_on", "ON"),  Color::srgba(0.25, 0.68, 0.42, 1.0))
    } else {
        (t.ui_or("anna_off", "OFF"), Color::srgba(0.40, 0.40, 0.45, 1.0))
    };
    let lang_active_bg   = Color::srgba(0.25, 0.45, 0.70, 0.90);
    let lang_inactive_bg = Color::srgba(0.18, 0.20, 0.26, 0.70);

    let scrim_alpha = palette::SETTINGS_SCRIM.to_srgba().alpha;
    commands.spawn((
        SettingsOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        UiBgFade { target: scrim_alpha, despawn_at_zero: false },
        GlobalZIndex(300),
    )).with_children(|root| {
        root.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(20.0),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                min_width: Val::Px(380.0),
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

            // Anna description — no spoilers!
            p.spawn((
                Text::new(t.ui_or("anna_desc",
                    "Tips, facts, and encouragement as you play.\nTurn off for a quiet experience.")),
                tf(11.5), TextColor(palette::SETTINGS_LABEL),
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

            // ── Fullscreen row ──
            let (fs_label, fs_bg) = toggle_label_bg(s.fullscreen, t);
            setting_row(p, &tf, t.ui_or("fullscreen", "Fullscreen"), |row| {
                row.spawn((
                    Button, FullscreenToggleBtn,
                    Node { padding: UiRect::axes(Val::Px(18.0), Val::Px(7.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                    BackgroundColor(fs_bg),
                )).with_child((Text::new(fs_label), tf(13.0), TextColor(Color::WHITE)));
            });

            // ── Bloom row ──
            let (bl_label, bl_bg) = toggle_label_bg(s.bloom_enabled, t);
            setting_row(p, &tf, t.ui_or("bloom", "Bloom"), |row| {
                row.spawn((
                    Button, BloomToggleBtn,
                    Node { padding: UiRect::axes(Val::Px(18.0), Val::Px(7.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                    BackgroundColor(bl_bg),
                )).with_child((Text::new(bl_label), tf(13.0), TextColor(Color::WHITE)));
            });

            // ── Volume row ──
            let vol_active_bg   = Color::srgba(0.25, 0.45, 0.70, 0.90);
            let vol_inactive_bg = Color::srgba(0.18, 0.20, 0.26, 0.70);
            setting_row(p, &tf, t.ui_or("volume", "Volume"), |row| {
                for (val, key, fallback) in &[
                    (0.3_f32, "volume_low", "Low"),
                    (0.7_f32, "volume_medium", "Medium"),
                    (1.0_f32, "volume_high", "High"),
                ] {
                    let active = (s.master_volume - val).abs() < 0.05;
                    row.spawn((
                        Button, VolumeBtn(*val),
                        Node { padding: UiRect::axes(Val::Px(12.0), Val::Px(7.0)),
                            border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                        BackgroundColor(if active { vol_active_bg } else { vol_inactive_bg }),
                    )).with_child((Text::new(t.ui_or(key, fallback)), tf(13.0), TextColor(Color::WHITE)));
                }
            });

            // ── Sound Effects row ──
            let (sfx_label, sfx_bg) = toggle_label_bg(s.sfx_enabled, t);
            setting_row(p, &tf, t.ui_or("sfx", "Sound Effects"), |row| {
                row.spawn((
                    Button, SfxToggleBtn,
                    Node { padding: UiRect::axes(Val::Px(18.0), Val::Px(7.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                    BackgroundColor(sfx_bg),
                )).with_child((Text::new(sfx_label), tf(13.0), TextColor(Color::WHITE)));
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

            // ── Seed row (read-only) ──
            setting_row(p, &tf, t.ui_or("seed", "Seed"), |row| {
                row.spawn((Node { padding: UiRect::axes(Val::Px(10.0), Val::Px(7.0)),
                    border_radius: BorderRadius::all(Val::Px(4.0)), ..default() },
                    BackgroundColor(Color::srgba(0.14, 0.16, 0.22, 0.70)),
                )).with_child((Text::new(format!("{:012X}", s.campaign_seed)),
                    tf(13.0), TextColor(Color::srgba(0.60, 0.65, 0.75, 1.0))));
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
fn setting_row(parent: &mut ChildSpawnerCommands, tf: &impl Fn(f32) -> TextFont,
    label: &str, build_controls: impl FnOnce(&mut ChildSpawnerCommands),
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

/// ON/OFF label and background color for a boolean toggle.
fn toggle_label_bg<'a>(enabled: bool, t: &'a Translations) -> (&'a str, Color) {
    if enabled {
        (t.ui_or("anna_on", "ON"),  Color::srgba(0.25, 0.68, 0.42, 1.0))
    } else {
        (t.ui_or("anna_off", "OFF"), Color::srgba(0.40, 0.40, 0.45, 1.0))
    }
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
    overlay_q: Query<(Entity, Option<&UiBgFade>), With<SettingsOverlay>>,
    mut text_q: Query<&mut Text>,
    mut bg_q:   Query<&mut BackgroundColor, Without<SimSpeedBtn>>,
    mut settings: ResMut<PlayerSettings>,
    mut translations: ResMut<Translations>,
    _req: ResMut<SettingsOpenRequest>,
    mut reopen_timer: ResMut<SettingsReopenTimer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if overlay_q.is_empty() { return; }
    // Don't process input while fading out (despawn_at_zero means closing)
    let is_fading_out = overlay_q.iter().any(|(_, fade)| {
        fade.is_some_and(|f| f.despawn_at_zero && f.target < 0.01)
    });
    if is_fading_out { return; }

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

    // Language toggle — fade out, change language, fade back in after delay
    for (lang_btn, interaction) in lang_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        if settings.language == lang_btn.0 { continue; }
        settings.language = lang_btn.0.clone();
        save_player_settings(&settings);
        *translations = load_translations(&settings.language);
        for (e, _) in overlay_q.iter() {
            commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
        }
        reopen_timer.0 = Some(0.3);
        return;
    }

    // Close — fade out, then auto-despawn when alpha reaches 0
    let close = close_q.iter().any(|i| *i == Interaction::Pressed)
        || keys.just_pressed(KeyCode::Escape);
    if close {
        for (e, _) in overlay_q.iter() {
            commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true });
        }
    }
}

// Graphics & sound settings input is in player_settings_gfx.rs
pub use crate::player_settings_gfx::{settings_gfx_sound_input, apply_bloom_setting};
