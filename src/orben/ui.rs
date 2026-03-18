// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Spawn the full game UI.
pub fn spawn_game_ui(
    mut commands: Commands,
    font: Res<OrbenFont>,
) {
    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    // Star background
    spawn_stars(&mut commands);

    // Main layout: vertical stack
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        padding: UiRect::all(Val::Px(16.0)),
        ..default()
    }).with_children(|root| {
        // Top section: NPC hand + treasure
        root.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.0),
            ..default()
        }).with_children(|top| {
            // NPC treasure
            spawn_treasure_panel(top, &tf, "NPC", true);
            // NPC hand (face down)
            spawn_hand_area(top, true);
            // Deck count
            top.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Deck: 26"),
                    tf(SMALL_FONT_O),
                    TextColor(Color::srgba(0.6, 0.65, 0.7, 0.7)),
                    DeckCountText,
                ));
            });
        });

        // Middle: status + table
        root.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        }).with_children(|mid| {
            // Status message
            mid.spawn((
                Text::new(""),
                tf(STATUS_FONT_O),
                TextColor(Color::srgba(1.0, 0.9, 0.5, 0.9)),
                StatusText,
            ));
            // Ronda message
            mid.spawn((
                Text::new(""),
                tf(LABEL_FONT_O),
                TextColor(Color::srgba(1.0, 0.85, 0.2, 0.0)),
                RondaText,
            ));
            // Table area
            spawn_table_area(mid);
        });

        // Bottom section: Player hand + treasure
        root.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        }).with_children(|bottom| {
            // Se cayo timer bar
            spawn_se_cayo_bar(bottom);
            // Player hand + treasure
            bottom.spawn(Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(24.0),
                ..default()
            }).with_children(|row| {
                spawn_treasure_panel(row, &tf, "You", false);
                spawn_hand_area(row, false);
            });
        });
    });

    // Mesa limpia flash overlay (hidden by default)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(
            MESA_LIMPIA_COLOR.0, MESA_LIMPIA_COLOR.1,
            MESA_LIMPIA_COLOR.2, 0.0,
        )),
        MesaLimpiaFlash,
    ));
}

fn spawn_treasure_panel(
    parent: &mut ChildSpawnerCommands,
    tf: &impl Fn(f32) -> TextFont,
    label: &str,
    is_npc: bool,
) {
    parent.spawn((
        Node {
            width: Val::Px(TREASURE_W),
            height: Val::Px(TREASURE_H),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            TREASURE_BG.0, TREASURE_BG.1, TREASURE_BG.2, TREASURE_BG.3,
        )),
    )).with_children(|panel| {
        panel.spawn((
            Text::new(label),
            tf(SMALL_FONT_O),
            TextColor(Color::srgba(0.6, 0.65, 0.7, 0.8)),
        ));
        // Treasure count
        let treasure_id = panel.spawn((
            Text::new("0"),
            tf(TREASURE_FONT),
            TextColor(Color::srgba(0.9, 0.85, 0.4, 1.0)),
        )).id();
        if is_npc {
            panel.commands().entity(treasure_id).insert(NpcTreasureText);
        } else {
            panel.commands().entity(treasure_id).insert(PlayerTreasureText);
        }
        // Captured orbs count
        let cap_id = panel.spawn((
            Text::new("(0 orbs)"),
            tf(SMALL_FONT_O),
            TextColor(Color::srgba(0.5, 0.55, 0.6, 0.7)),
        )).id();
        if is_npc {
            panel.commands().entity(cap_id).insert(NpcCapturedText);
        } else {
            panel.commands().entity(cap_id).insert(PlayerCapturedText);
        }
    });
}

fn spawn_hand_area(parent: &mut ChildSpawnerCommands, is_npc: bool) {
    let id = parent.spawn((
        Node {
            width: Val::Px(HAND_PANEL_W),
            height: Val::Px(HAND_PANEL_H),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(ORB_GAP),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            HAND_BG.0, HAND_BG.1, HAND_BG.2, HAND_BG.3,
        )),
    )).id();
    if is_npc {
        parent.commands().entity(id).insert(NpcHandArea);
    } else {
        parent.commands().entity(id).insert(PlayerHandArea);
    }
}

fn spawn_table_area(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            min_width: Val::Px(TABLE_PANEL_W),
            min_height: Val::Px(TABLE_PANEL_H),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(ORB_GAP),
            padding: UiRect::all(Val::Px(12.0)),
            border_radius: BorderRadius::all(Val::Px(TABLE_CORNER)),
            border: UiRect::all(Val::Px(1.5)),
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(ORB_GAP),
            ..default()
        },
        BackgroundColor(Color::srgba(
            TABLE_BG.0, TABLE_BG.1, TABLE_BG.2, TABLE_BG.3,
        )),
        BorderColor::all(Color::srgba(
            TABLE_BORDER.0, TABLE_BORDER.1, TABLE_BORDER.2, TABLE_BORDER.3,
        )),
        TableArea,
    ));
}

fn spawn_se_cayo_bar(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Px(300.0),
            height: Val::Px(6.0),
            border_radius: BorderRadius::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.3)),
        SeCayoTimer,
        Visibility::Hidden,
    )).with_children(|bar_bg| {
        bar_bg.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 0.4, 0.3, 0.9)),
            SeCayoTimerBar,
        ));
    });
}

fn spawn_stars(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..STAR_COUNT_O {
        let x = rng.gen_range(0.0..100.0_f32);
        let y = rng.gen_range(0.0..100.0_f32);
        let size = rng.gen_range(1.0..3.0_f32);
        let alpha = rng.gen_range(0.1..0.4_f32);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(x),
                top: Val::Percent(y),
                width: Val::Px(size),
                height: Val::Px(size),
                border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.7, 0.75, 1.0, alpha)),
            StarDot,
        ));
    }
}

/// Spawn an orb UI node as a child of the given parent entity.
pub fn spawn_orb_node(
    commands: &mut Commands,
    parent: Entity,
    orb: &Orb,
    index: usize,
    hand: OrbHand,
    font: &Handle<Font>,
    is_selected: bool,
) {
    let (r, g, b) = orb.color.rgb();
    let border_color = if is_selected {
        Color::srgba(SELECTED_BORDER.0, SELECTED_BORDER.1,
                     SELECTED_BORDER.2, SELECTED_BORDER.3)
    } else {
        Color::srgba(r * 0.6, g * 0.6, b * 0.6, 0.5)
    };

    let child = commands.spawn((
        Button,
        Node {
            width: Val::Px(ORB_SIZE),
            height: Val::Px(ORB_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(ORB_BORDER)),
            border_radius: BorderRadius::all(Val::Px(ORB_CORNER)),
            ..default()
        },
        BackgroundColor(Color::srgba(r * 0.25, g * 0.25, b * 0.25, 0.9)),
        BorderColor::all(border_color),
        BoxShadow::new(
            Color::srgba(r, g, b, 0.3),
            Val::ZERO, Val::ZERO,
            Val::Px(4.0), Val::Px(8.0),
        ),
        OrbNode { hand, index },
    )).with_children(|orb_parent| {
        orb_parent.spawn((
            Text::new(format!("{}", orb.value)),
            TextFont { font: font.clone(), font_size: ORB_FONT, ..default() },
            TextColor(Color::srgb(r, g, b)),
        ));
    }).id();

    commands.entity(parent).add_children(&[child]);
}

/// Spawn a face-down orb (NPC hand).
pub fn spawn_back_orb(
    commands: &mut Commands,
    parent: Entity,
    index: usize,
    font: &Handle<Font>,
) {
    let child = commands.spawn((
        Node {
            width: Val::Px(ORB_SIZE),
            height: Val::Px(ORB_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(ORB_BORDER)),
            border_radius: BorderRadius::all(Val::Px(ORB_CORNER)),
            ..default()
        },
        BackgroundColor(Color::srgb(
            ORB_BACK_COLOR.0, ORB_BACK_COLOR.1, ORB_BACK_COLOR.2,
        )),
        BorderColor::all(Color::srgba(0.25, 0.26, 0.32, 0.5)),
        NpcHandSlot(index),
    )).with_children(|orb_parent| {
        orb_parent.spawn((
            Text::new("?"),
            TextFont { font: font.clone(), font_size: ORB_FONT, ..default() },
            TextColor(Color::srgba(0.4, 0.42, 0.5, 0.6)),
        ));
    }).id();

    commands.entity(parent).add_children(&[child]);
}

/// Rebuild all visual orb nodes to match game state.
pub fn rebuild_orbs(
    commands: &mut Commands,
    state: &OrbGameState,
    font: &Handle<Font>,
    player_area: Entity,
    npc_area: Entity,
    table_area: Entity,
) {
    // Clear existing children
    commands.entity(player_area).despawn_related::<Children>();
    commands.entity(npc_area).despawn_related::<Children>();
    commands.entity(table_area).despawn_related::<Children>();

    // Player hand
    for (i, orb) in state.player_hand.iter().enumerate() {
        let selected = state.selected_orb == Some(i);
        spawn_orb_node(commands, player_area, orb, i, OrbHand::Player, font, selected);
    }

    // NPC hand (face down)
    for i in 0..state.npc_hand.len() {
        spawn_back_orb(commands, npc_area, i, font);
    }

    // Table orbs
    for (i, orb) in state.table.iter().enumerate() {
        spawn_orb_node(commands, table_area, orb, i, OrbHand::Table, font, false);
    }
}
