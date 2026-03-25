// SPDX-License-Identifier: GPL-3.0-or-later
//! Chapter title overlay and smooth background color transitions for player mode.

use bevy::prelude::*;
use crate::constants::*;
use crate::ui_helpers::gf;
use crate::i18n::Translations;

pub const CHAPTER_KEYS: &[&str] = &[
    "ch_turns", "ch_turn_tiles", "ch_arrows", "ch_arrow_tiles",
    "ch_teleports", "ch_teleport_tiles", "ch_bounce", "ch_bounce_tiles",
    "ch_painters", "ch_doors_switches", "ch_color_switches", "ch_color_switch_tiles",
    "ch_grand_mastery",
];

/// Combined resource: background fade target + current chapter index.
#[derive(Resource)]
pub struct ChapterState {
    pub bg_target: Color,
    pub current: usize,
}

#[derive(Component)]
pub struct ChapterTitleOverlay {
    pub timer: f32,
    pub phase: u8, // 0=fadein 1=hold 2=fadeout
}

pub fn chapter_index(level_idx: usize) -> usize {
    // Chapters 0-11: 11 levels each (introduces one new tile type per chapter).
    // Chapter 12: remaining 17 levels — "Grand Mastery", all tile types, longest chapter.
    if level_idx < 132 { level_idx / 11 } else { 12 }
}

/// Set the background color target and spawn a chapter title if the chapter changed.
pub fn set_chapter(idx: usize, commands: &mut Commands, font: &Handle<Font>,
    state: &mut ChapterState, t: &Translations,
) {
    let ci = chapter_index(idx);
    let c = CHAPTER_COLORS[ci.min(12)];
    state.bg_target = Color::srgb(c.0, c.1, c.2);
    if ci != state.current { state.current = ci; spawn_chapter_title(commands, ci, font, t); }
}

fn spawn_chapter_title(commands: &mut Commands, ch: usize, font: &Handle<Font>, t: &Translations) {
    let ci = ch.min(12);
    let prefix = t.ui_or("chapter_prefix", "Chapter");
    let name = t.ui_or(CHAPTER_KEYS[ci], CHAPTER_NAMES[ci]);
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
            height: Val::Percent(100.0), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0), ..default() },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)), GlobalZIndex(200),
        ChapterTitleOverlay { timer: 0.0, phase: 0 },
    )).with_children(|p| {
        p.spawn((Text::new(format!("{} {}", prefix, ch + 1)), gf(CHAPTER_NUM_FONT, font),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0))));
        p.spawn((Text::new(name.to_string()), gf(CHAPTER_NAME_FONT, font),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0))));
    });
}

/// Smoothly lerp ClearColor toward bg_target each frame.
pub fn animate_bg_color(mut cc: ResMut<ClearColor>, state: Res<ChapterState>, time: Res<Time>) {
    let c = cc.0.to_srgba();
    let t = state.bg_target.to_srgba();
    let s = (BG_FADE_SPEED * time.delta_secs()).clamp(0.0, 1.0);
    cc.0 = Color::srgb(c.red + (t.red - c.red) * s, c.green + (t.green - c.green) * s,
        c.blue + (t.blue - c.blue) * s);
}

/// Animate chapter title overlay: fade in, hold, fade out, despawn.
pub fn animate_chapter_title(
    mut commands: Commands, time: Res<Time>,
    mut q: Query<(Entity, &mut ChapterTitleOverlay, &mut BackgroundColor, &Children)>,
    mut tc: Query<&mut TextColor>,
) {
    for (ent, mut ov, mut bg, children) in &mut q {
        ov.timer += time.delta_secs();
        let alpha = match ov.phase {
            0 => {
                let a = (ov.timer / CHAPTER_TITLE_FADE).clamp(0.0, 1.0);
                if ov.timer >= CHAPTER_TITLE_FADE { ov.phase = 1; ov.timer = 0.0; }
                a
            }
            1 => {
                if ov.timer >= CHAPTER_TITLE_HOLD { ov.phase = 2; ov.timer = 0.0; }
                1.0
            }
            _ => {
                let a = 1.0 - (ov.timer / CHAPTER_TITLE_FADE).clamp(0.0, 1.0);
                if ov.timer >= CHAPTER_TITLE_FADE { commands.entity(ent).despawn(); continue; }
                a
            }
        };
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha * CHAPTER_TITLE_BG_ALPHA);
        for child in children.iter() {
            if let Ok(mut c) = tc.get_mut(child) { c.0 = Color::srgba(1.0, 1.0, 1.0, alpha); }
        }
    }
}
