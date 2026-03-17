// SPDX-License-Identifier: GPL-3.0-or-later

//! Multiple ending narratives for protocol: play.
//! Ending is determined by crew survival percentage and player decisions.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use crate::save_state::GameState;

/// The six possible endings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    Golden,
    Bittersweet,
    TheCost,
    TheMachine,
    LastHope,
    Drift,
}

impl Ending {
    pub fn title(&self) -> &'static str {
        match self {
            Ending::Golden => "The Golden Dawn",
            Ending::Bittersweet => "The Bittersweet Arrival",
            Ending::TheCost => "The Cost",
            Ending::TheMachine => "The Machine",
            Ending::LastHope => "Last Hope",
            Ending::Drift => "Drift",
        }
    }

    /// Glow color for Anna's portrait during this ending.
    pub fn glow_color(&self) -> (f32, f32, f32) {
        match self {
            Ending::Golden => (0.95, 0.85, 0.5),       // warm gold
            Ending::Bittersweet => (0.3, 0.5, 0.9),    // blue
            Ending::TheCost => (0.7, 0.6, 0.5),        // muted amber
            Ending::TheMachine => (0.4, 0.7, 0.9),     // cold cyan
            Ending::LastHope => (0.6, 0.5, 0.7),       // purple-grey
            Ending::Drift => (0.8, 0.2, 0.2),          // red
        }
    }
}

/// Determine which ending the player gets.
pub fn determine_ending(state: &GameState) -> Ending {
    let crew_pct = state.crew_count as f32 / 14892.0;
    let augmented = state.decisions.iter().any(|d| d == "q6_augment");

    if crew_pct > 0.9 && !augmented {
        Ending::Golden
    } else if crew_pct > 0.7 && augmented {
        Ending::Bittersweet
    } else if crew_pct > 0.5 {
        Ending::TheCost
    } else if crew_pct > 0.2 && augmented {
        Ending::TheMachine
    } else if crew_pct > 0.1 {
        Ending::LastHope
    } else {
        Ending::Drift
    }
}

/// Get the narrative paragraphs for an ending.
pub fn ending_paragraphs(ending: Ending, crew_count: u32) -> Vec<String> {
    let lost = 14892_u32.saturating_sub(crew_count);
    match ending {
        Ending::Golden => vec![
            "New Earth.".to_string(),
            "Green hills, blue oceans. 14,000 people wake to sunrise.".to_string(),
            "Anna's voice, gentle: \"Welcome home.\"".to_string(),
            "You stand among them -- human, tired, alive.".to_string(),
            "The children run toward the light.".to_string(),
            "Someone asks your name. You tell them. They smile.".to_string(),
            "This is the beginning.".to_string(),
        ],
        Ending::Bittersweet => vec![
            "You found it. New Earth.".to_string(),
            "The crew wakes, blinking. They see you and hesitate.".to_string(),
            "Your eyes glow faintly -- nanorepair changed you more than Anna promised."
                .to_string(),
            "\"Who are you?\" a child asks.".to_string(),
            "\"I'm the one who brought you here,\" you say.".to_string(),
            "Anna whispers in your mind: \"We did it. Together.\"".to_string(),
            "You watch them walk into the sunlight.".to_string(),
            "You stay on the ship. It's home now.".to_string(),
        ],
        Ending::TheCost => vec![
            "New Earth.".to_string(),
            "But the celebrations are quiet.".to_string(),
            format!("{} people didn't make it.", lost),
            "Their cryopods failed while you struggled with resources.".to_string(),
            "Anna reads their names. Every single one. It takes hours.".to_string(),
            "\"Was it worth it?\" she asks.".to_string(),
            "You look at the survivors, building shelters, planting seeds.".to_string(),
            "\"Ask them,\" you say.".to_string(),
        ],
        Ending::TheMachine => vec![
            "You barely remember what hunger felt like. Or cold.".to_string(),
            "The augmentations took those away, along with... other things.".to_string(),
            "The crew looks at you with gratitude and fear.".to_string(),
            "\"Thank you,\" they say, keeping distance.".to_string(),
            "Anna understands: \"You gave up your humanity to save theirs.\""
                .to_string(),
            "The colony thrives.".to_string(),
            "You watch from the ship's bridge, forever.".to_string(),
        ],
        Ending::LastHope => vec![
            "A small group. Too small, maybe. But alive.".to_string(),
            format!(
                "\"We need at least 500 for genetic diversity,\" Anna says quietly. You have {}.",
                crew_count
            ),
            "The math is cruel. But they're determined.".to_string(),
            "They name the settlement \"Second Chance.\"".to_string(),
            "You help them build.".to_string(),
            "Some nights, you stare at the stars and wonder about the ones you lost."
                .to_string(),
        ],
        Ending::Drift => vec![
            "The ship is quiet. Too quiet.".to_string(),
            "Anna's voice fades in and out.".to_string(),
            "\"I'm sorry,\" she says.".to_string(),
            "\"I'm sorry I woke you. I'm sorry I couldn't--\"".to_string(),
            "Static.".to_string(),
            format!(
                "The remaining {} crew sleep on, drifting.",
                crew_count
            ),
            "Maybe someone will find them.".to_string(),
            "Maybe the universe is kinder than you think.".to_string(),
            "Maybe.".to_string(),
        ],
    }
}

// === Ending Screen State ===

/// Resource tracking the ending sequence state.
#[derive(Resource)]
pub struct EndingState {
    pub ending: Ending,
    pub paragraphs: Vec<String>,
    /// Index of next paragraph to reveal.
    pub next_para: usize,
    /// Timer for paragraph pacing.
    pub para_timer: f32,
    /// Whether all paragraphs have been shown.
    pub narrative_done: bool,
    /// Whether stats are shown.
    pub stats_shown: bool,
    /// Overall fade-in alpha.
    pub fade_alpha: f32,
}

/// Marker for ending screen root.
#[derive(Component)]
pub struct EndingScreen;

/// Marker for ending title text.
#[derive(Component)]
pub struct EndingTitle;

/// Marker for a narrative paragraph.
#[derive(Component)]
pub struct EndingParagraph(pub usize);

/// Marker for the stats container (shown after narrative).
#[derive(Component)]
pub struct EndingStats;

/// Marker for the "New Journey" button.
#[derive(Component)]
pub struct NewJourneyBtn;

/// Marker for Anna's glow circle in the ending screen.
#[derive(Component)]
pub struct EndingAnnaGlow;

/// Marker for the "Final Voyage" button on Mission Control.
#[derive(Component)]
pub struct FinalVoyageBtn;

// === Constants ===
const NARRATIVE_FONT: f32 = 18.0;
const TITLE_FONT: f32 = 14.0;
const STAT_FONT: f32 = 15.0;
const STAT_LABEL_COLOR: (f32, f32, f32) = (0.55, 0.6, 0.7);
const STAT_VALUE_COLOR: (f32, f32, f32) = (0.85, 0.88, 0.95);
const ENDING_MAX_WIDTH: f32 = 650.0;

/// Spawn the full-screen ending sequence.
pub fn spawn_ending_screen(
    commands: &mut Commands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    let ending = determine_ending(gs);
    let paragraphs = ending_paragraphs(ending, gs.crew_count);
    let glow = ending.glow_color();

    commands.insert_resource(EndingState {
        ending,
        paragraphs: paragraphs.clone(),
        next_para: 0,
        para_timer: 1.0, // initial delay before first paragraph
        narrative_done: false,
        stats_shown: false,
        fade_alpha: 0.0,
    });

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.03, 0.06, 0.0)),
        GlobalZIndex(40),
        EndingScreen,
    )).with_children(|overlay| {
        overlay.spawn(Node {
            max_width: Val::Px(ENDING_MAX_WIDTH),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        }).with_children(|col| {
            // Anna glow circle
            col.spawn((
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    border_radius: BorderRadius::all(Val::Px(25.0)),
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(glow.0, glow.1, glow.2)),
                BoxShadow::new(
                    Color::srgba(glow.0, glow.1, glow.2, 0.6),
                    Val::ZERO, Val::ZERO,
                    Val::Px(8.0), Val::Px(20.0),
                ),
                EndingAnnaGlow,
            ));

            // Ending title
            col.spawn((
                Text::new(format!("Ending: {}", ending.title())),
                TextFont { font: font.clone(), font_size: TITLE_FONT, ..default() },
                TextColor(Color::srgba(0.5, 0.55, 0.65, 0.0)),
                EndingTitle,
            ));

            // Paragraph placeholders (hidden until revealed)
            for (i, para) in paragraphs.iter().enumerate() {
                col.spawn((
                    Text::new(para.as_str()),
                    TextFont { font: font.clone(), font_size: NARRATIVE_FONT, ..default() },
                    TextColor(Color::srgba(0.88, 0.86, 0.82, 0.0)),
                    EndingParagraph(i),
                ));
            }

            // Stats container (hidden initially)
            col.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    margin: UiRect::top(Val::Px(24.0)),
                    ..default()
                },
                EndingStats,
            )).with_children(|stats| {
                let decisions_count = gs.decisions.len();
                spawn_stat_row(stats, font, "Crew survived",
                    &format!("{} / 14,892", fmt_num(gs.crew_count)));
                spawn_stat_row(stats, font, "Days traveled",
                    &format!("{}", gs.day));
                spawn_stat_row(stats, font, "Crystals gathered",
                    &format!("{}", gs.total_crystals_gathered));
                spawn_stat_row(stats, font, "Decisions made",
                    &format!("{}", decisions_count));
            });

            // New Journey button (hidden initially)
            col.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.12, 0.14, 0.20, 0.0)),
                BorderColor::all(Color::srgba(0.3, 0.35, 0.5, 0.0)),
                NewJourneyBtn,
            )).with_child((
                Text::new("New Journey"),
                TextFont { font: font.clone(), font_size: 16.0, ..default() },
                TextColor(Color::srgba(0.85, 0.88, 0.95, 0.0)),
            ));
        });
    });
}

fn spawn_stat_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    value: &str,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.0),
        ..default()
    }).with_children(|row| {
        row.spawn((
            Text::new(format!("{}:", label)),
            TextFont { font: font.clone(), font_size: STAT_FONT, ..default() },
            TextColor(Color::srgba(
                STAT_LABEL_COLOR.0, STAT_LABEL_COLOR.1, STAT_LABEL_COLOR.2, 0.0,
            )),
        ));
        row.spawn((
            Text::new(value),
            TextFont { font: font.clone(), font_size: STAT_FONT, ..default() },
            TextColor(Color::srgba(
                STAT_VALUE_COLOR.0, STAT_VALUE_COLOR.1, STAT_VALUE_COLOR.2, 0.0,
            )),
        ));
    });
}

/// Format number with thousands separator.
fn fmt_num(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}
