// SPDX-License-Identifier: GPL-3.0-or-later

//! Shared Anna in-game comment system. Brief, rare, contextual comments
//! from Anna that appear as subtle text in the bottom-left corner.

use bevy::prelude::*;

// === Constants ===
pub const ANNA_FONT_SIZE: f32 = 13.0;
pub const ANNA_COLOR: (f32, f32, f32) = (0.45, 0.65, 0.85); // warm blue
pub const ANNA_MAX_ALPHA: f32 = 0.6;
pub const ANNA_DISPLAY_TIME: f32 = 3.5;
pub const ANNA_FADE_TIME: f32 = 0.3;

/// Resource that drives Anna's comment queue and current display.
#[derive(Resource)]
pub struct AnnaComments {
    /// Queued comments: (seconds_until_show, text).
    pub queue: Vec<(f32, String)>,
    /// Currently displayed comment: (text, remaining_display_time).
    pub current: Option<(String, f32)>,
}

impl Default for AnnaComments {
    fn default() -> Self {
        Self { queue: Vec::new(), current: None }
    }
}

/// Component marker for Anna's text UI node.
#[derive(Component)]
pub struct AnnaCommentText;

/// Pick `count` random comments from `pool`, spaced 40-80s apart (first at 20-40s).
pub fn build_queue(pool: &[&str], count: usize) -> Vec<(f32, String)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n = count.min(pool.len());
    // Shuffle indices
    let mut indices: Vec<usize> = (0..pool.len()).collect();
    for i in (1..indices.len()).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }
    let mut queue = Vec::new();
    let mut t = rng.gen_range(20.0..40.0f32);
    for &idx in indices.iter().take(n) {
        queue.push((t, pool[idx].to_string()));
        t += rng.gen_range(40.0..80.0f32);
    }
    queue
}

/// Spawn the Anna comment text node (call once in setup).
pub fn spawn_anna_ui(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(14.0),
            bottom: Val::Px(24.0),
            max_width: Val::Px(360.0),
            ..default()
        },
        GlobalZIndex(50),
    )).with_child((
        Text::new(""),
        TextFont { font: font.clone(), font_size: ANNA_FONT_SIZE, ..default() },
        TextColor(Color::srgba(ANNA_COLOR.0, ANNA_COLOR.1, ANNA_COLOR.2, 0.0)),
        AnnaCommentText,
    ));
}

/// Tick the comment system: decrement queue timers, manage display and fading.
pub fn tick_anna_comments(
    time: Res<Time>,
    mut anna: ResMut<AnnaComments>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<AnnaCommentText>>,
) {
    let dt = time.delta_secs();

    // Tick queue timers
    for (t, _) in anna.queue.iter_mut() {
        *t -= dt;
    }

    // Check if a queued comment is ready and no current comment is showing
    if anna.current.is_none() {
        if let Some(idx) = anna.queue.iter().position(|(t, _)| *t <= 0.0) {
            let (_, text) = anna.queue.remove(idx);
            anna.current = Some((text, ANNA_DISPLAY_TIME + ANNA_FADE_TIME * 2.0));
        }
    }

    // Update display
    if let Some((ref text, ref mut timer)) = anna.current {
        let total = ANNA_DISPLAY_TIME + ANNA_FADE_TIME * 2.0;
        let elapsed = total - *timer;
        let alpha = if elapsed < ANNA_FADE_TIME {
            // Fade in
            (elapsed / ANNA_FADE_TIME) * ANNA_MAX_ALPHA
        } else if *timer < ANNA_FADE_TIME {
            // Fade out
            (*timer / ANNA_FADE_TIME) * ANNA_MAX_ALPHA
        } else {
            ANNA_MAX_ALPHA
        };

        for (mut t, mut color) in text_q.iter_mut() {
            **t = text.clone();
            *color = TextColor(Color::srgba(
                ANNA_COLOR.0, ANNA_COLOR.1, ANNA_COLOR.2, alpha,
            ));
        }

        *timer -= dt;
        if *timer <= 0.0 {
            // Done — clear
            anna.current = None;
            for (mut t, mut color) in text_q.iter_mut() {
                **t = String::new();
                *color = TextColor(Color::srgba(ANNA_COLOR.0, ANNA_COLOR.1, ANNA_COLOR.2, 0.0));
            }
        }
    }
}
