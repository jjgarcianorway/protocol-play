// SPDX-License-Identifier: GPL-3.0-or-later

//! Decision tree data model, constants, and interactive systems.
//! Shows the player's journey as a connected node map with parallax depth.

use bevy::prelude::*;
use crate::save_state::GameState;
use super::stats_screen::{decision_description, decision_group};

// === Constants ===

// Overlay
pub const DT_OVERLAY_BG: (f32, f32, f32, f32) = (0.02, 0.03, 0.06, 0.95);
pub const DT_PANEL_BG: (f32, f32, f32, f32) = (0.05, 0.06, 0.10, 0.98);
pub const DT_PANEL_CORNER: f32 = 14.0;
pub const DT_PANEL_MAX_W: f32 = 1100.0;
pub const DT_PANEL_MAX_H_PCT: f32 = 90.0;
pub const DT_PANEL_PAD: f32 = 24.0;

// Title
pub const DT_TITLE_FONT: f32 = 22.0;
pub const DT_TITLE_COLOR: (f32, f32, f32) = (0.7, 0.8, 0.95);

// Chapter headers
pub const DT_CHAPTER_FONT: f32 = 16.0;
pub const DT_CHAPTER_COLOR: (f32, f32, f32) = (0.65, 0.72, 0.88);

// Nodes
pub const DT_NODE_FONT: f32 = 13.0;
pub const DT_NODE_CHOSEN_COLOR: (f32, f32, f32) = (0.85, 0.9, 1.0);
pub const DT_NODE_UNCHOSEN_COLOR: (f32, f32, f32, f32) = (0.4, 0.45, 0.55, 0.6);
pub const DT_NODE_BG_CHOSEN: (f32, f32, f32, f32) = (0.12, 0.14, 0.22, 0.9);
pub const DT_NODE_BG_UNCHOSEN: (f32, f32, f32, f32) = (0.08, 0.09, 0.13, 0.5);
pub const DT_NODE_CORNER: f32 = 8.0;
pub const DT_NODE_PAD_X: f32 = 14.0;
pub const DT_NODE_PAD_Y: f32 = 8.0;

// Branch indicator
pub const DT_BRANCH_COLOR: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.5);
pub const DT_BRANCH_CHOSEN_COLOR: (f32, f32, f32, f32) = (0.4, 0.65, 0.95, 0.8);
pub const DT_BRANCH_WIDTH: f32 = 3.0;
pub const DT_BRANCH_CHOSEN_WIDTH: f32 = 4.0;

// Character arc colors
pub const DT_ARC_AMIRA: (f32, f32, f32) = (0.25, 0.55, 0.85);   // water blue
pub const DT_ARC_VIKTOR: (f32, f32, f32) = (0.85, 0.65, 0.25);  // amber
pub const DT_ARC_MEILIN: (f32, f32, f32) = (0.35, 0.75, 0.35);  // green
pub const DT_ARC_TWINS: (f32, f32, f32) = (0.75, 0.55, 0.85);   // purple
pub const DT_ARC_ANNA: (f32, f32, f32) = (0.4, 0.7, 1.0);       // bright blue
pub const DT_ARC_FACTION: (f32, f32, f32) = (0.85, 0.45, 0.35);  // red-orange
pub const DT_ARC_GOVERN: (f32, f32, f32) = (0.9, 0.8, 0.3);     // gold
pub const DT_ARC_EARTH: (f32, f32, f32) = (0.5, 0.55, 0.65);    // grey
pub const DT_ARC_KEY: (f32, f32, f32) = (0.8, 0.85, 0.95);      // white-ish
pub const DT_ARC_FIRST_WORD: (f32, f32, f32) = (0.7, 0.8, 0.5); // lime

// Glow
pub const DT_GLOW_COLOR: (f32, f32, f32, f32) = (0.25, 0.45, 0.75, 0.1);
pub const DT_GLOW_BLUR: f32 = 22.0;
pub const DT_GLOW_SPREAD: f32 = 6.0;

// Parallax
pub const DT_PARALLAX_BG: f32 = 0.05;
pub const DT_PARALLAX_MID: f32 = 0.10;
pub const DT_PARALLAX_FG: f32 = 0.15;
pub const DT_PARALLAX_RANGE: f32 = 20.0;

// Hint
pub const DT_HINT_FONT: f32 = 11.0;
pub const DT_HINT_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.65, 0.55);

// Bot level indicator
pub const DT_BOT_LEVEL_FONT: f32 = 11.0;
pub const DT_BOT_LEVEL_COLOR: (f32, f32, f32, f32) = (0.45, 0.5, 0.6, 0.7);

// Star dots for parallax background
pub const DT_STAR_COUNT: usize = 40;
pub const DT_STAR_MIN_SIZE: f32 = 2.0;
pub const DT_STAR_MAX_SIZE: f32 = 4.0;
pub const DT_STAR_COLOR: (f32, f32, f32, f32) = (0.5, 0.55, 0.7, 0.2);

// Tooltip
pub const DT_TOOLTIP_FONT: f32 = 12.0;
pub const DT_TOOLTIP_BG: (f32, f32, f32, f32) = (0.1, 0.11, 0.16, 0.95);
pub const DT_TOOLTIP_COLOR: (f32, f32, f32) = (0.8, 0.85, 0.95);
pub const DT_TOOLTIP_CORNER: f32 = 6.0;
pub const DT_TOOLTIP_PAD: f32 = 10.0;

// === Components ===

/// Marker for the decision tree overlay root.
#[derive(Component)]
pub struct DecisionTreeOverlay;

/// Marker for the decision tree panel glow.
#[derive(Component)]
pub struct DecisionTreeGlow;

/// Parallax layer marker with depth multiplier.
#[derive(Component)]
pub struct ParallaxLayer {
    pub depth: f32,
    pub base_x: f32,
    pub base_y: f32,
}

/// A star dot in the parallax background.
#[derive(Component)]
pub struct ParallaxStar;

/// Tooltip text shown on hover.
#[derive(Component)]
pub struct DecisionTooltip;

/// Marker for a decision node (interactive).
#[derive(Component)]
pub struct DecisionNodeMarker {
    pub key: String,
    pub description: String,
    pub chosen: bool,
    pub group: String,
}

/// Hoverable node with full text for tooltip.
#[derive(Component)]
pub struct HoverableNode {
    pub hover_text: String,
}

// === Data model ===

/// A single decision point in the tree.
#[derive(Clone, Debug)]
pub struct DecisionNode {
    pub key: String,
    pub label: String,
    pub chosen: bool,
    pub character: String,
    pub alternatives: Vec<DecisionAlt>,
}

/// An alternative (unchosen) path at a decision point.
#[derive(Clone, Debug)]
pub struct DecisionAlt {
    pub key: String,
    pub label: String,
}

/// A chapter in the decision tree (grouped decisions).
#[derive(Clone, Debug)]
pub struct DecisionChapter {
    pub name: String,
    pub arc_color: (f32, f32, f32),
    pub nodes: Vec<DecisionNode>,
}

// === Decision groupings ===

/// All possible decisions for a given group, for showing unchosen alternatives.
fn all_decisions_for_group(group: &str) -> Vec<(&'static str, &'static str)> {
    let all_keys: Vec<&str> = match group {
        "Amira's Arc" => vec![
            "amira_build", "amira_delay", "amira_trust",
            "amira_override", "amira_sacrifice", "amira_protect",
        ],
        "Viktor's Arc" => vec![
            "viktor_redeemed", "viktor_punished", "viktor_forgiven",
            "viktor_exiled", "viktor_trust", "viktor_restrict",
        ],
        "Mei-Lin's Seeds" => vec![
            "seeds_protect", "seeds_share", "seeds_ration",
            "seeds_plant_now", "seeds_save",
        ],
        "The Twins" => vec![
            "twins_together", "twins_separate", "twins_lead", "twins_supervise",
        ],
        "Anna" => vec![
            "anna_trust", "anna_limit", "anna_confide",
            "anna_distance", "anna_upgrade", "anna_restrict_ai",
        ],
        "Faction Politics" => vec![
            "faction_founders", "faction_pioneers", "faction_keepers",
            "faction_unite", "faction_neutral", "faction_whitfield",
            "faction_volkov", "faction_rashidi",
        ],
        "Governance" => vec![
            "constitution_democracy", "constitution_council",
            "constitution_meritocracy", "constitution_consensus",
            "constitution_captain",
        ],
        "First Word" => vec![
            "first_word_hope", "first_word_survive", "first_word_together",
            "first_word_remember", "first_word_forward",
        ],
        "Earth's Fate" => vec![
            "collapse_climate", "collapse_resource", "collapse_pandemic",
            "collapse_nuclear", "collapse_ai", "collapse_political",
            "collapse_economic", "severity_mild", "severity_moderate",
            "severity_extreme",
        ],
        _ => vec![
            "revealed_truth", "kept_secret", "sacrificed_power",
            "preserved_power", "saved_stowaway", "reported_stowaway",
            "broadcast_sent", "broadcast_silence", "children_teach",
            "children_protect", "memorial_built", "orben_played",
        ],
    };
    all_keys.iter()
        .filter_map(|k| decision_description(k).map(|d| (*k, d)))
        .collect()
}

/// Get the arc color for a decision group.
pub fn arc_color_for_group(group: &str) -> (f32, f32, f32) {
    match group {
        "Amira's Arc" => DT_ARC_AMIRA,
        "Viktor's Arc" => DT_ARC_VIKTOR,
        "Mei-Lin's Seeds" => DT_ARC_MEILIN,
        "The Twins" => DT_ARC_TWINS,
        "Anna" => DT_ARC_ANNA,
        "Faction Politics" => DT_ARC_FACTION,
        "Governance" => DT_ARC_GOVERN,
        "First Word" => DT_ARC_FIRST_WORD,
        "Earth's Fate" => DT_ARC_EARTH,
        _ => DT_ARC_KEY,
    }
}

/// Build the decision tree chapters from the player's game state.
pub fn build_decision_tree(gs: &GameState) -> Vec<DecisionChapter> {
    // Collect all chosen decisions with descriptions
    let chosen: Vec<(&str, &str)> = gs.decisions.iter()
        .filter_map(|d| decision_description(d).map(|desc| (d.as_str(), desc)))
        .collect();

    if chosen.is_empty() {
        return Vec::new();
    }

    // Group by category, preserving order
    let mut chapters: Vec<DecisionChapter> = Vec::new();
    for &(key, desc) in &chosen {
        let group = decision_group(key);
        let all_in_group = all_decisions_for_group(group);

        // Build alternatives (decisions in same group that weren't chosen)
        let alternatives: Vec<DecisionAlt> = all_in_group.iter()
            .filter(|(k, _)| !gs.decisions.contains(&k.to_string()))
            .map(|(k, d)| DecisionAlt {
                key: k.to_string(),
                label: d.to_string(),
            })
            .collect();

        let node = DecisionNode {
            key: key.to_string(),
            label: desc.to_string(),
            chosen: true,
            character: group.to_string(),
            alternatives,
        };

        if let Some(ch) = chapters.iter_mut().find(|c| c.name == group) {
            // Avoid duplicate nodes
            if !ch.nodes.iter().any(|n| n.key == key) {
                ch.nodes.push(node);
            }
        } else {
            chapters.push(DecisionChapter {
                name: group.to_string(),
                arc_color: arc_color_for_group(group),
                nodes: vec![node],
            });
        }
    }
    chapters
}

// === Systems ===

/// Dismiss the decision tree overlay on ESC or background click.
pub fn decision_tree_dismiss(
    overlay_q: Query<(&Interaction, Entity), (With<DecisionTreeOverlay>, With<Button>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for (_, entity) in overlay_q.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }
    for (interaction, entity) in overlay_q.iter() {
        if *interaction == Interaction::Pressed {
            commands.entity(entity).despawn();
        }
    }
}

/// Animate the decision tree panel glow.
pub fn animate_decision_tree_glow(
    time: Res<Time>,
    mut query: Query<&mut BoxShadow, With<DecisionTreeGlow>>,
    overlay_q: Query<Entity, With<DecisionTreeOverlay>>,
) {
    if overlay_q.is_empty() { return; }
    let t = time.elapsed_secs();
    let pulse = 0.07 + 0.04 * (t * 1.0).sin();
    for mut shadow in query.iter_mut() {
        *shadow = BoxShadow::new(
            Color::srgba(DT_GLOW_COLOR.0, DT_GLOW_COLOR.1, DT_GLOW_COLOR.2, pulse),
            Val::ZERO, Val::ZERO,
            Val::Px(DT_GLOW_SPREAD), Val::Px(DT_GLOW_BLUR),
        );
    }
}

/// Parallax effect: shift layers based on mouse position.
pub fn parallax_system(
    windows: Query<&Window>,
    mut layers: Query<(&mut Node, &ParallaxLayer)>,
    overlay_q: Query<Entity, With<DecisionTreeOverlay>>,
) {
    if overlay_q.is_empty() { return; }
    let Ok(window) = windows.single() else { return };
    let Some(pos) = window.cursor_position() else { return };
    let cx = (pos.x / window.width() - 0.5) * 2.0;
    let cy = (pos.y / window.height() - 0.5) * 2.0;
    for (mut node, layer) in layers.iter_mut() {
        let offset_x = cx * layer.depth * DT_PARALLAX_RANGE;
        let offset_y = cy * layer.depth * DT_PARALLAX_RANGE;
        node.left = Val::Px(layer.base_x + offset_x);
        node.top = Val::Px(layer.base_y + offset_y);
    }
}

/// Hover effect on decision nodes — subtle brightening.
pub fn decision_node_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &DecisionNodeMarker),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bg, mut border, marker) in query.iter_mut() {
        if marker.chosen {
            match interaction {
                Interaction::Hovered => {
                    bg.0 = Color::srgba(0.16, 0.18, 0.28, 0.95);
                    *border = BorderColor::all(Color::srgba(0.5, 0.6, 0.85, 0.8));
                }
                _ => {
                    bg.0 = Color::srgba(
                        DT_NODE_BG_CHOSEN.0, DT_NODE_BG_CHOSEN.1,
                        DT_NODE_BG_CHOSEN.2, DT_NODE_BG_CHOSEN.3,
                    );
                    *border = BorderColor::all(Color::srgba(
                        DT_BRANCH_CHOSEN_COLOR.0, DT_BRANCH_CHOSEN_COLOR.1,
                        DT_BRANCH_CHOSEN_COLOR.2, 0.4,
                    ));
                }
            }
        }
    }
}
