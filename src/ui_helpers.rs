// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(dead_code)]

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

// === Font helper ===
pub fn gf(size: f32, font: &Handle<Font>) -> TextFont {
    TextFont { font: font.clone(), font_size: size, ..default() }
}

// === Color helpers ===
pub fn rgb(c: (f32, f32, f32)) -> Color { Color::srgb(c.0, c.1, c.2) }
pub fn rgba(c: (f32, f32, f32, f32)) -> Color { Color::srgba(c.0, c.1, c.2, c.3) }

// === Common colors ===
pub fn slot_bg() -> Color { rgb(SLOT_BG) }
pub fn btn_bg() -> Color { rgb(BTN_BG) }
pub fn border_sel() -> Color { rgba(BORDER_SELECTED) }
pub fn border_unsel() -> Color { rgba(BORDER_UNSELECTED) }
pub fn border_hovered() -> Color { rgba(BORDER_HOVERED) }
pub fn border_for(selected: bool) -> BorderColor {
    BorderColor::all(if selected { border_sel() } else { border_unsel() })
}

// === Node builders ===
/// Compute effective slot sizes for `n` items to fit within the screen width.
/// Returns (slot_vw, slot_height_vw, icon_vw).
pub fn fit_slot_sizes(n: usize, max_slot: f32) -> (f32, f32, f32) {
    let max_vw = 96.0;
    let s = ((max_vw - INVENTORY_PAD_VW * 2.0 - INVENTORY_GAP_VW * (n as f32 - 1.0)) / n as f32).min(max_slot);
    let h = s * SLOT_HEIGHT_VW / SLOT_VW;
    let i = s * ICON_VW / SLOT_VW;
    (s, h, i)
}

pub fn slot_node_sized(w: f32, h: f32) -> Node {
    Node {
        width: Val::Vw(w), height: Val::Vw(h),
        border: UiRect::all(Val::Px(SLOT_BORDER_PX)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        overflow: Overflow::clip(),
        border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)),
        ..default()
    }
}

pub fn icon_node_sized(s: f32) -> Node {
    Node { width: Val::Vw(s), height: Val::Vw(s), ..default() }
}

pub fn text_btn_node() -> Node {
    Node { padding: UiRect::axes(Val::Px(TEXT_BTN_PAD.0), Val::Px(TEXT_BTN_PAD.1)), ..default() }
}

pub fn dialog_btn_node() -> Node {
    Node { padding: UiRect::axes(Val::Px(DIALOG_BTN_PAD.0), Val::Px(DIALOG_BTN_PAD.1)), ..default() }
}

pub fn dialog_panel_node(row_gap: f32) -> Node {
    Node { flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(DIALOG_PAD)),
        row_gap: Val::Px(row_gap), align_items: AlignItems::Center,
        min_width: Val::Px(DIALOG_MIN_WIDTH), ..default() }
}

// === Dialog spawn ===
pub fn spawn_dialog<M: Component>(
    commands: &mut Commands, marker: M, panel_node: Node,
    build_panel: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
            height: Val::Percent(100.0), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, ..default() },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(200), marker, Interaction::default(),
        UiBgFade { target: DIALOG_FADE_TARGET, despawn_at_zero: false },
    )).with_children(|bg| {
        let mut pn = panel_node;
        pn.border_radius = BorderRadius::all(Val::Px(UI_CORNER_RADIUS * 2.0));
        bg.spawn((pn, BackgroundColor(rgb(DIALOG_PANEL_BG)),
        )).with_children(build_panel);
    });
}

/// Fade out all entities matching the query and despawn when done.
pub fn fade_out<T: Component>(commands: &mut Commands, q: &Query<Entity, With<T>>) {
    for e in q { commands.entity(e).insert(UiBgFade { target: 0.0, despawn_at_zero: true }); }
}

/// Suppress ghost hover until mouse moves to a different tile.
pub fn suppress_ghost(hovered: &HoveredCell, ghost_cell: &mut GhostCell) {
    ghost_cell.last_placed = hovered.0;
}
