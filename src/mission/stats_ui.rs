// SPDX-License-Identifier: GPL-3.0-or-later

//! Stats screen UI layout — spawns the "Your Story" overlay with two columns:
//! decisions on the left, game statistics and crew manifest on the right.

use bevy::prelude::*;
use crate::save_state::GameState;
use super::stats_screen::*;

/// Spawn the full-screen stats overlay.
pub fn spawn_stats_overlay(
    commands: &mut Commands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            STATS_OVERLAY_BG.0, STATS_OVERLAY_BG.1,
            STATS_OVERLAY_BG.2, STATS_OVERLAY_BG.3,
        )),
        GlobalZIndex(45),
        StatsOverlay,
        Button,
    )).with_children(|overlay| {
        // Central panel
        overlay.spawn((
            Node {
                max_width: Val::Px(STATS_MAX_WIDTH),
                width: Val::Percent(90.0),
                max_height: Val::Percent(STATS_MAX_HEIGHT_PCT),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(STATS_PADDING)),
                row_gap: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(STATS_PANEL_CORNER)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(
                STATS_PANEL_BG.0, STATS_PANEL_BG.1,
                STATS_PANEL_BG.2, STATS_PANEL_BG.3,
            )),
            BoxShadow::new(
                Color::srgba(
                    STATS_GLOW_COLOR.0, STATS_GLOW_COLOR.1,
                    STATS_GLOW_COLOR.2, STATS_GLOW_COLOR.3,
                ),
                Val::ZERO, Val::ZERO,
                Val::Px(STATS_GLOW_SPREAD), Val::Px(STATS_GLOW_BLUR),
            ),
            StatsPanelGlow,
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("YOUR STORY"),
                TextFont { font: font.clone(), font_size: STATS_TITLE_FONT, ..default() },
                TextColor(Color::srgb(
                    STATS_TITLE_COLOR.0, STATS_TITLE_COLOR.1, STATS_TITLE_COLOR.2,
                )),
            ));

            // Two-column layout (scrollable)
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(STATS_COLUMN_GAP),
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                overflow: Overflow::scroll_y(),
                ..default()
            }).with_children(|cols| {
                // Left column: Decisions
                spawn_decisions_column(cols, font, gs);
                // Right column: Statistics + Crew
                spawn_stats_column(cols, font, gs);
            });

            // Dismiss hint
            panel.spawn((
                Text::new("Press ESC or click outside to close"),
                TextFont { font: font.clone(), font_size: STATS_HINT_FONT, ..default() },
                TextColor(Color::srgba(
                    STATS_HINT_COLOR.0, STATS_HINT_COLOR.1,
                    STATS_HINT_COLOR.2, STATS_HINT_COLOR.3,
                )),
            ));
        });
    });
}

/// Left column: player decisions grouped by category.
fn spawn_decisions_column(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(STATS_ROW_GAP),
        width: Val::Percent(50.0),
        ..default()
    }).with_children(|col| {
        // Section header
        spawn_section_header(col, font, "DECISIONS");

        // Count meaningful decisions (those with descriptions)
        let described: Vec<_> = gs.decisions.iter()
            .filter_map(|d| decision_description(d).map(|desc| (d.as_str(), desc)))
            .collect();

        if described.is_empty() {
            col.spawn((
                Text::new("No decisions recorded yet.\nPlay the game to shape your story."),
                TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
                TextColor(Color::srgba(
                    STATS_DIM_COLOR.0, STATS_DIM_COLOR.1,
                    STATS_DIM_COLOR.2, STATS_DIM_COLOR.3,
                )),
            ));
            return;
        }

        // Group decisions by category
        let groups = group_decisions(&described);
        for (group_name, entries) in &groups {
            spawn_decision_group(col, font, group_name, entries);
        }

        // Total count
        col.spawn(Node { height: Val::Px(8.0), ..default() });
        col.spawn((
            Text::new(format!("Total decisions: {}", described.len())),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(Color::srgb(
                STATS_SECTION_COLOR.0, STATS_SECTION_COLOR.1, STATS_SECTION_COLOR.2,
            )),
        ));
    });
}

/// Group decisions by category, preserving order of first appearance.
fn group_decisions<'a>(
    described: &[(&'a str, &'static str)],
) -> Vec<(&'static str, Vec<&'static str>)> {
    let mut groups: Vec<(&'static str, Vec<&'static str>)> = Vec::new();
    for &(key, desc) in described {
        let group = decision_group(key);
        if let Some(g) = groups.iter_mut().find(|(name, _)| *name == group) {
            g.1.push(desc);
        } else {
            groups.push((group, vec![desc]));
        }
    }
    groups
}

/// Spawn a decision group: header + bulleted entries.
fn spawn_decision_group(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    group_name: &str,
    entries: &[&str],
) {
    // Group header
    parent.spawn(Node { height: Val::Px(4.0), ..default() });
    parent.spawn((
        Text::new(group_name.to_uppercase()),
        TextFont { font: font.clone(), font_size: STATS_HINT_FONT, ..default() },
        TextColor(Color::srgb(
            STATS_SECTION_COLOR.0, STATS_SECTION_COLOR.1, STATS_SECTION_COLOR.2,
        )),
    ));

    let color = Color::srgb(
        STATS_DECISION_COLOR.0, STATS_DECISION_COLOR.1, STATS_DECISION_COLOR.2,
    );
    for desc in entries {
        spawn_bullet_line(parent, font, desc, color);
    }
}

/// Known characters with their pod numbers for crew manifest display.
const CREW_CHARACTERS: &[(u32, &str)] = &[
    (4231,  "Dr. Amira Hassan"),
    (8744,  "Viktor Petrov"),
    (2891,  "Mei-Lin Chen"),
    (6100,  "Kwame & Kofi Asante"),
    (0,     "Anna"),
    (9415,  "Yuki Tanabe"),
    (5776,  "Marcus Cole"),
    (10302, "Dr. Aisha Okonkwo"),
    (7891,  "Tomas Herrera"),
    (3445,  "Carlos Mendoza"),
    (1208,  "Sister Magdalena Santos"),
    (10150, "General Fatou Diallo"),
    (6891,  "Priya Nair"),
    (1001,  "James Whitfield"),
    (5200,  "Kira Volkov"),
    (7500,  "Hassan al-Rashidi"),
];

/// Right column: game statistics and crew manifest.
fn spawn_stats_column(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(STATS_ROW_GAP),
        width: Val::Percent(50.0),
        ..default()
    }).with_children(|col| {
        // --- Game Statistics ---
        spawn_section_header(col, font, "SHIP LOG");
        spawn_stat_line(col, font, "Bot levels completed",
            &format!("{} / 149", gs.bot_level));
        spawn_stat_line(col, font, "Gathering runs", &gs.gathering_runs.to_string());
        spawn_stat_line(col, font, "Crystals gathered",
            &format_number(gs.total_crystals_gathered));
        spawn_stat_line(col, font, "Orben games played",
            &gs.orben_games_played.to_string());
        spawn_stat_line(col, font, "Days survived", &format_number(gs.day as u64));
        spawn_stat_line(col, font, "Distance traveled",
            &format!("{:.1} AU", gs.distance_au));
        spawn_stat_line(col, font, "Crew remaining",
            &format_number(gs.crew_count as u64));
        spawn_stat_line(col, font, "World seed",
            &format!("{:016X}", gs.world_seed));
        spawn_stat_line(col, font, "Playthrough",
            &format!("#{}", gs.playthrough_count + 1));

        col.spawn(Node { height: Val::Px(12.0), ..default() });

        // --- Crew Manifest Summary ---
        spawn_section_header(col, font, "CREW MANIFEST");
        let discovered = &gs.discovered_crew;
        let total_chars = CREW_CHARACTERS.len();

        col.spawn((
            Text::new(format!("{} of {} characters discovered",
                discovered.len(), total_chars)),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(Color::srgb(
                STATS_BODY_COLOR.0, STATS_BODY_COLOR.1, STATS_BODY_COLOR.2,
            )),
        ));

        // List discovered character names
        if !discovered.is_empty() {
            spawn_discovered_names(col, font, gs);
        }

        // Story scenes seen
        if !gs.story_seen.is_empty() {
            col.spawn(Node { height: Val::Px(8.0), ..default() });
            spawn_stat_line(col, font, "Story scenes witnessed",
                &gs.story_seen.len().to_string());
        }

        // Reached New Earth badge
        if gs.reached_new_earth {
            col.spawn(Node { height: Val::Px(8.0), ..default() });
            col.spawn((
                Text::new("You reached New Earth"),
                TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
                TextColor(Color::srgb(0.85, 0.9, 0.5)),
            ));
        }
    });
}

/// Spawn a section header with underline-style emphasis.
fn spawn_section_header(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    title: &str,
) {
    parent.spawn((
        Text::new(title),
        TextFont { font: font.clone(), font_size: STATS_SECTION_FONT, ..default() },
        TextColor(Color::srgb(
            STATS_SECTION_COLOR.0, STATS_SECTION_COLOR.1, STATS_SECTION_COLOR.2,
        )),
    ));
    // Thin separator line
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.4, 0.45, 0.6, 0.3)),
    ));
}

/// Spawn a single stat line: "Label ........ Value".
fn spawn_stat_line(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    value: &str,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        padding: UiRect::horizontal(Val::Px(4.0)),
        ..default()
    }).with_children(|row| {
        row.spawn((
            Text::new(label),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(Color::srgb(
                STATS_BODY_COLOR.0, STATS_BODY_COLOR.1, STATS_BODY_COLOR.2,
            )),
        ));
        row.spawn((
            Text::new(value),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(Color::srgb(
                STATS_HIGHLIGHT_COLOR.0, STATS_HIGHLIGHT_COLOR.1,
                STATS_HIGHLIGHT_COLOR.2,
            )),
        ));
    });
}

/// List discovered character names from the crew character registry.
fn spawn_discovered_names(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    let names: Vec<&str> = CREW_CHARACTERS.iter()
        .filter(|&&(pod, _)| pod > 0 && gs.discovered_crew.contains(&pod))
        .map(|&(_, name)| name)
        .collect();
    if names.is_empty() { return; }
    let color = Color::srgb(
        STATS_HIGHLIGHT_COLOR.0, STATS_HIGHLIGHT_COLOR.1, STATS_HIGHLIGHT_COLOR.2,
    );
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(2.0),
        padding: UiRect::left(Val::Px(8.0)),
        ..default()
    }).with_children(|list| {
        for name in &names {
            spawn_bullet_line(list, font, name, color);
        }
    });
}

/// Spawn a bulleted line: "* text" with configurable text color.
fn spawn_bullet_line(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    text: &str,
    text_color: Color,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(6.0),
        padding: UiRect::left(Val::Px(8.0)),
        ..default()
    }).with_children(|row| {
        row.spawn((
            Text::new("\u{2022}"),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(Color::srgba(
                STATS_DIM_COLOR.0, STATS_DIM_COLOR.1,
                STATS_DIM_COLOR.2, STATS_DIM_COLOR.3,
            )),
        ));
        row.spawn((
            Text::new(text),
            TextFont { font: font.clone(), font_size: STATS_BODY_FONT, ..default() },
            TextColor(text_color),
        ));
    });
}

/// Format a number with comma separators for readability.
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(ch);
    }
    result.chars().rev().collect()
}
