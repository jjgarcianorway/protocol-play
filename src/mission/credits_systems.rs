// SPDX-License-Identifier: GPL-3.0-or-later

//! Credits screen systems — scrolling, fading, keyboard input.

use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use super::constants::*;
use super::credits::*;

/// Tick credits: fade in, scroll, fade out, cleanup.
pub fn update_credits(
    time: Res<Time>,
    mut state: ResMut<CreditsState>,
    mut scroller_q: Query<&mut Node, With<CreditsScroller>>,
    mut fade_q: Query<&mut BackgroundColor, With<CreditsFadeOverlay>>,
    computed_q: Query<&ComputedNode, With<CreditsScroller>>,
) {
    let dt = time.delta_secs();
    state.elapsed += dt;

    // Update content height from computed layout
    if let Ok(computed) = computed_q.single() {
        let h = computed.size().y;
        if h > 10.0 {
            state.content_height = h;
        }
    }

    match state.phase {
        CreditsPhase::FadingIn => {
            let alpha = 1.0 - (state.elapsed / CREDITS_FADE_IN_DURATION).clamp(0.0, 1.0);
            for mut bg in fade_q.iter_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if state.elapsed >= CREDITS_FADE_IN_DURATION {
                state.phase = CreditsPhase::Scrolling;
                state.elapsed = 0.0;
            }
        }
        CreditsPhase::Scrolling => {
            state.scroll_offset += CREDITS_SCROLL_SPEED * dt;
            let start_y = 800.0_f32;
            let new_top = start_y - state.scroll_offset;

            for mut node in scroller_q.iter_mut() {
                node.top = Val::Px(new_top);
            }

            // End scrolling when content has fully scrolled past
            let end_threshold = state.content_height + 200.0;
            if state.scroll_offset >= end_threshold {
                state.phase = CreditsPhase::FadingOut;
                state.elapsed = 0.0;
            }
        }
        CreditsPhase::FadingOut => {
            let alpha = (state.elapsed / CREDITS_FADE_OUT_DURATION).clamp(0.0, 1.0);
            for mut bg in fade_q.iter_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if state.elapsed >= CREDITS_FADE_OUT_DURATION {
                state.phase = CreditsPhase::Done;
            }
        }
        CreditsPhase::Done => {}
    }
}

/// ESC to skip credits (triggers fade out or immediate cleanup).
pub fn credits_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<CreditsState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.phase {
            CreditsPhase::FadingIn | CreditsPhase::Scrolling => {
                state.phase = CreditsPhase::FadingOut;
                state.elapsed = 0.0;
            }
            CreditsPhase::FadingOut => {
                // Speed up fade
                state.phase = CreditsPhase::Done;
            }
            CreditsPhase::Done => {}
        }
    }
}

/// Cleanup credits entities when done.
pub fn cleanup_credits(
    state: Res<CreditsState>,
    mut commands: Commands,
    root_q: Query<Entity, With<CreditsRoot>>,
    fade_q: Query<Entity, With<CreditsFadeOverlay>>,
    hint_q: Query<Entity, With<CreditsSkipHint>>,
) {
    if state.phase != CreditsPhase::Done {
        return;
    }

    for entity in root_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in fade_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in hint_q.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<CreditsState>();
}
