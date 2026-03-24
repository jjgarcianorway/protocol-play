// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's in-game message display.
//! A full-width bottom card with left accent stripe — same visual language
//! as chapter titles, adapted for non-intrusive in-game presence.

use bevy::prelude::*;
use crate::ui_theme::{palette, typo, spacing};

// ─── Display constants ────────────────────────────────────────────────────────

pub const ANNA_FADE_IN:     f32 = 0.55;
pub const ANNA_FADE_OUT:    f32 = 0.80;
pub const ANNA_WORDS_PER_S: f32 = 2.5;   // comfortable reading pace
pub const ANNA_MIN_HOLD:    f32 = 5.0;
pub const ANNA_MAX_HOLD:    f32 = 18.0;

// ─── Resource ─────────────────────────────────────────────────────────────────

/// Drives Anna's comment queue and current display.
#[derive(Resource, Default)]
pub struct AnnaComments {
    /// Queued comments: (seconds_until_show, text).
    pub queue: Vec<(f32, String)>,
    /// (text, elapsed, total_duration)
    pub current: Option<(String, f32, f32)>,
}

// ─── Components ───────────────────────────────────────────────────────────────

#[derive(Component)] pub struct AnnaPanel;
#[derive(Component)] pub struct AnnaLabelText;
#[derive(Component)] pub struct AnnaCommentText;

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Hold duration proportional to word count.
pub fn hold_time(text: &str) -> f32 {
    let words = text.split_whitespace().count() as f32;
    (words / ANNA_WORDS_PER_S).clamp(ANNA_MIN_HOLD, ANNA_MAX_HOLD)
}

/// Pick `count` random comments from `pool`, spaced 40–80 s apart (first at 20–40 s).
pub fn build_queue(pool: &[String], count: usize) -> Vec<(f32, String)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n = count.min(pool.len());
    let mut indices: Vec<usize> = (0..pool.len()).collect();
    for i in (1..indices.len()).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }
    let mut queue = Vec::new();
    let mut t = rng.gen_range(20.0..40.0f32);
    for &idx in indices.iter().take(n) {
        queue.push((t, pool[idx].clone()));
        t += rng.gen_range(40.0..80.0f32);
    }
    queue
}

// ─── Spawn ────────────────────────────────────────────────────────────────────

/// Spawn the Anna card panel. Call once in setup; it stays hidden (alpha=0) until
/// a message is ready to show. `name_label` is the sender name shown above the
/// message — typically "ANNA" or "ANNA" in all languages (it's a name).
pub fn spawn_anna_ui(commands: &mut Commands, font: &Handle<Font>, name_label: &str) {
    let label_font = TextFont { font: font.clone(), font_size: typo::MICRO, ..default() };
    let msg_font   = TextFont { font: font.clone(), font_size: typo::SMALL, ..default() };

    commands.spawn((
        AnnaPanel,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                left: Val::Px(24.0), right: Val::Px(24.0),
                top: Val::Px(14.0),  bottom: Val::Px(14.0),
            },
            border: UiRect { left: Val::Px(spacing::RADIUS_SM), ..default() },
            row_gap: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(palette::ANNA_BG.with_alpha(0.0)),
        BorderColor::all(palette::ANNA_ACCENT.with_alpha(0.0)),
        GlobalZIndex(70),
    )).with_children(|p| {
        p.spawn((
            Text::new(name_label.to_string()),
            label_font,
            TextColor(palette::ANNA_ACCENT.with_alpha(0.0)),
            AnnaLabelText,
        ));
        p.spawn((
            Text::new(""),
            msg_font,
            TextColor(palette::ANNA_TEXT.with_alpha(0.0)),
            AnnaCommentText,
        ));
    });
}

// ─── Tick ─────────────────────────────────────────────────────────────────────

/// Drive the comment queue and animate the card panel.
pub fn tick_anna_comments(
    time:      Res<Time>,
    mut anna:  ResMut<AnnaComments>,
    mut panel: Query<(&mut BackgroundColor, &mut BorderColor), With<AnnaPanel>>,
    mut label: Query<&mut TextColor, (With<AnnaLabelText>, Without<AnnaCommentText>)>,
    mut msg:   Query<(&mut Text, &mut TextColor), With<AnnaCommentText>>,
) {
    let dt = time.delta_secs();

    // Tick queue timers
    for (t, _) in anna.queue.iter_mut() { *t -= dt; }

    // Promote next queued comment when idle
    if anna.current.is_none() {
        if let Some(idx) = anna.queue.iter().position(|(t, _)| *t <= 0.0) {
            let (_, text) = anna.queue.remove(idx);
            let hold = hold_time(&text);
            let total = ANNA_FADE_IN + hold + ANNA_FADE_OUT;
            anna.current = Some((text, 0.0, total));
        }
    }

    let alpha = if let Some((ref text, ref mut elapsed, total)) = anna.current {
        *elapsed += dt;
        let a = if *elapsed < ANNA_FADE_IN {
            *elapsed / ANNA_FADE_IN
        } else if *elapsed > total - ANNA_FADE_OUT {
            ((total - *elapsed) / ANNA_FADE_OUT).max(0.0)
        } else {
            1.0
        };
        // Update message text
        for (mut t, _) in msg.iter_mut() { **t = text.clone(); }
        if *elapsed >= total { anna.current = None; }
        a
    } else {
        // Clear text when nothing showing
        for (mut t, _) in msg.iter_mut() { if !(**t).is_empty() { **t = String::new(); } }
        0.0
    };

    // Apply alpha to panel background, accent border, label, and message
    let bg_base = palette::ANNA_BG.to_srgba();
    for (mut bg, mut border) in panel.iter_mut() {
        bg.0 = Color::srgba(bg_base.red, bg_base.green, bg_base.blue, bg_base.alpha * alpha);
        *border = BorderColor::all(palette::ANNA_ACCENT.with_alpha(alpha));
    }
    for mut tc in label.iter_mut() {
        tc.0 = palette::ANNA_ACCENT.with_alpha(alpha * 0.75);
    }
    for (_, mut tc) in msg.iter_mut() {
        tc.0 = palette::ANNA_TEXT.with_alpha(alpha);
    }
}
