// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::slot_ui::*;

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
    play_mode: Res<PlayMode>,
    placed_goals: Res<PlacedGoals>,
    placed_teleports: Res<PlacedTeleports>,
    children_q: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
) {
    if *play_mode != PlayMode::Editing { return; }
    let mut clicked = None;
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed { clicked = Some(*slot); }
    }
    let Some(clicked) = clicked else { return };

    match clicked {
        InventorySlot::Floor | InventorySlot::Delete | InventorySlot::Switch => {
            selected_tool.0 = match clicked {
                InventorySlot::Floor => Tool::Floor, InventorySlot::Switch => Tool::Switch, _ => Tool::Delete,
            };
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1; inv_state.direction = None; inv_state.color_index = None;
            }
        }
        InventorySlot::Source => {
            if inv_state.level == 1 || selected_tool.0 != Tool::Source {
                if inv_state.level > 1 {
                    collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                }
                let dir = inv_state.direction.unwrap_or(Direction::North);
                inv_state.direction = Some(dir);
                let preferred = inv_state.last_placed_color.or(inv_state.color_index);
                inv_state.color_index = preferred.filter(|ci| !placed_sources.0.contains(ci))
                    .or_else(|| (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci)));
                inv_state.level = 3;
                selected_tool.0 = Tool::Source;
                let expansion = expansion_q.single();
                spawn_l2_directions(&mut commands, expansion,
                    Direction::all().map(|d| (InventorySlot::SourceDir(d), icons.source_dir(d))), Some(dir));
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::SourceColor(ci), icons.source_color_dir(ci, dir), !placed_sources.0.contains(&ci))).collect(),
                    inv_state.color_index, "1");
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &mut inv_state, &mut selected_tool);
            }
        }
        slot @ (InventorySlot::Goal | InventorySlot::Teleport) => {
            let is_tp = matches!(slot, InventorySlot::Teleport);
            let tool = if is_tp { Tool::Teleport } else { Tool::Goal };
            if inv_state.level == 1 || selected_tool.0 != tool {
                if inv_state.level > 1 {
                    collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                }
                inv_state.direction = None;
                let preferred = inv_state.last_placed_color.or(inv_state.color_index);
                if is_tp {
                    inv_state.color_index = preferred.filter(|ci| *ci < NUM_TELEPORTS && placed_teleports.0[*ci] < 2)
                        .or_else(|| (0..NUM_TELEPORTS).find(|n| placed_teleports.0[*n] < 2));
                } else {
                    inv_state.color_index = preferred.filter(|ci| !placed_goals.0.contains(ci))
                        .or_else(|| (0..NUM_COLORS).find(|ci| !placed_goals.0.contains(ci)));
                }
                inv_state.level = 3;
                selected_tool.0 = tool;
                let expansion = expansion_q.single();
                if is_tp {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_TELEPORTS).map(|n| (InventorySlot::TeleportNum(n), icons.teleport_num(n), placed_teleports.0[n] < 2)).collect(),
                        inv_state.color_index, "2");
                } else {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::GoalColor(ci), icons.goal_color(ci), !placed_goals.0.contains(&ci))).collect(),
                        inv_state.color_index, "1");
                }
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &mut inv_state, &mut selected_tool);
            }
        }
        slot @ (InventorySlot::Turn | InventorySlot::TurnBut | InventorySlot::Bounce | InventorySlot::BounceBut) => {
            let (tool, has_dir) = match slot {
                InventorySlot::Turn => (Tool::Turn, true),
                InventorySlot::TurnBut => (Tool::TurnBut, true),
                InventorySlot::Bounce => (Tool::Bounce, false),
                _ => (Tool::BounceBut, false),
            };
            if inv_state.level == 1 || selected_tool.0 != tool {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots); }
                if has_dir {
                    inv_state.direction = Some(inv_state.direction.unwrap_or(Direction::North));
                } else {
                    inv_state.direction = None;
                }
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3;
                selected_tool.0 = tool;
                let expansion = expansion_q.single();
                let dir = inv_state.direction.unwrap_or(Direction::North);
                match slot {
                    InventorySlot::TurnBut => {
                        spawn_l2_directions(&mut commands, expansion,
                            Direction::all().map(|d| (InventorySlot::TurnButDir(d), icons.turnbut_dir(d))), Some(dir));
                        rebuild_l3_colors(&mut commands, expansion,
                            (0..NUM_COLORS).map(|ci| (InventorySlot::TurnButColor(ci), icons.turnbut_color_dir(ci, dir), true)).collect(),
                            inv_state.color_index, "\u{221e}");
                    }
                    InventorySlot::Turn => {
                        spawn_l2_directions(&mut commands, expansion,
                            Direction::all().map(|d| (InventorySlot::TurnDir(d), icons.turn_dir(d))), Some(dir));
                        rebuild_l3_colors(&mut commands, expansion,
                            (0..NUM_TURN_COLORS).map(|ci| (InventorySlot::TurnColor(ci), icons.turn_color_dir(ci, dir), true)).collect(),
                            inv_state.color_index, "\u{221e}");
                    }
                    InventorySlot::Bounce => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_BOUNCE_COLORS).map(|ci| (InventorySlot::BounceColor(ci), icons.bounce_color(ci), true)).collect(),
                        inv_state.color_index, "\u{221e}"),
                    _ => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::BounceButColor(ci), icons.bouncebot_color(ci), true)).collect(),
                        inv_state.color_index, "\u{221e}"),
                };
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &mut inv_state, &mut selected_tool);
            }
        }
        InventorySlot::Door => {
            if inv_state.level == 1 || selected_tool.0 != Tool::Door {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots); }
                inv_state.color_index = Some(0); inv_state.level = 2; selected_tool.0 = Tool::Door;
                inv_state.direction = None;
                let exp = expansion_q.single();
                for (open, ico) in [(true, icons.door_open.clone()), (false, icons.door_closed.clone())] {
                    let c = spawn_slot(&mut commands, exp, InventorySlot::DoorState(open), ico, open, true);
                    commands.entity(c).insert(Level2Slot);
                }
            } else { collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &mut inv_state, &mut selected_tool); }
        }
        InventorySlot::DoorState(open) => {
            inv_state.color_index = Some(if open { 0 } else { 1 });
            selected_tool.0 = Tool::Door;
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Source;
            let expansion = expansion_q.single();
            if inv_state.level == 2 {
                inv_state.level = 3;
                inv_state.color_index = inv_state.color_index.filter(|ci| !placed_sources.0.contains(ci))
                    .or_else(|| (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci)));
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::SourceColor(ci), icons.source_color_dir(ci, dir), !placed_sources.0.contains(&ci))).collect(),
                    inv_state.color_index, "1");
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
        slot @ (InventorySlot::TurnDir(dir) | InventorySlot::TurnButDir(dir)) => {
            let is_turnbut = matches!(slot, InventorySlot::TurnButDir(_));
            let tool = if is_turnbut { Tool::TurnBut } else { Tool::Turn };
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = tool;
            let expansion = expansion_q.single();
            if inv_state.level == 2 {
                inv_state.level = 3;
                if inv_state.color_index.is_none() { inv_state.color_index = Some(0); }
                if is_turnbut {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::TurnButColor(ci), icons.turnbut_color_dir(ci, dir), true)).collect(),
                        inv_state.color_index, "\u{221e}");
                } else {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_TURN_COLORS).map(|ci| (InventorySlot::TurnColor(ci), icons.turn_color_dir(ci, dir), true)).collect(),
                        inv_state.color_index, "\u{221e}");
                }
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                for (entity, slot) in &l3_slots {
                    let ci = match slot {
                        InventorySlot::TurnColor(c) if !is_turnbut => Some(*c),
                        InventorySlot::TurnButColor(c) if is_turnbut => Some(*c),
                        _ => None,
                    };
                    if let Some(ci) = ci {
                        let new_icon = if is_turnbut { icons.turnbut_color_dir(ci, dir) } else { icons.turn_color_dir(ci, dir) };
                        if let Ok(children) = children_q.get(entity) {
                            for &child in children.iter() {
                                if let Ok(mut img) = image_q.get_mut(child) { img.image = new_icon.clone(); }
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
        InventorySlot::GoalColor(ci) => {
            if !placed_goals.0.contains(&ci) {
                inv_state.color_index = Some(ci);
                selected_tool.0 = Tool::Goal;
            }
        }
        InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci)
        | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = match clicked {
                InventorySlot::TurnColor(_) => Tool::Turn,
                InventorySlot::TurnButColor(_) => Tool::TurnBut,
                InventorySlot::BounceColor(_) => Tool::Bounce,
                _ => Tool::BounceBut,
            };
        }
        InventorySlot::TeleportNum(num) => {
            if placed_teleports.0[num] < 2 {
                inv_state.color_index = Some(num);
                selected_tool.0 = Tool::Teleport;
            }
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
            InventorySlot::Goal => selected_tool.0 == Tool::Goal && inv_state.level >= 2,
            InventorySlot::Turn => selected_tool.0 == Tool::Turn && inv_state.level >= 2,
            InventorySlot::TurnBut => selected_tool.0 == Tool::TurnBut && inv_state.level >= 2,
            InventorySlot::Teleport => selected_tool.0 == Tool::Teleport && inv_state.level >= 2,
            InventorySlot::Bounce => selected_tool.0 == Tool::Bounce && inv_state.level >= 2,
            InventorySlot::BounceBut => selected_tool.0 == Tool::BounceBut && inv_state.level >= 2,
            InventorySlot::Door => selected_tool.0 == Tool::Door && inv_state.level >= 2,
            InventorySlot::Switch => selected_tool.0 == Tool::Switch,
            InventorySlot::Delete => selected_tool.0 == Tool::Delete,
            InventorySlot::SourceDir(dir) | InventorySlot::TurnDir(dir) | InventorySlot::TurnButDir(dir) => inv_state.direction == Some(*dir),
            InventorySlot::DoorState(open) => inv_state.color_index == Some(if *open { 0 } else { 1 }),
            InventorySlot::SourceColor(ci) | InventorySlot::GoalColor(ci)
            | InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci)
            | InventorySlot::TeleportNum(ci)
            | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci) => inv_state.color_index == Some(*ci),
        };
        border.0 = match (*interaction, selected) {
            (Interaction::Hovered | Interaction::Pressed, _) => border_hovered(),
            (_, true) => border_sel(),
            (_, false) => border_unsel(),
        };
    }
}

pub fn update_l3_availability(
    mut commands: Commands,
    placed_sources: Res<PlacedSources>,
    placed_goals: Res<PlacedGoals>,
    placed_teleports: Res<PlacedTeleports>,
    inv_state: Res<InventoryState>,
    l3_slots: Query<(Entity, &InventorySlot, &Node), With<Level3Slot>>,
) {
    if inv_state.level != 3 { return; }
    let sources_changed = placed_sources.is_changed();
    let goals_changed = placed_goals.is_changed();
    let teleports_changed = placed_teleports.is_changed();
    if !sources_changed && !goals_changed && !teleports_changed { return; }
    for (entity, slot, node) in &l3_slots {
        let should_show = match slot {
            InventorySlot::SourceColor(ci) if sources_changed => Some(!placed_sources.0.contains(ci)),
            InventorySlot::GoalColor(ci) if goals_changed => Some(!placed_goals.0.contains(ci)),
            InventorySlot::TeleportNum(num) if teleports_changed => Some(placed_teleports.0[*num] < 2),
            _ => None,
        };
        if let Some(show) = should_show {
            let target = if show { SLOT_VW } else { 0.0 };
            let current = match node.width { Val::Vw(w) => w, _ => target };
            if (current - target).abs() > 0.1 {
                commands.entity(entity).insert(NodeWidthAnim { target, despawn_at_zero: false });
            }
        }
    }
}
