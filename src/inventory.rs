// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

// === Slot helpers ===
fn slot_node() -> Node {
    Node {
        width: Val::Vw(SLOT_VW),
        height: Val::Vw(SLOT_HEIGHT_VW),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        ..default()
    }
}

fn slot_bg() -> Color { Color::srgb(SLOT_BG.0, SLOT_BG.1, SLOT_BG.2) }
fn border_selected() -> Color { Color::srgba(BORDER_SELECTED.0, BORDER_SELECTED.1, BORDER_SELECTED.2, BORDER_SELECTED.3) }
fn border_unselected() -> Color { Color::srgba(BORDER_UNSELECTED.0, BORDER_UNSELECTED.1, BORDER_UNSELECTED.2, BORDER_UNSELECTED.3) }

pub fn spawn_slot(
    commands: &mut Commands, parent: Entity,
    slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool,
) -> Entity {
    let border = if selected { border_selected() } else { border_unselected() };
    let mut node = slot_node();
    if animated { node.width = Val::Vw(0.0); }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg()), BorderColor(border), slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
            ImageNode::new(icon),
        ));
        parent.spawn((
            Text::new(" "),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ));
    }).id();

    if animated {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false });
    }
    commands.entity(parent).add_child(child);
    child
}

pub fn spawn_color_slot(
    commands: &mut Commands, parent: Entity,
    slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool, available: bool, count_text: &str,
) -> Entity {
    let border = if selected { border_selected() } else { border_unselected() };
    let mut node = slot_node();
    if !available || animated { node.width = Val::Vw(0.0); }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg()), BorderColor(border), slot, Level3Slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
            ImageNode::new(icon),
        ));
        parent.spawn((
            Text::new(count_text),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        ));
    }).id();

    if animated && available {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false });
    }
    commands.entity(parent).add_child(child);
    child
}

fn spawn_l2_source_directions(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    selected_dir: Option<Direction>,
) {
    for dir in Direction::all() {
        let selected = selected_dir == Some(dir);
        let child = spawn_slot(commands, container, InventorySlot::SourceDir(dir),
            icons.source_dir(dir), selected, true);
        commands.entity(child).insert(Level2Slot);
    }
}

fn spawn_l2_turn_directions(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    selected_dir: Option<Direction>,
) {
    for dir in Direction::all() {
        let selected = selected_dir == Some(dir);
        let child = spawn_slot(commands, container, InventorySlot::TurnDir(dir),
            icons.turn_dir(dir), selected, true);
        commands.entity(child).insert(Level2Slot);
    }
}

fn rebuild_l3_source_colors(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    dir: Direction, selected_color: Option<usize>, placed: &PlacedSources,
) {
    for ci in 0..NUM_COLORS {
        let available = !placed.0.contains(&ci);
        let selected = selected_color == Some(ci);
        let icon = icons.source_color_dir(ci, dir);
        spawn_color_slot(commands, container, InventorySlot::SourceColor(ci), icon, selected, true, available, "1");
    }
}

fn rebuild_l3_turn_colors(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    dir: Direction, selected_color: Option<usize>,
) {
    for ci in 0..NUM_COLORS {
        let selected = selected_color == Some(ci);
        let icon = icons.turn_color_dir(ci, dir);
        spawn_color_slot(commands, container, InventorySlot::TurnColor(ci), icon, selected, true, true, "∞");
    }
}

fn collapse_expansion(
    commands: &mut Commands,
    l2_slots: &Query<Entity, With<Level2Slot>>,
    l3_slots: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
) {
    for entity in l2_slots.iter() {
        commands.entity(entity).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true });
    }
    for (entity, _) in l3_slots.iter() {
        commands.entity(entity).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true });
    }
}

// === Inventory interaction ===
pub fn inventory_interaction(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    slots: Query<(&Interaction, &InventorySlot), Changed<Interaction>>,
    l2_slots: Query<Entity, With<Level2Slot>>,
    l3_slots: Query<(Entity, &InventorySlot), With<Level3Slot>>,
    expansion_q: Query<Entity, With<ExpansionContainer>>,
    icons: Res<InventoryIcons>,
    placed_sources: Res<PlacedSources>,
    children_q: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
) {
    let mut clicked = None;
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed { clicked = Some(*slot); }
    }
    let Some(clicked) = clicked else { return };

    match clicked {
        InventorySlot::Floor => {
            selected_tool.0 = Tool::Floor;
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::Delete => {
            selected_tool.0 = Tool::Delete;
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::Source => {
            if inv_state.level == 1 || selected_tool.0 == Tool::Turn {
                if inv_state.level > 1 {
                    collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                }
                let dir = inv_state.direction.unwrap_or(Direction::North);
                inv_state.direction = Some(dir);
                if inv_state.color_index.is_none() || placed_sources.0.contains(&inv_state.color_index.unwrap_or(0)) {
                    inv_state.color_index = (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci));
                }
                inv_state.level = 3;
                selected_tool.0 = Tool::Source;
                let expansion = expansion_q.single();
                spawn_l2_source_directions(&mut commands, expansion, &icons, Some(dir));
                rebuild_l3_source_colors(&mut commands, expansion, &icons, dir, inv_state.color_index, &placed_sources);
            } else {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                selected_tool.0 = Tool::Floor;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::Turn => {
            if inv_state.level == 1 || selected_tool.0 == Tool::Source {
                if inv_state.level > 1 {
                    collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                }
                let dir = inv_state.direction.unwrap_or(Direction::North);
                inv_state.direction = Some(dir);
                if inv_state.color_index.is_none() {
                    inv_state.color_index = Some(0);
                }
                inv_state.level = 3;
                selected_tool.0 = Tool::Turn;
                let expansion = expansion_q.single();
                spawn_l2_turn_directions(&mut commands, expansion, &icons, Some(dir));
                rebuild_l3_turn_colors(&mut commands, expansion, &icons, dir, inv_state.color_index);
            } else {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                selected_tool.0 = Tool::Floor;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Source;
            let expansion = expansion_q.single();
            if inv_state.level == 2 {
                inv_state.level = 3;
                if inv_state.color_index.is_none() || placed_sources.0.contains(&inv_state.color_index.unwrap_or(0)) {
                    inv_state.color_index = (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci));
                }
                rebuild_l3_source_colors(&mut commands, expansion, &icons, dir, inv_state.color_index, &placed_sources);
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                for (entity, slot) in &l3_slots {
                    if let InventorySlot::SourceColor(ci) = slot {
                        let new_icon = icons.source_color_dir(*ci, dir);
                        if let Ok(children) = children_q.get(entity) {
                            for &child in children.iter() {
                                if let Ok(mut img) = image_q.get_mut(child) {
                                    img.image = new_icon.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        InventorySlot::TurnDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Turn;
            let _expansion = expansion_q.single();
            if inv_state.level == 2 {
                inv_state.level = 3;
                if inv_state.color_index.is_none() {
                    inv_state.color_index = Some(0);
                }
                rebuild_l3_turn_colors(&mut commands, _expansion, &icons, dir, inv_state.color_index);
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                for (entity, slot) in &l3_slots {
                    if let InventorySlot::TurnColor(ci) = slot {
                        let new_icon = icons.turn_color_dir(*ci, dir);
                        if let Ok(children) = children_q.get(entity) {
                            for &child in children.iter() {
                                if let Ok(mut img) = image_q.get_mut(child) {
                                    img.image = new_icon.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        InventorySlot::SourceColor(ci) => {
            if !placed_sources.0.contains(&ci) {
                inv_state.color_index = Some(ci);
                selected_tool.0 = Tool::Source;
            }
        }
        InventorySlot::TurnColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = Tool::Turn;
        }
    }
}

// === Inventory visuals ===
pub fn update_inventory_visuals(
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    mut slots: Query<(&Interaction, &InventorySlot, &mut BorderColor)>,
) {
    for (interaction, slot, mut border) in &mut slots {
        let selected = match slot {
            InventorySlot::Floor => selected_tool.0 == Tool::Floor,
            InventorySlot::Source => selected_tool.0 == Tool::Source && inv_state.level >= 2,
            InventorySlot::Turn => selected_tool.0 == Tool::Turn && inv_state.level >= 2,
            InventorySlot::Delete => selected_tool.0 == Tool::Delete,
            InventorySlot::SourceDir(dir) | InventorySlot::TurnDir(dir) => inv_state.direction == Some(*dir),
            InventorySlot::SourceColor(ci) | InventorySlot::TurnColor(ci) => inv_state.color_index == Some(*ci),
        };
        border.0 = match (*interaction, selected) {
            (Interaction::Hovered | Interaction::Pressed, _) => Color::srgba(BORDER_HOVERED.0, BORDER_HOVERED.1, BORDER_HOVERED.2, BORDER_HOVERED.3),
            (_, true) => border_selected(),
            (_, false) => border_unselected(),
        };
    }
}

pub fn update_l3_availability(
    mut commands: Commands,
    placed: Res<PlacedSources>,
    inv_state: Res<InventoryState>,
    l3_slots: Query<(Entity, &InventorySlot, &Node), With<Level3Slot>>,
) {
    if !placed.is_changed() || inv_state.level != 3 { return; }
    for (entity, slot, node) in &l3_slots {
        if let InventorySlot::SourceColor(ci) = slot {
            let should_show = !placed.0.contains(ci);
            let target = if should_show { SLOT_VW } else { 0.0 };
            let current = match node.width { Val::Vw(w) => w, _ => target };
            if (current - target).abs() > 0.1 {
                commands.entity(entity).insert(NodeWidthAnim { target, despawn_at_zero: false });
            }
        }
    }
}
