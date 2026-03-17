// SPDX-License-Identifier: GPL-3.0-or-later

//! Ending screen animation systems — paragraph reveal, stats fade, glow pulse.

use bevy::prelude::*;
use std::fs;
use std::process::Command;
use super::endings::*;
use crate::save_state::{self, GameState};

const FADE_SPEED: f32 = 1.5;
const GLOW_PULSE_SPEED: f32 = 1.0;

/// System: animate the ending screen — fade in background, reveal paragraphs.
pub fn animate_ending_screen(
    time: Res<Time>,
    mut state: ResMut<EndingState>,
    mut screen_q: Query<&mut BackgroundColor, With<EndingScreen>>,
    mut title_q: Query<&mut TextColor, With<EndingTitle>>,
    mut para_q: Query<(&EndingParagraph, &mut TextColor), Without<EndingTitle>>,
) {
    let dt = time.delta_secs();

    // Fade in background
    state.fade_alpha = (state.fade_alpha + FADE_SPEED * dt).min(1.0);
    for mut bg in screen_q.iter_mut() {
        bg.0 = Color::srgba(0.02, 0.03, 0.06, state.fade_alpha * 0.95);
    }

    // Fade in title
    for mut tc in title_q.iter_mut() {
        let a = (state.fade_alpha * 0.8).min(1.0);
        tc.0 = Color::srgba(0.5, 0.55, 0.65, a);
    }

    // Paragraph reveal timer
    state.para_timer -= dt;
    if state.para_timer <= 0.0 && state.next_para < state.paragraphs.len() {
        state.next_para += 1;
        state.para_timer = 2.0; // 2 seconds between paragraphs

        if state.next_para >= state.paragraphs.len() {
            state.narrative_done = true;
        }
    }

    // Fade in revealed paragraphs
    for (para, mut tc) in para_q.iter_mut() {
        if para.0 < state.next_para {
            let c = tc.0.to_srgba();
            let new_a = (c.alpha + FADE_SPEED * dt).min(1.0);
            tc.0 = Color::srgba(0.88, 0.86, 0.82, new_a);
        }
    }
}

/// System: show stats and New Journey button after narrative completes.
pub fn animate_ending_stats(
    time: Res<Time>,
    mut state: ResMut<EndingState>,
    stats_q: Query<&Children, With<EndingStats>>,
    mut text_q: Query<&mut TextColor>,
    children_q: Query<&Children>,
    mut btn_q: Query<
        (&mut BackgroundColor, &mut BorderColor, &Children),
        With<NewJourneyBtn>,
    >,
) {
    if !state.narrative_done { return; }

    let dt = time.delta_secs();

    if !state.stats_shown {
        // Brief extra delay after last paragraph
        state.para_timer -= dt;
        if state.para_timer > -1.5 { return; }
        state.stats_shown = true;
    }

    // Fade in stat rows
    for children in stats_q.iter() {
        for child in children.iter() {
            // Each stat row has children (label text, value text)
            if let Ok(row_children) = children_q.get(child) {
                for text_ent in row_children.iter() {
                    if let Ok(mut tc) = text_q.get_mut(text_ent) {
                        let c = tc.0.to_srgba();
                        let new_a = (c.alpha + FADE_SPEED * dt).min(1.0);
                        // Preserve original color, just update alpha
                        tc.0 = Color::srgba(c.red, c.green, c.blue, new_a);
                    }
                }
            }
        }
    }

    // Fade in New Journey button
    for (mut bg, mut border, children) in btn_q.iter_mut() {
        let c = bg.0.to_srgba();
        let new_a = (c.alpha + FADE_SPEED * dt).min(0.9);
        bg.0 = Color::srgba(0.12, 0.14, 0.20, new_a);
        *border = BorderColor::all(Color::srgba(0.3, 0.35, 0.5, new_a));

        for child in children.iter() {
            if let Ok(mut tc) = text_q.get_mut(child) {
                let c = tc.0.to_srgba();
                let new_a = (c.alpha + FADE_SPEED * dt).min(1.0);
                tc.0 = Color::srgba(c.red, c.green, c.blue, new_a);
            }
        }
    }
}

/// System: pulse Anna's glow circle during the ending.
pub fn animate_ending_glow(
    time: Res<Time>,
    state: Res<EndingState>,
    mut query: Query<(&mut BackgroundColor, &mut BoxShadow), With<EndingAnnaGlow>>,
) {
    let t = time.elapsed_secs();
    let glow = state.ending.glow_color();
    let brightness = 0.7 + 0.3 * (t * GLOW_PULSE_SPEED).sin();

    for (mut bg, mut shadow) in query.iter_mut() {
        let r = glow.0 * brightness;
        let g = glow.1 * brightness;
        let b = glow.2 * brightness;
        *bg = BackgroundColor(Color::srgb(r, g, b));
        *shadow = BoxShadow::new(
            Color::srgba(r, g, b, 0.6 * brightness),
            Val::ZERO, Val::ZERO,
            Val::Px(8.0), Val::Px(20.0),
        );
    }
}

/// System: handle New Journey button hover.
pub fn new_journey_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NewJourneyBtn>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(0.20, 0.24, 0.34, 0.95));
                *border = BorderColor::all(Color::srgba(0.5, 0.6, 0.85, 0.9));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(0.12, 0.14, 0.20, 0.9));
                *border = BorderColor::all(Color::srgba(0.3, 0.35, 0.5, 0.6));
            }
        }
    }
}

/// System: handle New Journey button click — reset game for fresh playthrough.
pub fn new_journey_click(
    query: Query<&Interaction, (Changed<Interaction>, With<NewJourneyBtn>)>,
    mut gs: ResMut<GameState>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed { continue; }

        // Reset game state (increments playthrough_count)
        save_state::reset_for_new_game(&mut gs);
        save_state::save_game_state(&gs);

        // Delete bot game progress files
        let dir = save_state::exe_dir();
        delete_progress_files(&dir);

        // Try to regenerate campaign levels
        let gen_path = dir.join("generate-campaign");
        let gen_exists = gen_path.exists()
            || dir.join("generate-campaign.exe").exists();

        if gen_exists {
            let bin = if gen_path.exists() {
                gen_path
            } else {
                dir.join("generate-campaign.exe")
            };
            let _ = Command::new(bin).arg("--force").spawn();
        } else {
            // No generator — delete existing levels so fresh start
            delete_level_files(&dir);
        }

        // Exit the app so the player relaunches into a fresh game
        std::process::exit(0);
    }
}

/// Delete bot game progress files in the given directory.
fn delete_progress_files(dir: &std::path::Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".progress.json")
                || name_str == "stats.json"
                || name_str == "stats.jsonl"
            {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}

/// Delete campaign level files in the given directory.
fn delete_level_files(dir: &std::path::Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".level.json") {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}
