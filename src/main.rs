// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod textures;
mod gen_textures;
mod board;
mod ui_helpers;
mod slot_ui;
mod inventory;
mod systems;
mod simulation;
mod mat_helpers;
mod test_mode;
mod level_io;

use bevy::prelude::*;
use constants::*;
use types::*;
use textures::*;
use board::*;
use inventory::*;
use systems::*;
use simulation::*;
use ui_helpers::*;
use mat_helpers::*;
use test_mode::*;
use level_io::*;

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
        .insert_resource(DoorToggleCount::default())
        .insert_resource(OriginalDoorStates::default())
        .insert_resource(SimulationResult::default())
        .insert_resource(SavedBoardState::default())
        .insert_resource(SavedTestState::default())
        .insert_resource(TestInventory::default())
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, (
            animate_node_width, button_interaction, inventory_interaction,
            update_inventory_visuals.after(inventory_interaction),
            update_l3_availability.after(inventory_interaction),
            update_hovered_cell,
            update_ghost_and_highlight.after(update_hovered_cell),
            handle_tile_click.after(update_hovered_cell),
            animate_scale.after(update_ghost_and_highlight).after(move_bots),
            animate_ui_slides, cleanup_despawned.after(animate_scale),
        ))
        .add_systems(Update, (
            overlay_button_interaction,
            play_stop_interaction.after(overlay_button_interaction),
            move_bots.after(play_stop_interaction),
            toggle_doors.after(move_bots),
            check_simulation_result.after(move_bots),
            spawn_simulation_overlay.after(check_simulation_result),
            adapt_camera,
        ))
        .add_systems(Update, (
            mark_button_interaction, handle_mark_click.after(update_hovered_cell),
            test_button_interaction, stop_test_interaction.after(play_stop_interaction), reset_test_interaction,
            handle_test_tile_click.after(update_hovered_cell), test_inventory_interaction,
            sync_editor_buttons_visibility,
        ))
        .add_systems(Update, (
            save_button_interaction, save_dialog_input, save_dialog_buttons,
            load_button_interaction, load_dialog_buttons,
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
    let floor_material = materials.add(StandardMaterial { base_color_texture: Some(floor_texture.clone()), ..default() });
    let floor_mesh = meshes.add(Cuboid::new(1.0, TILE_HEIGHT, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture), base_color: Color::srgba(1.0, 1.0, 1.0, GHOST_ALPHA),
        alpha_mode: AlphaMode::Blend, ..default()
    });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: rgba(DELETE_OVERLAY_COLOR), alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, OVERLAY_MESH_THICKNESS, 1.02));
    let empty_marker_texture = create_empty_marker_texture(&mut images);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(empty_marker_texture), alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let empty_mesh = meshes.add(Cuboid::new(0.95, OVERLAY_MESH_THICKNESS, 0.95));
    let highlight_texture = create_highlight_texture(&mut images);
    let highlight_material = materials.add(StandardMaterial {
        base_color_texture: Some(highlight_texture), alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let highlight_mesh = meshes.add(Cuboid::new(1.05, OVERLAY_MESH_THICKNESS, 1.05));
    let marker_texture = create_inv_marker_texture(&mut images);
    let marker_material = materials.add(StandardMaterial {
        base_color_texture: Some(marker_texture), alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let marker_mesh = meshes.add(Cuboid::new(1.03, OVERLAY_MESH_THICKNESS, 1.03));

    // Load symbol textures from files (editable PNGs, loaded synchronously)
    let sym_mesh = meshes.add(Cuboid::new(0.99, OVERLAY_MESH_THICKNESS, 0.99));
    let (source_symbol_materials, ghost_symbol_materials, _, _) = load_tile_mats(&mut materials, &mut images, "source");
    let (goal_symbol_materials, ghost_goal_materials, _, _) = load_tile_mats(&mut materials, &mut images, "goal");
    let (mut turn_symbol_materials, mut ghost_turn_materials, tb, tm) = load_tile_mats(&mut materials, &mut images, "turn");
    add_grey_mat(&mut materials, &mut turn_symbol_materials, &mut ghost_turn_materials, &tb, &tm);
    let (turnbut_symbol_materials, ghost_turnbut_materials, _, _) = load_tile_mats(&mut materials, &mut images, "turnbut");
    let (mut bounce_symbol_materials, mut ghost_bounce_materials, bb, bm) = load_tile_mats(&mut materials, &mut images, "bounce");
    add_grey_mat(&mut materials, &mut bounce_symbol_materials, &mut ghost_bounce_materials, &bb, &bm);
    let (bouncebot_symbol_materials, ghost_bouncebot_materials, _, _) = load_tile_mats(&mut materials, &mut images, "bouncebut");

    let (mut teleport_symbol_materials, mut ghost_teleport_materials) = (Vec::new(), Vec::new());
    for num in 0..NUM_TELEPORTS {
        let (base, mask) = create_teleport_tile_textures(&mut images, TILE_TEX_SIZE, num);
        let (m, g) = make_grey_mat(&mut materials, base, mask);
        teleport_symbol_materials.push(m); ghost_teleport_materials.push(g);
    }

    // Door/Switch materials (grey only, file-based)
    let dob = load_png_texture(&mut images, "assets/textures/door_open_base.png", true);
    let dom = load_png_texture(&mut images, "assets/textures/door_open_mask.png", false);
    let (door_open_material, ghost_door_open_material) = make_grey_mat(&mut materials, dob, dom);
    let dcb = load_png_texture(&mut images, "assets/textures/door_closed_base.png", true);
    let dcm = load_png_texture(&mut images, "assets/textures/door_closed_mask.png", false);
    let (door_closed_material, ghost_door_closed_material) = make_grey_mat(&mut materials, dcb, dcm);
    let sb = load_png_texture(&mut images, "assets/textures/switch_base.png", true);
    let sm = load_png_texture(&mut images, "assets/textures/switch_mask.png", false);
    let (switch_material, ghost_switch_material) = make_grey_mat(&mut materials, sb, sm);

    let bot_mesh = meshes.add(Cuboid::new(BOT_SIZE, BOT_SIZE, BOT_SIZE));
    let eye_mesh = meshes.add(Cuboid::new(BOT_EYE_W, BOT_EYE_H, BOT_EYE_D));
    let eye_material = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    let bot_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)|
        materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() })).collect();

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
        door_open_material, door_closed_material,
        ghost_door_open_material, ghost_door_closed_material,
        switch_material, ghost_switch_material,
        marker_mesh, marker_material,
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
        Transform::from_xyz(0.0, FLOOR_TOP_Y + HIGHLIGHT_Y_OFFSET, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), TileHighlight,
    ));
    commands.spawn((
        DirectionalLight { illuminance: LIGHT_ILLUMINANCE, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, LIGHT_ELEVATION, LIGHT_AZIMUTH, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let floor_tex_data = tile_texture_data(TEX_SIZE, TEX_BORDER);
    let floor_icon = create_isometric_icon(&mut images, &floor_tex_data, TEX_SIZE, ICON_SIZE);

    let white = ICON_WHITE;
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

    let goal_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| {
        icon(&mut images, &goal_texture_colored_data(TEX_SIZE, TEX_BORDER, color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2)))
    }).collect();

    let bounce_icon = icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, white, false));
    let bouncebot_icon = icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, white, true));
    let cfill = |ci: usize| color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
    let mut bounce_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci|
        icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, cfill(ci), false))).collect();
    bounce_color_icons.push(icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, grey_fill, false)));
    let bouncebot_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci|
        icon(&mut images, &bounce_texture_colored_data(TEX_SIZE, TEX_BORDER, cfill(ci), true))).collect();

    // Door/Switch icons
    let door_icon = icon(&mut images, &door_texture_data(TEX_SIZE, TEX_BORDER, white, true));
    let door_open_icon = icon(&mut images, &door_texture_data(TEX_SIZE, TEX_BORDER, grey_fill, false));
    let door_closed_icon = icon(&mut images, &door_texture_data(TEX_SIZE, TEX_BORDER, grey_fill, true));
    let switch_icon = icon(&mut images, &switch_texture_data(TEX_SIZE, TEX_BORDER, white));

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
        door: door_icon.clone(), door_open: door_open_icon, door_closed: door_closed_icon,
        switch: switch_icon.clone(),
    };
    commands.insert_resource(icons);

    // Top controls
    let bc = btn_bg();
    let btn = Node { width: Val::Px(TOP_BTN_SIZE), height: Val::Px(TOP_BTN_SIZE),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(BTN_MARGIN)), ..default() };
    let ts = TextFont { font_size: TOP_BTN_FONT, ..default() };
    let mut tbtn = text_btn_node();
    tbtn.margin = UiRect::left(Val::Px(TEXT_BTN_LEFT_MARGIN));
    let tfs = TextFont { font_size: LABEL_FONT, ..default() };
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(-50.0),
        flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: Val::Px(4.0), ..default()
    }, UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false })).with_children(|p| {
        p.spawn((Button, btn.clone(), BackgroundColor(bc), BoardButton::Decrease))
            .with_child((Text::new("-"), ts.clone(), TextColor(Color::WHITE)));
        p.spawn(Node { width: Val::Px(70.0), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new("3x3"), ts.clone(), TextColor(Color::WHITE), BoardSizeText));
        p.spawn((Button, btn, BackgroundColor(bc), BoardButton::Increase))
            .with_child((Text::new("+"), ts, TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), BorderColor(border_unsel()), MarkButton, MarkButtonImage))
            .with_child((Text::new("Mark"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), TestButton))
            .with_child((Text::new("Test"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), SaveButton))
            .with_child((Text::new("Save"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn, BackgroundColor(bc), LoadButton))
            .with_child((Text::new("Load"), tfs, TextColor(Color::WHITE)));
    });

    // Play/Stop button
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });
    commands.spawn((Node { position_type: PositionType::Absolute, right: Val::Px(10.0), top: Val::Px(-60.0), ..default()
    }, UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false })).with_child((
        Button, Node { width: Val::Px(PLAY_BTN_SIZE), height: Val::Px(PLAY_BTN_SIZE), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, border: UiRect::all(Val::Px(PLAY_BTN_BORDER)), ..default() },
        BackgroundColor(slot_bg()),
        BorderColor(border_unsel()),
        PlayStopButton, ImageNode::new(play_icon), PlayButtonImage,
    ));

    // Inventory bar
    let sn = slot_node();

    let l1_slots: Vec<(InventorySlot, Handle<Image>, bool)> = vec![
        (InventorySlot::Floor, floor_icon, true),
        (InventorySlot::Source, source_icon, false),
        (InventorySlot::Goal, goal_icon, false),
        (InventorySlot::Turn, turn_icon, false),
        (InventorySlot::TurnBut, turnbut_icon, false),
        (InventorySlot::Teleport, teleport_icon, false),
        (InventorySlot::Bounce, bounce_icon, false),
        (InventorySlot::BounceBut, bouncebot_icon, false),
        (InventorySlot::Door, door_icon, false),
        (InventorySlot::Switch, switch_icon, false),
    ];

    commands.spawn((Node {
        position_type: PositionType::Absolute, bottom: Val::Px(INV_SLIDE_HIDE),
        width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default()
    }, InventoryContainer, UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false },
    )).with_children(|parent| {
        parent.spawn((
            Node { flex_direction: FlexDirection::Row, padding: UiRect::all(Val::Vw(INVENTORY_PAD_VW)),
                column_gap: Val::Vw(INVENTORY_GAP_VW), align_items: AlignItems::Center, ..default() },
            BackgroundColor(rgba(INVENTORY_BG)),
        )).with_children(|container| {
            let sf = TextFont { font_size: COUNT_FONT, ..default() };
            let sc = TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
            for (slot_type, icon_handle, selected) in &l1_slots {
                container.spawn((Button, sn.clone(), BackgroundColor(slot_bg()), border_for(*selected), *slot_type))
                    .with_children(|slot| {
                        slot.spawn((icon_node(), ImageNode::new(icon_handle.clone())));
                        slot.spawn((Text::new(" "), sf.clone(), sc));
                    });
            }
            container.spawn((
                Node { flex_direction: FlexDirection::Row, column_gap: Val::Vw(INVENTORY_GAP_VW),
                    align_items: AlignItems::Center, ..default() },
                ExpansionContainer,
            ));
            container.spawn((Button, sn, BackgroundColor(slot_bg()), border_for(false), InventorySlot::Delete))
                .with_children(|slot| {
                    slot.spawn((icon_node(), ImageNode::new(delete_icon)));
                    slot.spawn((Text::new(" "), sf, sc));
                });
        });
    });
}
