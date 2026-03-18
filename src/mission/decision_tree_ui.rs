// SPDX-License-Identifier: GPL-3.0-or-later

//! Decision tree UI layout — spawns the "Journey Map" overlay showing the
//! player's decisions as a scrollable, parallax-layered chapter view.

use bevy::prelude::*;
use rand::Rng;
use crate::save_state::GameState;
use super::decision_tree::*;

/// Spawn the full decision tree overlay.
pub fn spawn_decision_tree_overlay(
    commands: &mut Commands,
    font: &Handle<Font>,
    gs: &GameState,
) {
    let chapters = build_decision_tree(gs);
    let bot_level = gs.bot_level;
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            DT_OVERLAY_BG.0, DT_OVERLAY_BG.1, DT_OVERLAY_BG.2, DT_OVERLAY_BG.3,
        )),
        GlobalZIndex(46), DecisionTreeOverlay, Button,
    )).with_children(|overlay| {
        spawn_parallax_stars(overlay);
        overlay.spawn((
            Node {
                max_width: Val::Px(DT_PANEL_MAX_W), width: Val::Percent(92.0),
                max_height: Val::Percent(DT_PANEL_MAX_H_PCT),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(DT_PANEL_PAD)),
                row_gap: Val::Px(14.0),
                border_radius: BorderRadius::all(Val::Px(DT_PANEL_CORNER)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(
                DT_PANEL_BG.0, DT_PANEL_BG.1, DT_PANEL_BG.2, DT_PANEL_BG.3,
            )),
            BoxShadow::new(
                Color::srgba(DT_GLOW_COLOR.0, DT_GLOW_COLOR.1, DT_GLOW_COLOR.2, DT_GLOW_COLOR.3),
                Val::ZERO, Val::ZERO, Val::Px(DT_GLOW_SPREAD), Val::Px(DT_GLOW_BLUR),
            ),
            DecisionTreeGlow, Interaction::None,
        )).with_children(|panel| {
            spawn_title_row(panel, font, bot_level);
            panel.spawn(Node {
                flex_direction: FlexDirection::Column, row_gap: Val::Px(20.0),
                width: Val::Percent(100.0), flex_grow: 1.0,
                overflow: Overflow::scroll_y(),
                padding: UiRect::right(Val::Px(8.0)),
                ..default()
            }).with_children(|scroll| {
                if chapters.is_empty() {
                    spawn_empty_state(scroll, font);
                } else {
                    for chapter in &chapters { spawn_chapter(scroll, font, chapter); }
                }
            });
            panel.spawn((
                Text::new("Press ESC or click outside to close"),
                TextFont { font: font.clone(), font_size: DT_HINT_FONT, ..default() },
                TextColor(Color::srgba(
                    DT_HINT_COLOR.0, DT_HINT_COLOR.1, DT_HINT_COLOR.2, DT_HINT_COLOR.3,
                )),
            ));
        });
    });
}

fn spawn_title_row(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, bot_level: u32) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center, width: Val::Percent(100.0),
        ..default()
    }).with_children(|row| {
        row.spawn((
            Text::new("JOURNEY MAP"),
            TextFont { font: font.clone(), font_size: DT_TITLE_FONT, ..default() },
            TextColor(Color::srgb(DT_TITLE_COLOR.0, DT_TITLE_COLOR.1, DT_TITLE_COLOR.2)),
        ));
        row.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End, row_gap: Val::Px(2.0),
            ..default()
        }).with_children(|right| {
            right.spawn((
                Text::new(format!("Bot Level {} / 149", bot_level)),
                TextFont { font: font.clone(), font_size: DT_BOT_LEVEL_FONT, ..default() },
                TextColor(Color::srgba(
                    DT_BOT_LEVEL_COLOR.0, DT_BOT_LEVEL_COLOR.1,
                    DT_BOT_LEVEL_COLOR.2, DT_BOT_LEVEL_COLOR.3,
                )),
            ));
            let pct = (bot_level as f32 / 149.0 * 100.0).clamp(0.0, 100.0);
            right.spawn((
                Node {
                    width: Val::Px(120.0), height: Val::Px(3.0),
                    border_radius: BorderRadius::all(Val::Px(1.5)), ..default()
                },
                BackgroundColor(Color::srgba(0.15, 0.16, 0.22, 0.8)),
            )).with_children(|bar| {
                bar.spawn((
                    Node {
                        width: Val::Percent(pct), height: Val::Percent(100.0),
                        border_radius: BorderRadius::all(Val::Px(1.5)), ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.6, 0.9, 0.7)),
                ));
            });
        });
    });
    parent.spawn((
        Node { width: Val::Percent(100.0), height: Val::Px(1.0), ..default() },
        BackgroundColor(Color::srgba(0.3, 0.35, 0.5, 0.3)),
    ));
}

fn spawn_empty_state(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center, justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(40.0)), row_gap: Val::Px(12.0),
        ..default()
    }).with_children(|center| {
        center.spawn((
            Text::new("Your journey map is empty"),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(Color::srgba(0.6, 0.65, 0.75, 0.8)),
        ));
        center.spawn((
            Text::new("Play the game to shape your story.\nEvery decision you make will appear here."),
            TextFont { font: font.clone(), font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.5, 0.55, 0.65, 0.6)),
        ));
    });
}

fn spawn_chapter(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>, chapter: &DecisionChapter,
) {
    let arc = chapter.arc_color;
    parent.spawn(Node {
        flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0),
        width: Val::Percent(100.0), ..default()
    }).with_children(|col| {
        col.spawn(Node {
            flex_direction: FlexDirection::Row, align_items: AlignItems::Center,
            column_gap: Val::Px(10.0), ..default()
        }).with_children(|header| {
            header.spawn((
                Node {
                    width: Val::Px(8.0), height: Val::Px(8.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)), ..default()
                },
                BackgroundColor(Color::srgba(arc.0, arc.1, arc.2, 0.8)),
            ));
            header.spawn((
                Text::new(chapter.name.to_uppercase()),
                TextFont { font: font.clone(), font_size: DT_CHAPTER_FONT, ..default() },
                TextColor(Color::srgb(
                    arc.0 * 0.9 + 0.1, arc.1 * 0.9 + 0.1, arc.2 * 0.9 + 0.1,
                )),
            ));
        });
        for node in &chapter.nodes { spawn_decision_node(col, font, node, arc); }
    });
}

fn spawn_decision_node(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    node: &DecisionNode, arc_color: (f32, f32, f32),
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row, column_gap: Val::Px(6.0),
        width: Val::Percent(100.0), padding: UiRect::left(Val::Px(18.0)),
        ..default()
    }).with_children(|row| {
        // Branch line + dot
        row.spawn(Node {
            flex_direction: FlexDirection::Column, align_items: AlignItems::Center,
            width: Val::Px(16.0), ..default()
        }).with_children(|branch| {
            branch.spawn((
                Node { width: Val::Px(DT_BRANCH_CHOSEN_WIDTH), height: Val::Px(8.0), ..default() },
                BackgroundColor(Color::srgba(
                    DT_BRANCH_CHOSEN_COLOR.0, DT_BRANCH_CHOSEN_COLOR.1,
                    DT_BRANCH_CHOSEN_COLOR.2, DT_BRANCH_CHOSEN_COLOR.3,
                )),
            ));
            branch.spawn((
                Node {
                    width: Val::Px(10.0), height: Val::Px(10.0),
                    border_radius: BorderRadius::all(Val::Px(5.0)), ..default()
                },
                BackgroundColor(Color::srgba(arc_color.0, arc_color.1, arc_color.2, 0.9)),
                BoxShadow::new(
                    Color::srgba(arc_color.0, arc_color.1, arc_color.2, 0.4),
                    Val::ZERO, Val::ZERO, Val::Px(2.0), Val::Px(6.0),
                ),
            ));
            branch.spawn((
                Node { width: Val::Px(DT_BRANCH_WIDTH), height: Val::Px(8.0), ..default() },
                BackgroundColor(Color::srgba(
                    DT_BRANCH_COLOR.0, DT_BRANCH_COLOR.1,
                    DT_BRANCH_COLOR.2, DT_BRANCH_COLOR.3,
                )),
            ));
        });
        // Node content column
        row.spawn(Node {
            flex_direction: FlexDirection::Column, row_gap: Val::Px(4.0),
            flex_grow: 1.0, ..default()
        }).with_children(|content| {
            spawn_chosen_node(content, font, node, arc_color);
            if !node.alternatives.is_empty() {
                spawn_alternatives(content, font, &node.alternatives);
            }
        });
    });
}

fn spawn_chosen_node(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    node: &DecisionNode, arc_color: (f32, f32, f32),
) {
    parent.spawn((
        Button,
        DecisionNodeMarker {
            key: node.key.clone(), description: node.label.clone(),
            chosen: true, group: node.character.clone(),
        },
        Node {
            padding: UiRect::axes(Val::Px(DT_NODE_PAD_X), Val::Px(DT_NODE_PAD_Y)),
            border_radius: BorderRadius::all(Val::Px(DT_NODE_CORNER)),
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center, column_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::srgba(
            DT_NODE_BG_CHOSEN.0, DT_NODE_BG_CHOSEN.1,
            DT_NODE_BG_CHOSEN.2, DT_NODE_BG_CHOSEN.3,
        )),
        BorderColor::all(Color::srgba(
            DT_BRANCH_CHOSEN_COLOR.0, DT_BRANCH_CHOSEN_COLOR.1,
            DT_BRANCH_CHOSEN_COLOR.2, 0.4,
        )),
    )).with_children(|chosen_node| {
        chosen_node.spawn((
            Node {
                width: Val::Px(6.0), height: Val::Px(6.0),
                border_radius: BorderRadius::all(Val::Px(3.0)), ..default()
            },
            BackgroundColor(Color::srgba(arc_color.0, arc_color.1, arc_color.2, 0.9)),
        ));
        chosen_node.spawn((
            Text::new(&node.label),
            TextFont { font: font.clone(), font_size: DT_NODE_FONT, ..default() },
            TextColor(Color::srgb(
                DT_NODE_CHOSEN_COLOR.0, DT_NODE_CHOSEN_COLOR.1, DT_NODE_CHOSEN_COLOR.2,
            )),
        ));
    });
}

fn spawn_alternatives(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>, alternatives: &[DecisionAlt],
) {
    let show = if alternatives.len() > 3 { &alternatives[..3] } else { alternatives };
    for alt in show {
        parent.spawn((
            Node {
                padding: UiRect::axes(Val::Px(DT_NODE_PAD_X), Val::Px(5.0)),
                border_radius: BorderRadius::all(Val::Px(DT_NODE_CORNER)),
                margin: UiRect::left(Val::Px(14.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center, column_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(
                DT_NODE_BG_UNCHOSEN.0, DT_NODE_BG_UNCHOSEN.1,
                DT_NODE_BG_UNCHOSEN.2, DT_NODE_BG_UNCHOSEN.3,
            )),
        )).with_children(|alt_node| {
            alt_node.spawn((
                Node { width: Val::Px(DT_BRANCH_WIDTH), height: Val::Px(12.0), ..default() },
                BackgroundColor(Color::srgba(
                    DT_BRANCH_COLOR.0, DT_BRANCH_COLOR.1, DT_BRANCH_COLOR.2, 0.3,
                )),
            ));
            let display = format!("??? {}", shorten_alt_hint(&alt.label));
            alt_node.spawn((
                Text::new(display),
                TextFont { font: font.clone(), font_size: DT_NODE_FONT - 1.0, ..default() },
                TextColor(Color::srgba(
                    DT_NODE_UNCHOSEN_COLOR.0, DT_NODE_UNCHOSEN_COLOR.1,
                    DT_NODE_UNCHOSEN_COLOR.2, DT_NODE_UNCHOSEN_COLOR.3,
                )),
            ));
        });
    }
    if alternatives.len() > 3 {
        parent.spawn((
            Text::new(format!("  +{} more paths not taken", alternatives.len() - 3)),
            TextFont { font: font.clone(), font_size: DT_HINT_FONT, ..default() },
            TextColor(Color::srgba(
                DT_HINT_COLOR.0, DT_HINT_COLOR.1, DT_HINT_COLOR.2, DT_HINT_COLOR.3,
            )),
            Node { margin: UiRect::left(Val::Px(14.0)), ..default() },
        ));
    }
}

fn shorten_alt_hint(label: &str) -> String {
    let trimmed = label
        .strip_prefix("You chose to ")
        .or_else(|| label.strip_prefix("You "))
        .unwrap_or(label);
    let mut chars = trimmed.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

fn spawn_parallax_stars(parent: &mut ChildSpawnerCommands) {
    let mut rng = rand::thread_rng();
    parent.spawn(Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0), height: Val::Percent(100.0),
        ..default()
    }).with_children(|star_layer| {
        for _ in 0..DT_STAR_COUNT {
            let (x, y) = (rng.gen_range(5.0..95.0_f32), rng.gen_range(5.0..95.0_f32));
            let size = rng.gen_range(DT_STAR_MIN_SIZE..DT_STAR_MAX_SIZE);
            let alpha = rng.gen_range(0.1..0.3_f32);
            star_layer.spawn((
                ParallaxLayer { depth: DT_PARALLAX_BG, base_x: 0.0, base_y: 0.0 },
                ParallaxStar,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x), top: Val::Percent(y),
                    width: Val::Px(size), height: Val::Px(size),
                    border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(
                    DT_STAR_COLOR.0, DT_STAR_COLOR.1, DT_STAR_COLOR.2, alpha,
                )),
            ));
        }
    });
}
