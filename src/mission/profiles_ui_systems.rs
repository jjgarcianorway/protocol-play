// SPDX-License-Identifier: GPL-3.0-or-later

//! Profile selection screen — interaction systems (hover, click, rename, delete).

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::profiles::*;
use super::profiles_ui::*;

// ── Fade in ─────────────────────────────────────────────────────────────

/// Fade in profile selection screen elements over time.
pub fn animate_profile_fade_in(
    time: Res<Time>,
    mut state: ResMut<ProfileSelectState>,
    mut name_q: Query<&mut TextColor, With<ProfileNameText>>,
    mut detail_q: Query<&mut TextColor, (With<ProfileDetailText>, Without<ProfileNameText>)>,
    mut title_q: Query<
        &mut TextColor,
        (With<ProfileTitleText>, Without<ProfileNameText>, Without<ProfileDetailText>),
    >,
    mut slot_q: Query<(&ProfileSlot, &mut BackgroundColor, &mut BorderColor)>,
) {
    if state.fade_out_active { return; }
    state.fade_in_timer += time.delta_secs();
    let alpha = (state.fade_in_timer / PROFILE_FADE_DURATION).clamp(0.0, 1.0);

    for mut color in name_q.iter_mut() {
        let c = color.0.to_srgba();
        color.0 = Color::srgba(c.red, c.green, c.blue, alpha);
    }
    for mut color in detail_q.iter_mut() {
        let c = color.0.to_srgba();
        color.0 = Color::srgba(c.red, c.green, c.blue, alpha);
    }
    for mut color in title_q.iter_mut() {
        let c = color.0.to_srgba();
        color.0 = Color::srgba(c.red, c.green, c.blue, alpha);
    }
    for (_slot, mut bg, mut border) in slot_q.iter_mut() {
        bg.0 = Color::srgba(
            PROFILE_CARD_BG.0, PROFILE_CARD_BG.1,
            PROFILE_CARD_BG.2, PROFILE_CARD_BG.3 * alpha,
        );
        *border = BorderColor::all(Color::srgba(
            PROFILE_CARD_BORDER.0, PROFILE_CARD_BORDER.1,
            PROFILE_CARD_BORDER.2, PROFILE_CARD_BORDER.3 * alpha,
        ));
    }
}

// ── Slot hover ──────────────────────────────────────────────────────────

/// Handle profile slot hover effects.
pub fn profile_slot_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &mut BoxShadow),
        (With<ProfileSlot>, Changed<Interaction>),
    >,
    state: Res<ProfileSelectState>,
) {
    if state.fade_out_active || state.rename_active { return; }
    for (interaction, mut bg, mut border, mut shadow) in query.iter_mut() {
        let (bg_c, brd_c, glow_a) = match interaction {
            Interaction::Hovered => (
                PROFILE_CARD_HOVER_BG, PROFILE_CARD_HOVER_BORDER,
                PROFILE_GLOW_COLOR.3,
            ),
            _ => (PROFILE_CARD_BG, PROFILE_CARD_BORDER, 0.0),
        };
        bg.0 = Color::srgba(bg_c.0, bg_c.1, bg_c.2, bg_c.3);
        *border = BorderColor::all(Color::srgba(brd_c.0, brd_c.1, brd_c.2, brd_c.3));
        *shadow = BoxShadow::new(
            Color::srgba(PROFILE_GLOW_COLOR.0, PROFILE_GLOW_COLOR.1,
                PROFILE_GLOW_COLOR.2, glow_a),
            Val::ZERO, Val::ZERO, Val::Px(5.0), Val::Px(12.0),
        );
    }
}

// ── Slot click ──────────────────────────────────────────────────────────

/// Handle profile slot click — begin transition to main menu.
pub fn profile_slot_click(
    query: Query<(&Interaction, &ProfileSlot), Changed<Interaction>>,
    mut state: ResMut<ProfileSelectState>,
    confirm_q: Query<Entity, With<ProfileDeleteConfirm>>,
) {
    if state.fade_out_active || state.rename_active { return; }
    if !confirm_q.is_empty() { return; }

    for (interaction, slot) in query.iter() {
        if *interaction == Interaction::Pressed {
            state.selected_profile = slot.0;
            state.fade_out_active = true;
            state.fade_out_timer = 0.0;
        }
    }
}

// ── Fade out + transition ───────────────────────────────────────────────

/// Fade to black and transition to MainMenu after selecting a profile.
pub fn profile_fade_out(
    time: Res<Time>,
    mut state: ResMut<ProfileSelectState>,
    mut fade_q: Query<&mut BackgroundColor, With<ProfileFadeOverlay>>,
    mut next_state: ResMut<NextState<AppPhase>>,
    mut profile_mgr: ResMut<ProfileManager>,
) {
    if !state.fade_out_active { return; }
    state.fade_out_timer += time.delta_secs();
    let alpha = (state.fade_out_timer / PROFILE_FADE_OUT_DURATION).clamp(0.0, 1.0);

    for mut bg in fade_q.iter_mut() {
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    }

    if state.fade_out_timer >= PROFILE_FADE_OUT_DURATION {
        let index = state.selected_profile;
        profile_mgr.active_profile = index;
        activate_profile(index);
        next_state.set(AppPhase::MainMenu);
    }
}

// ── Delete button ───────────────────────────────────────────────────────

/// Handle delete button hover — change text color.
pub fn profile_delete_hover(
    mut query: Query<
        (&Interaction, &Children),
        (With<ProfileDeleteBtn>, Changed<Interaction>),
    >,
    mut text_q: Query<&mut TextColor>,
) {
    for (interaction, children) in query.iter_mut() {
        let c = match interaction {
            Interaction::Hovered | Interaction::Pressed => PROFILE_DELETE_HOVER_COLOR,
            _ => PROFILE_DELETE_COLOR,
        };
        for child in children.iter() {
            if let Ok(mut tc) = text_q.get_mut(child) {
                tc.0 = Color::srgb(c.0, c.1, c.2);
            }
        }
    }
}

/// Handle delete button click — spawn confirmation dialog.
pub fn profile_delete_click(
    query: Query<(&Interaction, &ProfileDeleteBtn), Changed<Interaction>>,
    mut commands: Commands,
    confirm_q: Query<Entity, With<ProfileDeleteConfirm>>,
    font: Res<MissionFont>,
    state: Res<ProfileSelectState>,
) {
    if state.fade_out_active { return; }
    if !confirm_q.is_empty() { return; }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            spawn_delete_confirm(&mut commands, &font.0, btn.0);
        }
    }
}

// ── Delete confirm buttons ──────────────────────────────────────────────

/// Handle confirm/cancel button clicks on delete dialog.
pub fn profile_confirm_click(
    query: Query<(&Interaction, &ProfileConfirmBtn), Changed<Interaction>>,
    confirm_q: Query<Entity, With<ProfileDeleteConfirm>>,
    mut commands: Commands,
    root_q: Query<Entity, With<ProfileSelectRoot>>,
    fade_overlay_q: Query<Entity, With<ProfileFadeOverlay>>,
    font: Res<MissionFont>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        match btn {
            ProfileConfirmBtn::YesDelete(index) => {
                delete_profile(*index);
                // Dismiss and rebuild
                for e in confirm_q.iter() { commands.entity(e).despawn(); }
                for e in root_q.iter() { commands.entity(e).despawn(); }
                for e in fade_overlay_q.iter() { commands.entity(e).despawn(); }
                commands.remove_resource::<ProfileSelectState>();
                // Re-spawn profile UI via the enter system
                let f = font.0.clone();
                commands.queue(move |world: &mut World| {
                    world.insert_resource(ProfileSelectState {
                        fade_in_timer: PROFILE_FADE_DURATION, // skip fade-in
                        ..Default::default()
                    });
                    // Trigger enter_profile_select manually via schedule is tricky,
                    // so we call the spawn function directly.
                    let profiles = load_all_profiles();
                    super::profiles_ui::spawn_profiles_from_world(world, &f, &profiles);
                });
            }
            ProfileConfirmBtn::NoCancel => {
                for e in confirm_q.iter() { commands.entity(e).despawn(); }
            }
        }
    }
}

/// Hover effect for confirm dialog buttons.
pub fn profile_confirm_hover(
    mut q: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<ProfileConfirmBtn>, Changed<Interaction>),
    >,
) {
    for (int, mut bg, mut border) in q.iter_mut() {
        let (c, d) = if *int == Interaction::Hovered {
            (MENU_BUTTON_HOVER_BG, MENU_BUTTON_HOVER_BORDER)
        } else {
            (MENU_BUTTON_BG, MENU_BUTTON_BORDER_COLOR)
        };
        bg.0 = Color::srgba(c.0, c.1, c.2, c.3);
        *border = BorderColor::all(Color::srgba(d.0, d.1, d.2, d.3));
    }
}

// ── Rename ──────────────────────────────────────────────────────────────

/// Click on a profile name to start renaming.
pub fn profile_name_click(
    query: Query<(&Interaction, &ProfileNameText), Changed<Interaction>>,
    mut state: ResMut<ProfileSelectState>,
    confirm_q: Query<Entity, With<ProfileDeleteConfirm>>,
    name_q: Query<(&ProfileNameText, &Text)>,
) {
    if state.fade_out_active { return; }
    if !confirm_q.is_empty() { return; }

    for (interaction, name_marker) in query.iter() {
        if *interaction == Interaction::Pressed {
            state.rename_active = true;
            state.rename_index = name_marker.0;
            for (nm, text) in name_q.iter() {
                if nm.0 == name_marker.0 {
                    state.rename_text = text.0.clone();
                }
            }
        }
    }
}

/// Handle keyboard input during rename.
pub fn profile_rename_keyboard(
    mut state: ResMut<ProfileSelectState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut name_q: Query<(&ProfileNameText, &mut Text, &mut TextColor)>,
) {
    if !state.rename_active { return; }

    if keys.just_pressed(KeyCode::Escape) {
        let info = load_profile_info(state.rename_index);
        for (nm, mut text, mut color) in name_q.iter_mut() {
            if nm.0 == state.rename_index {
                **text = info.name.clone();
                color.0 = Color::srgb(
                    PROFILE_NAME_COLOR.0, PROFILE_NAME_COLOR.1, PROFILE_NAME_COLOR.2,
                );
            }
        }
        state.rename_active = false;
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        let name = state.rename_text.trim().to_string();
        if !name.is_empty() {
            save_profile_name(state.rename_index, &name);
        }
        for (nm, _text, mut color) in name_q.iter_mut() {
            if nm.0 == state.rename_index {
                color.0 = Color::srgb(
                    PROFILE_NAME_COLOR.0, PROFILE_NAME_COLOR.1, PROFILE_NAME_COLOR.2,
                );
            }
        }
        state.rename_active = false;
        return;
    }

    let mut changed = false;
    if keys.just_pressed(KeyCode::Backspace) && !state.rename_text.is_empty() {
        state.rename_text.pop();
        changed = true;
    }

    for key in keys.get_just_pressed() {
        if state.rename_text.len() >= 20 { break; }
        let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        if let Some(ch) = key_to_char(*key, shift) {
            state.rename_text.push(ch);
            changed = true;
        }
    }

    if changed {
        let display = if state.rename_text.is_empty() {
            "Type a name...".to_string()
        } else {
            format!("{}|", state.rename_text)
        };
        for (nm, mut text, mut color) in name_q.iter_mut() {
            if nm.0 == state.rename_index {
                **text = display.clone();
                color.0 = Color::srgb(0.95, 0.9, 0.7);
            }
        }
    }
}

fn key_to_char(key: KeyCode, shift: bool) -> Option<char> {
    let ch = match key {
        KeyCode::KeyA => 'a', KeyCode::KeyB => 'b', KeyCode::KeyC => 'c',
        KeyCode::KeyD => 'd', KeyCode::KeyE => 'e', KeyCode::KeyF => 'f',
        KeyCode::KeyG => 'g', KeyCode::KeyH => 'h', KeyCode::KeyI => 'i',
        KeyCode::KeyJ => 'j', KeyCode::KeyK => 'k', KeyCode::KeyL => 'l',
        KeyCode::KeyM => 'm', KeyCode::KeyN => 'n', KeyCode::KeyO => 'o',
        KeyCode::KeyP => 'p', KeyCode::KeyQ => 'q', KeyCode::KeyR => 'r',
        KeyCode::KeyS => 's', KeyCode::KeyT => 't', KeyCode::KeyU => 'u',
        KeyCode::KeyV => 'v', KeyCode::KeyW => 'w', KeyCode::KeyX => 'x',
        KeyCode::KeyY => 'y', KeyCode::KeyZ => 'z',
        KeyCode::Digit0 => '0', KeyCode::Digit1 => '1', KeyCode::Digit2 => '2',
        KeyCode::Digit3 => '3', KeyCode::Digit4 => '4', KeyCode::Digit5 => '5',
        KeyCode::Digit6 => '6', KeyCode::Digit7 => '7', KeyCode::Digit8 => '8',
        KeyCode::Digit9 => '9',
        KeyCode::Space => ' ',
        _ => return None,
    };
    Some(if shift && ch.is_ascii_lowercase() { ch.to_ascii_uppercase() } else { ch })
}

// ── Cleanup ─────────────────────────────────────────────────────────────

/// Cleanup profile selection screen when leaving the state.
pub fn cleanup_profile_select(
    mut commands: Commands,
    root_q: Query<Entity, With<ProfileSelectRoot>>,
    fade_q: Query<Entity, With<ProfileFadeOverlay>>,
    confirm_q: Query<Entity, With<ProfileDeleteConfirm>>,
) {
    for entity in root_q.iter() { commands.entity(entity).despawn(); }
    for entity in fade_q.iter() { commands.entity(entity).despawn(); }
    for entity in confirm_q.iter() { commands.entity(entity).despawn(); }
    commands.remove_resource::<ProfileSelectState>();
}

/// Called periodically to sync game_state.json back to the active profile.
#[allow(dead_code)]
pub fn sync_profile_on_save(profile_mgr: Res<ProfileManager>) {
    let index = profile_mgr.active_profile;
    let gs = crate::save_state::load_game_state();
    crate::save_state::save_profile_game_state(index, &gs);
}
