// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod textures;
mod board;
mod inventory;
mod systems;

use bevy::prelude::*;
use constants::*;
use types::*;
use textures::*;
use board::*;
use inventory::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "protocol: play".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(BoardSize(3))
        .insert_resource(SelectedTool::default())
        .insert_resource(HoveredCell::default())
        .insert_resource(HiddenTileEntity::default())
        .insert_resource(GhostCell::default())
        .insert_resource(InventoryState { level: 1, direction: None, color_index: None })
        .insert_resource(PlacedSources::default())
        .insert_resource(PlayMode::default())
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, (
            animate_node_width,
            button_interaction,
            inventory_interaction,
            update_inventory_visuals.after(inventory_interaction),
            update_l3_availability.after(inventory_interaction),
            update_hovered_cell,
            update_ghost_and_highlight.after(update_hovered_cell),
            handle_tile_click.after(update_hovered_cell),
            animate_scale.after(update_ghost_and_highlight),
            cleanup_despawned.after(animate_scale),
            play_stop_interaction,
            adapt_camera,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    board_size: Res<BoardSize>,
) {
    let floor_texture = create_tile_texture(&mut images, TILE_TEX_SIZE, TILE_TEX_BORDER);
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture.clone()),
        ..default()
    });
    let floor_mesh = meshes.add(Cuboid::new(1.0, TILE_HEIGHT, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture),
        base_color: Color::srgba(1.0, 1.0, 1.0, GHOST_ALPHA),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: Color::srgba(DELETE_OVERLAY_COLOR.0, DELETE_OVERLAY_COLOR.1, DELETE_OVERLAY_COLOR.2, DELETE_OVERLAY_COLOR.3),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, 0.001, 1.02));
    let empty_marker_texture = create_empty_marker_texture(&mut images);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(empty_marker_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let empty_mesh = meshes.add(Cuboid::new(0.95, 0.001, 0.95));
    let highlight_texture = create_highlight_texture(&mut images);
    let highlight_material = materials.add(StandardMaterial {
        base_color_texture: Some(highlight_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let highlight_mesh = meshes.add(Cuboid::new(1.05, 0.001, 1.05));

    let source_symbol_texture = create_source_symbol_texture(&mut images, TILE_TEX_SIZE);
    let source_symbol_mesh = meshes.add(Cuboid::new(0.99, 0.001, 0.99));
    let source_symbol_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            base_color_texture: Some(source_symbol_texture.clone()),
            alpha_mode: AlphaMode::Mask(0.5),
            unlit: true,
            ..default()
        })
    }).collect();
    let ghost_symbol_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(r, g, b, GHOST_ALPHA),
            base_color_texture: Some(source_symbol_texture.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }).collect();

    let turn_symbol_texture = create_turn_symbol_texture(&mut images, TILE_TEX_SIZE);
    let turn_symbol_mesh = meshes.add(Cuboid::new(0.99, 0.001, 0.99));
    let turn_symbol_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            base_color_texture: Some(turn_symbol_texture.clone()),
            alpha_mode: AlphaMode::Mask(0.5),
            unlit: true,
            ..default()
        })
    }).collect();
    let ghost_turn_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(r, g, b, GHOST_ALPHA),
            base_color_texture: Some(turn_symbol_texture.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }).collect();

    let bot_mesh = meshes.add(Cuboid::new(BOT_SIZE, BOT_SIZE, BOT_SIZE));
    let eye_mesh = meshes.add(Cuboid::new(BOT_EYE_W, BOT_EYE_H, BOT_EYE_D));
    let eye_material = materials.add(StandardMaterial {
        base_color: Color::WHITE, unlit: true, ..default()
    });
    let bot_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() })
    }).collect();

    let assets = GameAssets {
        floor_mesh: floor_mesh.clone(), floor_material,
        empty_mesh, empty_material,
        ghost_floor_material: ghost_floor_material.clone(),
        ghost_delete_mesh: ghost_delete_mesh.clone(),
        ghost_delete_material: ghost_delete_material.clone(),
        highlight_mesh: highlight_mesh.clone(),
        highlight_material: highlight_material.clone(),
        source_symbol_mesh: source_symbol_mesh.clone(), source_symbol_materials,
        ghost_symbol_materials: ghost_symbol_materials.clone(),
        turn_symbol_mesh: turn_symbol_mesh.clone(), turn_symbol_materials,
        ghost_turn_materials: ghost_turn_materials.clone(),
        bot_mesh, eye_mesh, bot_materials, eye_material,
    };

    spawn_board(&mut commands, board_size.0, &assets);
    commands.insert_resource(assets);

    commands.spawn((
        Mesh3d(floor_mesh), MeshMaterial3d(ghost_floor_material),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), GhostPreview,
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(source_symbol_mesh),
            MeshMaterial3d(ghost_symbol_materials[0].clone()),
            Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0))
                .with_scale(Vec3::ZERO),
            GhostSymbolOverlay,
        ));
    });
    commands.spawn((
        Mesh3d(highlight_mesh), MeshMaterial3d(highlight_material),
        Transform::from_xyz(0.0, FLOOR_TOP_Y + 0.01, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), TileHighlight,
    ));
    commands.spawn((
        DirectionalLight { illuminance: 3000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let floor_tex_data = tile_texture_data(TEX_SIZE, TEX_BORDER);
    let floor_icon = create_isometric_icon(&mut images, &floor_tex_data, TEX_SIZE, ICON_SIZE);

    let source_tex_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, 0.0, [240, 240, 240, 255]);
    let source_icon = create_isometric_icon(&mut images, &source_tex_data, TEX_SIZE, ICON_SIZE);

    let delete_icon = create_delete_icon(&mut images);

    let turn_tex_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, 0.0, [240, 240, 240, 255]);
    let turn_icon = create_isometric_icon(&mut images, &turn_tex_data, TEX_SIZE, ICON_SIZE);

    // Source direction icons (L2)
    let source_north_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::North.rotation(), [240, 240, 240, 255]);
    let source_north = create_isometric_icon(&mut images, &source_north_data, TEX_SIZE, ICON_SIZE);
    let source_east_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::East.rotation(), [240, 240, 240, 255]);
    let source_east = create_isometric_icon(&mut images, &source_east_data, TEX_SIZE, ICON_SIZE);
    let source_south_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::South.rotation(), [240, 240, 240, 255]);
    let source_south = create_isometric_icon(&mut images, &source_south_data, TEX_SIZE, ICON_SIZE);
    let source_west_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::West.rotation(), [240, 240, 240, 255]);
    let source_west = create_isometric_icon(&mut images, &source_west_data, TEX_SIZE, ICON_SIZE);

    // Turn direction icons (L2)
    let turn_north_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::North.rotation(), [240, 240, 240, 255]);
    let turn_north = create_isometric_icon(&mut images, &turn_north_data, TEX_SIZE, ICON_SIZE);
    let turn_east_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::East.rotation(), [240, 240, 240, 255]);
    let turn_east = create_isometric_icon(&mut images, &turn_east_data, TEX_SIZE, ICON_SIZE);
    let turn_south_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::South.rotation(), [240, 240, 240, 255]);
    let turn_south = create_isometric_icon(&mut images, &turn_south_data, TEX_SIZE, ICON_SIZE);
    let turn_west_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -Direction::West.rotation(), [240, 240, 240, 255]);
    let turn_west = create_isometric_icon(&mut images, &turn_west_data, TEX_SIZE, ICON_SIZE);

    // Source color icons: 10 colors x 4 directions
    let mut source_color_icons = Vec::with_capacity(NUM_COLORS * 4);
    for ci in 0..NUM_COLORS {
        let (r, g, b) = SOURCE_COLORS[ci];
        let fill = color_to_u8(r, g, b);
        for dir in Direction::all() {
            let tex_data = source_texture_colored_data(TEX_SIZE, TEX_BORDER, -dir.rotation(), fill);
            source_color_icons.push(create_isometric_icon(&mut images, &tex_data, TEX_SIZE, ICON_SIZE));
        }
    }

    // Turn color icons: 10 colors x 4 directions
    let mut turn_color_icons = Vec::with_capacity(NUM_COLORS * 4);
    for ci in 0..NUM_COLORS {
        let (r, g, b) = SOURCE_COLORS[ci];
        let fill = color_to_u8(r, g, b);
        for dir in Direction::all() {
            let tex_data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -dir.rotation(), fill);
            turn_color_icons.push(create_isometric_icon(&mut images, &tex_data, TEX_SIZE, ICON_SIZE));
        }
    }

    let icons = InventoryIcons {
        floor: floor_icon.clone(), source: source_icon.clone(),
        turn: turn_icon.clone(), delete: delete_icon.clone(),
        source_north, source_east, source_south, source_west,
        source_color_icons,
        turn_north, turn_east, turn_south, turn_west,
        turn_color_icons,
    };
    commands.insert_resource(icons);

    // Board size controls
    let btn_node = Node {
        width: Val::Px(40.0), height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(2.0)),
        ..default()
    };
    let btn_color = Color::srgb(0.25, 0.25, 0.25);
    let text_style = TextFont { font_size: 24.0, ..default() };

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(10.0), top: Val::Px(10.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(4.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn((Button, btn_node.clone(), BackgroundColor(btn_color), BoardButton::Decrease))
            .with_child((Text::new("-"), text_style.clone(), TextColor(Color::WHITE)));
        parent.spawn(Node {
            width: Val::Px(70.0), justify_content: JustifyContent::Center, ..default()
        }).with_child((Text::new("3x3"), text_style.clone(), TextColor(Color::WHITE), BoardSizeText));
        parent.spawn((Button, btn_node, BackgroundColor(btn_color), BoardButton::Increase))
            .with_child((Text::new("+"), text_style, TextColor(Color::WHITE)));
    });

    // Play/Stop button
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(10.0), top: Val::Px(10.0),
        ..default()
    }).with_child((
        Button,
        Node {
            width: Val::Px(48.0), height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(SLOT_BG.0, SLOT_BG.1, SLOT_BG.2)),
        BorderColor(Color::srgba(BORDER_UNSELECTED.0, BORDER_UNSELECTED.1, BORDER_UNSELECTED.2, BORDER_UNSELECTED.3)),
        PlayStopButton,
        ImageNode::new(play_icon),
        PlayButtonImage,
    ));

    // Inventory bar
    let slot_bg = Color::srgb(SLOT_BG.0, SLOT_BG.1, SLOT_BG.2);
    let sn = Node {
        width: Val::Vw(SLOT_VW),
        height: Val::Vw(SLOT_HEIGHT_VW),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        ..default()
    };
    let spacer_font = TextFont { font_size: 14.0, ..default() };
    let spacer_color = TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
    let border_sel = Color::srgba(BORDER_SELECTED.0, BORDER_SELECTED.1, BORDER_SELECTED.2, BORDER_SELECTED.3);
    let border_unsel = Color::srgba(BORDER_UNSELECTED.0, BORDER_UNSELECTED.1, BORDER_UNSELECTED.2, BORDER_UNSELECTED.3);

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(20.0), width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Vw(0.6)),
                column_gap: Val::Vw(0.5),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(INVENTORY_BG.0, INVENTORY_BG.1, INVENTORY_BG.2, INVENTORY_BG.3)),
            InventoryContainer,
        )).with_children(|container| {
            container.spawn((
                Button, sn.clone(), BackgroundColor(slot_bg),
                BorderColor(border_sel), InventorySlot::Floor,
            )).with_children(|slot| {
                slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(floor_icon)));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            container.spawn((
                Button, sn.clone(), BackgroundColor(slot_bg),
                BorderColor(border_unsel), InventorySlot::Source,
            )).with_children(|slot| {
                slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(source_icon)));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            container.spawn((
                Button, sn.clone(), BackgroundColor(slot_bg),
                BorderColor(border_unsel), InventorySlot::Turn,
            )).with_children(|slot| {
                slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(turn_icon)));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            container.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vw(0.5),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ExpansionContainer,
            ));
            container.spawn((
                Button, sn, BackgroundColor(slot_bg),
                BorderColor(border_unsel), InventorySlot::Delete,
            )).with_children(|slot| {
                slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(delete_icon)));
                slot.spawn((Text::new(" "), spacer_font, spacer_color));
            });
        });
    });
}
