// SPDX-License-Identifier: GPL-3.0-or-later
//! Bot puzzle scene enter/exit — entity spawning/despawning for integrated mode.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use crate::constants::*;
use crate::types::*;
use crate::textures::*;
use crate::board::*;
use crate::test_mode::group_tiles;
use crate::simulation::SimulationResult;
use crate::ui_helpers::*;
use crate::mission::types::MissionCamera;
use crate::bot_puzzle_assets::create_bot_puzzle_assets;
use super::bot_puzzle_integrated::{BotPuzzleEntity, BotPuzzleLevel};

/// OnEnter(BotPuzzle): hide MC camera, create puzzle camera + assets + level.
pub fn enter_bot_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut fonts: ResMut<Assets<Font>>,
    camera_q: Query<Entity, With<MissionCamera>>,
    root_ui_q: Query<Entity, (With<Node>, Without<bevy::prelude::ChildOf>)>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    // Hide Mission Control camera + UI
    for e in camera_q.iter() {
        commands.entity(e).insert(Visibility::Hidden);
    }
    for e in root_ui_q.iter() {
        commands.entity(e).insert(Visibility::Hidden);
    }

    // Bot puzzle visual settings
    *clear_color = ClearColor(Color::srgb(CLEAR_COLOR.0, CLEAR_COLOR.1, CLEAR_COLOR.2));
    *ambient = GlobalAmbientLight {
        color: Color::srgb(AMBIENT_COLOR.0, AMBIENT_COLOR.1, AMBIENT_COLOR.2),
        brightness: AMBIENT_BRIGHTNESS, ..default()
    };

    // Create game assets
    let assets = create_bot_puzzle_assets(&mut meshes, &mut materials, &mut images);
    let ghost_floor_mat = assets.ghost_floor_material.clone();
    let ghost_sym = assets.ghost_symbol_materials[0].clone();
    let floor_mesh_h = assets.floor_mesh.clone();
    let sym_mesh_h = assets.source_symbol_mesh.clone();
    let hl_mesh = assets.highlight_mesh.clone();
    let hl_mat = assets.highlight_material.clone();

    // Insert bot puzzle resources
    insert_bot_resources(&mut commands);

    // Font
    let font_bytes = include_bytes!("../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(GameFont(font.clone()));

    // Icons
    let delete_icon = create_delete_icon(&mut images);
    let icons = crate::icon_render::build_inventory_icons(&mut images, delete_icon);
    commands.insert_resource(icons);

    // Play/Stop icons
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });

    // Load the level
    let gs = crate::save_state::load_game_state();
    let level_idx = gs.bot_level as usize;
    let search_dir = crate::save_state::exe_dir();
    let (level, filename) = load_bot_level(&search_dir, level_idx);

    let mut board_size = BoardSize(3);
    let mut test_inv = TestInventory::default();
    let mut play_mode = PlayMode::default();
    let mut selected_tool = SelectedTool::default();

    if let Some(ref level_data) = level {
        setup_level_board(&mut commands, level_data, &assets,
            &mut board_size, &mut test_inv, &mut play_mode, &mut selected_tool);
    } else {
        spawn_board(&mut commands, board_size.0, &assets);
    }

    commands.insert_resource(board_size);
    commands.insert_resource(selected_tool);
    commands.insert_resource(test_inv);
    commands.insert_resource(play_mode);

    if let Some(level_data) = level {
        commands.insert_resource(BotPuzzleLevel {
            level: level_data, filename: filename.unwrap_or_default(),
        });
    }

    spawn_scene_entities(&mut commands, &mut images, &font, play_icon,
        floor_mesh_h, ghost_floor_mat, sym_mesh_h, ghost_sym, hl_mesh, hl_mat, level_idx);

    commands.insert_resource(assets);
}

fn insert_bot_resources(commands: &mut Commands) {
    commands.insert_resource(BoardSize(3));
    commands.insert_resource(SelectedTool::default());
    commands.insert_resource(HoveredCell::default());
    commands.insert_resource(HiddenTileEntity::default());
    commands.insert_resource(GhostCell::default());
    commands.insert_resource(crate::test_mode::LastPlacementTracker::default());
    commands.insert_resource(InventoryState {
        level: 1, direction: None, color_index: None, last_placed_color: None,
    });
    commands.insert_resource(PlayMode::default());
    commands.insert_resource(crate::simulation::DoorToggleCount::default());
    commands.insert_resource(crate::simulation::OriginalDoorStates::default());
    commands.insert_resource(SimulationResult::default());
    commands.insert_resource(crate::bot_formation::PrevTileCounts::default());
    commands.insert_resource(SavedBoardState::default());
    commands.insert_resource(SavedTestState::default());
    commands.insert_resource(TestInventory::default());
    commands.insert_resource(LevelValidated::default());
}

fn setup_level_board(
    commands: &mut Commands, level_data: &LevelData, assets: &GameAssets,
    board_size: &mut BoardSize, test_inv: &mut TestInventory,
    play_mode: &mut PlayMode, selected_tool: &mut SelectedTool,
) {
    board_size.0 = level_data.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);
    let mut board_tiles = Vec::new();
    let mut marked_kinds = Vec::new();
    for &(col, row, kind, is_marked) in &level_data.tiles {
        if col >= board_size.0 || row >= board_size.0 { continue; }
        if is_marked {
            marked_kinds.push(kind);
            board_tiles.push((col, row, TileKind::Empty));
        } else {
            board_tiles.push((col, row, kind));
        }
    }
    let grid: std::collections::HashSet<_> = board_tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    for row in 0..board_size.0 { for col in 0..board_size.0 {
        if !grid.contains(&(col, row)) { board_tiles.push((col, row, TileKind::Empty)); }
    }}
    for &(col, row, kind) in &board_tiles {
        spawn_tile(commands, col, row, board_size.0, kind, assets);
    }
    let default_inv = group_tiles(marked_kinds.into_iter());
    commands.insert_resource(SavedTestState { tiles: board_tiles, inventory: default_inv.clone() });
    test_inv.items = default_inv;
    test_inv.selected = if test_inv.items.is_empty() { None } else { Some(0) };
    test_inv.remove_mode = false;
    if let Some((kind, _)) = test_inv.items.first() {
        selected_tool.0 = match kind {
            TileKind::Turn(..) => Tool::Turn, TileKind::TurnBut(..) => Tool::TurnBut,
            TileKind::Arrow(..) => Tool::Arrow, TileKind::ArrowBut(..) => Tool::ArrowBut,
            TileKind::Source(..) => Tool::Source, TileKind::Goal(..) => Tool::Goal,
            TileKind::Teleport(..) => Tool::Teleport, TileKind::TeleportBut(..) => Tool::TeleportBut,
            TileKind::Bounce(..) => Tool::Bounce, TileKind::BounceBut(..) => Tool::BounceBut,
            TileKind::Painter(..) => Tool::Painter, TileKind::Door(..) => Tool::Door,
            TileKind::Switch => Tool::Switch, TileKind::ColorSwitch(..) => Tool::ColorSwitch,
            TileKind::ColorSwitchBut(..) => Tool::ColorSwitchBut,
            _ => Tool::Floor,
        };
    }
    *play_mode = PlayMode::TestEditing;
}

#[allow(clippy::too_many_arguments)]
fn spawn_scene_entities(
    commands: &mut Commands, images: &mut Assets<Image>,
    font: &Handle<Font>, play_icon: Handle<Image>,
    floor_mesh: Handle<Mesh>, ghost_floor_mat: Handle<StandardMaterial>,
    sym_mesh: Handle<Mesh>, ghost_sym: Handle<StandardMaterial>,
    hl_mesh: Handle<Mesh>, hl_mat: Handle<StandardMaterial>,
    level_idx: usize,
) {
    // Ghost preview
    commands.spawn((
        Mesh3d(floor_mesh), MeshMaterial3d(ghost_floor_mat),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), GhostPreview, BotPuzzleEntity,
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(sym_mesh.clone()), MeshMaterial3d(ghost_sym),
            Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0))
                .with_scale(Vec3::ZERO),
            GhostSymbolOverlay,
        ));
    });
    // Highlight
    commands.spawn((
        Mesh3d(hl_mesh), MeshMaterial3d(hl_mat),
        Transform::from_xyz(0.0, FLOOR_TOP_Y + HIGHLIGHT_Y_OFFSET, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), TileHighlight, BotPuzzleEntity,
    ));
    // Light
    commands.spawn((
        DirectionalLight { illuminance: LIGHT_ILLUMINANCE, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, LIGHT_ELEVATION, LIGHT_AZIMUTH, 0.0)),
        BotPuzzleEntity,
    ));
    // Camera
    commands.spawn((
        Camera3d::default(),
        Bloom { intensity: BLOOM_INTENSITY, low_frequency_boost: BLOOM_LF_BOOST,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 1.0, ..default() },
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        BotPuzzleEntity,
    ));
    // Vignette
    let vignette = create_vignette_texture(images);
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), ..default() }, ImageNode::new(vignette), BotPuzzleEntity));
    // Play/stop button
    commands.spawn((Node { position_type: PositionType::Absolute, right: Val::Px(10.0),
        top: Val::Px(-60.0), ..default() },
        UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false }, BotPuzzleEntity,
    )).with_child((
        Button, Node { width: Val::Px(PLAY_BTN_SIZE), height: Val::Px(PLAY_BTN_SIZE),
            justify_content: JustifyContent::Center, align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(PLAY_BTN_BORDER)),
            border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() },
        BackgroundColor(slot_bg()), BorderColor::all(border_unsel()),
        PlayStopButton, ImageNode::new(play_icon), PlayButtonImage,
    ));
    // Version label
    commands.spawn((Node { position_type: PositionType::Absolute, right: Val::Px(6.0),
        bottom: Val::Px(4.0), ..default() }, BotPuzzleEntity))
        .with_child((Text::new(format!("The Repairing · v{}", env!("CARGO_PKG_VERSION"))),
            gf(VERSION_FONT, font), TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35))));
    // Level name (top left, no nav arrows)
    let level_label = format!("Level {}", level_idx + 1);
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0),
        top: Val::Px(-50.0), flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0),
        align_items: AlignItems::Center, ..default() },
        UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false },
        TestTopButtons, BotPuzzleEntity,
    )).with_children(|p| {
        p.spawn(Node { min_width: Val::Px(LEVEL_NAME_MIN_W),
            justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new(&level_label), gf(LEVEL_NAME_FONT, font),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5))));
        let mut rb = text_btn_node();
        rb.border_radius = BorderRadius::all(Val::Px(UI_CORNER_RADIUS));
        rb.margin = UiRect::left(Val::Px(8.0));
        p.spawn((Button, ResetTestButton, rb, BackgroundColor(btn_bg())))
            .with_child((Text::new("Reset"), gf(LABEL_FONT, font), TextColor(Color::WHITE)));
    });
}

/// OnExit(BotPuzzle): despawn everything, remove resources, restore MC.
pub fn exit_bot_puzzle(
    mut commands: Commands,
    bp_q: Query<Entity, With<BotPuzzleEntity>>,
    tile_q: Query<Entity, With<Tile>>,
    bot_q: Query<Entity, With<Bot>>,
    test_inv_q: Query<Entity, With<TestInventoryContainer>>,
    test_top_q: Query<Entity, With<TestTopButtons>>,
    sim_overlay_q: Query<Entity, With<crate::simulation::SimulationOverlay>>,
    ghost_trail_q: Query<Entity, With<GhostTrail>>,
    merge_flash_q: Query<Entity, With<MergeFlash>>,
    camera_q: Query<Entity, With<MissionCamera>>,
    root_ui_q: Query<Entity, (With<Node>, Without<bevy::prelude::ChildOf>)>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    for e in bp_q.iter() { commands.entity(e).despawn(); }
    for e in tile_q.iter() { commands.entity(e).despawn(); }
    for e in bot_q.iter() { commands.entity(e).despawn(); }
    for e in test_inv_q.iter() { commands.entity(e).despawn(); }
    for e in test_top_q.iter() { commands.entity(e).despawn(); }
    for e in sim_overlay_q.iter() { commands.entity(e).despawn(); }
    for e in ghost_trail_q.iter() { commands.entity(e).despawn(); }
    for e in merge_flash_q.iter() { commands.entity(e).despawn(); }

    // Remove bot puzzle resources
    commands.remove_resource::<GameAssets>();
    commands.remove_resource::<GameFont>();
    commands.remove_resource::<InventoryIcons>();
    commands.remove_resource::<PlayIcons>();
    commands.remove_resource::<BoardSize>();
    commands.remove_resource::<SelectedTool>();
    commands.remove_resource::<HoveredCell>();
    commands.remove_resource::<HiddenTileEntity>();
    commands.remove_resource::<GhostCell>();
    commands.remove_resource::<crate::test_mode::LastPlacementTracker>();
    commands.remove_resource::<InventoryState>();
    commands.remove_resource::<PlayMode>();
    commands.remove_resource::<crate::simulation::DoorToggleCount>();
    commands.remove_resource::<crate::simulation::OriginalDoorStates>();
    commands.remove_resource::<SimulationResult>();
    commands.remove_resource::<crate::bot_formation::PrevTileCounts>();
    commands.remove_resource::<SavedBoardState>();
    commands.remove_resource::<SavedTestState>();
    commands.remove_resource::<TestInventory>();
    commands.remove_resource::<LevelValidated>();
    commands.remove_resource::<BotPuzzleLevel>();
    commands.remove_resource::<crate::simulation::PlayTimer>();

    // Restore Mission Control camera + UI
    for e in camera_q.iter() {
        commands.entity(e).insert(Visibility::Visible);
    }
    for e in root_ui_q.iter() {
        commands.entity(e).insert(Visibility::Visible);
    }

    // Restore MC visual settings
    use crate::mission::constants::*;
    *clear_color = ClearColor(Color::srgb(CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2));
    *ambient = GlobalAmbientLight {
        color: Color::srgb(0.5, 0.55, 0.7), brightness: 50.0, ..default()
    };
}

/// Load a single level by index from campaign files.
fn load_bot_level(
    search_dir: &std::path::Path, idx: usize,
) -> (Option<LevelData>, Option<String>) {
    let mut json_files: Vec<_> = match std::fs::read_dir(search_dir) {
        Ok(entries) => entries.flatten()
            .filter(|e| {
                let p = e.path();
                p.extension().is_some_and(|ext| ext == "json")
                    && !p.file_name().unwrap_or_default().to_string_lossy().ends_with(".progress.json")
                    && p.file_name().unwrap_or_default() != "stats.json"
                    && p.file_name().unwrap_or_default() != "game_state.json"
                    && !p.file_name().unwrap_or_default().to_string_lossy().starts_with("profile_")
                    && p.file_name().unwrap_or_default() != "current_profile.json"
            }).collect(),
        Err(_) => return (None, None),
    };
    json_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    if json_files.is_empty() { return (None, None); }
    let actual_idx = idx.min(json_files.len() - 1);
    let entry = &json_files[actual_idx];
    let path = entry.path();
    let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let json = std::fs::read_to_string(&path).ok();
    let level = json.and_then(|j| serde_json::from_str(&j).ok());
    (level, Some(stem))
}
