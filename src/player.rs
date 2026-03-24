// SPDX-License-Identifier: GPL-3.0-or-later
use bevy::prelude::*;
use crate::constants::*; use crate::types::*; use crate::ui_helpers::*;
use crate::board::spawn_tile;
use crate::test_mode::{group_tiles, spawn_test_inventory, set_tool_from_kind};
use crate::simulation::SimulationResult;
use crate::messages::{pick_creative_msg, pick_congrats, format_time, format_attempts, format_resets, format_stars, format_solution_count, star_label};
use crate::i18n::Translations;
use crate::player_settings::PlayerSettings;
#[path = "player_progress.rs"] mod player_progress; use player_progress::*;
#[path = "player_chapter.rs"] pub mod player_chapter; pub use player_chapter::*;

#[derive(Component)] pub struct SpeedHudBtn(pub f32);
#[derive(Component)] pub struct SpeedHudContainer;

#[derive(Resource)]
pub struct PlayerLevels { pub levels: Vec<LevelData>, pub current: usize }
#[derive(Component)] pub struct PrevLevelButton;
#[derive(Component)] pub struct NextLevelButton;
#[derive(Component)] pub struct LevelNameText;
#[derive(Component)] pub struct CongratsScreen;
#[derive(Resource, Default)]
pub struct LevelStats {
    pub editing_time: f32, pub play_count: u32,
    pub reset_count: u32, pub last_stats_write: f32,
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
    mut selected_tool: ResMut<SelectedTool>,
    mut ch_state: ResMut<ChapterState>,
    t: Res<Translations>,
    t_settings: Res<PlayerSettings>,
) {
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()));
    let search_dir = exe_dir.unwrap_or_else(|| std::path::PathBuf::from("."));
    let do_reset = std::env::args().any(|a| a == "--reset-stats");
    let mut levels: Vec<LevelData> = Vec::new();
    let mut filenames: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&search_dir) {
        let mut json_files: Vec<_> = entries.flatten()
            .filter(|e| {
                let name = e.path().file_name().unwrap_or_default().to_string_lossy().to_string();
                // Campaign levels match pattern: NN_NN_name.json (e.g. "01_03_the_zigzag.json")
                name.ends_with(".json") && name.len() > 6
                    && name.as_bytes()[2] == b'_' && name.as_bytes()[0].is_ascii_digit()
            }).collect();
        json_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        for entry in json_files {
            let path = entry.path();
            let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
            match std::fs::read_to_string(&path) {
                Ok(json) => match serde_json::from_str::<LevelData>(&json) {
                    Ok(level) => { levels.push(level); filenames.push(stem); }
                    Err(e) => { warn!("Skipping {} (parse error: {e})", path.display()); }
                },
                Err(e) => { warn!("Skipping {} (read error: {e})", path.display()); }
            }
        }
    }

    spawn_speed_hud(&mut commands, &font.0, &t_settings);
    if levels.is_empty() {
        spawn_error_message(&mut commands, &font.0, &t);
        commands.insert_resource(PlayerLevels { levels: vec![], current: 0 });
        commands.insert_resource(PlayerProgress { data: vec![], filenames: vec![], save_dir: search_dir });
        return commands.insert_resource(LevelStats::default());
    }
    if do_reset {
        reset_all_progress(&search_dir, &filenames);
        println!("All progress and stats have been reset ({} levels).", filenames.len());
        std::process::exit(0);
    }
    ensure_stats_file(&search_dir);
    let progress_data: Vec<LevelProgress> = filenames.iter().map(|f| load_progress(&search_dir, f)).collect();
    let start_idx = first_unsolved(&progress_data).unwrap_or(0);
    let player_levels = PlayerLevels { levels, current: start_idx };
    let progress = PlayerProgress { data: progress_data, filenames, save_dir: search_dir };
    let mut stats = LevelStats::default();
    let p = progress.data[start_idx].clone();
    for e in &tiles { commands.entity(e).despawn(); }
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons,
        &font.0, &mut play_mode, &player_levels, &p, &mut stats, true, &mut selected_tool, &t);
    set_chapter(start_idx, &mut commands, &font.0, &mut ch_state, &t);
    clear_color.0 = ch_state.bg_target;
    if first_unsolved(&progress.data).is_none() { spawn_congrats(&mut commands, &font.0, &progress, &t); }
    commands.insert_resource(player_levels); commands.insert_resource(progress);
    commands.insert_resource(stats);
}

fn spawn_error_message(commands: &mut Commands, f: &Handle<Font>, t: &Translations) {
    commands.spawn(Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, flex_direction: FlexDirection::Column, row_gap: Val::Px(12.0), ..default() })
    .with_children(|p| {
        p.spawn((Text::new(t.ui_or("no_levels", "No level files found").to_string()), gf(DIALOG_TITLE_FONT, f), TextColor(rgb(SIM_ERROR_COLOR))));
        p.spawn((Text::new(t.ui_or("place_json", "Place .json level files next to the executable.").to_string()), gf(DIALOG_BODY_FONT, f), TextColor(Color::WHITE)));
    });
}

fn load_level(
    commands: &mut Commands, assets: &GameAssets, board_size: &mut BoardSize,
    test_inv: &mut TestInventory, icons: &InventoryIcons, font: &Handle<Font>,
    play_mode: &mut PlayMode, player_levels: &PlayerLevels,
    progress: &LevelProgress, stats: &mut LevelStats, first_load: bool,
    selected_tool: &mut SelectedTool, t: &Translations,
) {
    let level = &player_levels.levels[player_levels.current];
    board_size.0 = level.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);

    let s = &progress.stats;
    *stats = LevelStats { editing_time: s.editing_time, play_count: s.play_count,
        reset_count: s.reset_count, last_stats_write: 0.0 };
    let parse_tiles = |marks_as_empty: bool| -> (Vec<(u32, u32, TileKind)>, Vec<TileKind>) {
        let (mut bt, mut mk) = (Vec::new(), Vec::new());
        for &(col, row, kind, is_marked) in &level.tiles {
            if col >= board_size.0 || row >= board_size.0 { continue; }
            if is_marked && marks_as_empty { mk.push(kind); bt.push((col, row, TileKind::Empty)); }
            else { bt.push((col, row, if is_marked { TileKind::Empty } else { kind })); }
        }
        let grid: std::collections::HashSet<_> = bt.iter().map(|&(c, r, _)| (c, r)).collect();
        for row in 0..board_size.0 { for col in 0..board_size.0 {
            if !grid.contains(&(col, row)) { bt.push((col, row, TileKind::Empty)); }
        }}
        (bt, mk)
    };
    let apply_saved = |bt: &mut Vec<(u32, u32, TileKind)>, saved: &Option<Vec<(u32, u32, TileKind)>>| {
        if let Some(s) = saved { for &(sc, sr, sk) in s {
            if let Some(t) = bt.iter_mut().find(|(c, r, _)| *c == sc && *r == sr) { t.2 = sk; }
        }}
    };

    if progress.completed {
        let (mut board_tiles, _) = parse_tiles(false);
        let placed_set: std::collections::HashSet<_> = progress.board_state.as_ref()
            .map(|bs| bs.iter().map(|&(c, r, _)| (c, r)).collect()).unwrap_or_default();
        apply_saved(&mut board_tiles, &progress.board_state);
        for &(col, row, kind) in &board_tiles {
            let e = spawn_tile(commands, col, row, board_size.0, kind, assets);
            if placed_set.contains(&(col, row)) {
                commands.entity(e).with_children(|p| {
                    p.spawn((Mesh3d(assets.marker_mesh.clone()), MeshMaterial3d(assets.marker_material.clone()),
                        Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0))));
                });
            }
        }
        test_inv.items.clear(); test_inv.selected = None; test_inv.remove_mode = false;
        selected_tool.0 = Tool::Floor;
        commands.insert_resource(SavedTestState { tiles: vec![], inventory: vec![] });
        spawn_player_buttons(commands, font, player_levels, progress, first_load, t);
        *play_mode = PlayMode::Playing;
        return;
    }

    let (mut board_tiles, marked_kinds) = parse_tiles(true);
    let default_inv = group_tiles(marked_kinds.into_iter());
    commands.insert_resource(SavedTestState { tiles: board_tiles.clone(), inventory: default_inv.clone() });
    let placed_set: std::collections::HashSet<_> = progress.board_state.as_ref()
        .map(|bs| bs.iter().map(|&(c, r, _)| (c, r)).collect()).unwrap_or_default();
    apply_saved(&mut board_tiles, &progress.board_state);
    for &(col, row, kind) in &board_tiles {
        let e = spawn_tile(commands, col, row, board_size.0, kind, assets);
        if placed_set.contains(&(col, row)) {
            commands.entity(e).with_children(|p| {
                p.spawn((Mesh3d(assets.marker_mesh.clone()), MeshMaterial3d(assets.marker_material.clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + MARKER_Y_OFFSET, 0.0))));
            });
        }
    }
    test_inv.items = progress.inventory_state.clone().unwrap_or(default_inv);
    test_inv.selected = if test_inv.items.is_empty() { None } else { Some(0) };
    test_inv.remove_mode = false;
    // Set tool to match first inventory item so ghost preview works immediately
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
    spawn_test_inventory(commands, test_inv, icons, first_load, font);
    spawn_player_buttons(commands, font, player_levels, progress, first_load, t);
    *play_mode = PlayMode::TestEditing;
}

fn spawn_player_buttons(commands: &mut Commands, f: &Handle<Font>, levels: &PlayerLevels,
    progress: &LevelProgress, animate: bool, t: &Translations,
) {
    let (tf, tc) = (gf(LABEL_FONT, f), TextColor(Color::WHITE));
    let mut btn = text_btn_node(); btn.border_radius = BorderRadius::all(Val::Px(UI_CORNER_RADIUS));
    let nav = Node { padding: UiRect::axes(Val::Px(TEXT_BTN_PAD.0), Val::Px(TEXT_BTN_PAD.1)), border_radius: BorderRadius::all(Val::Px(UI_CORNER_RADIUS)), ..default() };
    let level = &levels.levels[levels.current];
    let stars = star_label(progress.stars);
    let suffix = if progress.completed {
        if stars.is_empty() { format!(" {}", t.ui_or("completed", "(completed)")) } else { format!(" {stars}") }
    } else if progress.board_state.is_some() {
        format!(" {}", t.ui_or("in_progress", "(in progress)"))
    } else {
        String::new()
    };
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
            .with_child((Text::new(&label), gf(LEVEL_NAME_FONT, f), TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)), LevelNameText));
        if levels.levels.len() > 1 {
            p.spawn((Button, NextLevelButton, nav, BackgroundColor(btn_bg())))
                .with_child((Text::new(">"), gf(NAV_ARROW_FONT, f), tc));
        }
        if progress.completed {
            let s = &progress.stats; let secs = s.editing_time as u64;
            let att_lbl = t.ui_or("attempts_short", "attempts");
            let mut stat_str = format!("{}:{:02} · {} {att_lbl}", secs / 60, secs % 60, s.play_count);
            if s.reset_count > 0 {
                let rst_lbl = t.ui_or("resets_short", "resets");
                stat_str += &format!(" · {} {rst_lbl}", s.reset_count);
            }
            p.spawn(Node::default()).with_child((Text::new(stat_str),
                gf(LEVEL_NAME_FONT, f), TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5))));
        } else {
            let mut rb = btn.clone(); rb.margin = UiRect::left(Val::Px(8.0));
            p.spawn((Button, ResetTestButton, rb, BackgroundColor(btn_bg())))
                .with_child((Text::new(t.ui_or("reset", "Reset").to_string()), tf, tc));
        }
    });
}

fn spawn_congrats(commands: &mut Commands, f: &Handle<Font>, progress: &PlayerProgress, t: &Translations) {
    let (tt, ta, tr) = progress.data.iter().fold((0.0f32, 0u32, 0u32), |(tt, a, r), p| (tt + p.stats.editing_time, a + p.stats.play_count, r + p.stats.reset_count));
    let total_stars: u32 = progress.data.iter().map(|p| p.stars as u32).sum();
    let max_stars = (progress.data.len() * 3) as u32;
    let (secs, tc, bf) = (tt as u64, TextColor(Color::WHITE), gf(DIALOG_BODY_FONT, f));
    let (ct, cm) = pick_congrats(t);
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0),
        justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        BackgroundColor(rgba(SIM_OVERLAY_BG)), GlobalZIndex(150), CongratsScreen, Interaction::default(),
    )).with_children(|parent| {
        parent.spawn((Node { flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(SIM_CARD_PAD)),
            align_items: AlignItems::Center, row_gap: Val::Px(SIM_CARD_GAP), ..default() },
            BackgroundColor(rgb(SIM_CARD_BG)),
        )).with_children(|card| {
            card.spawn((Text::new(ct), gf(SIM_MSG_FONT, f), TextColor(rgb(SIM_SUCCESS_COLOR))));
            card.spawn((Text::new(cm), gf(DIALOG_TITLE_FONT, f), tc));
            card.spawn((Text::new(format!("★  {total_stars} / {max_stars}")), gf(DIALOG_TITLE_FONT, f), TextColor(Color::srgb(1.0, 0.85, 0.2))));
            card.spawn((Text::new(format!("{} {}:{:02}", t.ui_or("total_time", "Total time:"), secs / 60, secs % 60)), bf.clone(), tc));
            card.spawn((Text::new(format!("{} {ta}", t.ui_or("total_attempts", "Total attempts:"))), bf.clone(), tc));
            if tr > 0 { card.spawn((Text::new(format!("{} {tr}", t.ui_or("total_resets", "Total resets:"))), bf, tc)); }
        });
    });
}

pub fn player_nav_interaction(
    mut commands: Commands, mut levels: ResMut<PlayerLevels>,
    nav_q: Query<(&Interaction, Has<PrevLevelButton>), (Or<(With<PrevLevelButton>, With<NextLevelButton>)>, Changed<Interaction>)>,
    tiles: Query<Entity, With<Tile>>, assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>, mut test_inv: ResMut<TestInventory>,
    icons: Res<InventoryIcons>, font: Res<GameFont>, mut play_mode: ResMut<PlayMode>,
    cleanup: Query<Entity, Or<(With<TestInventoryContainer>, With<TestTopButtons>, With<Bot>, With<CongratsScreen>)>>,
    mut stats: ResMut<LevelStats>, progress: Res<PlayerProgress>,
    mut ch_state: ResMut<ChapterState>, mut selected_tool: ResMut<SelectedTool>,
    t: Res<Translations>,
) {
    if levels.levels.is_empty() { return; }
    let d = if nav_q.iter().any(|(i, is_prev)| *i == Interaction::Pressed && is_prev) { -1 }
        else if nav_q.iter().any(|(i, is_prev)| *i == Interaction::Pressed && !is_prev) { 1 } else { return };
    if !matches!(*play_mode, PlayMode::TestEditing | PlayMode::Playing) { return; }
    let live = ProgressStats { editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
    save_stats_summary(&progress.save_dir, &progress.filenames, &levels.levels, &progress.data, levels.current, &live);
    let next = next_level(&progress.data, levels.current, d);
    levels.current = next;
    for e in &cleanup { commands.entity(e).despawn(); }
    for e in &tiles { commands.entity(e).despawn(); }
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons,
        &font.0, &mut play_mode, &levels, &progress.data[next].clone(), &mut stats, false, &mut selected_tool, &t);
    set_chapter(next, &mut commands, &font.0, &mut ch_state, &t);
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
        let live = ProgressStats { editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
        save_stats_summary(&progress.save_dir, &progress.filenames, &levels.levels, &progress.data, levels.current, &live);
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
    mut queries: ParamSet<(
        Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
        Query<Entity, Or<(With<TestInventoryContainer>, With<TestTopButtons>)>>,
    )>,
    assets: Res<GameAssets>, mut board_size: ResMut<BoardSize>,
    mut test_inv: ResMut<TestInventory>, icons: Res<InventoryIcons>,
    font: Res<GameFont>, mut play_mode: ResMut<PlayMode>,
    saved_test: Res<SavedTestState>, mut ch_state: ResMut<ChapterState>,
    mut selected_tool: ResMut<SelectedTool>,
    t: Res<Translations>,
) {
    if levels.levels.is_empty() || !validated.is_changed() || !validated.0 { return; }
    validated.0 = false;
    let idx = levels.current;
    if progress.data[idx].completed { return; }
    let saved_set: std::collections::HashSet<(u32, u32)> = saved_test.tiles.iter()
        .filter(|(_, _, k)| !matches!(k, TileKind::Empty)).map(|(c, r, _)| (*c, *r)).collect();
    let (placed, tile_entities): (Vec<_>, Vec<Entity>) = {
        let q = queries.p0();
        let placed = q.iter()
            .filter(|(_, c, k)| !matches!(k, TileKind::Empty) && !saved_set.contains(&(c.col, c.row)))
            .map(|(_, c, k)| (c.col, c.row, *k)).collect();
        let tile_ents = q.iter().map(|(e, _, _)| e).collect();
        (placed, tile_ents)
    };
    let cleanup_entities: Vec<Entity> = queries.p1().iter().collect();
    let creative = is_creative_solution(&levels.levels[idx].solution, &placed);
    let stars = compute_stars(stats.play_count, stats.reset_count);
    progress.data[idx].completed = true; progress.data[idx].creative_solution = creative;
    progress.data[idx].stars = stars; progress.data[idx].inventory_state = None;
    progress.data[idx].stats = ProgressStats { editing_time: stats.editing_time, play_count: stats.play_count, reset_count: stats.reset_count };
    save_one(&progress, idx);
    append_stats_log(&progress.save_dir, &progress.filenames[idx], &levels.levels[idx].name, &progress.data[idx].stats, creative);
    // Update cross-game save state
    let mut gs = crate::save_state::load_game_state();
    gs.bot_level = (idx as u32 + 1).max(gs.bot_level); crate::save_state::save_game_state(&gs);
    let next = first_unsolved(&progress.data).unwrap_or(idx);
    levels.current = next;
    for e in cleanup_entities { commands.entity(e).despawn(); }
    for e in tile_entities { commands.entity(e).despawn(); }
    let p = progress.data[next].clone();
    load_level(&mut commands, &assets, &mut board_size, &mut test_inv, &icons, &font.0, &mut play_mode, &levels, &p, &mut stats, false, &mut selected_tool, &t);
    set_chapter(next, &mut commands, &font.0, &mut ch_state, &t);
    if first_unsolved(&progress.data).is_none() { spawn_congrats(&mut commands, &font.0, &progress, &t); }
}

pub fn cleanup_stale_inventory(
    mut commands: Commands, containers: Query<Entity, With<TestInventoryContainer>>,
    play_mode: Res<PlayMode>,
) {
    let entities: Vec<Entity> = containers.iter().collect();
    if entities.len() <= 1 && *play_mode != PlayMode::Playing { return; }
    let keep = if *play_mode == PlayMode::Playing { 0 } else { 1 };
    for &e in &entities[..entities.len().saturating_sub(keep)] { commands.entity(e).despawn(); }
}

pub fn populate_stats(
    mut sim_result: ResMut<SimulationResult>,
    stats: Res<LevelStats>,
    levels: Res<PlayerLevels>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    saved_test: Res<SavedTestState>,
    t: Res<Translations>,
) {
    if levels.levels.is_empty() { return; }
    if !matches!(sim_result.result, Some(crate::simulation::SimResult::Success)) { return; }
    if sim_result.overlay_spawned || !sim_result.stats_lines.is_empty() { return; }
    let stars = compute_stars(stats.play_count, stats.reset_count);
    sim_result.stats_lines.push(format_stars(stars, &t));
    sim_result.stats_lines.push(format_time(stats.editing_time as u64, &t));
    sim_result.stats_lines.push(format_attempts(stats.play_count, &t));
    if stats.reset_count > 0 { sim_result.stats_lines.push(format_resets(stats.reset_count, &t)); }
    let saved_set: std::collections::HashSet<(u32, u32)> = saved_test.tiles.iter()
        .filter(|(_, _, k)| !matches!(k, TileKind::Empty)).map(|(c, r, _)| (*c, *r)).collect();
    let placed: Vec<_> = tiles.iter()
        .filter(|(c, k)| !matches!(k, TileKind::Empty) && !saved_set.contains(&(c.col, c.row)))
        .map(|(c, k)| (c.col, c.row, *k)).collect();
    let solution_count = levels.levels[levels.current].solution_count;
    // Only show "creative solution" when there are few known solutions — otherwise it's meaningless
    let few_solutions = solution_count.map_or(true, |c| c <= 3);
    if few_solutions && is_creative_solution(&levels.levels[levels.current].solution, &placed) {
        sim_result.stats_lines.push(pick_creative_msg(&t));
    }
    // Show solution count if pre-computed and meaningful
    if let Some(count) = solution_count {
        if count > 0 {
            sim_result.stats_lines.push(format_solution_count(count, &t));
        }
    }
}

/// Spawn the persistent 1×/2×/4× speed HUD in the top-right corner.
pub fn spawn_speed_hud(commands: &mut Commands, f: &Handle<Font>, settings: &PlayerSettings) {
    let active_bg   = Color::srgba(0.25, 0.48, 0.75, 0.90);
    let inactive_bg = Color::srgba(0.10, 0.12, 0.18, 0.70);
    commands.spawn((
        SpeedHudContainer,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(70.0), top: Val::Px(10.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(6.0),
            align_items: AlignItems::Center,
            ..default()
        },
        GlobalZIndex(50),
    )).with_children(|p| {
        for (val, label) in &[(1.0_f32, "1×"), (2.0_f32, "2×"), (4.0_f32, "4×")] {
            let active = (settings.sim_speed - val).abs() < 0.05;
            p.spawn((
                Button, SpeedHudBtn(*val),
                Node {
                    width: Val::Px(38.0), height: Val::Px(38.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(if active { active_bg } else { inactive_bg }),
            )).with_child((Text::new(*label),
                gf(16.0, f), TextColor(Color::srgba(1.0, 1.0, 1.0, 0.90))));
        }
    });
}

/// Handle 1×/2×/4× speed button clicks in the in-game HUD.
pub fn speed_hud_interaction(
    mut btn_q: Query<(Entity, &SpeedHudBtn, &Interaction, &mut BackgroundColor)>,
    mut settings: ResMut<PlayerSettings>,
) {
    let active_bg   = Color::srgba(0.25, 0.48, 0.75, 0.90);
    let inactive_bg = Color::srgba(0.10, 0.12, 0.18, 0.70);
    // Find which button was pressed
    let pressed = btn_q.iter().find_map(|(_, btn, i, _)| {
        if *i == Interaction::Pressed && (settings.sim_speed - btn.0).abs() > 0.05 {
            Some(btn.0)
        } else { None }
    });
    if let Some(new_speed) = pressed {
        settings.sim_speed = new_speed;
        crate::player_settings::save_player_settings(&settings);
        for (_, sb, _, mut bg) in btn_q.iter_mut() {
            let active = (settings.sim_speed - sb.0).abs() < 0.05;
            bg.0 = if active { active_bg } else { inactive_bg };
        }
    }
}

/// Scale virtual time during bot simulation so players can watch at their preferred speed.
/// Resets to 1× when not simulating so UI and menu animations stay normal.
pub fn apply_sim_speed(
    play_mode: Res<PlayMode>,
    settings: Res<PlayerSettings>,
    mut vtime: ResMut<Time<Virtual>>,
) {
    let target = if *play_mode == PlayMode::TestPlaying { settings.sim_speed } else { 1.0 };
    if (vtime.relative_speed() - target).abs() > 0.01 {
        vtime.set_relative_speed(target);
    }
}

pub fn update_version_label(
    levels: Res<PlayerLevels>,
    mut label: Query<&mut Text, With<VersionLabel>>,
) {
    if !levels.is_changed() || levels.levels.is_empty() { return; }
    let level = &levels.levels[levels.current];
    let seed = level.seed.map(|s| format!("{:08X}", s)).unwrap_or_default();
    for mut text in &mut label {
        **text = format!("protocol play: repairing · v{} · {seed}", env!("CARGO_PKG_VERSION"));
    }
}
