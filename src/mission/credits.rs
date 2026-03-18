// SPDX-License-Identifier: GPL-3.0-or-later

//! Credits screen — cinematic scrolling credits accessible from main menu.
//! Kojima-style: emotional, acknowledging everyone and everything.

use bevy::prelude::*;
use super::constants::*;
use super::credits_content;
use super::types::MissionFont;

// === Components ===

#[derive(Component)]
pub struct CreditsRoot;

#[derive(Component)]
pub struct CreditsScroller;

#[derive(Component)]
pub struct CreditsFadeOverlay;

#[derive(Component)]
pub struct CreditsSkipHint;

// === Resources ===

#[derive(Resource)]
pub struct CreditsState {
    pub elapsed: f32,
    pub scroll_offset: f32,
    pub phase: CreditsPhase,
    pub content_height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreditsPhase {
    FadingIn,
    Scrolling,
    FadingOut,
    Done,
}

impl Default for CreditsState {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            scroll_offset: 0.0,
            phase: CreditsPhase::FadingIn,
            content_height: 2000.0,
        }
    }
}

// === Spawn ===

pub fn spawn_credits(commands: &mut Commands, font: &Handle<Font>) {
    commands.insert_resource(CreditsState::default());

    // Root overlay — covers full screen, absorbs clicks
    commands.spawn((
        CreditsRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(80),
    )).with_children(|root| {
        // Scrolling column
        root.spawn((
            CreditsScroller,
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(800.0), // starts below viewport
                width: Val::Px(600.0),
                padding: UiRect::vertical(Val::Px(200.0)),
                ..default()
            },
        )).with_children(|col| {
            spawn_credits_content(col, font);
        });
    });

    // Fade overlay
    commands.spawn((
        CreditsFadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        GlobalZIndex(81),
    ));

    // Skip hint — bottom right
    commands.spawn((
        CreditsSkipHint,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(12.0),
            ..default()
        },
        GlobalZIndex(82),
    )).with_child((
        Text::new("ESC to skip"),
        TextFont {
            font: font.clone(),
            font_size: CREDITS_SKIP_FONT,
            ..default()
        },
        TextColor(Color::srgba(
            CREDITS_SKIP_COLOR.0, CREDITS_SKIP_COLOR.1,
            CREDITS_SKIP_COLOR.2, CREDITS_SKIP_COLOR.3,
        )),
    ));
}

fn spawn_credits_content(col: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    let sections = credits_content::credits_sections();

    for section in &sections {
        // Section gap
        col.spawn(Node {
            height: Val::Px(CREDITS_SECTION_GAP),
            ..default()
        });

        // Heading
        if !section.heading.is_empty() {
            col.spawn((
                Text::new(section.heading),
                TextFont {
                    font: font.clone(),
                    font_size: CREDITS_HEADING_FONT,
                    ..default()
                },
                TextColor(Color::srgb(
                    CREDITS_HEADING_COLOR.0,
                    CREDITS_HEADING_COLOR.1,
                    CREDITS_HEADING_COLOR.2,
                )),
            ));
            col.spawn(Node {
                height: Val::Px(10.0),
                ..default()
            });
        }

        // Body lines
        for line in section.lines {
            if line.is_empty() {
                col.spawn(Node {
                    height: Val::Px(CREDITS_LINE_GAP * 2.0),
                    ..default()
                });
                continue;
            }
            // First section = title block (special styling)
            let (size, color) = if section.heading.is_empty() && *line == "protocol: play" {
                (CREDITS_TITLE_FONT, Color::srgb(
                    CREDITS_TITLE_COLOR.0, CREDITS_TITLE_COLOR.1, CREDITS_TITLE_COLOR.2,
                ))
            } else if section.heading.is_empty() {
                (CREDITS_TAGLINE_FONT, Color::srgba(
                    CREDITS_TAGLINE_COLOR.0, CREDITS_TAGLINE_COLOR.1,
                    CREDITS_TAGLINE_COLOR.2, CREDITS_TAGLINE_COLOR.3,
                ))
            } else {
                (CREDITS_BODY_FONT, Color::srgb(
                    CREDITS_BODY_COLOR.0, CREDITS_BODY_COLOR.1, CREDITS_BODY_COLOR.2,
                ))
            };

            col.spawn((
                Text::new(*line),
                TextFont { font: font.clone(), font_size: size, ..default() },
                TextColor(color),
                Node {
                    margin: UiRect::vertical(Val::Px(CREDITS_LINE_GAP / 2.0)),
                    ..default()
                },
            ));
        }
    }

    // Characters section
    spawn_character_block(col, font);

    // Closing lines
    let closing = credits_content::closing_lines();
    col.spawn(Node {
        height: Val::Px(CREDITS_SECTION_GAP),
        ..default()
    });

    for line in &closing {
        if line.is_empty() {
            col.spawn(Node {
                height: Val::Px(CREDITS_LINE_GAP * 2.0),
                ..default()
            });
            continue;
        }
        let is_crew_line = line.contains("14,892");
        let is_quote = line.starts_with('\u{201c}') || line.starts_with("It's ");
        let is_attribution = line.starts_with('\u{2014}');

        let (size, color) = if is_crew_line {
            (CREDITS_HEADING_FONT, Color::srgb(
                CREDITS_FINAL_CREW_COLOR.0, CREDITS_FINAL_CREW_COLOR.1,
                CREDITS_FINAL_CREW_COLOR.2,
            ))
        } else if is_quote || is_attribution {
            (CREDITS_QUOTE_FONT, Color::srgba(
                CREDITS_QUOTE_COLOR.0, CREDITS_QUOTE_COLOR.1,
                CREDITS_QUOTE_COLOR.2, CREDITS_QUOTE_COLOR.3,
            ))
        } else {
            (CREDITS_BODY_FONT, Color::srgb(
                CREDITS_BODY_COLOR.0, CREDITS_BODY_COLOR.1, CREDITS_BODY_COLOR.2,
            ))
        };

        col.spawn((
            Text::new(*line),
            TextFont { font: font.clone(), font_size: size, ..default() },
            TextColor(color),
            Node {
                margin: UiRect::vertical(Val::Px(CREDITS_LINE_GAP / 2.0)),
                ..default()
            },
        ));
    }
}

fn spawn_character_block(col: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    let characters = credits_content::character_credits();
    col.spawn(Node {
        height: Val::Px(12.0),
        ..default()
    });

    for (name, desc) in &characters {
        // Name
        col.spawn((
            Text::new(*name),
            TextFont {
                font: font.clone(),
                font_size: CREDITS_BODY_FONT,
                ..default()
            },
            TextColor(Color::srgb(
                CREDITS_CHARACTER_NAME_COLOR.0,
                CREDITS_CHARACTER_NAME_COLOR.1,
                CREDITS_CHARACTER_NAME_COLOR.2,
            )),
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
        ));
        // Description
        col.spawn((
            Text::new(*desc),
            TextFont {
                font: font.clone(),
                font_size: CREDITS_SMALL_FONT,
                ..default()
            },
            TextColor(Color::srgba(
                CREDITS_CHARACTER_DESC_COLOR.0,
                CREDITS_CHARACTER_DESC_COLOR.1,
                CREDITS_CHARACTER_DESC_COLOR.2,
                CREDITS_CHARACTER_DESC_COLOR.3,
            )),
            Node {
                margin: UiRect::bottom(Val::Px(4.0)),
                ..default()
            },
        ));
    }
}
