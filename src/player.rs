// SPDX-License-Identifier: GPL-3.0-or-later
use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;
use crate::test_mode::{group_tiles, spawn_test_inventory};
use crate::simulation::SimulationResult;
#[path = "player_progress.rs"] mod player_progress;
use player_progress::*;

#[derive(Resource)]
pub struct PlayerLevels {
    pub levels: Vec<LevelData>,
    pub current: usize,
}

#[derive(Component)] pub struct PrevLevelButton;
#[derive(Component)] pub struct NextLevelButton;
#[derive(Component)] pub struct LevelNameText;
#[derive(Component)] pub struct CongratsScreen;

#[derive(Resource, Default)]
pub struct LevelStats {
    pub editing_time: f32,
    pub play_count: u32,
    pub reset_count: u32,
    pub last_stats_write: f32,
}

fn chapter_bg(level_idx: usize) -> Color {
    let ch = if level_idx < 132 { level_idx / 11 } else { 12 };
    let c = CHAPTER_COLORS[ch.min(12)];
    Color::srgb(c.0, c.1, c.2)
}

pub fn setup_player(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    mut test_inv: ResMut<TestInventory>,
    icons: Res<InventoryIcons>,
    font: Res<GameFont>,
    mut play_mode: ResMut<PlayMode>,
    mut clear_color: ResMut<ClearColor>,
) {
    let exe_dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));
    let search_dir = exe_dir.unwrap_or_else(|| std::path::PathBuf::from("."));
    let do_reset = std::env::args().any(|a| a == "--reset-stats");

    let mut levels: Vec<LevelData> = Vec::new();
    let mut filenames: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        let mut json_files: Vec<_> = entries.flatten()
            .filter(|e| {
                let p = e.path();
                p.extension().is_some_and(|ext| ext == "json")
                    && !p.file_name().unwrap_or_default().to_string_lossy().ends_with(".progress.json")
            }).collect();
        json_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        for entry in json_files {
            let path = entry.path();
            let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            match std::fs::read_to_string(&path) {
                Ok(json) => match serde_json::from_str::<LevelData>(&json) {
                    Ok(level) => { info!("Loaded: {}", path.display()); levels.push(level); filenames.push(stem); }
                    Err(e) => { warn!("Skipping {} (parse error: {e})", path.display()); }
                },
                Err(e) => { warn!("Skipping {} (read error: {e})", path.display()); }
            }
        }
    }

    if levels.is_empty() {
        spawn_error_message(&mut commands, &font.0);
        commands.insert_resource(PlayerLevels { levels: vec![], current: 0 });
        commands.insert_resource(PlayerProgress { data: vec![], filenames: vec![], save_dir: search_dir });
        commands.insert_resource(LevelStats::default());
        return;
    }

    if do_reset {
        reset_all_progress(&search_dir, &filenames);
        println!("All progress and stats have been reset ({} levels).", filenames.len());
        std::process::exit(0);
    }
    ensure_stats_file(&search_dir);
    let progress_data: Vec<LevelProgress> = filenames.iter()
        .map(|f| load_progress(&search_dir, f)).collect();
    let start_idx = first_unsolved(&progress_data).unwrap_or(0);

    let player_levels = PlayerLevels { levels, current: start_idx };
    let progress = PlayerProgress { data: progress_data, filenames, save_dir: search_dir };
    let mut stats = LevelStats::default();

    let p = progress.data[start_idx].clone();
    for e in &tiles { commands.entity(e).despawn(); }
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons,
        &font.0, &mut play_mode, &player_levels, &p, &mut stats, true);
    clear_color.0 = chapter_bg(start_idx);

    if first_unsolved(&progress.data).is_none() { spawn_congrats(&mut commands, &font.0, &progress); }
    commands.insert_resource(player_levels);
    commands.insert_resource(progress);
    commands.insert_resource(stats);
}

fn spawn_error_message(commands: &mut Commands, f: &Handle<Font>) {
    commands.spawn(Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, flex_direction: FlexDirection::Column,
        row_gap: Val::Px(12.0), ..default() })
    .with_children(|p| {
        p.spawn((Text::new("No level files found"), gf(DIALOG_TITLE_FONT, f),
            TextColor(rgb(SIM_ERROR_COLOR))));
        p.spawn((Text::new("Place .json level files next to the executable."),
            gf(DIALOG_BODY_FONT, f), TextColor(Color::WHITE)));
    });
}

fn load_level(
    commands: &mut Commands, assets: &GameAssets, board_size: &mut BoardSize,
    test_inv: &mut TestInventory, icons: &InventoryIcons, font: &Handle<Font>,
    play_mode: &mut PlayMode, player_levels: &PlayerLevels,
    progress: &LevelProgress, stats: &mut LevelStats, first_load: bool,
) {
    let level = &player_levels.levels[player_levels.current];
    board_size.0 = level.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);

    *stats = LevelStats { editing_time: progress.stats.editing_time,
        play_count: progress.stats.play_count, reset_count: progress.stats.reset_count,
        last_stats_write: 0.0 };

    if progress.completed {
        let mut board_tiles: Vec<(u32, u32, TileKind)> = Vec::new();
        for &(col, row, kind, is_marked) in &level.tiles {
            if col >= board_size.0 || row >= board_size.0 { continue; }
            board_tiles.push((col, row, if is_marked { TileKind::Empty } else { kind }));
        }
        let mut grid = std::collections::HashSet::new();
        for &(col, row, _) in &board_tiles { grid.insert((col, row)); }
        for row in 0..board_size.0 { for col in 0..board_size.0 {
            if !grid.contains(&(col, row)) { board_tiles.push((col, row, TileKind::Empty)); }
        }}
        let placed_set: std::collections::HashSet<(u32, u32)> = progress.board_state.as_ref()
            .map(|bs| bs.iter().map(|&(c, r, _)| (c, r)).collect()).unwrap_or_default();
        if let Some(ref saved) = progress.board_state {
            for &(sc, sr, sk) in saved {
                if let Some(bt) = board_tiles.iter_mut().find(|(c, r, _)| *c == sc && *r == sr) { bt.2 = sk; }
            }
        }
        for &(col, row, kind) in &board_tiles {
            let e = spawn_tile(commands, col, row, board_size.0, kind, assets);
            if placed_set.contains(&(col, row)) {
                commands.entity(e).with_children(|parent| {
                    parent.spawn((Mesh3d(assets.marker_mesh.clone()), MeshMaterial3d(assets.marker_material.clone()),
                        Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0))));
                });
            }
        }
        test_inv.items.clear(); test_inv.selected = None; test_inv.remove_mode = false;
        commands.insert_resource(SavedTestState { tiles: vec![], inventory: vec![] });
        spawn_player_buttons(commands, font, player_levels, progress, first_load);
        *play_mode = PlayMode::Playing;
        return;
    }

    let (mut board_tiles, mut marked_kinds) = (Vec::new(), Vec::new());
    for &(col, row, kind, is_marked) in &level.tiles {
        if col >= board_size.0 || row >= board_size.0 { continue; }
        if is_marked { marked_kinds.push(kind); board_tiles.push((col, row, TileKind::Empty)); }
        else { board_tiles.push((col, row, kind)); }
    }
    let mut grid = std::collections::HashSet::new();
    for &(col, row, _) in &board_tiles { grid.insert((col, row)); }
    for row in 0..board_size.0 { for col in 0..board_size.0 {
        if !grid.contains(&(col, row)) { board_tiles.push((col, row, TileKind::Empty)); }
    }}

    let default_inv = group_tiles(marked_kinds.into_iter());
    commands.insert_resource(SavedTestState { tiles: board_tiles.clone(), inventory: default_inv.clone() });
    if let Some(ref saved) = progress.board_state {
        for &(sc, sr, sk) in saved {
            if let Some(bt) = board_tiles.iter_mut().find(|(c, r, _)| *c == sc && *r == sr) { bt.2 = sk; }
        }
    }

    for &(col, row, kind) in &board_tiles { spawn_tile(commands, col, row, board_size.0, kind, assets); }

    test_inv.items = progress.inventory_state.clone().unwrap_or(default_inv);
    test_inv.selected = None; test_inv.remove_mode = false;

    spawn_test_inventory(commands, test_inv, icons, first_load, font);
    spawn_player_buttons(commands, font, player_levels, progress, first_load);
    *play_mode = PlayMode::TestEditing;
}

fn spawn_player_buttons(commands: &mut Commands, f: &Handle<Font>, levels: &PlayerLevels,
    progress: &LevelProgress, animate: bool,
) {
    let (tf, tc) = (gf(LABEL_FONT, f), TextColor(Color::WHITE));
    let mut btn = text_btn_node(); btn.border_radius = BorderRadius::all(Val::Px(UI_CORNER_RADIUS));
    let nav = Node { padding: UiRect::axes(Val::Px(TEXT_BTN_PAD.0), Val::Px(TEXT_BTN_PAD.1)), border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() };
    let level = &levels.levels[levels.current];
    let suffix = if progress.completed { " (completed)" }
        else if progress.board_state.is_some() { " (in progress)" } else { "" };
    let label = format!("{}{suffix} ({}/{})", level.name, levels.current + 1, levels.levels.len());

    let start_top = if animate { -50.0 } else { TOP_SLIDE_SHOW };
    let mut ec = commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(start_top),
        flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0), align_items: AlignItems::Center, ..default() },
        TestTopButtons));
    if animate { ec.insert(UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false }); }
    ec.with_children(|p| {
        if levels.levels.len() > 1 {
            p.spawn((Button, PrevLevelButton, nav.clone(), BackgroundColor(btn_bg())))
                .with_child((Text::new("<"), gf(NAV_ARROW_FONT, f), tc));
        }
        p.spawn(Node { min_width: Val::Px(LEVEL_NAME_MIN_W), justify_content: JustifyContent::Center, ..default() })
            .with_child((Text::new(&label), gf(LEVEL_NAME_FONT, f), tc, LevelNameText));
        if levels.levels.len() > 1 {
            p.spawn((Button, NextLevelButton, nav, BackgroundColor(btn_bg())))
                .with_child((Text::new(">"), gf(NAV_ARROW_FONT, f), tc));
        }
        if progress.completed {
            let s = &progress.stats; let secs = s.editing_time as u64;
            let mut stat_str = format!("{}:{:02} · {} attempts", secs / 60, secs % 60, s.play_count);
            if s.reset_count > 0 { stat_str += &format!(" · {} resets", s.reset_count); }
            p.spawn(Node::default()).with_child((Text::new(stat_str),
                gf(LEVEL_NAME_FONT, f), TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5))));
        } else {
            let mut rb = btn.clone(); rb.margin = UiRect::left(Val::Px(8.0));
            p.spawn((Button, ResetTestButton, rb, BackgroundColor(btn_bg())))
                .with_child((Text::new("Reset"), tf, tc));
        }
    });
}

fn spawn_congrats(commands: &mut Commands, f: &Handle<Font>, progress: &PlayerProgress) {
    let (tt, ta, tr) = progress.data.iter().fold((0.0f32, 0u32, 0u32), |(t, a, r), p| (t + p.stats.editing_time, a + p.stats.play_count, r + p.stats.reset_count));
    let (secs, tc, bf) = (tt as u64, TextColor(Color::WHITE), gf(DIALOG_BODY_FONT, f));
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        BackgroundColor(rgba(SIM_OVERLAY_BG)), GlobalZIndex(150), CongratsScreen, Interaction::default(),
    )).with_children(|parent| {
        parent.spawn((Node { flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(SIM_CARD_PAD)),
            align_items: AlignItems::Center, row_gap: Val::Px(SIM_CARD_GAP), ..default() },
            BackgroundColor(rgb(SIM_CARD_BG)),
        )).with_children(|card| {
            card.spawn((Text::new("Congratulations!"), gf(SIM_MSG_FONT, f), TextColor(rgb(SIM_SUCCESS_COLOR))));
            card.spawn((Text::new("All levels completed!"), gf(DIALOG_TITLE_FONT, f), tc));
            card.spawn((Text::new(format!("Total time: {}:{:02}", secs / 60, secs % 60)), bf.clone(), tc));
            card.spawn((Text::new(format!("Total attempts: {ta}")), bf.clone(), tc));
            if tr > 0 { card.spawn((Text::new(format!("Total resets: {tr}")), bf, tc)); }
        });
    });
}

pub fn player_nav_interaction(
    mut commands: Commands, mut levels: ResMut<PlayerLevels>,
    prev_q: Query<&Interaction, (With<PrevLevelButton>, Changed<Interaction>)>,
    next_q: Query<&Interaction, (With<NextLevelButton>, Changed<Interaction>)>,
    tiles: Query<Entity, With<Tile>>, assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>, mut test_inv: ResMut<TestInventory>,
    icons: Res<InventoryIcons>, font: Res<GameFont>, mut play_mode: ResMut<PlayMode>,
    cleanup: Query<Entity, Or<(With<TestInventoryContainer>, With<TestTopButtons>, With<Bot>, With<CongratsScreen>)>>,
    mut stats: ResMut<LevelStats>, progress: Res<PlayerProgress>,
    mut clear_color: ResMut<ClearColor>,
) {
    if levels.levels.is_empty() { return; }
    let d = if prev_q.iter().any(|i| *i == Interaction::Pressed) { -1 }
        else if next_q.iter().any(|i| *i == Interaction::Pressed) { 1 } else { return };
    if !matches!(*play_mode, PlayMode::TestEditing | PlayMode::Playing) { return; }
    let live = ProgressStats { editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
    save_stats_summary(&progress.save_dir, &progress.filenames, &levels.levels, &progress.data, levels.current, &live);
    let next = next_level(&progress.data, levels.current, d);
    levels.current = next;
    for e in &cleanup { commands.entity(e).despawn(); }
    for e in &tiles { commands.entity(e).despawn(); }
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons,
        &font.0, &mut play_mode, &levels, &progress.data[next].clone(), &mut stats, false);
    clear_color.0 = chapter_bg(next);
}

pub fn update_player_stats(
    time: Res<Time>, play_mode: Res<PlayMode>, mut stats: ResMut<LevelStats>,
    reset_q: Query<&Interaction, (With<ResetTestButton>, Changed<Interaction>)>,
    progress: Res<PlayerProgress>, levels: Res<PlayerLevels>,
) {
    if levels.levels.is_empty() { return; }
    if progress.data[levels.current].completed { return; }
    let mut changed = false;
    if *play_mode == PlayMode::TestEditing { stats.editing_time += time.delta_secs(); }
    if play_mode.is_changed() && *play_mode == PlayMode::TestPlaying { stats.play_count += 1; changed = true; }
    if *play_mode == PlayMode::TestEditing && reset_q.iter().any(|i| *i == Interaction::Pressed) {
        stats.reset_count += 1; changed = true;
    }
    if changed || stats.editing_time - stats.last_stats_write >= STATS_WRITE_INTERVAL {
        stats.last_stats_write = stats.editing_time;
        let live = ProgressStats { editing_time: stats.editing_time,
            play_count: stats.play_count, reset_count: stats.reset_count };
        save_stats_summary(&progress.save_dir, &progress.filenames,
            &levels.levels, &progress.data, levels.current, &live);
    }
}

pub fn auto_save_progress(
    test_inv: Res<TestInventory>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    levels: Res<PlayerLevels>,
    stats: Res<LevelStats>,
    mut progress: ResMut<PlayerProgress>,
    play_mode: Res<PlayMode>,
    saved_test: Res<SavedTestState>,
) {
    if levels.levels.is_empty() { return; }
    if !test_inv.is_changed() || *play_mode != PlayMode::TestEditing { return; }
    let idx = levels.current;
    if progress.data[idx].completed { return; }

    let placements: Vec<(u32, u32, TileKind)> = tiles.iter()
        .filter(|(c, k)| !matches!(k, TileKind::Empty)
            && saved_test.tiles.iter().any(|&(sc, sr, sk)| sc == c.col && sr == c.row && matches!(sk, TileKind::Empty)))
        .map(|(c, k)| (c.col, c.row, *k)).collect();

    progress.data[idx].board_state = if placements.is_empty() { None } else { Some(placements) };
    progress.data[idx].inventory_state = Some(test_inv.items.clone());
    progress.data[idx].stats = ProgressStats {
        editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
    save_one(&progress, idx);
    save_stats_summary(&progress.save_dir, &progress.filenames,
        &levels.levels, &progress.data, idx, &progress.data[idx].stats);
}

pub fn handle_level_complete(
    mut commands: Commands, mut validated: ResMut<LevelValidated>,
    mut progress: ResMut<PlayerProgress>, mut levels: ResMut<PlayerLevels>,
    mut stats: ResMut<LevelStats>,
    tile_q: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>, mut board_size: ResMut<BoardSize>,
    mut test_inv: ResMut<TestInventory>, icons: Res<InventoryIcons>,
    font: Res<GameFont>, mut play_mode: ResMut<PlayMode>,
    cleanup: Query<Entity, Or<(With<TestInventoryContainer>, With<TestTopButtons>)>>,
    saved_test: Res<SavedTestState>, mut clear_color: ResMut<ClearColor>,
) {
    if levels.levels.is_empty() { return; }
    if !validated.is_changed() || !validated.0 { return; }
    validated.0 = false;
    let idx = levels.current;
    if progress.data[idx].completed { return; }
    let saved_set: std::collections::HashSet<(u32, u32)> = saved_test.tiles.iter()
        .filter(|(_, _, k)| !matches!(k, TileKind::Empty)).map(|(c, r, _)| (*c, *r)).collect();
    let placed: Vec<_> = tile_q.iter()
        .filter(|(_, c, k)| !matches!(k, TileKind::Empty) && !saved_set.contains(&(c.col, c.row)))
        .map(|(_, c, k)| (c.col, c.row, *k)).collect();
    let creative = is_creative_solution(&levels.levels[idx].solution, &placed);
    progress.data[idx].completed = true; progress.data[idx].creative_solution = creative;
    progress.data[idx].inventory_state = None;
    progress.data[idx].stats = ProgressStats { editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
    save_one(&progress, idx);
    append_stats_log(&progress.save_dir, &progress.filenames[idx], &levels.levels[idx].name, &progress.data[idx].stats, creative);
    let next = first_unsolved(&progress.data).unwrap_or(idx);
    levels.current = next;
    for e in &cleanup { commands.entity(e).despawn(); }
    for (e, _, _) in &tile_q { commands.entity(e).despawn(); }
    let p = progress.data[next].clone();
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons, &font.0, &mut play_mode, &levels, &p, &mut stats, false);
    clear_color.0 = chapter_bg(next);
    if first_unsolved(&progress.data).is_none() { spawn_congrats(&mut commands, &font.0, &progress); }
}

pub fn populate_stats(
    mut sim_result: ResMut<SimulationResult>,
    stats: Res<LevelStats>,
    levels: Res<PlayerLevels>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    saved_test: Res<SavedTestState>,
) {
    if levels.levels.is_empty() { return; }
    if !matches!(sim_result.result, Some(crate::simulation::SimResult::Success)) { return; }
    if sim_result.overlay_spawned || !sim_result.stats_lines.is_empty() { return; }

    let secs = stats.editing_time as u64;
    sim_result.stats_lines.push(format!("Time: {}:{:02}", secs / 60, secs % 60));
    sim_result.stats_lines.push(format!("Attempts: {}", stats.play_count));
    if stats.reset_count > 0 {
        sim_result.stats_lines.push(format!("Resets: {}", stats.reset_count));
    }

    let saved_set: std::collections::HashSet<(u32, u32)> = saved_test.tiles.iter()
        .filter(|(_, _, k)| !matches!(k, TileKind::Empty)).map(|(c, r, _)| (*c, *r)).collect();
    let placed: Vec<_> = tiles.iter()
        .filter(|(c, k)| !matches!(k, TileKind::Empty) && !saved_set.contains(&(c.col, c.row)))
        .map(|(c, k)| (c.col, c.row, *k)).collect();
    if is_creative_solution(&levels.levels[levels.current].solution, &placed) {
        sim_result.stats_lines.push("Creative solution!".into());
    }
}
