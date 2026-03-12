// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;

pub fn spawn_base_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, is_l3: bool, available: bool, animate: bool, count_text: &str, font: &Handle<Font>,
) -> Entity {
    let bc = border_for(selected);
    let mut node = slot_node();
    let grow = animate && available;
    if (is_l3 && !available) || grow { node.width = Val::Vw(0.0); }
    let bg = slot_bg();
    let alpha = if grow { 0.0 } else { 1.0 };
    let mut ec = commands.spawn((Button, node, BackgroundColor(bg.with_alpha(alpha)), bc, slot));
    if is_l3 { ec.insert(Level3Slot); }
    if grow { ec.insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false }); }
    let child = ec.with_children(|p| {
        p.spawn((icon_node(), ImageNode::new(icon)));
        if is_l3 {
            p.spawn(Node { position_type: PositionType::Absolute, bottom: Val::Px(2.0),
                width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() })
                .with_child((Text::new(count_text), gf(COUNT_FONT, font),
                    TextColor(rgb(COUNT_AVAIL_COLOR))));
        }
    }).id();
    commands.entity(parent).add_child(child);
    child
}

pub fn spawn_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, font: &Handle<Font>,
) -> Entity {
    spawn_base_slot(commands, parent, slot, icon, selected, false, true, true, " ", font)
}

pub fn spawn_color_slot(
    commands: &mut Commands, parent: Entity, slot: InventorySlot, icon: Handle<Image>,
    selected: bool, available: bool, count_text: &str, font: &Handle<Font>,
) -> Entity {
    spawn_base_slot(commands, parent, slot, icon, selected, true, available, true, count_text, font)
}

pub fn spawn_l2_directions(
    commands: &mut Commands, container: Entity,
    slots_and_icons: [(InventorySlot, Handle<Image>); 4],
    selected_dir: Option<Direction>, font: &Handle<Font>,
) {
    for (slot, icon) in slots_and_icons {
        let dir = match slot {
            InventorySlot::SourceDir(d) | InventorySlot::TurnDir(d) | InventorySlot::TurnButDir(d)
            | InventorySlot::ArrowDir(d) | InventorySlot::ArrowButDir(d) => d,
            _ => continue,
        };
        let child = spawn_slot(commands, container, slot, icon, selected_dir == Some(dir), font);
        commands.entity(child).insert(Level2Slot);
    }
}

pub fn expand_container(commands: &mut Commands, container: Entity) {
    commands.entity(container).insert(ExpHeightAnim { target: EXPANSION_HEIGHT_VW });
}

pub fn collapse_container(commands: &mut Commands, container: Entity) {
    commands.entity(container).insert(ExpHeightAnim { target: 0.0 });
}

pub fn spawn_l2l3_divider(commands: &mut Commands, container: Entity) {
    let child = commands.spawn((
        Node { width: Val::Px(L2L3_DIVIDER_WIDTH), height: Val::Vw(SLOT_HEIGHT_VW * 0.6),
            ..default() },
        BackgroundColor(rgba(L2L3_DIVIDER_COLOR)), L2L3Divider,
    )).id();
    commands.entity(container).add_child(child);
}

pub fn rebuild_l3_colors(
    commands: &mut Commands, container: Entity,
    slots_and_icons: Vec<(InventorySlot, Handle<Image>, bool)>,
    selected_color: Option<usize>, count_text: &str, font: &Handle<Font>,
) {
    for (slot, icon, available) in slots_and_icons {
        let ci = match slot {
            InventorySlot::SourceColor(c) | InventorySlot::GoalColor(c)
            | InventorySlot::TurnColor(c) | InventorySlot::TurnButColor(c)
            | InventorySlot::TeleportColor(c) | InventorySlot::TeleportButColor(c)
            | InventorySlot::PainterColor(c)
            | InventorySlot::BounceColor(c) | InventorySlot::BounceButColor(c)
            | InventorySlot::SwitchColor(c) | InventorySlot::SwitchButColor(c)
            | InventorySlot::ArrowColor(c) | InventorySlot::ArrowButColor(c) => c,
            _ => continue,
        };
        spawn_color_slot(commands, container, slot, icon, selected_color == Some(ci), available, count_text, font);
    }
}

pub fn collapse_expansion(
    commands: &mut Commands, l2: &Query<Entity, With<Level2Slot>>,
    l3: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
    dividers: &Query<Entity, With<L2L3Divider>>,
    expansion: Entity,
) {
    for e in l2.iter() { commands.entity(e).despawn(); }
    for (e, _) in l3.iter() { commands.entity(e).despawn(); }
    for e in dividers.iter() { commands.entity(e).despawn(); }
    collapse_container(commands, expansion);
}

pub fn collapse_and_reset(
    commands: &mut Commands, l2: &Query<Entity, With<Level2Slot>>,
    l3: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
    dividers: &Query<Entity, With<L2L3Divider>>,
    expansion: Entity,
    inv: &mut InventoryState, tool: &mut SelectedTool,
) {
    collapse_expansion(commands, l2, l3, dividers, expansion);
    *inv = InventoryState { level: 1, ..default() };
    tool.0 = Tool::Floor;
}
