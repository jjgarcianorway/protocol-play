// SPDX-License-Identifier: GPL-3.0-or-later
//! Centralized UI theme: colors, fonts, spacing, and reusable widget builders.
//! Import this instead of hardcoding colors/sizes in individual files.
//! Design: Minimal. Mysterious. Meditative. (see .impeccable.md)

#![allow(dead_code)]
use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════════
// PALETTE — the single source of truth for all UI colors
// ═══════════════════════════════════════════════════════════════════════════════

pub mod palette {
    use bevy::prelude::Color;

    // ── Backgrounds ──
    pub const PANEL:       Color = Color::srgba(0.06, 0.07, 0.11, 0.85);
    pub const PANEL_MENU:  Color = Color::srgba(0.05, 0.06, 0.09, 0.72);
    pub const PANEL_LIGHT: Color = Color::srgba(0.10, 0.12, 0.16, 0.80);
    pub const OVERLAY:     Color = Color::srgba(0.02, 0.03, 0.06, 0.50);
    pub const SCRIM:       Color = Color::srgba(0.00, 0.00, 0.00, 0.65);
    pub const FADE_BLACK:  Color = Color::srgba(0.00, 0.00, 0.00, 0.00);

    // ── Primary (teal — the game's signature, used sparingly) ──
    pub const PRIMARY:       Color = Color::srgba(0.18, 0.54, 0.48, 0.92);
    pub const PRIMARY_HOVER: Color = Color::srgba(0.24, 0.64, 0.56, 0.95);
    pub const PRIMARY_TEXT:  Color = Color::WHITE;
    pub const PRIMARY_GLOW:    Color = Color::srgba(0.18, 0.54, 0.48, 0.20);
    pub const PRIMARY_PRESSED: Color = Color::srgba(0.14, 0.44, 0.38, 0.95);

    // ── Secondary / outline buttons ──
    pub const OUTLINE_BG:       Color = Color::srgba(1.0, 1.0, 1.0, 0.04);
    pub const OUTLINE_HOVER:    Color = Color::srgba(1.0, 1.0, 1.0, 0.10);
    pub const OUTLINE_PRESSED:  Color = Color::srgba(1.0, 1.0, 1.0, 0.14);
    pub const OUTLINE_BORDER:   Color = Color::srgba(1.0, 1.0, 1.0, 0.15);
    pub const OUTLINE_TEXT:     Color = Color::srgba(1.0, 1.0, 1.0, 0.70);

    // ── Danger (red) ──
    pub const DANGER:       Color = Color::srgba(0.65, 0.18, 0.18, 0.90);
    pub const DANGER_HOVER: Color = Color::srgba(0.75, 0.25, 0.25, 0.95);

    // ── Text links (ghost buttons) ──
    pub const LINK_BG:       Color = Color::NONE;
    pub const LINK_HOVER_BG: Color = Color::srgba(1.0, 1.0, 1.0, 0.06);
    pub const LINK_TEXT:     Color = Color::srgba(0.65, 0.72, 0.80, 0.90);
    pub const LINK_DIM_TEXT: Color = Color::srgba(0.50, 0.55, 0.62, 0.70);

    // ── Text colors ──
    pub const TEXT_BRIGHT:  Color = Color::WHITE;
    pub const TEXT_MAIN:    Color = Color::srgba(0.92, 0.93, 0.96, 1.0);
    pub const TEXT_SUB:     Color = Color::srgba(0.55, 0.80, 0.72, 0.90);
    pub const TEXT_MUTED:   Color = Color::srgba(0.55, 0.60, 0.68, 0.65);
    pub const TEXT_WHISPER: Color = Color::srgba(0.45, 0.52, 0.58, 0.65);
    pub const TEXT_DIM:     Color = Color::srgba(1.0, 1.0, 1.0, 0.30);
    pub const TEXT_GHOST:   Color = Color::srgba(1.0, 1.0, 1.0, 0.15);
    pub const TEXT_SEPARATOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.25);

    // ── Active/selected states ──
    pub const ACTIVE:   Color = Color::srgba(0.25, 0.48, 0.75, 0.90);
    pub const INACTIVE: Color = Color::srgba(0.10, 0.12, 0.18, 0.70);

    // ── Toggle states (settings ON/OFF) ──
    pub const TOGGLE_ON:  Color = Color::srgba(0.25, 0.68, 0.42, 1.0);
    pub const TOGGLE_OFF: Color = Color::srgba(0.40, 0.40, 0.45, 1.0);

    // ── Accent colors ──
    pub const STAR_GOLD:  Color = Color::srgb(1.0, 0.85, 0.2);
    pub const ANNA_BG:    Color = Color::srgba(0.04, 0.07, 0.12, 0.92);
    pub const ANNA_ACCENT: Color = Color::srgba(0.45, 0.65, 0.85, 1.0);
    pub const ANNA_TEXT:  Color = Color::srgba(0.88, 0.90, 0.95, 1.0);

    // ── Simulation overlays ──
    pub const SIM_ERROR_BG: Color = Color::srgba(0.12, 0.08, 0.08, 0.92);
    pub const SIM_TEXT_DIM: Color = Color::srgba(0.7, 0.7, 0.7, 0.7);

    // ── Settings panel ──
    pub const SETTINGS_BG:    Color = Color::srgba(0.07, 0.08, 0.12, 0.98);
    pub const SETTINGS_SCRIM: Color = Color::srgba(0.0, 0.0, 0.0, 0.72);
    pub const SETTINGS_LABEL: Color = Color::srgba(0.45, 0.48, 0.56, 1.0);
    pub const SETTINGS_TEXT:  Color = Color::srgba(0.72, 0.75, 0.82, 1.0);
    pub const SETTINGS_BTN:   Color = Color::srgba(0.22, 0.40, 0.65, 0.90);

    // ── Count badge ──
    pub const COUNT_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPOGRAPHY — consistent font sizes
// ═══════════════════════════════════════════════════════════════════════════════

pub mod typo {
    pub const H1: f32 = 44.0;      // main title (was 52 — less aggressive)
    pub const H2: f32 = 20.0;      // subtitle (tracks with title as a unit)
    pub const H3: f32 = 18.0;      // section heading
    pub const BODY: f32 = 16.0;    // buttons, labels
    pub const SMALL: f32 = 14.0;   // links, secondary actions
    pub const CAPTION: f32 = 13.0; // taglines, hints (visibility via alpha, not size)
    pub const MICRO: f32 = 11.0;   // version, fine print
}

// ═══════════════════════════════════════════════════════════════════════════════
// SPACING — consistent padding, margins, gaps
// ═══════════════════════════════════════════════════════════════════════════════

pub mod spacing {
    use bevy::prelude::*;

    pub const RADIUS: f32 = 8.0;
    pub const RADIUS_SM: f32 = 4.0;
    pub const RADIUS_LG: f32 = 12.0;

    pub fn btn_pad() -> UiRect { UiRect::axes(Val::Px(52.0), Val::Px(14.0)) }
    pub fn btn_pad_sm() -> UiRect { UiRect::axes(Val::Px(36.0), Val::Px(10.0)) }
    pub fn link_pad() -> UiRect { UiRect::axes(Val::Px(18.0), Val::Px(7.0)) }
    pub fn panel_pad() -> UiRect { UiRect::all(Val::Px(24.0)) }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPONENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Attach to any Button for automatic hover/pressed color transitions.
#[derive(Component)]
pub struct Hoverable {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Hoverable {
    /// Convenience: pressed defaults to hovered (for links where pressed isn't meaningful).
    pub fn simple(normal: Color, hovered: Color) -> Self {
        Self { normal, hovered, pressed: hovered }
    }
}

/// System: drives hover/pressed effects for all Hoverable buttons.
pub fn hover_system(
    mut q: Query<(&Interaction, &Hoverable, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, h, mut bg) in q.iter_mut() {
        bg.0 = match interaction {
            Interaction::Pressed => h.pressed,
            Interaction::Hovered => h.hovered,
            Interaction::None => h.normal,
        };
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WIDGET BUILDERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Primary action button: filled teal with static subtle glow.
pub fn spawn_button(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    parent.spawn((
        Button, marker,
        Hoverable { normal: palette::PRIMARY, hovered: palette::PRIMARY_HOVER, pressed: palette::PRIMARY_PRESSED },
        Node {
            padding: spacing::btn_pad(),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
            justify_content: JustifyContent::Center,
            min_width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(palette::PRIMARY),
        BoxShadow::new(palette::PRIMARY_GLOW,
            Val::ZERO, Val::Px(4.0), Val::Px(6.0), Val::Px(12.0)),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: typo::BODY, ..default() },
        TextColor(palette::PRIMARY_TEXT),
    ));
}

/// Outline button: transparent with subtle border.
pub fn spawn_button_outline(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str,
) {
    parent.spawn((
        Button, marker,
        Hoverable { normal: palette::OUTLINE_BG, hovered: palette::OUTLINE_HOVER, pressed: palette::OUTLINE_PRESSED },
        Node {
            padding: spacing::btn_pad_sm(),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS)),
            border: UiRect::all(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            min_width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(palette::OUTLINE_BG),
        BorderColor::all(palette::OUTLINE_BORDER),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: typo::SMALL, ..default() },
        TextColor(palette::OUTLINE_TEXT),
    ));
}

/// Text link button: no background, subtle hover.
pub fn spawn_link(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    marker: impl Component, label: &str, color: Color,
) {
    parent.spawn((
        Button, marker,
        Hoverable::simple(palette::LINK_BG, palette::LINK_HOVER_BG),
        Node { padding: spacing::link_pad(),
            border_radius: BorderRadius::all(Val::Px(spacing::RADIUS_SM)), ..default() },
        BackgroundColor(palette::LINK_BG),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: typo::SMALL, ..default() },
        TextColor(color),
    ));
}

/// Heading text.
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

/// Label with optional top margin.
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

/// Fully customized button (escape hatch).
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
        Button, marker, Hoverable::simple(normal, hovered), node, BackgroundColor(normal),
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size, ..default() },
        TextColor(text_color),
    ));
}
