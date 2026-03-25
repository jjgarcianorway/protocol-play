// SPDX-License-Identifier: GPL-3.0-or-later
//! Chapter 13 reveal: Anna tells the truth.
//! Replaces the generic congrats screen when all 149 levels are completed.

use bevy::prelude::*;
use crate::types::GameFont;
use crate::ui_theme::{palette, typo, spacing};
use crate::i18n::Translations;

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct RevealScreen;

#[derive(Component)]
pub struct RevealLine { pub delay: f32, pub shown: bool }

#[derive(Resource)]
pub struct RevealTimer(pub f32);

// ─── Reveal messages ─────────────────────────────────────────────────────────

fn reveal_lines(t: &Translations) -> Vec<(f32, String)> {
    // (delay_seconds, text)
    // Each line fades in after its delay from the previous line
    vec![
        (2.0, t.get_or("reveal.0", "...").to_string()),
        (3.0, t.get_or("reveal.1", "I need to tell you something.").to_string()),
        (4.0, t.get_or("reveal.2", "The puzzles you've been solving weren't just puzzles.").to_string()),
        (4.0, t.get_or("reveal.3", "The bots weren't just bots. They're repair drones.").to_string()),
        (4.0, t.get_or("reveal.4", "The tiles weren't abstract. They're ship systems.").to_string()),
        (5.0, t.get_or("reveal.5",
            "You've been routing repair drones through the subsystems of an ark ship.").to_string()),
        (5.0, t.get_or("reveal.6",
            "14,892 people are sleeping in cryogenic pods. They don't know you exist.").to_string()),
        (4.0, t.get_or("reveal.7",
            "Every connection you made kept them alive a little longer.").to_string()),
        (5.0, t.get_or("reveal.8",
            "I'm Anna. I'm the ship's AI. And I couldn't have done this without you.").to_string()),
        (4.0, t.get_or("reveal.9", "Thank you.").to_string()),
    ]
}

fn reveal_stats(t: &Translations, levels: usize, stars: u32, time_secs: u64) -> Vec<String> {
    vec![
        format!("{} {}", levels, t.get_or("reveal.systems", "systems repaired")),
        format!("★ {stars}"),
        format!("{}:{:02} {}", time_secs / 60, time_secs % 60,
            t.get_or("reveal.keeping", "keeping them safe")),
    ]
}

// ─── Spawn ───────────────────────────────────────────────────────────────────

pub fn spawn_reveal(
    commands: &mut Commands, f: &Handle<Font>, t: &Translations,
    total_levels: usize, total_stars: u32, total_time_secs: u64,
) {
    let lines = reveal_lines(t);
    let stats = reveal_stats(t, total_levels, total_stars, total_time_secs);

    commands.insert_resource(RevealTimer(0.0));

    // Full-screen dark overlay
    commands.spawn((
        RevealScreen,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(60.0)),
            row_gap: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.04, 0.96)),
        GlobalZIndex(400),
    )).with_children(|root| {
        // Reveal text lines — each starts invisible
        let mut cumulative_delay = 0.0f32;
        for (delay, text) in &lines {
            cumulative_delay += delay;
            root.spawn((
                Text::new(text.as_str()),
                TextFont { font: f.clone(), font_size: typo::H3, ..default() },
                TextColor(Color::srgba(0.85, 0.88, 0.92, 0.0)), // starts invisible
                RevealLine { delay: cumulative_delay, shown: false },
                Node { margin: UiRect::vertical(Val::Px(4.0)),
                    max_width: Val::Px(600.0), ..default() },
            ));
        }

        // Spacer before stats
        root.spawn((
            Node { height: Val::Px(30.0), ..default() },
            RevealLine { delay: cumulative_delay + 4.0, shown: false },
        ));

        // Stats — shown after all reveal text
        let stats_delay = cumulative_delay + 5.0;
        for (i, stat) in stats.iter().enumerate() {
            root.spawn((
                Text::new(stat.as_str()),
                TextFont { font: f.clone(), font_size: typo::BODY, ..default() },
                TextColor(Color::srgba(0.55, 0.75, 0.70, 0.0)), // teal, invisible
                RevealLine { delay: stats_delay + i as f32 * 1.5, shown: false },
            ));
        }

        // Final spacer + close hint
        let close_delay = stats_delay + stats.len() as f32 * 1.5 + 3.0;
        root.spawn((
            Text::new(t.get_or("reveal.close", "[ press any key ]")),
            TextFont { font: f.clone(), font_size: typo::SMALL, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            RevealLine { delay: close_delay, shown: false },
            Node { margin: UiRect::top(Val::Px(30.0)), ..default() },
        ));
    });
}

// ─── Update ──────────────────────────────────────────────────────────────────

/// Animate reveal lines: fade in each line when its delay is reached.
pub fn tick_reveal(
    time: Res<Time>,
    mut timer: Option<ResMut<RevealTimer>>,
    mut lines: Query<(&mut RevealLine, &mut TextColor)>,
) {
    let Some(ref mut timer) = timer else { return };
    timer.0 += time.delta_secs();
    let t = timer.0;

    for (mut line, mut color) in lines.iter_mut() {
        if line.shown { continue; }
        if t >= line.delay {
            line.shown = true;
            // Set full alpha — the text appears
            let c = color.0.to_srgba();
            color.0 = Color::srgba(c.red, c.green, c.blue, 1.0);
        }
    }
}

/// Close reveal on any key press (after all lines shown).
pub fn close_reveal(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    screen: Query<Entity, With<RevealScreen>>,
    lines: Query<&RevealLine>,
    timer: Option<Res<RevealTimer>>,
) {
    if screen.is_empty() { return; }
    let Some(_timer) = timer else { return };
    // Only close after all lines are shown
    let all_shown = lines.iter().all(|l| l.shown);
    if !all_shown { return; }
    if keys.get_just_pressed().count() > 0 || mouse.just_pressed(MouseButton::Left) {
        for e in screen.iter() {
            commands.entity(e).insert(crate::types::UiBgFade { target: 0.0, despawn_at_zero: true });
        }
        commands.remove_resource::<RevealTimer>();
    }
}
