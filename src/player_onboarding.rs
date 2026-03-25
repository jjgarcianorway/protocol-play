// SPDX-License-Identifier: GPL-3.0-or-later
//! Tutorial hints for new players in chapter 1.
//! Brief overlays that fade in, stay 3 seconds, then fade out.

use bevy::prelude::*;
use crate::types::{GameFont, UiBgFade};
use crate::ui_theme::{palette, typo, spacing};
use crate::i18n::Translations;
use crate::player::PlayerLevels;
use crate::player::player_progress::PlayerProgress;

// ─── Components & Resources ──────────────────────────────────────────────────

#[derive(Component)]
pub struct HintOverlay;

#[derive(Resource)]
pub struct HintTimer {
    pub elapsed: f32,
    pub phase: HintPhase,
}

#[derive(PartialEq)]
pub enum HintPhase {
    FadeIn,
    Hold,
    FadeOut,
}

/// Tracks which hints have already been shown this session.
#[derive(Resource, Default)]
pub struct ShownHints(pub Vec<usize>);

// ─── Constants ───────────────────────────────────────────────────────────────

const HINT_FADE_IN: f32 = 0.5;
const HINT_HOLD: f32 = 3.0;
const HINT_FADE_OUT: f32 = 0.8;

// ─── Hint lookup ─────────────────────────────────────────────────────────────

/// Returns (i18n_key, english_fallback) for a tutorial level, or None.
fn hint_for_level(level_idx: usize) -> Option<(&'static str, &'static str)> {
    match level_idx {
        0 => Some(("hint.route", "Route the bot from source to goal")),
        1 => Some(("hint.turns", "Place turns to change direction")),
        2 => Some(("hint.paths", "Try different paths")),
        // Level 1-4 through 1-11: no hint
        11 => Some(("hint.inventory", "Drag tiles from your inventory to the board")),
        _ => None,
    }
}

// ─── Show hint on level load ─────────────────────────────────────────────────

/// Called after load_level. Checks if the current level needs a tutorial hint.
pub fn check_onboarding_hint(
    mut commands: Commands,
    levels: Res<PlayerLevels>,
    progress: Res<PlayerProgress>,
    font: Res<GameFont>,
    t: Res<Translations>,
    mut shown: ResMut<ShownHints>,
    existing: Query<Entity, With<HintOverlay>>,
) {
    if !levels.is_changed() || levels.levels.is_empty() { return; }
    // Don't show hint if one is already visible
    if !existing.is_empty() { return; }

    let idx = levels.current;

    // Don't show hint if already shown this session
    if shown.0.contains(&idx) { return; }

    // Don't show hint if level already has progress (player has seen it before)
    if idx < progress.data.len() && progress.data[idx].completed { return; }

    let Some((key, fallback)) = hint_for_level(idx) else { return };

    shown.0.push(idx);
    spawn_hint(&mut commands, &font.0, &t, key, fallback);
}

fn spawn_hint(
    commands: &mut Commands, font: &Handle<Font>,
    t: &Translations, key: &str, fallback: &str,
) {
    let text = t.get_or(key, fallback);

    commands.insert_resource(HintTimer { elapsed: 0.0, phase: HintPhase::FadeIn });

    commands.spawn((
        HintOverlay,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(90.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(80),
    )).with_children(|root| {
        root.spawn((
            Node {
                padding: UiRect::axes(Val::Px(28.0), Val::Px(12.0)),
                border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.06, 0.10, 0.0)),
        )).with_child((
            Text::new(text),
            TextFont { font: font.clone(), font_size: typo::BODY, ..default() },
            TextColor(Color::srgba(0.85, 0.88, 0.92, 0.0)),
        ));
    });
}

// ─── Animate hint ────────────────────────────────────────────────────────────

pub fn animate_hint(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Option<ResMut<HintTimer>>,
    overlay_q: Query<Entity, With<HintOverlay>>,
    mut bg_q: Query<&mut BackgroundColor>,
    mut text_q: Query<&mut TextColor>,
    children_q: Query<&Children>,
) {
    let Some(ref mut timer) = timer else { return };
    if overlay_q.is_empty() { return; }

    timer.elapsed += time.delta_secs();

    let alpha = match timer.phase {
        HintPhase::FadeIn => {
            let a = (timer.elapsed / HINT_FADE_IN).min(1.0);
            if timer.elapsed >= HINT_FADE_IN {
                timer.elapsed = 0.0;
                timer.phase = HintPhase::Hold;
            }
            a
        }
        HintPhase::Hold => {
            if timer.elapsed >= HINT_HOLD {
                timer.elapsed = 0.0;
                timer.phase = HintPhase::FadeOut;
            }
            1.0
        }
        HintPhase::FadeOut => {
            let a = 1.0 - (timer.elapsed / HINT_FADE_OUT).min(1.0);
            if timer.elapsed >= HINT_FADE_OUT {
                // Despawn
                for e in overlay_q.iter() { commands.entity(e).despawn(); }
                commands.remove_resource::<HintTimer>();
                return;
            }
            a
        }
    };

    // Apply alpha to background and text
    for overlay_e in overlay_q.iter() {
        if let Ok(children) = children_q.get(overlay_e) {
            for child in children.iter() {
                if let Ok(mut bg) = bg_q.get_mut(child) {
                    bg.0 = Color::srgba(0.04, 0.06, 0.10, 0.75 * alpha);
                }
                if let Ok(inner_children) = children_q.get(child) {
                    for inner in inner_children.iter() {
                        if let Ok(mut tc) = text_q.get_mut(inner) {
                            tc.0 = Color::srgba(0.85, 0.88, 0.92, alpha);
                        }
                    }
                }
            }
        }
    }
}
