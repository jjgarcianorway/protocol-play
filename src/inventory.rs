// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
pub use crate::slot_ui::*;

/// Refresh L3 slot icons when direction changes (shared by Source/Turn/Arrow handlers).
fn refresh_l3_icons(
    l3_slots: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
    children_q: &Query<&Children>,
    image_q: &mut Query<&mut ImageNode>,
    mut get_icon: impl FnMut(&InventorySlot) -> Option<Handle<Image>>,
) {
    for (entity, slot) in l3_slots {
        if let Some(ni) = get_icon(slot) {
            if let Ok(ch) = children_q.get(entity) {
                for &c in ch.iter() { if let Ok(mut img) = image_q.get_mut(c) { img.image = ni.clone(); } }
            }
        }
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
        InventorySlot::Floor | InventorySlot::Delete => {
            selected_tool.0 = match clicked { InventorySlot::Delete => Tool::Delete, _ => Tool::Floor };
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
                        selected_tool.0 = Tool::Floor; inv_state.level = 1; inv_state.direction = None; return;
                    }
                } else {
                    inv_state.color_index = Some(preferred.unwrap_or(0));
                }
                inv_state.level = 3; selected_tool.0 = tool;
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
        slot @ (InventorySlot::Turn | InventorySlot::TurnBut | InventorySlot::Bounce | InventorySlot::BounceBut
            | InventorySlot::Switch | InventorySlot::SwitchBut | InventorySlot::Painter) => {
            let (tool, has_dir) = match slot {
                InventorySlot::Turn => (Tool::Turn, true), InventorySlot::TurnBut => (Tool::TurnBut, true),
                InventorySlot::Bounce => (Tool::Bounce, false), InventorySlot::BounceBut => (Tool::BounceBut, false),
                InventorySlot::Switch => (Tool::Switch, false), InventorySlot::Painter => (Tool::Painter, false),
                _ => (Tool::ColorSwitchBut, false),
            };
            let sw = matches!(slot, InventorySlot::Switch);
            let is_active = selected_tool.0 == tool || (sw && selected_tool.0 == Tool::ColorSwitch);
            if inv_state.level == 1 || !is_active {
                if inv_state.level > 1 { collapse_expansion(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion); }
                if has_dir { inv_state.direction = Some(inv_state.direction.unwrap_or(Direction::North)); }
                else { inv_state.direction = None; }
                inv_state.color_index = Some(inv_state.last_placed_color.or(inv_state.color_index).unwrap_or(0));
                inv_state.level = 3;
                selected_tool.0 = if sw && inv_state.color_index != Some(NUM_COLORS) { Tool::ColorSwitch } else { tool };
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
                    InventorySlot::BounceBut => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::BounceButColor(ci), icons.bouncebot_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                    InventorySlot::Switch => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_SWITCH_COLORS).map(|ci| (InventorySlot::SwitchColor(ci), icons.switch_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                    InventorySlot::Painter => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::PainterColor(ci), icons.painter_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                    _ => rebuild_l3_colors(&mut commands, expansion,
                        (0..NUM_COLORS).map(|ci| (InventorySlot::SwitchButColor(ci), icons.switchbut_color(ci), true)).collect(),
                        inv_state.color_index, " ", &font.0),
                };
            } else {
                collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool);
            }
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
                    let c = spawn_base_slot(&mut commands, expansion, InventorySlot::DoorState(open), ico, open, true, true, true, " ", &font.0);
                    commands.entity(c).remove::<Level3Slot>().insert(Level2Slot);
                }
            } else { collapse_and_reset(&mut commands, &l2_slots, &l3_slots, &divider_slots, expansion, &mut inv_state, &mut selected_tool); }
        }
        InventorySlot::DoorState(open) => {
            inv_state.color_index = Some(if open { 0 } else { 1 }); selected_tool.0 = Tool::Door;
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir); selected_tool.0 = Tool::Source;
            if inv_state.level == 2 {
                inv_state.level = 3;
                if inv_state.color_index.is_none() { inv_state.color_index = Some(0); }
                rebuild_l3_colors(&mut commands, expansion,
                    (0..NUM_COLORS).map(|ci| (InventorySlot::SourceColor(ci), icons.source_color_dir(ci, dir), true)).collect(),
                    inv_state.color_index, " ", &font.0);
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                refresh_l3_icons(&l3_slots, &children_q, &mut image_q, |s| {
                    if let InventorySlot::SourceColor(ci) = s { Some(icons.source_color_dir(*ci, dir)) } else { None }
                });
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
                refresh_l3_icons(&l3_slots, &children_q, &mut image_q, |s| match s {
                    InventorySlot::TurnColor(c) if !tb => Some(icons.turn_color_dir(*c, dir)),
                    InventorySlot::TurnButColor(c) if tb => Some(icons.turnbut_color_dir(*c, dir)),
                    _ => None,
                });
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
                refresh_l3_icons(&l3_slots, &children_q, &mut image_q, |s| match s {
                    InventorySlot::ArrowColor(c) if !ab => Some(icons.arrow_color_dir(*c, dir)),
                    InventorySlot::ArrowButColor(c) if ab => Some(icons.arrowbut_color_dir(*c, dir)),
                    _ => None,
                });
            }
        }
        InventorySlot::SourceColor(ci) => { inv_state.color_index = Some(ci); selected_tool.0 = Tool::Source; }
        InventorySlot::GoalColor(ci) => { inv_state.color_index = Some(ci); selected_tool.0 = Tool::Goal; }
        InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci)
        | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci)
        | InventorySlot::SwitchColor(ci) | InventorySlot::SwitchButColor(ci)
        | InventorySlot::PainterColor(ci) | InventorySlot::ArrowColor(ci) | InventorySlot::ArrowButColor(ci) => {
            inv_state.color_index = Some(ci);
            selected_tool.0 = match clicked {
                InventorySlot::TurnColor(_) => Tool::Turn, InventorySlot::TurnButColor(_) => Tool::TurnBut,
                InventorySlot::BounceColor(_) => Tool::Bounce,
                InventorySlot::SwitchColor(c) => if c == NUM_COLORS { Tool::Switch } else { Tool::ColorSwitch },
                InventorySlot::SwitchButColor(_) => Tool::ColorSwitchBut, InventorySlot::PainterColor(_) => Tool::Painter,
                InventorySlot::ArrowColor(_) => Tool::Arrow, InventorySlot::ArrowButColor(_) => Tool::ArrowBut,
                _ => Tool::BounceBut,
            };
        }
        InventorySlot::TeleportNum(num) => {
            if placed_teleports.0[num] < 2 { inv_state.color_index = Some(num); selected_tool.0 = Tool::Teleport; }
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
        let t = selected_tool.0; let lv2 = inv_state.level >= 2;
        let selected = match slot {
            InventorySlot::Floor => t == Tool::Floor,
            InventorySlot::Delete => t == Tool::Delete,
            InventorySlot::Switch => (t == Tool::Switch || t == Tool::ColorSwitch) && lv2,
            InventorySlot::Source => t == Tool::Source && lv2,
            InventorySlot::Goal => t == Tool::Goal && lv2,
            InventorySlot::Turn => t == Tool::Turn && lv2,
            InventorySlot::TurnBut => t == Tool::TurnBut && lv2,
            InventorySlot::Teleport => t == Tool::Teleport && lv2,
            InventorySlot::Bounce => t == Tool::Bounce && lv2,
            InventorySlot::BounceBut => t == Tool::BounceBut && lv2,
            InventorySlot::Arrow => t == Tool::Arrow && lv2,
            InventorySlot::ArrowBut => t == Tool::ArrowBut && lv2,
            InventorySlot::Door => t == Tool::Door && lv2,
            InventorySlot::SwitchBut => t == Tool::ColorSwitchBut && lv2,
            InventorySlot::Painter => t == Tool::Painter && lv2,
            InventorySlot::SourceDir(dir) | InventorySlot::TurnDir(dir) | InventorySlot::TurnButDir(dir)
            | InventorySlot::ArrowDir(dir) | InventorySlot::ArrowButDir(dir) => inv_state.direction == Some(*dir),
            InventorySlot::DoorState(open) => inv_state.color_index == Some(if *open { 0 } else { 1 }),
            InventorySlot::SourceColor(ci) | InventorySlot::GoalColor(ci)
            | InventorySlot::TurnColor(ci) | InventorySlot::TurnButColor(ci) | InventorySlot::TeleportNum(ci)
            | InventorySlot::BounceColor(ci) | InventorySlot::BounceButColor(ci)
            | InventorySlot::SwitchColor(ci) | InventorySlot::SwitchButColor(ci)
            | InventorySlot::PainterColor(ci) | InventorySlot::ArrowColor(ci) | InventorySlot::ArrowButColor(ci) => inv_state.color_index == Some(*ci),
        };
        if matches!(*interaction, Interaction::Hovered | Interaction::Pressed) {
            border.0 = border_hovered(); commands.entity(entity).remove::<BorderFade>();
        } else if selected {
            border.0 = border_sel(); commands.entity(entity).remove::<BorderFade>();
        } else {
            let c = border.0.to_srgba(); let t = BORDER_UNSELECTED;
            if (c.red - t.0).abs() > 0.02 || (c.alpha - t.3).abs() > 0.02 {
                commands.entity(entity).insert(BorderFade { target: [t.0, t.1, t.2, t.3], speed: HOVER_FADE_SPEED });
            }
        }
    }
}

// === Status bar descriptions ===
fn tile_desc(idx: usize) -> &'static str {
    const D: &[&str] = &[
        "Floor \u{2013} A simple tile for bots to walk on",
        "Source \u{2013} Launches a colored bot in the arrow direction",
        "Goal \u{2013} The destination! Guide the matching bot here to win",
        "Turn \u{2013} Redirects bots along the L-path (grey = all bots)",
        "Turn But \u{2013} Redirects all bots EXCEPT this color",
        "Teleport \u{2013} Zap! Sends the bot to the matching portal",
        "Bounce \u{2013} Sends bots back the way they came (grey = all bots)",
        "Bounce But \u{2013} Bounces all bots EXCEPT this color",
        "Door \u{2013} Blocks the path until a switch opens it",
        "Switch \u{2013} Toggles all doors (grey = all bots)",
        "Switch But \u{2013} All bots EXCEPT this color toggle doors",
        "Painter \u{2013} Changes the bot's color as it walks over",
        "Arrow \u{2013} Redirects bots in the arrow direction (grey = all bots)",
        "Arrow But \u{2013} Redirects all bots EXCEPT this color",
        "Eraser \u{2013} Removes a tile from the board", "Empty",
    ]; D[idx]
}
fn slot_description(slot: &InventorySlot) -> &'static str {
    match slot {
        InventorySlot::Floor => tile_desc(0),
        InventorySlot::Source | InventorySlot::SourceDir(_) | InventorySlot::SourceColor(_) => tile_desc(1),
        InventorySlot::Goal | InventorySlot::GoalColor(_) => tile_desc(2),
        InventorySlot::Turn | InventorySlot::TurnDir(_) | InventorySlot::TurnColor(_) => tile_desc(3),
        InventorySlot::TurnBut | InventorySlot::TurnButDir(_) | InventorySlot::TurnButColor(_) => tile_desc(4),
        InventorySlot::Teleport | InventorySlot::TeleportNum(_) => tile_desc(5),
        InventorySlot::Bounce | InventorySlot::BounceColor(_) => tile_desc(6),
        InventorySlot::BounceBut | InventorySlot::BounceButColor(_) => tile_desc(7),
        InventorySlot::Door | InventorySlot::DoorState(_) => tile_desc(8),
        InventorySlot::Switch | InventorySlot::SwitchColor(_) => tile_desc(9),
        InventorySlot::SwitchBut | InventorySlot::SwitchButColor(_) => tile_desc(10),
        InventorySlot::Painter | InventorySlot::PainterColor(_) => tile_desc(11),
        InventorySlot::Arrow | InventorySlot::ArrowDir(_) | InventorySlot::ArrowColor(_) => tile_desc(12),
        InventorySlot::ArrowBut | InventorySlot::ArrowButDir(_) | InventorySlot::ArrowButColor(_) => tile_desc(13),
        InventorySlot::Delete => tile_desc(14),
    }
}

fn tilekind_description(kind: &TileKind) -> &'static str {
    match kind {
        TileKind::Empty => tile_desc(15),
        TileKind::Floor => tile_desc(0),
        TileKind::Source(_, _) => tile_desc(1),  TileKind::Goal(_) => tile_desc(2),
        TileKind::Turn(_, _) => tile_desc(3),    TileKind::TurnBut(_, _) => tile_desc(4),
        TileKind::Teleport(_) => tile_desc(5),   TileKind::Bounce(_) => tile_desc(6),
        TileKind::BounceBut(_) => tile_desc(7),  TileKind::Door(_) => tile_desc(8),
        TileKind::Switch | TileKind::ColorSwitch(_) => tile_desc(9),
        TileKind::ColorSwitchBut(_) => tile_desc(10), TileKind::Painter(_) => tile_desc(11),
        TileKind::Arrow(_, _) => tile_desc(12),  TileKind::ArrowBut(_, _) => tile_desc(13),
    }
}

pub fn update_status_bar(
    slots: Query<(&Interaction, &InventorySlot)>,
    test_slots: Query<(&Interaction, &TestInventorySlot)>,
    test_inv: Res<TestInventory>,
    mut text_q: Query<(&mut Text, &mut TextColor), With<StatusBarText>>,
    time: Res<Time>,
) {
    let Ok((mut text, mut color)) = text_q.get_single_mut() else { return };
    let desc = slots.iter()
        .find(|(i, _)| matches!(i, Interaction::Hovered | Interaction::Pressed))
        .map(|(_, s)| slot_description(s))
        .or_else(|| test_slots.iter()
            .find(|(i, _)| matches!(i, Interaction::Hovered | Interaction::Pressed))
            .and_then(|(_, s)| {
                if s.0 == usize::MAX { Some("Remove \u{2013} Pick up a placed tile") }
                else { test_inv.items.get(s.0).map(|(k, _)| tilekind_description(k)) }
            }));
    let target = if desc.is_some() { 0.85 } else { 0.0 };
    if let Some(d) = desc { if **text != d { **text = d.to_string() } }
    let cur = color.0.alpha();
    let new = cur + (target - cur) * STATUS_FADE_SPEED * time.delta_secs();
    color.0 = Color::srgba(TOOLTIP_COLOR.0, TOOLTIP_COLOR.1, TOOLTIP_COLOR.2,
        if (new - target).abs() < 0.01 { target } else { new });
}
