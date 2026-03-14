// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::collapsible_if)]

mod constants; mod types; mod textures; mod gen_textures; mod board;
mod ui_helpers; mod slot_ui; mod inventory; mod systems; mod simulation;
mod bot_formation; mod mat_helpers; mod test_mode; mod level_io; mod save_dialog;
mod level_gen_sim; mod level_gen_tiles; mod level_gen_algo; mod level_gen_ui; mod level_gen_interact;
#[cfg(feature = "player")] mod player;
#[cfg(feature = "gathering")] mod gathering;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use constants::*;
use types::*;
use textures::*;
use board::*;
use inventory::*;
use systems::*;
use simulation::*;
use bot_formation::*;
use ui_helpers::*;
use mat_helpers::*;
use test_mode::*;
use level_io::*;
use save_dialog::*;
use level_gen_algo::*;
use level_gen_ui::*;
use level_gen_interact::*;

fn main() {
    #[cfg(feature = "gathering")]
    {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "The Gathering".into(), ..default() }), ..default()
        }));
        app.set_error_handler(bevy::ecs::error::warn);
        gathering::build_app(&mut app);
        app.run();
        return;
    }

    gen_textures::ensure_textures();
    let title = if cfg!(feature = "player") { "protocol: play (player)" } else { "protocol: play" };
    let mut app = App::new();
    app.set_error_handler(bevy::ecs::error::warn);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: title.into(), ..default() }), ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(CLEAR_COLOR.0, CLEAR_COLOR.1, CLEAR_COLOR.2)))
        .insert_resource(GlobalAmbientLight { color: Color::srgb(AMBIENT_COLOR.0, AMBIENT_COLOR.1, AMBIENT_COLOR.2), brightness: AMBIENT_BRIGHTNESS, ..default() })
        .insert_resource(BoardSize(3))
        .insert_resource(SelectedTool::default()).insert_resource(HoveredCell::default()).insert_resource(HiddenTileEntity::default())
        .insert_resource(GhostCell::default())
        .insert_resource(InventoryState { level: 1, direction: None, color_index: None, last_placed_color: None })
        .insert_resource(PlayMode::default()).insert_resource(DoorToggleCount::default()).insert_resource(OriginalDoorStates::default())
        .insert_resource(SimulationResult::default()).insert_resource(PrevTileCounts::default())
        .insert_resource(SavedBoardState::default()).insert_resource(SavedTestState::default())
        .insert_resource(TestInventory::default()).insert_resource(LevelValidated::default())
        .insert_resource(CursorBlinkTimer::default()).insert_resource(LoadedLevelName::default()).insert_resource(PendingSave::default()).insert_resource(ScrollbarDrag::default())
        .insert_resource(GenSettings::default()).insert_resource(GeneratorState::default())
        .add_systems(Startup, (setup_scene, setup_ui));
    #[cfg(feature = "player")]
    app.add_systems(Startup, player::setup_player.after(setup_scene).after(setup_ui));
    app.add_systems(Update, (
            animate_node_width, update_hovered_cell,
            update_ghost_and_highlight.after(update_hovered_cell),
            animate_scale.after(update_ghost_and_highlight).after(move_bots).after(apply_bot_formation),
            animate_ui_slides, animate_border_fade, cleanup_despawned.after(animate_scale),
        ))
        .add_systems(Update, (escape_to_quit, quit_dialog_buttons))
        .add_systems(Update, (
            overlay_button_interaction, play_stop_interaction.after(overlay_button_interaction),
            move_bots.after(play_stop_interaction), update_bot_formation.after(move_bots),
            apply_bot_formation.after(update_bot_formation), animate_merge_flashes,
            paint_bots.after(move_bots), toggle_doors.after(move_bots),
            check_simulation_result.after(move_bots),
            spawn_simulation_overlay.after(check_simulation_result),
            adapt_camera,
        ));
    #[cfg(not(feature = "player"))]
    app.add_systems(Update, (
            button_interaction, inventory_interaction,
            update_inventory_visuals.after(inventory_interaction),
            handle_tile_click.after(update_hovered_cell),
            sync_inventory_play_mode,
        ))
        .add_systems(Update, (
            mark_button_interaction, handle_mark_click.after(update_hovered_cell),
            test_button_interaction, stop_test_interaction.after(play_stop_interaction), reset_test_interaction,
            handle_test_tile_click.after(update_hovered_cell), test_inventory_interaction,
            sync_editor_buttons_visibility, save_button_interaction, save_dialog_input,
            blink_save_cursor, overwrite_dialog_buttons, validation_error_ok,
        ))
        .add_systems(Update, save_dialog_buttons)
        .add_systems(Update, (
            load_button_interaction, load_dialog_buttons, load_entry_hover,
            delete_level_button_interaction, delete_level_dialog_buttons,
            update_status_bar, update_load_scrollbar, scrollbar_drag,
        ))
        .add_systems(Update, (
            gen_button_interaction, gen_stepper_interaction, gen_hole_place_interaction,
            gen_inv_interaction, gen_difficulty_interaction, gen_weight_interaction,
            gen_all_equal_interaction, gen_clear_weights_interaction, gen_toggle_interaction,
            gen_cancel_interaction, gen_generate_interaction, gen_chain_interaction,
            gen_path_share_interaction, gen_confusion_interaction,
            gen_preset_interaction, gen_update_progress, gen_btn_pulse,
            gen_apply_result, update_generator,
        ));
    #[cfg(feature = "player")]
    { app.add_systems(Update, (handle_test_tile_click.after(update_hovered_cell),
        test_inventory_interaction, reset_test_interaction, update_status_bar));
      app.add_systems(Update, (player::player_nav_interaction, player::update_player_stats));
      app.add_systems(Update, (player::auto_save_progress, player::handle_level_complete));
      app.add_systems(Update, player::populate_stats.before(spawn_simulation_overlay));
      app.add_systems(Update, player::cleanup_stale_inventory); }
    app.run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    board_size: Res<BoardSize>,
) {
    let floor_texture = create_tile_texture(&mut images, TILE_TEX_SIZE, TILE_TEX_BORDER);
    let floor_material = materials.add(StandardMaterial { base_color_texture: Some(floor_texture.clone()),
        base_color: Color::srgb(FLOOR_TINT.0, FLOOR_TINT.1, FLOOR_TINT.2), perceptual_roughness: 0.6, ..default() });
    let floor_mesh = meshes.add(Cuboid::new(1.0, TILE_HEIGHT, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture), base_color: Color::srgba(1.0, 1.0, 1.0, GHOST_ALPHA),
        alpha_mode: AlphaMode::Blend, ..default() });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: rgba(DELETE_OVERLAY_COLOR), alpha_mode: AlphaMode::Blend, unlit: true, ..default() });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, OVERLAY_MESH_THICKNESS, 1.02));
    let overlay_tex_mat = |mats: &mut Assets<StandardMaterial>, tex| mats.add(StandardMaterial {
        base_color_texture: Some(tex), alpha_mode: AlphaMode::Blend, unlit: true, ..default() });
    let empty_material = overlay_tex_mat(&mut materials, create_empty_marker_texture(&mut images));
    let empty_mesh = meshes.add(Cuboid::new(0.95, OVERLAY_MESH_THICKNESS, 0.95));
    let highlight_material = overlay_tex_mat(&mut materials, create_highlight_texture(&mut images));
    let highlight_mesh = meshes.add(Cuboid::new(1.05, OVERLAY_MESH_THICKNESS, 1.05));
    let marker_material = overlay_tex_mat(&mut materials, create_inv_marker_texture(&mut images));
    let marker_mesh = meshes.add(Cuboid::new(1.03, OVERLAY_MESH_THICKNESS, 1.03));
    let sym_mesh = meshes.add(Cuboid::new(0.99, OVERLAY_MESH_THICKNESS, 0.99));
    let (source_symbol_materials, ghost_symbol_materials, _, _) = load_tile_mats(&mut materials, &mut images, "source");
    let (goal_symbol_materials, ghost_goal_materials, _, _) = load_tile_mats(&mut materials, &mut images, "goal");
    let (mut turn_symbol_materials, mut ghost_turn_materials, tb, tm) = load_tile_mats(&mut materials, &mut images, "turn");
    add_grey_mat(&mut materials, &mut turn_symbol_materials, &mut ghost_turn_materials, &tb, &tm);
    let (turnbut_symbol_materials, ghost_turnbut_materials, _, _) = load_tile_mats(&mut materials, &mut images, "turnbut");
    let (mut bounce_symbol_materials, mut ghost_bounce_materials, bb, bm) = load_tile_mats(&mut materials, &mut images, "bounce");
    add_grey_mat(&mut materials, &mut bounce_symbol_materials, &mut ghost_bounce_materials, &bb, &bm);
    let (bouncebot_symbol_materials, ghost_bouncebot_materials, _, _) = load_tile_mats(&mut materials, &mut images, "bouncebut");

    let load_grey = |mats: &mut Assets<StandardMaterial>, imgs: &mut Assets<Image>, name: &str|
        { let b = load_png_texture(imgs, &format!("assets/textures/{name}_base.png"), true);
          let m = load_png_texture(imgs, &format!("assets/textures/{name}_mask.png"), false);
          make_grey_mat(mats, b, m) };
    let (mut teleport_symbol_materials, mut ghost_teleport_materials) = (Vec::new(), Vec::new());
    let (mut teleportbut_symbol_materials, mut ghost_teleportbut_materials) = (Vec::new(), Vec::new());
    for num in 0..NUM_TELEPORTS {
        let (mut ms, mut gs, b, m) = load_tile_mats(&mut materials, &mut images, &format!("teleport_{num}"));
        add_grey_mat(&mut materials, &mut ms, &mut gs, &b, &m);
        teleport_symbol_materials.extend(ms); ghost_teleport_materials.extend(gs);
        let (ms2, gs2, _, _) = load_tile_mats(&mut materials, &mut images, &format!("teleportbut_{num}"));
        teleportbut_symbol_materials.extend(ms2); ghost_teleportbut_materials.extend(gs2);
    }
    let (door_open_material, ghost_door_open_material) = load_grey(&mut materials, &mut images, "door_open");
    let (door_closed_material, ghost_door_closed_material) = load_grey(&mut materials, &mut images, "door_closed");
    let (switch_material, ghost_switch_material) = load_grey(&mut materials, &mut images, "switch");

    let (colorswitch_symbol_materials, ghost_colorswitch_materials, _, _) = load_tile_mats(&mut materials, &mut images, "colorswitch");
    let (colorswitchbut_symbol_materials, ghost_colorswitchbut_materials, _, _) = load_tile_mats(&mut materials, &mut images, "colorswitchbut");
    let (painter_symbol_materials, ghost_painter_materials, _, _) = load_tile_mats(&mut materials, &mut images, "painter");
    let (mut arrow_symbol_materials, mut ghost_arrow_materials, ab, am) = load_tile_mats(&mut materials, &mut images, "arrow");
    add_grey_mat(&mut materials, &mut arrow_symbol_materials, &mut ghost_arrow_materials, &ab, &am);
    let (arrowbut_symbol_materials, ghost_arrowbut_materials, _, _) = load_tile_mats(&mut materials, &mut images, "arrowbut");

    let bot_mesh = meshes.add(Cuboid::new(BOT_SIZE, BOT_SIZE, BOT_SIZE));
    let eye_mesh = meshes.add(Cuboid::new(BOT_EYE_W, BOT_EYE_H, BOT_EYE_D));
    let eye_material = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    let bot_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)|
        materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() })).collect();
    let flash_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.6), alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });

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
        teleportbut_symbol_materials, ghost_teleportbut_materials: ghost_teleportbut_materials.clone(),
        bounce_symbol_materials, ghost_bounce_materials: ghost_bounce_materials.clone(),
        bouncebot_symbol_materials, ghost_bouncebot_materials: ghost_bouncebot_materials.clone(),
        door_open_material, door_closed_material,
        ghost_door_open_material, ghost_door_closed_material,
        switch_material, ghost_switch_material,
        colorswitch_symbol_materials, ghost_colorswitch_materials: ghost_colorswitch_materials.clone(),
        colorswitchbut_symbol_materials, ghost_colorswitchbut_materials: ghost_colorswitchbut_materials.clone(),
        painter_symbol_materials, ghost_painter_materials: ghost_painter_materials.clone(),
        arrow_symbol_mesh: sym_mesh.clone(), arrow_symbol_materials, ghost_arrow_materials: ghost_arrow_materials.clone(),
        arrowbut_symbol_mesh: sym_mesh.clone(), arrowbut_symbol_materials, ghost_arrowbut_materials: ghost_arrowbut_materials.clone(),
        marker_mesh, marker_material,
        bot_mesh, eye_mesh, bot_materials, eye_material, flash_material,
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
    commands.spawn((Camera3d::default(),
        Bloom { intensity: BLOOM_INTENSITY, low_frequency_boost: BLOOM_LF_BOOST,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 1.0, ..default() },
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

fn setup_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut fonts: ResMut<Assets<Font>>) {
    let vignette = create_vignette_texture(&mut images);
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), ..default() }, ImageNode::new(vignette)));
    let font_bytes = include_bytes!("../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let f = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(GameFont(f.clone()));

    let floor_tex_data = tile_texture_data(TEX_SIZE, TEX_BORDER);
    let floor_icon = create_isometric_icon(&mut images, &floor_tex_data, TEX_SIZE, ICON_SIZE);
    let white = ICON_WHITE;
    let icon = |images: &mut Assets<Image>, data: &[u8]| create_isometric_icon(images, data, TEX_SIZE, ICON_SIZE);
    let delete_icon = create_delete_icon(&mut images);
    let p = |n: &str| load_png_pair(n);
    let (src_b, src_m, ps) = p("source"); let (goal_b, goal_m, _) = p("goal");
    let (turn_b, turn_m, _) = p("turn"); let (tbut_b, tbut_m, _) = p("turnbut");
    let (bnc_b, bnc_m, _) = p("bounce"); let (bbot_b, bbot_m, _) = p("bouncebut");
    let (do_b, do_m, _) = p("door_open"); let (dc_b, dc_m, _) = p("door_closed");
    let (sw_b, sw_m, _) = p("switch"); let (csw_b, csw_m, _) = p("colorswitch");
    let (cswb_b, cswb_m, _) = p("colorswitchbut"); let (pnt_b, pnt_m, _) = p("painter");
    let (arr_b, arr_m, _) = p("arrow"); let (abut_b, abut_m, _) = p("arrowbut");
    let cp = |b: &[u8], m: &[u8], rot: f32, fill: [u8; 4]| composite_icon_from_png(b, m, ps, TEX_SIZE, TEX_BORDER, rot, fill);
    let source_icon = icon(&mut images, &cp(&src_b, &src_m, 0.0, white));
    let goal_icon = icon(&mut images, &cp(&goal_b, &goal_m, 0.0, white));
    let turn_icon = icon(&mut images, &cp(&turn_b, &turn_m, 0.0, white));
    let turnbut_icon = icon(&mut images, &cp(&tbut_b, &tbut_m, 0.0, white));

    let dir_icons = |images: &mut Assets<Image>, b: &[u8], m: &[u8]|
        Direction::all().map(|d| icon(images, &cp(b, m, -d.rotation(), white)));
    let source_dir_icons = dir_icons(&mut images, &src_b, &src_m);
    let turn_dir_icons = dir_icons(&mut images, &turn_b, &turn_m);
    let turnbut_dir_icons = dir_icons(&mut images, &tbut_b, &tbut_m);
    let arrow_dir_icons = dir_icons(&mut images, &arr_b, &arr_m);
    let arrowbut_dir_icons = dir_icons(&mut images, &abut_b, &abut_m);

    let cfill = |ci: usize| color_to_u8(SOURCE_COLORS[ci].0, SOURCE_COLORS[ci].1, SOURCE_COLORS[ci].2);
    let grey_fill = color_to_u8(GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2);
    let color_dir_icons = |images: &mut Assets<Image>, b: &[u8], m: &[u8]| -> Vec<_> {
        (0..NUM_COLORS).flat_map(|ci| { let f = cfill(ci); Direction::all().map(move |d| (f, d)) })
            .map(|(f, d)| icon(images, &cp(b, m, -d.rotation(), f))).collect()
    };
    let source_color_icons = color_dir_icons(&mut images, &src_b, &src_m);
    let mut turn_color_icons = color_dir_icons(&mut images, &turn_b, &turn_m);
    for d in Direction::all() { turn_color_icons.push(icon(&mut images, &cp(&turn_b, &turn_m, -d.rotation(), grey_fill))); }
    let turnbut_color_icons = color_dir_icons(&mut images, &tbut_b, &tbut_m);
    let mut arrow_color_icons = color_dir_icons(&mut images, &arr_b, &arr_m);
    for d in Direction::all() { arrow_color_icons.push(icon(&mut images, &cp(&arr_b, &arr_m, -d.rotation(), grey_fill))); }
    let arrowbut_color_icons = color_dir_icons(&mut images, &abut_b, &abut_m);

    let (tp_b, tp_m, _) = p("teleport_0"); let (tpb_b, tpb_m, _) = p("teleportbut_0");
    let teleport_icon = icon(&mut images, &cp(&tp_b, &tp_m, 0.0, white));
    let teleportbut_icon = icon(&mut images, &cp(&tpb_b, &tpb_m, 0.0, white));
    let mut teleport_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&tp_b, &tp_m, 0.0, cfill(ci)))).collect();
    teleport_color_icons.push(icon(&mut images, &cp(&tp_b, &tp_m, 0.0, grey_fill)));
    let teleportbut_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&tpb_b, &tpb_m, 0.0, cfill(ci)))).collect();
    let goal_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&goal_b, &goal_m, 0.0, cfill(ci)))).collect();

    let bounce_icon = icon(&mut images, &cp(&bnc_b, &bnc_m, 0.0, white));
    let bouncebot_icon = icon(&mut images, &cp(&bbot_b, &bbot_m, 0.0, white));
    let mut bounce_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&bnc_b, &bnc_m, 0.0, cfill(ci)))).collect();
    bounce_color_icons.push(icon(&mut images, &cp(&bnc_b, &bnc_m, 0.0, grey_fill)));
    let bouncebot_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&bbot_b, &bbot_m, 0.0, cfill(ci)))).collect();

    let door_icon = icon(&mut images, &cp(&dc_b, &dc_m, 0.0, white));
    let door_open_icon = icon(&mut images, &cp(&do_b, &do_m, 0.0, grey_fill));
    let door_closed_icon = icon(&mut images, &cp(&dc_b, &dc_m, 0.0, grey_fill));
    let switch_icon = icon(&mut images, &cp(&sw_b, &sw_m, 0.0, white));
    let mut switch_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&csw_b, &csw_m, 0.0, cfill(ci)))).collect();
    switch_color_icons.push(icon(&mut images, &cp(&sw_b, &sw_m, 0.0, grey_fill)));
    let switchbut_icon = icon(&mut images, &cp(&cswb_b, &cswb_m, 0.0, white));
    let switchbut_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&cswb_b, &cswb_m, 0.0, cfill(ci)))).collect();
    let painter_icon = icon(&mut images, &cp(&pnt_b, &pnt_m, 0.0, white));
    let painter_color_icons: Vec<_> = (0..NUM_COLORS).map(|ci| icon(&mut images, &cp(&pnt_b, &pnt_m, 0.0, cfill(ci)))).collect();
    let arrow_icon = icon(&mut images, &cp(&arr_b, &arr_m, 0.0, white));
    let arrowbut_icon = icon(&mut images, &cp(&abut_b, &abut_m, 0.0, white));

    commands.insert_resource(InventoryIcons {
        floor: floor_icon.clone(), source: source_icon.clone(),
        goal: goal_icon.clone(), turn: turn_icon.clone(), delete: delete_icon.clone(),
        source_dir_icons, source_color_icons, goal_color_icons,
        turn_dir_icons, turn_color_icons,
        turnbut: turnbut_icon.clone(), turnbut_dir_icons, turnbut_color_icons,
        teleport: teleport_icon.clone(), teleport_color_icons,
        teleportbut: teleportbut_icon.clone(), teleportbut_color_icons,
        bounce: bounce_icon.clone(), bounce_color_icons,
        bouncebot: bouncebot_icon.clone(), bouncebot_color_icons,
        door: door_icon.clone(), door_open: door_open_icon, door_closed: door_closed_icon,
        switch: switch_icon.clone(), switch_color_icons,
        switchbut: switchbut_icon.clone(), switchbut_color_icons,
        painter: painter_icon.clone(), painter_color_icons,
        arrow: arrow_icon.clone(), arrow_dir_icons, arrow_color_icons,
        arrowbut: arrowbut_icon.clone(), arrowbut_dir_icons, arrowbut_color_icons,
    });

    // Top controls (editor only)
    if !cfg!(feature = "player") {
    let bc = btn_bg();
    let btn = Node { width: Val::Px(TOP_BTN_SIZE), height: Val::Px(TOP_BTN_SIZE),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(BTN_MARGIN)), border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() };
    let ts = gf(TOP_BTN_FONT, &f);
    let mut tbtn = text_btn_node();
    tbtn.margin = UiRect::left(Val::Px(TEXT_BTN_LEFT_MARGIN));
    tbtn.border_radius = BorderRadius::all(Val::Px(UI_CORNER_RADIUS));
    let tfs = gf(LABEL_FONT, &f);
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(-50.0),
        flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: Val::Px(4.0), ..default()
    }, UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false }, TopControlsBar)).with_children(|p| {
        p.spawn((Button, btn.clone(), BackgroundColor(bc), BoardButton::Decrease))
            .with_child((Text::new("-"), ts.clone(), TextColor(Color::WHITE)));
        p.spawn(Node { width: Val::Px(70.0), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new("3x3"), ts.clone(), TextColor(Color::WHITE), BoardSizeText));
        p.spawn((Button, btn, BackgroundColor(bc), BoardButton::Increase))
            .with_child((Text::new("+"), ts, TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), BorderColor::all(border_unsel()), MarkButton, MarkButtonImage))
            .with_child((Text::new("Mark"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), TestButton))
            .with_child((Text::new("Test"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn.clone(), BackgroundColor(bc), LoadButton))
            .with_child((Text::new("Load"), tfs.clone(), TextColor(Color::WHITE)));
        p.spawn((Button, tbtn, BackgroundColor(bc), GenButton))
            .with_child((Text::new("Gen"), tfs, TextColor(Color::WHITE)));
    });
    }

    // Play/Stop button
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });
    commands.spawn((Node { position_type: PositionType::Absolute, right: Val::Px(10.0), top: Val::Px(-60.0), ..default()
    }, UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false })).with_child((
        Button, Node { width: Val::Px(PLAY_BTN_SIZE), height: Val::Px(PLAY_BTN_SIZE), justify_content: JustifyContent::Center,
            align_items: AlignItems::Center, border: UiRect::all(Val::Px(PLAY_BTN_BORDER)),
            border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() },
        BackgroundColor(slot_bg()), BorderColor::all(border_unsel()),
        PlayStopButton, ImageNode::new(play_icon), PlayButtonImage,
    ));

    // Inventory bar (editor only)
    if !cfg!(feature = "player") {
    use InventorySlot::*;
    let l1: Vec<(InventorySlot, Handle<Image>)> = vec![
        (Floor, floor_icon), (Source, source_icon), (Goal, goal_icon), (Turn, turn_icon),
        (TurnBut, turnbut_icon), (Teleport, teleport_icon), (TeleportBut, teleportbut_icon),
        (Bounce, bounce_icon), (BounceBut, bouncebot_icon), (Door, door_icon),
        (Switch, switch_icon), (SwitchBut, switchbut_icon), (Painter, painter_icon),
        (Arrow, arrow_icon), (ArrowBut, arrowbut_icon) ];
    let n = l1.len() + 1; // +1 for delete
    let (sw, sh, iw) = fit_slot_sizes(n, SLOT_VW);
    let sn = slot_node_sized(sw, sh);
    let in_node = icon_node_sized(iw);

    commands.spawn((Node {
        position_type: PositionType::Absolute, bottom: Val::Px(INV_SLIDE_HIDE),
        width: Val::Percent(100.0), flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center, ..default()
    }, InventoryContainer, UiBottomAnim { target: INV_SLIDE_SHOW, despawn_at_target: false },
    )).with_children(|outer| {
        outer.spawn(Node { position_type: PositionType::Absolute, top: Val::Px(-22.0),
            width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new(""), gf(STATUS_FONT, &f),
                TextColor(Color::srgba(TOOLTIP_COLOR.0, TOOLTIP_COLOR.1, TOOLTIP_COLOR.2, 0.0)), StatusBarText));
        outer.spawn((Node { flex_direction: FlexDirection::Row, padding: UiRect::axes(Val::Vw(INVENTORY_PAD_VW), Val::ZERO),
            column_gap: Val::Vw(INVENTORY_GAP_VW), height: Val::Vw(0.0),
            align_items: AlignItems::Center, justify_content: JustifyContent::Center,
            overflow: Overflow::clip(), border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() },
            BackgroundColor(rgba(INVENTORY_EXP_BG)),
            ExpansionContainer));
        outer.spawn(Node { width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() })
            .with_children(|row| {
                row.spawn((Node { flex_direction: FlexDirection::Row, padding: UiRect::all(Val::Vw(INVENTORY_PAD_VW)),
                    column_gap: Val::Vw(INVENTORY_GAP_VW), align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() },
                    BackgroundColor(rgba(INVENTORY_L1_BG)),
                )).with_children(|c| {
                    for (i, (slot_type, icon_handle)) in l1.iter().enumerate() {
                        let glow = BoxShadow::new(rgba(SLOT_GLOW_COLOR), Val::ZERO, Val::ZERO,
                            Val::Px(SLOT_GLOW_SPREAD), Val::Px(SLOT_GLOW_BLUR));
                        c.spawn((Button, sn.clone(), BackgroundColor(slot_bg()), border_for(i == 0), *slot_type, glow))
                            .with_child((in_node.clone(), ImageNode::new(icon_handle.clone())));
                    }
                    c.spawn((Button, sn.clone(), BackgroundColor(Color::NONE), BorderColor::all(Color::NONE), InventorySlot::Delete))
                        .with_child((in_node.clone(), ImageNode::new(delete_icon)));
                });
            });
    });
    }
}
