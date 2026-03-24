// SPDX-License-Identifier: GPL-3.0-or-later
//! Centralized UI theme: colors, fonts, spacing, and reusable widget builders.
//! Import this instead of hardcoding colors/sizes in individual files.

#![allow(dead_code)]
use bevy::prelude::*;
use crate::types::GameFont;

// ═══════════════════════════════════════════════════════════════════════════════
// PALETTE — the single source of truth for all UI colors
// ═══════════════════════════════════════════════════════════════════════════════

pub mod palette {
    use bevy::prelude::Color;

    // ── Backgrounds ──
    pub const PANEL:       Color = Color::srgba(0.06, 0.07, 0.11, 0.85);
    pub const PANEL_LIGHT: Color = Color::srgba(0.10, 0.12, 0.16, 0.80);
    pub const OVERLAY:     Color = Color::srgba(0.02, 0.03, 0.06, 0.50);
    pub const SCRIM:       Color = Color::srgba(0.00, 0.00, 0.00, 0.65);

    // ── Primary (teal — the game's signature color) ──
    pub const PRIMARY:       Color = Color::srgba(0.18, 0.54, 0.48, 0.92);
    pub const PRIMARY_HOVER: Color = Color::srgba(0.24, 0.64, 0.56, 0.95);
    pub const PRIMARY_TEXT:  Color = Color::WHITE;

    // ── Secondary (subtle, muted) ──
    pub const SECONDARY:       Color = Color::srgba(0.14, 0.18, 0.24, 0.80);
    pub const SECONDARY_HOVER: Color = Color::srgba(0.20, 0.26, 0.34, 0.85);
    pub const SECONDARY_TEXT:  Color = Color::srgba(1.0, 1.0, 1.0, 0.75);

    // ── Danger (red) ──
    pub const DANGER:       Color = Color::srgba(0.65, 0.18, 0.18, 0.90);
    pub const DANGER_HOVER: Color = Color::srgba(0.75, 0.25, 0.25, 0.95);

    // ── Text links (ghost buttons) ──
    pub const LINK_BG:       Color = Color::NONE;
    pub const LINK_HOVER_BG: Color = Color::srgba(1.0, 1.0, 1.0, 0.06);
    pub const LINK_TEXT:     Color = Color::srgba(0.65, 0.72, 0.80, 0.90);
    pub const LINK_DIM_TEXT: Color = Color::srgba(0.50, 0.55, 0.62, 0.70);

    // ── Text colors ──
    pub const TEXT_BRIGHT: Color = Color::WHITE;
    pub const TEXT_MAIN:   Color = Color::srgba(0.92, 0.93, 0.96, 1.0);
    pub const TEXT_SUB:    Color = Color::srgba(0.55, 0.80, 0.72, 0.90);
    pub const TEXT_MUTED:  Color = Color::srgba(0.55, 0.60, 0.68, 0.65);
    pub const TEXT_DIM:    Color = Color::srgba(1.0, 1.0, 1.0, 0.20);

    // ── Active/selected states ──
    pub const ACTIVE:   Color = Color::srgba(0.25, 0.48, 0.75, 0.90);
    pub const INACTIVE: Color = Color::srgba(0.10, 0.12, 0.18, 0.70);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPOGRAPHY — consistent font sizes
// ═══════════════════════════════════════════════════════════════════════════════

pub mod typo {
    pub const H1: f32 = 52.0;      // main title
    pub const H2: f32 = 22.0;      // subtitle
    pub const H3: f32 = 18.0;      // section heading
    pub const BODY: f32 = 16.0;    // buttons, labels
    pub const SMALL: f32 = 14.0;   // links, secondary
    pub const CAPTION: f32 = 13.0; // taglines, hints
    pub const MICRO: f32 = 11.0;   // version, fine print
}

// ═══════════════════════════════════════════════════════════════════════════════
// SPACING — consistent padding, margins, gaps
// ═══════════════════════════════════════════════════════════════════════════════

pub mod spacing {
    use bevy::prelude::*;

    pub const RADIUS: f32 = 8.0;       // standard border radius
    pub const RADIUS_SM: f32 = 4.0;    // small elements
    pub const RADIUS_LG: f32 = 12.0;   // large panels

    pub fn btn_pad() -> UiRect { UiRect::axes(Val::Px(52.0), Val::Px(14.0)) }
    pub fn btn_pad_sm() -> UiRect { UiRect::axes(Val::Px(36.0), Val::Px(10.0)) }
    pub fn link_pad() -> UiRect { UiRect::axes(Val::Px(18.0), Val::Px(7.0)) }
    pub fn panel_pad() -> UiRect { UiRect::all(Val::Px(24.0)) }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENTS — reusable markers for hover/interaction
// ═══════════════════════════════════════════════════════════════════════════════

/// Attach to any Button for automatic hover color transitions.
#[derive(Component)]
pub struct Hoverable {
    pub normal: Color,
    pub hovered: Color,
}

/// System: drives hover effects for all Hoverable buttons.
pub fn hover_system(
    mut q: Query<(&Interaction, &Hoverable, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, h, mut bg) in q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered | Interaction::Pressed => h.hovered,
            Interaction::None => h.normal,
        };
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WIDGET BUILDERS — spawn consistent UI elements
// ═══════════════════════════════════════════════════════════════════════════════

/// Spawn a primary action button (filled, prominent).
pub fn spawn_button(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    spawn_styled_button(parent, font, marker, label, typo::BODY,
        palette::PRIMARY, palette::PRIMARY_HOVER, palette::PRIMARY_TEXT,
        spacing::btn_pad(), 0.0);
}

/// Spawn a secondary action button (muted, less prominent).
pub fn spawn_button_secondary(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    spawn_styled_button(parent, font, marker, label, typo::SMALL,
        palette::SECONDARY, palette::SECONDARY_HOVER, palette::SECONDARY_TEXT,
        spacing::btn_pad_sm(), 0.0);
}

/// Spawn a text link button (no background, subtle hover).
pub fn spawn_link(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str, color: Color,
) {
    parent.spawn((
        Button, marker,
        Hoverable { normal: palette::LINK_BG, hovered: palette::LINK_HOVER_BG },
        Node { padding: spacing::link_pad(),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS_SM)), ..default() },
        BackgroundColor(palette::LINK_BG),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: typo::SMALL, ..default() },
        TextColor(color),
    ));
}

/// Spawn a fully customized button.
pub fn spawn_styled_button(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str, font_size: f32,
    normal: Color, hovered: Color, text_color: Color,
    padding: UiRect, margin_bottom: f32,
) {
    let mut node = Node {
        padding, justify_content: JustifyContent::Center,
        border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
        ..default()
    };
    if margin_bottom > 0.0 { node.margin.bottom = Val::Px(margin_bottom); }
    parent.spawn((
        Button, marker, Hoverable { normal, hovered }, node,
        BackgroundColor(normal),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size, ..default() },
        TextColor(text_color),
    ));
}

/// Spawn a heading text.
pub fn spawn_heading(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    text: &str, size: f32, color: Color,
) {
    parent.spawn((
        Text::new(text),
        TextFont { font: font.clone(), font_size: size, ..default() },
        TextColor(color),
    ));
}

/// Spawn a text label with optional top margin.
pub fn spawn_label(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    text: &str, size: f32, color: Color, margin_top: f32,
) {
    parent.spawn((
        Text::new(text),
        TextFont { font: font.clone(), font_size: size, ..default() },
        TextColor(color),
        Node { margin: UiRect::top(Val::Px(margin_top)), ..default() },
    ));
}
