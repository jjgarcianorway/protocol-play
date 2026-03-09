// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;

pub fn spawn_base_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool, is_l3: bool, available: bool, count_text: &str,
) -> Entity {
    let bc = border_for(selected);
    let mut node = slot_node();
    if (is_l3 && !available) || animated { node.width = Val::Vw(0.0); }
    let mut ec = commands.spawn((Button, node, BackgroundColor(slot_bg()), bc, slot));
    if is_l3 { ec.insert(Level3Slot); }
    let child = ec.with_children(|p| {
        p.spawn((icon_node(), ImageNode::new(icon)));
        if is_l3 {
            p.spawn((Text::new(count_text), TextFont { font_size: COUNT_FONT, ..default() },
                TextColor(Color::srgba(1.0, 1.0, 1.0, COUNT_TEXT_ALPHA))));
        } else {
            p.spawn((Text::new(" "), TextFont { font_size: COUNT_FONT, ..default() },
                TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0))));
        }
    }).id();
    if animated && (!is_l3 || available) {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false });
    }
    commands.entity(parent).add_child(child);
    child
}

pub fn spawn_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool,
) -> Entity {
    spawn_base_slot(commands, parent, slot, icon, selected, animated, false, true, " ")
}

pub fn spawn_color_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool, available: bool, count_text: &str,
) -> Entity {
    spawn_base_slot(commands, parent, slot, icon, selected, animated, true, available, count_text)
}

pub fn spawn_l2_directions(
    commands: &mut Commands, container: Entity,
    slots_and_icons: [(InventorySlot, Handle<Image>); 4],
    selected_dir: Option<Direction>,
) {
    for (slot, icon) in slots_and_icons {
        let dir = match slot {
            InventorySlot::SourceDir(d) | InventorySlot::TurnDir(d) | InventorySlot::TurnButDir(d) => d,
            _ => continue,
        };
        let child = spawn_slot(commands, container, slot, icon, selected_dir == Some(dir), true);
        commands.entity(child).insert(Level2Slot);
    }
}

pub fn rebuild_l3_colors(
    commands: &mut Commands, container: Entity,
    slots_and_icons: Vec<(InventorySlot, Handle<Image>, bool)>,
    selected_color: Option<usize>, count_text: &str,
) {
    for (slot, icon, available) in slots_and_icons {
        let ci = match slot {
            InventorySlot::SourceColor(c) | InventorySlot::GoalColor(c)
            | InventorySlot::TurnColor(c) | InventorySlot::TurnButColor(c)
            | InventorySlot::TeleportNum(c) => c,
            _ => continue,
        };
        spawn_color_slot(commands, container, slot, icon, selected_color == Some(ci), true, available, count_text);
    }
}

pub fn collapse_expansion(
    commands: &mut Commands, l2: &Query<Entity, With<Level2Slot>>,
    l3: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
) {
    for e in l2.iter() { commands.entity(e).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true }); }
    for (e, _) in l3.iter() { commands.entity(e).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true }); }
}

pub fn collapse_and_reset(
    commands: &mut Commands, l2: &Query<Entity, With<Level2Slot>>,
    l3: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
    inv: &mut InventoryState, tool: &mut SelectedTool,
) {
    collapse_expansion(commands, l2, l3);
    *inv = InventoryState { level: 1, ..default() };
    tool.0 = Tool::Floor;
}
