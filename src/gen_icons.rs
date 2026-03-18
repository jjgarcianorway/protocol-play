// SPDX-License-Identifier: GPL-3.0-or-later
//! Generate inventory icons by screenshotting tiles from the real game renderer.
//! Uses the actual setup_scene, spawn_tile, adapt_camera — so icons match the board exactly.
//! Usage: ./generate-icons  (generates assets/icons/*.png then exits)
#![allow(dead_code, unused_imports)]

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
mod messages;
mod bot_formation;
mod mat_helpers;
mod test_mode;
mod level_io;
mod save_dialog;
mod level_gen_sim;
mod level_gen_tiles;
mod level_gen_algo;
mod level_gen_ui;
mod level_gen_interact;
mod icon_render;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use constants::*;
use types::*;
use textures::*;
use board::*;
use mat_helpers::*;

#[derive(Resource)]
struct IconQueue {
    items: Vec<(String, TileKind)>,
    index: usize,
    current_tiles: Vec<Entity>,
    wait_frames: u8,
    screenshot_taken: bool,
    done: bool,
}


fn main() {
    gen_textures::ensure_textures();
    let _ = std::fs::create_dir_all("assets/icons");

    let mut app = App::new();
    app.set_error_handler(bevy::ecs::error::ignore);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Generating icons...".into(),
            resolution: bevy::window::WindowResolution::new(ICON_SIZE, ICON_SIZE),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(CLEAR_COLOR.0, CLEAR_COLOR.1, CLEAR_COLOR.2)))
    .insert_resource(GlobalAmbientLight {
        color: Color::srgb(AMBIENT_COLOR.0, AMBIENT_COLOR.1, AMBIENT_COLOR.2),
        brightness: AMBIENT_BRIGHTNESS, ..default()
    })
    .insert_resource(BoardSize(1))
    .insert_resource(PlayMode::Playing)
    .add_systems(Startup, setup_icon_scene)
    .add_systems(Update, (icon_camera, icon_step));
    app.run();
}

fn setup_icon_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    _board_size: Res<BoardSize>,
) {
    // Same tile height as the real game
    let floor_texture = create_tile_texture(&mut images, TILE_TEX_SIZE, TILE_TEX_BORDER);
    let floor_material = materials.add(StandardMaterial { base_color_texture: Some(floor_texture.clone()),
        base_color: Color::srgb(FLOOR_TINT.0, FLOOR_TINT.1, FLOOR_TINT.2), perceptual_roughness: 0.6, ..default() });
    let floor_mesh = meshes.add(Cuboid::new(1.0, TILE_HEIGHT, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture), base_color: Color::srgba(1.0, 1.0, 1.0, GHOST_ALPHA),
        alpha_mode: AlphaMode::Blend, ..default() });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: ui_helpers::rgba(DELETE_OVERLAY_COLOR), alpha_mode: AlphaMode::Blend, unlit: true, ..default() });
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

    // Load all tile materials — same as the real game, plus grey variants for L1
    let (mut source_symbol_materials, ghost_symbol_materials, sb, sm) = load_tile_mats(&mut materials, &mut images, "source");
    add_grey_mat(&mut materials, &mut source_symbol_materials, &mut vec![], &sb, &sm); // add grey source
    let (mut goal_symbol_materials, ghost_goal_materials, gb, gm) = load_tile_mats(&mut materials, &mut images, "goal");
    add_grey_mat(&mut materials, &mut goal_symbol_materials, &mut vec![], &gb, &gm); // add grey goal
    let (mut turn_symbol_materials, mut ghost_turn_materials, tb, tm) = load_tile_mats(&mut materials, &mut images, "turn");
    add_grey_mat(&mut materials, &mut turn_symbol_materials, &mut ghost_turn_materials, &tb, &tm);
    let (mut turnbut_symbol_materials, ghost_turnbut_materials, tbb, tbm) = load_tile_mats(&mut materials, &mut images, "turnbut");
    add_grey_mat(&mut materials, &mut turnbut_symbol_materials, &mut vec![], &tbb, &tbm); // add grey turnbut
    let (mut bounce_symbol_materials, mut ghost_bounce_materials, bb, bm) = load_tile_mats(&mut materials, &mut images, "bounce");
    add_grey_mat(&mut materials, &mut bounce_symbol_materials, &mut ghost_bounce_materials, &bb, &bm);
    let (mut bouncebot_symbol_materials, ghost_bouncebot_materials, bbb, bbm) = load_tile_mats(&mut materials, &mut images, "bouncebut");
    add_grey_mat(&mut materials, &mut bouncebot_symbol_materials, &mut vec![], &bbb, &bbm);
    let load_grey = |mats: &mut Assets<StandardMaterial>, imgs: &mut Assets<Image>, name: &str| {
        let b = load_png_texture(imgs, &format!("assets/textures/{name}_base.png"), true);
        let m = load_png_texture(imgs, &format!("assets/textures/{name}_mask.png"), false);
        make_grey_mat(mats, b, m)
    };
    let (mut tp_mats, mut gtp) = (Vec::new(), Vec::new());
    let (mut tpb_mats, mut gtpb) = (Vec::new(), Vec::new());
    for num in 0..NUM_TELEPORTS {
        let (mut ms, mut gs, b, m) = load_tile_mats(&mut materials, &mut images, &format!("teleport_{num}"));
        add_grey_mat(&mut materials, &mut ms, &mut gs, &b, &m);
        tp_mats.extend(ms); gtp.extend(gs);
        let (mut ms2, gs2, b2, m2) = load_tile_mats(&mut materials, &mut images, &format!("teleportbut_{num}"));
        add_grey_mat(&mut materials, &mut ms2, &mut vec![], &b2, &m2);
        tpb_mats.extend(ms2); gtpb.extend(gs2);
    }
    let (door_open_material, ghost_door_open_material) = load_grey(&mut materials, &mut images, "door_open");
    let (door_closed_material, ghost_door_closed_material) = load_grey(&mut materials, &mut images, "door_closed");
    let (switch_material, ghost_switch_material) = load_grey(&mut materials, &mut images, "switch");
    let (mut cs_mats, ghost_colorswitch_materials, csb_tex, csm_tex) = load_tile_mats(&mut materials, &mut images, "colorswitch");
    add_grey_mat(&mut materials, &mut cs_mats, &mut vec![], &csb_tex, &csm_tex);
    let (mut csb_mats, ghost_colorswitchbut_materials, csbb_tex, csbm_tex) = load_tile_mats(&mut materials, &mut images, "colorswitchbut");
    add_grey_mat(&mut materials, &mut csb_mats, &mut vec![], &csbb_tex, &csbm_tex);
    let (mut painter_mats, ghost_painter_materials, pb, pm) = load_tile_mats(&mut materials, &mut images, "painter");
    add_grey_mat(&mut materials, &mut painter_mats, &mut vec![], &pb, &pm);
    let (mut arrow_symbol_materials, mut ghost_arrow_materials, ab, am) = load_tile_mats(&mut materials, &mut images, "arrow");
    add_grey_mat(&mut materials, &mut arrow_symbol_materials, &mut ghost_arrow_materials, &ab, &am);
    let (mut arrowbut_mats, ghost_arrowbut_materials, abb, abm) = load_tile_mats(&mut materials, &mut images, "arrowbut");
    add_grey_mat(&mut materials, &mut arrowbut_mats, &mut vec![], &abb, &abm);

    let bot_mesh = meshes.add(Cuboid::new(BOT_SIZE, BOT_SIZE, BOT_SIZE));
    let eye_mesh = meshes.add(Cuboid::new(BOT_EYE_W, BOT_EYE_H, BOT_EYE_D));
    let eye_material = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    let bot_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)|
        materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() })).collect();
    let flash_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.6), alpha_mode: AlphaMode::Blend, unlit: true, ..default() });

    let assets = GameAssets {
        floor_mesh: floor_mesh.clone(), floor_material,
        empty_mesh, empty_material,
        ghost_floor_material: ghost_floor_material.clone(),
        ghost_delete_mesh: ghost_delete_mesh.clone(), ghost_delete_material: ghost_delete_material.clone(),
        highlight_mesh: highlight_mesh.clone(), highlight_material: highlight_material.clone(),
        source_symbol_mesh: sym_mesh.clone(), source_symbol_materials,
        ghost_symbol_materials: ghost_symbol_materials.clone(),
        goal_symbol_mesh: sym_mesh.clone(), goal_symbol_materials,
        ghost_goal_materials: ghost_goal_materials.clone(),
        turn_symbol_mesh: sym_mesh.clone(), turn_symbol_materials,
        ghost_turn_materials: ghost_turn_materials.clone(),
        turnbut_symbol_mesh: sym_mesh.clone(), turnbut_symbol_materials,
        ghost_turnbut_materials: ghost_turnbut_materials.clone(),
        teleport_symbol_materials: tp_mats, ghost_teleport_materials: gtp.clone(),
        teleportbut_symbol_materials: tpb_mats, ghost_teleportbut_materials: gtpb.clone(),
        bounce_symbol_materials, ghost_bounce_materials: ghost_bounce_materials.clone(),
        bouncebot_symbol_materials, ghost_bouncebot_materials: ghost_bouncebot_materials.clone(),
        door_open_material, door_closed_material,
        ghost_door_open_material, ghost_door_closed_material,
        switch_material, ghost_switch_material,
        colorswitch_symbol_materials: cs_mats, ghost_colorswitch_materials: ghost_colorswitch_materials.clone(),
        colorswitchbut_symbol_materials: csb_mats, ghost_colorswitchbut_materials: ghost_colorswitchbut_materials.clone(),
        painter_symbol_materials: painter_mats, ghost_painter_materials: ghost_painter_materials.clone(),
        arrow_symbol_mesh: sym_mesh.clone(), arrow_symbol_materials, ghost_arrow_materials: ghost_arrow_materials.clone(),
        arrowbut_symbol_mesh: sym_mesh.clone(), arrowbut_symbol_materials: arrowbut_mats, ghost_arrowbut_materials: ghost_arrowbut_materials.clone(),
        marker_mesh, marker_material,
        bot_mesh, eye_mesh, bot_materials, eye_material, flash_material,
    };
    commands.insert_resource(assets);

    commands.spawn((
        DirectionalLight { illuminance: LIGHT_ILLUMINANCE, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, LIGHT_ELEVATION, LIGHT_AZIMUTH, 0.0)),
    ));
    commands.spawn((Camera3d::default(),
        Bloom { intensity: BLOOM_INTENSITY, low_frequency_boost: BLOOM_LF_BOOST,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 1.0, ..default() },
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y)));


    let items = build_icon_list();
    println!("Generating {} icon PNGs...", items.len());
    commands.insert_resource(IconQueue {
        items, index: 0, current_tiles: vec![], wait_frames: 10, // warm-up for GPU texture upload
        screenshot_taken: false, done: false,
    });
}

fn icon_camera(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection), With<Camera3d>>,
    board_size: Res<BoardSize>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok((mut transform, projection)) = cameras.single_mut() else { return };
    let aspect = window.width() / window.height();
    let fov = match projection { Projection::Perspective(p) => p.fov, _ => return };
    // Same camera math as the real game (adapt_camera in board.rs)
    let radius = board_bounding_radius(board_size.0);
    let radius_v = radius * 0.7; // isometric foreshortening
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let dist_v = radius_v / (half_fov_v).sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;
    let dir = camera_direction();
    let look_y = -0.06 * distance; // same offset as game stop mode
    let look_at = Vec3::new(0.0, look_y, 0.0);
    let target = Transform::from_translation(look_at + dir * distance).looking_at(look_at, Vec3::Y);
    transform.translation = target.translation;
    transform.rotation = target.rotation;
}

fn icon_step(
    mut commands: Commands,
    mut queue: ResMut<IconQueue>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
) {
    if queue.done { return; }
    if queue.index >= queue.items.len() && queue.current_tiles.is_empty() {
        // Wait extra frames for the last screenshot observer to fire
        if queue.wait_frames > 0 { queue.wait_frames -= 1; return; }
        println!("\nDone! Generated {} icons in assets/icons/", queue.items.len());
        queue.done = true;
        std::process::exit(0);
    }
    if !queue.current_tiles.is_empty() {
        if queue.wait_frames > 0 { queue.wait_frames -= 1; return; }
        if !queue.screenshot_taken {
            let idx = queue.index - 1;
            let filename = queue.items[idx].0.clone();
            commands.spawn(Screenshot::primary_window()).observe(
                move |trigger: On<ScreenshotCaptured>| {
                    let img = &trigger.event().image;
                    if let Some(data) = &img.data {
                        let w = img.width();
                        let h = img.height();
                        if let Some(buf) = image::RgbaImage::from_raw(w, h, data.clone()) {
                            let resized = image::imageops::resize(&buf, ICON_SIZE, ICON_SIZE,
                                image::imageops::FilterType::Lanczos3);
                            let _ = resized.save(format!("assets/icons/{filename}.png"));
                        }
                    }
                },
            );
            queue.screenshot_taken = true;
            queue.wait_frames = 2;
            return;
        }
        for e in &queue.current_tiles { commands.entity(*e).despawn(); }
        queue.current_tiles.clear();
        let idx = queue.index - 1;
        print!("\r  [{}/{}] {}                    ", queue.index, queue.items.len(), queue.items[idx].0);
        // If this was the last tile, wait extra frames for screenshot to save
        if queue.index >= queue.items.len() { queue.wait_frames = 5; }
    }
    if queue.index < queue.items.len() {
        let kind = queue.items[queue.index].1;
        let entity = spawn_tile_at_scale(&mut commands, 0, 0, board_size.0, kind, &assets, Vec3::ONE);
        queue.current_tiles.push(entity);
        queue.index += 1;
        queue.wait_frames = 3;
        queue.screenshot_taken = false;
    }
}

fn build_icon_list() -> Vec<(String, TileKind)> {
    let mut q = Vec::with_capacity(400);
    let mut push = |k: TileKind| { q.push((icon_render::tile_filename(k), k)); };

    // Colored variants first (textures warm up during these)
    for d in Direction::all() { push(TileKind::Source(0, d)); }
    for ci in 0..NUM_COLORS { for d in Direction::all() { push(TileKind::Source(ci, d)); } }
    for ci in 0..NUM_COLORS { push(TileKind::Goal(ci)); }
    for d in Direction::all() { push(TileKind::Turn(0, d)); }
    for ci in 0..NUM_COLORS { for d in Direction::all() { push(TileKind::Turn(ci, d)); } }
    for d in Direction::all() { push(TileKind::Turn(NUM_COLORS, d)); }
    for d in Direction::all() { push(TileKind::TurnBut(0, d)); }
    for ci in 0..NUM_COLORS { for d in Direction::all() { push(TileKind::TurnBut(ci, d)); } }
    push(TileKind::Teleport(0, 0));
    for ci in 0..NUM_COLORS { push(TileKind::Teleport(ci, 0)); }
    push(TileKind::TeleportBut(0, 0));
    for ci in 0..NUM_COLORS { push(TileKind::TeleportBut(ci, 0)); }
    push(TileKind::Bounce(0));
    for ci in 0..NUM_COLORS { push(TileKind::Bounce(ci)); }
    push(TileKind::BounceBut(0));
    for ci in 0..NUM_COLORS { push(TileKind::BounceBut(ci)); }
    push(TileKind::Door(false)); push(TileKind::Door(true));
    push(TileKind::Switch);
    for ci in 0..NUM_COLORS { push(TileKind::ColorSwitch(ci)); }
    push(TileKind::ColorSwitchBut(0));
    for ci in 0..NUM_COLORS { push(TileKind::ColorSwitchBut(ci)); }
    push(TileKind::Painter(0));
    for ci in 0..NUM_COLORS { push(TileKind::Painter(ci)); }
    push(TileKind::Arrow(0, Direction::North));
    for d in Direction::all() { push(TileKind::Arrow(0, d)); }
    for ci in 0..NUM_COLORS { for d in Direction::all() { push(TileKind::Arrow(ci, d)); } }
    for d in Direction::all() { push(TileKind::Arrow(NUM_COLORS, d)); }
    push(TileKind::ArrowBut(0, Direction::North));
    for d in Direction::all() { push(TileKind::ArrowBut(0, d)); }
    for ci in 0..NUM_COLORS { for d in Direction::all() { push(TileKind::ArrowBut(ci, d)); } }

    // L1 grey icons at the end (all textures fully loaded by now)
    push(TileKind::Source(NUM_COLORS, Direction::North));
    push(TileKind::Goal(NUM_COLORS));
    push(TileKind::Turn(NUM_COLORS, Direction::North));
    push(TileKind::TurnBut(NUM_COLORS, Direction::North));
    push(TileKind::Teleport(NUM_COLORS, 0));
    push(TileKind::TeleportBut(NUM_COLORS, 0));
    push(TileKind::Bounce(NUM_COLORS));
    push(TileKind::BounceBut(NUM_COLORS));
    push(TileKind::Painter(NUM_COLORS));
    push(TileKind::Arrow(NUM_COLORS, Direction::North));
    push(TileKind::ArrowBut(NUM_COLORS, Direction::North));
    push(TileKind::ColorSwitch(NUM_COLORS));
    push(TileKind::ColorSwitchBut(NUM_COLORS));
    push(TileKind::Floor); // last — needs all GPU resources loaded
    q
}
