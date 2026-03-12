// SPDX-License-Identifier: GPL-3.0-or-later

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
    BorderColor(if selected { border_sel() } else { border_unsel() })
}

// === Node builders ===
pub fn slot_node() -> Node {
    Node {
        width: Val::Vw(SLOT_VW), height: Val::Vw(SLOT_HEIGHT_VW),
        border: UiRect::all(Val::Px(SLOT_BORDER_PX)),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        overflow: Overflow::clip(),
        ..default()
    }
}

pub fn icon_node() -> Node {
    Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }
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
    build_panel: impl FnOnce(&mut ChildBuilder),
) {
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
            height: Val::Percent(100.0), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, ..default() },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        GlobalZIndex(200), marker, Interaction::default(),
        UiBgFade { target: DIALOG_FADE_TARGET, despawn_at_zero: false },
    )).with_children(|bg| {
        bg.spawn((panel_node, BackgroundColor(rgb(DIALOG_PANEL_BG)),
            BorderRadius::all(Val::Px(UI_CORNER_RADIUS * 2.0)),
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
