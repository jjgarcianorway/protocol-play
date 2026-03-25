// SPDX-License-Identifier: GPL-3.0-or-later
//! Graphics & sound settings input handling (split from player_settings to stay ≤400 lines).

use bevy::prelude::*;
use bevy::window::{WindowMode, MonitorSelection};
use bevy::post_process::bloom::Bloom;
use crate::i18n::Translations;
use crate::types::UiBgFade;
use crate::constants::BLOOM_INTENSITY;
use crate::player_settings::{
    PlayerSettings, SettingsOverlay, FullscreenToggleBtn, BloomToggleBtn,
    VolumeBtn, SfxToggleBtn, SimSpeedBtn, save_player_settings,
};

/// Handle graphics & sound settings (separate system to stay under the 16-param limit).
pub fn settings_gfx_sound_input(
    overlay_q: Query<(Entity, Option<&UiBgFade>), With<SettingsOverlay>>,
    fs_q:    Query<(Entity, &Interaction, &Children), (With<FullscreenToggleBtn>, Changed<Interaction>)>,
    bloom_q: Query<(Entity, &Interaction, &Children), (With<BloomToggleBtn>, Changed<Interaction>)>,
    mut vol_q:  Query<(Entity, &VolumeBtn, &Interaction, &mut BackgroundColor)>,
    sfx_q:   Query<(Entity, &Interaction, &Children), (With<SfxToggleBtn>, Changed<Interaction>)>,
    mut text_q: Query<&mut Text>,
    mut bg_q:   Query<&mut BackgroundColor, (Without<VolumeBtn>, Without<SimSpeedBtn>)>,
    mut settings: ResMut<PlayerSettings>,
    translations: Res<Translations>,
    mut windows: Query<&mut Window>,
    mut bloom_cam_q: Query<&mut Bloom, With<Camera3d>>,
    mut sound_settings: Option<ResMut<crate::sound::SoundSettings>>,
) {
    if overlay_q.is_empty() { return; }
    let is_fading_out = overlay_q.iter().any(|(_, fade)| {
        fade.is_some_and(|f| f.despawn_at_zero && f.target < 0.01)
    });
    if is_fading_out { return; }

    let on_color  = Color::srgba(0.25, 0.68, 0.42, 1.0);
    let off_color = Color::srgba(0.40, 0.40, 0.45, 1.0);

    // Fullscreen toggle
    for (btn_ent, interaction, children) in fs_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        settings.fullscreen = !settings.fullscreen;
        save_player_settings(&settings);
        let (label, color) = toggle_label(&translations, settings.fullscreen, on_color, off_color);
        update_btn_text(&mut text_q, children, label);
        if let Ok(mut bg) = bg_q.get_mut(btn_ent) { bg.0 = color; }
        if let Ok(mut window) = windows.single_mut() {
            window.mode = if settings.fullscreen {
                WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            } else {
                WindowMode::Windowed
            };
        }
    }

    // Bloom toggle
    for (btn_ent, interaction, children) in bloom_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        settings.bloom_enabled = !settings.bloom_enabled;
        save_player_settings(&settings);
        let (label, color) = toggle_label(&translations, settings.bloom_enabled, on_color, off_color);
        update_btn_text(&mut text_q, children, label);
        if let Ok(mut bg) = bg_q.get_mut(btn_ent) { bg.0 = color; }
        if let Ok(mut bloom) = bloom_cam_q.single_mut() {
            bloom.intensity = if settings.bloom_enabled { BLOOM_INTENSITY } else { 0.0 };
        }
    }

    // Volume buttons
    let vol_active   = Color::srgba(0.25, 0.45, 0.70, 0.90);
    let vol_inactive = Color::srgba(0.18, 0.20, 0.26, 0.70);
    let pressed_vol = vol_q.iter().find_map(|(_, vb, i, _)| {
        if *i == Interaction::Pressed && (settings.master_volume - vb.0).abs() > 0.05 { Some(vb.0) } else { None }
    });
    if let Some(new_vol) = pressed_vol {
        settings.master_volume = new_vol;
        save_player_settings(&settings);
        for (_, vb, _, mut bg) in vol_q.iter_mut() {
            bg.0 = if (settings.master_volume - vb.0).abs() < 0.05 { vol_active } else { vol_inactive };
        }
        if let Some(ref mut snd) = sound_settings {
            snd.master_volume = settings.master_volume;
        }
    }

    // SFX toggle
    for (btn_ent, interaction, children) in sfx_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        settings.sfx_enabled = !settings.sfx_enabled;
        save_player_settings(&settings);
        let (label, color) = toggle_label(&translations, settings.sfx_enabled, on_color, off_color);
        update_btn_text(&mut text_q, children, label);
        if let Ok(mut bg) = bg_q.get_mut(btn_ent) { bg.0 = color; }
        if let Some(ref mut snd) = sound_settings {
            snd.sfx_volume = if settings.sfx_enabled { 1.0 } else { 0.0 };
            snd.muted = !settings.sfx_enabled;
        }
    }
}

/// Apply saved bloom setting on startup (runs after scene setup creates the camera).
pub fn apply_bloom_setting(
    settings: Res<PlayerSettings>,
    mut bloom_q: Query<&mut Bloom, With<Camera3d>>,
) {
    if !settings.bloom_enabled {
        if let Ok(mut bloom) = bloom_q.single_mut() {
            bloom.intensity = 0.0;
        }
    }
}

fn toggle_label<'a>(t: &'a Translations, enabled: bool, on: Color, off: Color) -> (String, Color) {
    if enabled {
        (t.ui_or("anna_on", "ON").to_string(), on)
    } else {
        (t.ui_or("anna_off", "OFF").to_string(), off)
    }
}

fn update_btn_text(text_q: &mut Query<&mut Text>, children: &Children, label: String) {
    for child in children.iter() {
        if let Ok(mut t) = text_q.get_mut(child) { **t = label.clone(); }
    }
}
