// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod textures;
mod gen_textures;
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
    gen_textures::ensure_textures();
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
        .insert_resource(InventoryState { level: 1, direction: None, color_index: None, last_placed_color: None })
        .insert_resource(PlacedSources::default())
        .insert_resource(PlacedGoals::default())
        .insert_resource(PlacedTeleports::default())
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
            animate_scale.after(update_ghost_and_highlight).after(move_bots),
            cleanup_despawned.after(animate_scale),
            play_stop_interaction,
            move_bots.after(play_stop_interaction),
            adapt_camera,
        ))
        .run();
}

fn load_tile_mats(
    materials: &mut Assets<StandardMaterial>, images: &mut Assets<Image>, name: &str,
) -> (Vec<Handle<StandardMaterial>>, Vec<Handle<StandardMaterial>>, Handle<Image>, Handle<Image>) {
    let base = load_png_texture(images, &format!("assets/textures/{name}_base.png"), true);
    let mask = load_png_texture(images, &format!("assets/textures/{name}_mask.png"), false);
    let mats = SOURCE_COLORS.iter().map(|&(r, g, b)| materials.add(StandardMaterial {
        base_color: Color::WHITE, base_color_texture: Some(base.clone()),
        alpha_mode: AlphaMode::Mask(0.5), emissive: LinearRgba::from(Color::srgb(r, g, b)),
        emissive_texture: Some(mask.clone()), reflectance: 0.0, ..default()
    })).collect();
    let ghosts = SOURCE_COLORS.iter().map(|&(r, g, b)| materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, GHOST_ALPHA), base_color_texture: Some(mask.clone()),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    })).collect();
    (mats, ghosts, base, mask)
}

fn add_grey_mat(
    materials: &mut Assets<StandardMaterial>, mats: &mut Vec<Handle<StandardMaterial>>,
    ghosts: &mut Vec<Handle<StandardMaterial>>, base: &Handle<Image>, mask: &Handle<Image>,
) {
    let (r, g, b) = GREY_COLOR;
    mats.push(materials.add(StandardMaterial {
        base_color: Color::WHITE, base_color_texture: Some(base.clone()),
        alpha_mode: AlphaMode::Mask(0.5), emissive: LinearRgba::from(Color::srgb(r, g, b)),
        emissive_texture: Some(mask.clone()), reflectance: 0.0, ..default()
    }));
    ghosts.push(materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, GHOST_ALPHA), base_color_texture: Some(mask.clone()),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    }));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    board_size: Res<BoardSize>,
) {
    // Floor texture: procedural (needed immediately at startup)
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

    // Load symbol textures from files (editable PNGs, loaded synchronously)
    let sym_mesh = meshes.add(Cuboid::new(0.99, 0.001, 0.99));
    let (source_symbol_materials, ghost_symbol_materials, _, _) = load_tile_mats(&mut materials, &mut images, "source");
    let (goal_symbol_materials, ghost_goal_materials, _, _) = load_tile_mats(&mut materials, &mut images, "goal");
    let (mut turn_symbol_materials, mut ghost_turn_materials, tb, tm) = load_tile_mats(&mut materials, &mut images, "turn");
    add_grey_mat(&mut materials, &mut turn_symbol_materials, &mut ghost_turn_materials, &tb, &tm);
    let (turnbut_symbol_materials, ghost_turnbut_materials, _, _) = load_tile_mats(&mut materials, &mut images, "turnbut");
    let (mut bounce_symbol_materials, mut ghost_bounce_materials, bb, bm) = load_tile_mats(&mut materials, &mut images, "bounce");
    add_grey_mat(&mut materials, &mut bounce_symbol_materials, &mut ghost_bounce_materials, &bb, &bm);
    let (bouncebot_symbol_materials, ghost_bouncebot_materials, _, _) = load_tile_mats(&mut materials, &mut images, "bouncebut");

    // Teleport materials (procedural, grey)
    let (gr, gg, gb) = GREY_COLOR;
    let mut teleport_symbol_materials = Vec::new();
    let mut ghost_teleport_materials = Vec::new();
    for num in 0..NUM_TELEPORTS {
        let (base, mask) = create_teleport_tile_textures(&mut images, TILE_TEX_SIZE, num);
        teleport_symbol_materials.push(materials.add(StandardMaterial {
            base_color: Color::WHITE, base_color_texture: Some(base),
            alpha_mode: AlphaMode::Mask(0.5), emissive: LinearRgba::from(Color::srgb(gr, gg, gb)),
            emissive_texture: Some(mask.clone()), reflectance: 0.0, ..default()
        }));
        ghost_teleport_materials.push(materials.add(StandardMaterial {
            base_color: Color::srgba(gr, gg, gb, GHOST_ALPHA), base_color_texture: Some(mask),
            alpha_mode: AlphaMode::Blend, unlit: true, ..default()
        }));
    }

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
        source_symbol_mesh: sym_mesh.clone(), source_symbol_materials,
        ghost_symbol_materials: ghost_symbol_materials.clone(),
        goal_symbol_mesh: sym_mesh.clone(), goal_symbol_materials,
        ghost_goal_materials: ghost_goal_materials.clone(),
        turn_symbol_mesh: sym_mesh.clone(), turn_symbol_materials,
        ghost_turn_materials: ghost_turn_materials.clone(),
        turnbut_symbol_mesh: sym_mesh.clone(), turnbut_symbol_materials,
        ghost_turnbut_materials: ghost_turnbut_materials.clone(),
        teleport_symbol_materials, ghost_teleport_materials: ghost_teleport_materials.clone(),
        bounce_symbol_materials, ghost_bounce_materials: ghost_bounce_materials.clone(),
        bouncebot_symbol_materials, ghost_bouncebot_materials: ghost_bouncebot_materials.clone(),
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
            Mesh3d(sym_mesh),
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

    let white = [240, 240, 240, 255];
    let icon = |images: &mut Assets<Image>, data: &[u8]| create_isometric_icon(images, data, TEX_SIZE, ICON_SIZE);
    let source_icon = icon(&mut images, &source_texture_colored_data(TEX_SIZE, TEX_BORDER, 0.0, white));
    let goal_icon = icon(&mut images, &goal_texture_colored_data(TEX_SIZE, TEX_BORDER, white));
    let turn_icon = icon(&mut images, &turn_texture_colored_data(TEX_SIZE, TEX_BORDER, 0.0, white));
    let turnbut_icon = icon(&mut images, &turnbut_texture_colored_data(TEX_SIZE, TEX_BORDER, 0.0, white));
    let delete_icon = create_delete_icon(&mut images);

    // Direction icons (L2) for source and turn
    let dir_icon = |images: &mut Assets<Image>, tex_fn: fn(u32, u32, f32, [u8; 4]) -> Vec<u8>| {
        Direction::all().map(|d| {
            let data = tex_fn(TEX_SIZE, TEX_BORDER, -d.rotation(), white);
            create_isometric_icon(images, &data, TEX_SIZE, ICON_SIZE)
        })
    };
    let [source_north, source_east, source_south, source_west] = dir_icon(&mut images, source_texture_colored_data);
    let [turn_north, turn_east, turn_south, turn_west] = dir_icon(&mut images, turn_texture_colored_data);
    let turnbut_dir_icons = dir_icon(&mut images, turnbut_texture_colored_data);

    // Color icons per direction
    let color_dir_icons = |images: &mut Assets<Image>, tex_fn: fn(u32, u32, f32, [u8; 4]) -> Vec<u8>| {
        (0..NUM_COLORS).flat_map(|ci| {
            let fill = color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
            Direction::all().map(move |d| (fill, d))
        }).map(|(fill, d)| {
            let data = tex_fn(TEX_SIZE, TEX_BORDER, -d.rotation(), fill);
            create_isometric_icon(images, &data, TEX_SIZE, ICON_SIZE)
        }).collect()
    };
    let source_color_icons = color_dir_icons(&mut images, source_texture_colored_data);
    let mut turn_color_icons: Vec<_> = color_dir_icons(&mut images, turn_texture_colored_data);
    // Grey turn icons (appended at index NUM_COLORS * 4)
    let grey_fill = color_to_u8(GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2);
    for d in Direction::all() {
        let data = turn_texture_colored_data(TEX_SIZE, TEX_BORDER, -d.rotation(), grey_fill);
        turn_color_icons.push(create_isometric_icon(&mut images, &data, TEX_SIZE, ICON_SIZE));
    }
    let turnbut_color_icons = color_dir_icons(&mut images, turnbut_texture_colored_data);

    // Teleport icons
    let teleport_icon = icon(&mut images, &teleport_texture_colored_data(TEX_SIZE, TEX_BORDER, 0, white));
    let teleport_num_icons: Vec<_> = (0..NUM_TELEPORTS).map(|n| {
        icon(&mut images, &teleport_texture_colored_data(TEX_SIZE, TEX_BORDER, n, grey_fill))
    }).collect();

    // Goal color icons (no directions)
    let goal_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| {
        let fill = color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
        icon(&mut images, &goal_texture_colored_data(TEX_SIZE, TEX_BORDER, fill))
    }).collect();

    // Bounce/BounceBut icons (no directions, like Goal)
    let bounce_icon = icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, white, false));
    let bouncebot_icon = icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, white, true));
    let mut bounce_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| {
        let fill = color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
        icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, fill, false))
    }).collect();
    bounce_color_icons.push(icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, grey_fill, false)));
    let bouncebot_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| {
        let fill = color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
        icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, fill, true))
    }).collect();

    let icons = InventoryIcons {
        floor: floor_icon.clone(), source: source_icon.clone(),
        goal: goal_icon.clone(), turn: turn_icon.clone(), delete: delete_icon.clone(),
        source_north, source_east, source_south, source_west,
        source_color_icons, goal_color_icons,
        turn_north, turn_east, turn_south, turn_west,
        turn_color_icons,
        turnbut: turnbut_icon.clone(), turnbut_dir_icons, turnbut_color_icons,
        teleport: teleport_icon.clone(), teleport_num_icons,
        bounce: bounce_icon.clone(), bounce_color_icons,
        bouncebot: bouncebot_icon.clone(), bouncebot_color_icons,
    };
    commands.insert_resource(icons);

    // Board size controls
    let btn = Node { width: Val::Px(40.0), height: Val::Px(40.0), justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, margin: UiRect::all(Val::Px(2.0)), ..default() };
    let bc = Color::srgb(0.25, 0.25, 0.25);
    let ts = TextFont { font_size: 24.0, ..default() };
    commands.spawn(Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(10.0),
        flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: Val::Px(4.0), ..default()
    }).with_children(|p| {
        p.spawn((Button, btn.clone(), BackgroundColor(bc), BoardButton::Decrease))
            .with_child((Text::new("-"), ts.clone(), TextColor(Color::WHITE)));
        p.spawn(Node { width: Val::Px(70.0), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new("3x3"), ts.clone(), TextColor(Color::WHITE), BoardSizeText));
        p.spawn((Button, btn, BackgroundColor(bc), BoardButton::Increase))
            .with_child((Text::new("+"), ts, TextColor(Color::WHITE)));
    });

    // Play/Stop button
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });
    commands.spawn(Node { position_type: PositionType::Absolute, right: Val::Px(10.0), top: Val::Px(10.0), ..default()
    }).with_child((
        Button, Node { width: Val::Px(48.0), height: Val::Px(48.0), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, border: UiRect::all(Val::Px(3.0)), ..default() },
        BackgroundColor(Color::srgb(SLOT_BG.0, SLOT_BG.1, SLOT_BG.2)),
        BorderColor(Color::srgba(BORDER_UNSELECTED.0, BORDER_UNSELECTED.1, BORDER_UNSELECTED.2, BORDER_UNSELECTED.3)),
        PlayStopButton, ImageNode::new(play_icon), PlayButtonImage,
    ));

    // Inventory bar
    let slot_bg = Color::srgb(SLOT_BG.0, SLOT_BG.1, SLOT_BG.2);
    let sn = Node {
        width: Val::Vw(SLOT_VW), height: Val::Vw(SLOT_HEIGHT_VW),
        border: UiRect::all(Val::Px(2.0)), justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(), ..default()
    };
    let border_sel = Color::srgba(BORDER_SELECTED.0, BORDER_SELECTED.1, BORDER_SELECTED.2, BORDER_SELECTED.3);
    let border_unsel = Color::srgba(BORDER_UNSELECTED.0, BORDER_UNSELECTED.1, BORDER_UNSELECTED.2, BORDER_UNSELECTED.3);

    let l1_slots: Vec<(InventorySlot, Handle<Image>, bool)> = vec![
        (InventorySlot::Floor, floor_icon, true),
        (InventorySlot::Source, source_icon, false),
        (InventorySlot::Goal, goal_icon, false),
        (InventorySlot::Turn, turn_icon, false),
        (InventorySlot::TurnBut, turnbut_icon, false),
        (InventorySlot::Teleport, teleport_icon, false),
        (InventorySlot::Bounce, bounce_icon, false),
        (InventorySlot::BounceBut, bouncebot_icon, false),
    ];

    commands.spawn(Node {
        position_type: PositionType::Absolute, bottom: Val::Px(20.0),
        width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default()
    }).with_children(|parent| {
        parent.spawn((
            Node { flex_direction: FlexDirection::Row, padding: UiRect::all(Val::Vw(0.6)),
                column_gap: Val::Vw(0.5), align_items: AlignItems::Center, ..default() },
            BackgroundColor(Color::srgba(INVENTORY_BG.0, INVENTORY_BG.1, INVENTORY_BG.2, INVENTORY_BG.3)),
            InventoryContainer,
        )).with_children(|container| {
            let sf = TextFont { font_size: 14.0, ..default() };
            let sc = TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
            for (slot_type, icon_handle, selected) in &l1_slots {
                let border = if *selected { border_sel } else { border_unsel };
                container.spawn((Button, sn.clone(), BackgroundColor(slot_bg), BorderColor(border), *slot_type))
                    .with_children(|slot| {
                        slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(icon_handle.clone())));
                        slot.spawn((Text::new(" "), sf.clone(), sc));
                    });
            }
            container.spawn((
                Node { flex_direction: FlexDirection::Row, column_gap: Val::Vw(0.5),
                    align_items: AlignItems::Center, ..default() },
                ExpansionContainer,
            ));
            container.spawn((Button, sn, BackgroundColor(slot_bg), BorderColor(border_unsel), InventorySlot::Delete))
                .with_children(|slot| {
                    slot.spawn((Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() }, ImageNode::new(delete_icon)));
                    slot.spawn((Text::new(" "), sf, sc));
                });
        });
    });
}
