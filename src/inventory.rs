// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
pub use crate::slot_ui::*;

// === Inventory interaction ===
pub fn inventory_interaction(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    slots: Query<(&Interaction, &InventorySlot), Changed<Interaction>>,
    l2_slots: Query<Entity, With<Level2Slot>>,
    l3_slots: Query<(Entity, &InventorySlot), With<Level3Slot>>,
    divider_slots: Query<Entity, With<L2L3Divider>>,
    expansion_q: Query<Entity, With<ExpansionContainer>>,
    icons: Res<InventoryIcons>,
    play_mode: Res<PlayMode>,
    placed_teleports: Res<PlacedTeleports>,
    children_q: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
    font: Res<GameFont>,
) {
    if *play_mode != PlayMode::Editing { return; }
    let mut clicked = None;
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed { clicked = Some(*slot); }
    }
    let Some(clicked) = clicked else { return };
    let expansion = expansion_q.single();

    match clicked {
        InventorySlot::Floor | InventorySlot::Delete | InventorySlot::Switch => {
            selected_tool.0 = match clicked {
                InventorySlot::Floor => Tool::Floor, InventorySlot::Switch => Tool::Switch, _ => Tool::Delete,
            };
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion);
                inv_state.level = 1; inv_state.direction = None; inv_state.color_index = None;
            }
        }
        InventorySlot::Source => {
            if inv_state.level == 1 || selected_tool.0 != Tool::Source {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                let dir = inv_state.direction.unwrap_or(Direction::North);
                inv_state.direction = Some(dir);
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3; selected_tool.0 = Tool::Source;
                expand_container(&mut commands, expansion);
                spawn_l2_directions(&mut commands, expansion,
                    Direction::all().map(|d| (InventorySlot::SourceDir(d), icons.source_dir(d))), Some(dir), &font.0);
                spawn_l2l3_divider(&mut commands, expansion);
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::SourceColor(ci), icons.source_color_dir(ci, dir), true)).collect(),
                    inv_state.color_index, " ", &font.0);
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool);
            }
        }
        slot @ (InventorySlot::Goal | InventorySlot::Teleport) => {
            let is_tp = matches!(slot, InventorySlot::Teleport);
            let tool = if is_tp { Tool::Teleport } else { Tool::Goal };
            if inv_state.level == 1 || selected_tool.0 != tool {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                inv_state.direction = None;
                let preferred = inv_state.last_placed_color.or(inv_state.color_index);
                if is_tp {
                    inv_state.color_index = preferred.filter(|ci| *ci < NUM_TELEPORTS && placed_teleports.0[*ci] < 2)
                        .or_else(|| (0..NUM_TELEPORTS).find(|n| placed_teleports.0[*n] < 2));
                    if inv_state.color_index.is_none() {
                        selected_tool.0 = Tool::Floor;
                        inv_state.level = 1; inv_state.direction = None;
                        return;
                    }
                } else {
                    inv_state.color_index = Some(preferred.unwrap_or(0));
                }
                inv_state.level = 3;
                selected_tool.0 = tool;
                expand_container(&mut commands, expansion);
                if is_tp {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_TELEPORTS).map(|n| (InventorySlot::TeleportNum(n), icons.teleport_num(n), placed_teleports.0[n] < 2)).collect(),
                        inv_state.color_index, "2", &font.0);
                } else {
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::GoalColor(ci), icons.goal_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0);
                }
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool);
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
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                if has_dir {
                    inv_state.direction = Some(inv_state.direction.unwrap_or(Direction::North));
                } else {
                    inv_state.direction = None;
                }
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3;
                selected_tool.0 = tool;
                expand_container(&mut commands, expansion);
                let dir = inv_state.direction.unwrap_or(Direction::North);
                match slot {
                    InventorySlot::TurnBut => {
                        spawn_l2_directions(&mut commands, expansion,
                            Direction::all().map(|d| (InventorySlot::TurnButDir(d), icons.turnbut_dir(d))), Some(dir), &font.0);
                        spawn_l2l3_divider(&mut commands, expansion);
                        rebuild_l3_colors(&mut commands, expansion,
                            (0..NUM_COLORS).map(|ci| (InventorySlot::TurnButColor(ci), icons.turnbut_color_dir(ci, dir), true)).collect(),
                            inv_state.color_index, " ", &font.0);
                    }
                    InventorySlot::Turn => {
                        spawn_l2_directions(&mut commands, expansion,
                            Direction::all().map(|d| (InventorySlot::TurnDir(d), icons.turn_dir(d))), Some(dir), &font.0);
                        spawn_l2l3_divider(&mut commands, expansion);
                        rebuild_l3_colors(&mut commands, expansion,
                            (0..NUM_TURN_COLORS).map(|ci| (InventorySlot::TurnColor(ci), icons.turn_color_dir(ci, dir), true)).collect(),
                            inv_state.color_index, " ", &font.0);
                    }
                    InventorySlot::Bounce => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_BOUNCE_COLORS).map(|ci| (InventorySlot::BounceColor(ci), icons.bounce_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                    _ => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::BounceButColor(ci), icons.bouncebot_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                };
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool);
            }
        }
        InventorySlot::Painter => {
            if inv_state.level == 1 || selected_tool.0 != Tool::Painter {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                inv_state.direction = None;
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3; selected_tool.0 = Tool::Painter;
                expand_container(&mut commands, expansion);
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::PainterColor(ci), icons.painter_color(ci), true)).collect(),
                    inv_state.color_index, " ", &font.0);
            } else { collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool); }
        }
        slot @ (InventorySlot::Arrow | InventorySlot::ArrowBut) => {
            let (tool, is_but) = if matches!(slot, InventorySlot::Arrow) { (Tool::Arrow, false) } else { (Tool::ArrowBut, true) };
            if inv_state.level == 1 || selected_tool.0 != tool {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                inv_state.direction = Some(inv_state.direction.unwrap_or(Direction::North));
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3; selected_tool.0 = tool;
                expand_container(&mut commands, expansion);
                let dir = inv_state.direction.unwrap_or(Direction::North);
                if is_but {
                    spawn_l2_directions(&mut commands, expansion,
                        Direction::all().map(|d| (InventorySlot::ArrowButDir(d), icons.arrowbut_dir(d))), Some(dir), &font.0);
                    spawn_l2l3_divider(&mut commands, expansion);
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::ArrowButColor(ci), icons.arrowbut_color_dir(ci, dir), true)).collect(),
                        inv_state.color_index, " ", &font.0);
                } else {
                    spawn_l2_directions(&mut commands, expansion,
                        Direction::all().map(|d| (InventorySlot::ArrowDir(d), icons.arrow_dir(d))), Some(dir), &font.0);
                    spawn_l2l3_divider(&mut commands, expansion);
                    rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_ARROW_COLORS).map(|ci| (InventorySlot::ArrowColor(ci), icons.arrow_color_dir(ci, dir), true)).collect(),
                        inv_state.color_index, " ", &font.0);
                }
            } else { collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool); }
        }
        InventorySlot::Door => {
            if inv_state.level == 1 || selected_tool.0 != Tool::Door {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }

                inv_state.color_index = Some(0); inv_state.level = 2; selected_tool.0 = Tool::Door;
                inv_state.direction = None;
                expand_container(&mut commands, expansion);
                for (open, ico) in [(true, icons.door_open.clone()), (false, icons.door_closed.clone())] {
                    let c = spawn_base_slot(&mut commands, expansion, InventorySlot::DoorState(open), ico, open, true, true, " ", &font.0);
                    commands.entity(c).remove::<Level3Slot>().insert(Level2Slot);
                }
            } else { collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool); }
        }
        InventorySlot::DoorState(open) => {
            inv_state.color_index = Some(if open { 0 } else { 1 });
            selected_tool.0 = Tool::Door;
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Source;

            if inv_state.level == 2 {
                inv_state.level = 3;
                if inv_state.color_index.is_none() { inv_state.color_index = Some(0); }
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::SourceColor(ci), icons.source_color_dir(ci, dir), true)).collect(),
                    inv_state.color_index, " ", &font.0);
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
            let tb = matches!(slot, InventorySlot::TurnButDir(_));
            let old_dir = inv_state.direction; inv_state.direction = Some(dir);
            selected_tool.0 = if tb { Tool::TurnBut } else { Tool::Turn };

            if inv_state.level == 2 {
                inv_state.level = 3; if inv_state.color_index.is_none() { inv_state.color_index = Some(0); }
                if tb { rebuild_l3_colors(&mut commands, expansion, (0..NUM_COLORS).map(|ci| (InventorySlot::TurnButColor(ci), icons.turnbut_color_dir(ci, dir), true)).collect(), inv_state.color_index, " ", &font.0); }
                else { rebuild_l3_colors(&mut commands, expansion, (0..NUM_TURN_COLORS).map(|ci| (InventorySlot::TurnColor(ci), icons.turn_color_dir(ci, dir), true)).collect(), inv_state.color_index, " ", &font.0); }
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                for (entity, slot) in &l3_slots {
                    let ci = match slot { InventorySlot::TurnColor(c) if !tb => Some(*c), InventorySlot::TurnButColor(c) if tb => Some(*c), _ => None };
                    if let Some(ci) = ci { let ni = if tb { icons.turnbut_color_dir(ci, dir) } else { icons.turn_color_dir(ci, dir) };
                        if let Ok(ch) = children_q.get(entity) { for &c in ch.iter() { if let Ok(mut img) = image_q.get_mut(c) { img.image = ni.clone(); } } } }
                }
            }
        }
        slot @ (InventorySlot::ArrowDir(dir) | InventorySlot::ArrowButDir(dir)) => {
            let ab = matches!(slot, InventorySlot::ArrowButDir(_));
            let old_dir = inv_state.direction; inv_state.direction = Some(dir);
            selected_tool.0 = if ab { Tool::ArrowBut } else { Tool::Arrow };

            if inv_state.level == 2 {
                inv_state.level = 3; if inv_state.color_index.is_none() { inv_state.color_index = Some(0); }
                if ab { rebuild_l3_colors(&mut commands, expansion, (0..NUM_COLORS).map(|ci| (InventorySlot::ArrowButColor(ci), icons.arrowbut_color_dir(ci, dir), true)).collect(), inv_state.color_index, " ", &font.0); }
                else { rebuild_l3_colors(&mut commands, expansion, (0..NUM_ARROW_COLORS).map(|ci| (InventorySlot::ArrowColor(ci), icons.arrow_color_dir(ci, dir), true)).collect(), inv_state.color_index, " ", &font.0); }
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                for (entity, slot) in &l3_slots {
                    let ci = match slot { InventorySlot::ArrowColor(c) if !ab => Some(*c), InventorySlot::ArrowButColor(c) if ab => Some(*c), _ => None };
                    if let Some(ci) = ci { let ni = if ab { icons.arrowbut_color_dir(ci, dir) } else { icons.arrow_color_dir(ci, dir) };
                        if let Ok(ch) = children_q.get(entity) { for &c in ch.iter() { if let Ok(mut img) = image_q.get_mut(c) { img.image = ni.clone(); } } } }
                }
            }
        }
        InventorySlot::SourceColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = Tool::Source;
        }
        InventorySlot::GoalColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = Tool::Goal;
        }
        InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci)
        | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci)
        | InventorySlot::PainterColor(ci) | InventorySlot::ArrowColor(ci) | InventorySlot::ArrowButColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = match clicked {
                InventorySlot::TurnColor(_) => Tool::Turn,
                InventorySlot::TurnButColor(_) => Tool::TurnBut,
                InventorySlot::BounceColor(_) => Tool::Bounce,
                InventorySlot::PainterColor(_) => Tool::Painter,
                InventorySlot::ArrowColor(_) => Tool::Arrow,
                InventorySlot::ArrowButColor(_) => Tool::ArrowBut,
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
    mut commands: Commands,
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    mut slots: Query<(Entity, &Interaction, &InventorySlot, &mut BorderColor)>,
    play_mode: Res<PlayMode>,
) {
    if *play_mode == PlayMode::Marking { return; }
    for (entity, interaction, slot, mut border) in &mut slots {
        let selected = match slot {
            InventorySlot::Floor => selected_tool.0 == Tool::Floor,
            InventorySlot::Source => selected_tool.0 == Tool::Source && inv_state.level >= 2,
            InventorySlot::Goal => selected_tool.0 == Tool::Goal && inv_state.level >= 2,
            InventorySlot::Turn => selected_tool.0 == Tool::Turn && inv_state.level >= 2,
            InventorySlot::TurnBut => selected_tool.0 == Tool::TurnBut && inv_state.level >= 2,
            InventorySlot::Teleport => selected_tool.0 == Tool::Teleport && inv_state.level >= 2,
            InventorySlot::Bounce => selected_tool.0 == Tool::Bounce && inv_state.level >= 2,
            InventorySlot::BounceBut => selected_tool.0 == Tool::BounceBut && inv_state.level >= 2,
            InventorySlot::Arrow => selected_tool.0 == Tool::Arrow && inv_state.level >= 2,
            InventorySlot::ArrowBut => selected_tool.0 == Tool::ArrowBut && inv_state.level >= 2,
            InventorySlot::Door => selected_tool.0 == Tool::Door && inv_state.level >= 2,
            InventorySlot::Switch => selected_tool.0 == Tool::Switch,
            InventorySlot::Painter => selected_tool.0 == Tool::Painter && inv_state.level >= 2,
            InventorySlot::Delete => selected_tool.0 == Tool::Delete,
            InventorySlot::SourceDir(dir) | InventorySlot::TurnDir(dir) | InventorySlot::TurnButDir(dir)
            | InventorySlot::ArrowDir(dir) | InventorySlot::ArrowButDir(dir) => inv_state.direction == Some(*dir),
            InventorySlot::DoorState(open) => inv_state.color_index == Some(if *open { 0 } else { 1 }),
            InventorySlot::SourceColor(ci) | InventorySlot::GoalColor(ci)
            | InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci)
            | InventorySlot::TeleportNum(ci)
            | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci)
            | InventorySlot::PainterColor(ci) | InventorySlot::ArrowColor(ci) | InventorySlot::ArrowButColor(ci) => inv_state.color_index == Some(*ci),
        };
        match (*interaction, selected) {
            (Interaction::Hovered | Interaction::Pressed, _) => {
                border.0 = border_hovered();
                commands.entity(entity).remove::<BorderFade>();
            }
            (_, true) => {
                border.0 = border_sel();
                commands.entity(entity).remove::<BorderFade>();
            }
            (_, false) => {
                let c = border.0.to_srgba();
                let t = BORDER_UNSELECTED;
                if (c.red - t.0).abs() > 0.02 || (c.alpha - t.3).abs() > 0.02 {
                    commands.entity(entity).insert(BorderFade {
                        target: [t.0, t.1, t.2, t.3], speed: HOVER_FADE_SPEED,
                    });
                }
            }
        }
    }
}

// === Status bar descriptions ===
fn slot_description(slot: &InventorySlot) -> &'static str {
    match slot {
        InventorySlot::Floor => "Floor \u{2013} A simple tile for bots to walk on",
        InventorySlot::Source | InventorySlot::SourceDir(_) | InventorySlot::SourceColor(_) =>
            "Source \u{2013} Launches a colored bot in the arrow direction",
        InventorySlot::Goal | InventorySlot::GoalColor(_) =>
            "Goal \u{2013} The destination! Guide the matching bot here to win",
        InventorySlot::Turn | InventorySlot::TurnDir(_) | InventorySlot::TurnColor(_) =>
            "Turn \u{2013} Redirects bots along the L-path (grey = all bots)",
        InventorySlot::TurnBut | InventorySlot::TurnButDir(_) | InventorySlot::TurnButColor(_) =>
            "Turn But \u{2013} Redirects all bots EXCEPT this color",
        InventorySlot::Teleport | InventorySlot::TeleportNum(_) =>
            "Teleport \u{2013} Zap! Sends the bot to the matching portal",
        InventorySlot::Bounce | InventorySlot::BounceColor(_) =>
            "Bounce \u{2013} Sends bots back the way they came (grey = all bots)",
        InventorySlot::BounceBut | InventorySlot::BounceButColor(_) =>
            "Bounce But \u{2013} Bounces all bots EXCEPT this color",
        InventorySlot::Door | InventorySlot::DoorState(_) =>
            "Door \u{2013} Blocks the path until a switch opens it",
        InventorySlot::Switch => "Switch \u{2013} Press to toggle all doors open or closed",
        InventorySlot::Painter | InventorySlot::PainterColor(_) =>
            "Painter \u{2013} Changes the bot's color as it walks over",
        InventorySlot::Arrow | InventorySlot::ArrowDir(_) | InventorySlot::ArrowColor(_) =>
            "Arrow \u{2013} Redirects bots in the arrow direction (grey = all bots)",
        InventorySlot::ArrowBut | InventorySlot::ArrowButDir(_) | InventorySlot::ArrowButColor(_) =>
            "Arrow But \u{2013} Redirects all bots EXCEPT this color",
        InventorySlot::Delete => "Eraser \u{2013} Removes a tile from the board",
    }
}

pub fn update_status_bar(
    slots: Query<(&Interaction, &InventorySlot)>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<StatusBarText>>,
    time: Res<Time>,
) {
    let Ok((mut text, mut color)) = text_q.get_single_mut() else { return };
    let desc = slots.iter()
        .find(|(i, _)| matches!(i, Interaction::Hovered | Interaction::Pressed))
        .map(|(_, s)| slot_description(s));
    let target = if desc.is_some() { 0.85 } else { 0.0 };
    if let Some(d) = desc { if **text != d { **text = d.to_string() } }
    let cur = color.0.alpha();
    let new = cur + (target - cur) * STATUS_FADE_SPEED * time.delta_secs();
    color.0 = Color::srgba(TOOLTIP_COLOR.0, TOOLTIP_COLOR.1, TOOLTIP_COLOR.2,
        if (new - target).abs() < 0.01 { target } else { new });
}

