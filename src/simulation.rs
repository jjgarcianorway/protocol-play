// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::tile_world_pos;

// === Simulation types (moved from types.rs for line budget) ===
#[derive(Clone, Copy, PartialEq)] #[allow(dead_code)]
pub enum BotPhase {
    Accelerating, Cruising, Decelerating(Option<Direction>),
    Rotating { entry_dir: Direction, exit_dir: Direction, progress: f32 },
    Spinning, TeleportShrink { target_col: i32, target_row: i32 }, TeleportGrow,
    FallingDecel, FallingPause(f32), Falling(f32), Crushing(f32), Stopped,
}
#[derive(Component)]
pub struct BotMovement {
    pub direction: Direction, pub color_index: usize, pub col: i32, pub row: i32,
    pub progress: f32, pub speed: f32, pub phase: BotPhase, pub spawn_index: usize, pub switch_pending: bool,
}
#[derive(Resource)] pub struct PlayTimer(pub Timer);
#[derive(Resource, Default)] pub struct DoorToggleCount(pub u32);
#[derive(Resource, Default)] pub struct OriginalDoorStates(pub Vec<(u32, u32, bool)>);
pub enum SimResult { Error(&'static str), Success }
#[derive(Resource, Default)]
pub struct SimulationResult {
    pub result: Option<SimResult>, pub overlay_spawned: bool,
    pub stop_requested: bool, pub test_success_exit: bool,
    pub stats_lines: Vec<String>,
}
#[derive(Component)] pub struct SimulationOverlay;
#[derive(Component)] pub struct SimOverlayButton;

pub fn play_stop_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<PlayStopButton>, Changed<Interaction>)>,
    mut tiles: Query<(Entity, &TileCoord, &mut TileKind, Option<&Children>), (With<Tile>, Without<DespawnAtZeroScale>)>,
    bots: Query<Entity, With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut button_image: Query<&mut ImageNode, With<PlayButtonImage>>,
    play_icons: Res<PlayIcons>,
    mut door_states: ResMut<OriginalDoorStates>,
    mut door_toggle: ResMut<DoorToggleCount>,
    mut mat_q: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut sim_result: ResMut<SimulationResult>,
    overlay: Query<Entity, With<SimulationOverlay>>,
    mut validated: ResMut<LevelValidated>,
    merge_flashes: Query<Entity, With<MergeFlash>>,
) -> Result {
    let button_pressed = interaction_query.iter().any(|i| *i == Interaction::Pressed);
    if !button_pressed && !sim_result.stop_requested { return Ok(()); }
    let can_start = *play_mode == PlayMode::Editing || *play_mode == PlayMode::TestEditing;
    let can_stop = *play_mode == PlayMode::Playing || *play_mode == PlayMode::TestPlaying;
    if button_pressed && can_start {
        let next = if *play_mode == PlayMode::Editing { PlayMode::Playing } else { PlayMode::TestPlaying };
        *play_mode = next;
        sim_result.result = None; sim_result.overlay_spawned = false;
        sim_result.stop_requested = false; sim_result.test_success_exit = false;
        sim_result.stats_lines.clear();
        commands.insert_resource(PlayTimer(Timer::from_seconds(BOT_START_DELAY, TimerMode::Once)));
        let mut img = button_image.single_mut()?;
        img.image = play_icons.stop.clone();
        door_states.0.clear();
        for (_, coord, kind, _) in &tiles {
            if let TileKind::Door(open) = *kind { door_states.0.push((coord.col, coord.row, open)); }
        }
        let mut si = 0usize;
        for (_, coord, kind, _) in &tiles {
            if let TileKind::Source(ci, dir) = *kind {
                let pos = tile_world_pos(coord.col, coord.row, board_size.0, kind);
                let by = FLOOR_TOP_Y + BOT_SIZE / 2.0;
                commands.spawn((
                    Mesh3d(assets.bot_mesh.clone()), MeshMaterial3d(assets.bot_materials[ci].clone()),
                    Transform::from_translation(Vec3::new(pos.x, by, pos.z))
                        .with_rotation(Quat::from_rotation_y(dir.rotation())).with_scale(Vec3::ZERO),
                    TargetScale(Vec3::ONE), Bot, BotFormation::default(),
                    BotMovement { direction: dir, color_index: ci, col: coord.col as i32, row: coord.row as i32,
                        progress: 0.5, speed: 0.0, phase: BotPhase::Accelerating, spawn_index: si, switch_pending: false },
                )).with_children(|p| { let ez = -(BOT_SIZE/2.0 + BOT_EYE_D/2.0 + OVERLAY_MESH_THICKNESS);
                    for ex in [-BOT_EYE_SPACING, BOT_EYE_SPACING] { p.spawn((Mesh3d(assets.eye_mesh.clone()),
                        MeshMaterial3d(assets.eye_material.clone()), Transform::from_translation(Vec3::new(ex, BOT_EYE_Y_OFFSET, ez)))); }
                });
                si += 1;
            }
        }
        for (e, _, kind, _) in &tiles { if matches!(*kind, TileKind::Empty) { commands.entity(e).insert(TargetScale(Vec3::ZERO)); } }
    } else if can_stop {
        if matches!(sim_result.result, Some(SimResult::Success))
            || (*play_mode == PlayMode::TestPlaying && sim_result.test_success_exit) { validated.0 = true; }
        *play_mode = if *play_mode == PlayMode::Playing { PlayMode::Editing } else { PlayMode::TestEditing };
        sim_result.stop_requested = false; sim_result.test_success_exit = false;
        sim_result.result = None; sim_result.overlay_spawned = false;
        door_toggle.0 = 0;
        button_image.single_mut()?.image = play_icons.play.clone();
        for entity in &bots { commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale)); }
        for e in &overlay { commands.entity(e).despawn(); }
        for e in &merge_flashes { commands.entity(e).despawn(); }
        commands.remove_resource::<PlayTimer>();
        for (e, _, kind, _) in tiles.iter() { if matches!(*kind, TileKind::Empty) { commands.entity(e).insert(TargetScale(Vec3::ONE)); } }
        for (col, row, was_open) in door_states.0.drain(..) {
            for (_, coord, mut kind, children) in &mut tiles {
                if coord.col == col && coord.row == row {
                    if let TileKind::Door(ref mut open) = *kind { if *open != was_open {
                        *open = was_open;
                        let mat = if was_open { assets.door_open_material.clone() }
                            else { assets.door_closed_material.clone() };
                        if let Some(children) = children {
                            for child in children.iter() {
                                if let Ok(mut m) = mat_q.get_mut(child) { m.0 = mat.clone(); }
                            }
                        }
                    }} break;
                }
            }
        }
    }
    Ok(())
}

pub fn move_bots(
    time: Res<Time>,
    play_mode: Res<PlayMode>,
    mut play_timer: Option<ResMut<PlayTimer>>,
    board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut bots: Query<(&mut Transform, &mut BotMovement, &mut TargetScale), With<Bot>>,
    mut door_toggle: ResMut<DoorToggleCount>,
) {
    if *play_mode != PlayMode::Playing && *play_mode != PlayMode::TestPlaying { return; }
    if let Some(ref mut timer) = play_timer { timer.0.tick(time.delta()); if !timer.0.is_finished() { return; } }
    let dt = time.delta_secs();
    let half = (board_size.0 as f32 - 1.0) / 2.0;
    let bot_y = FLOOR_TOP_Y + BOT_SIZE / 2.0;
    let tile_at = |col: i32, row: i32| -> Option<TileKind> {
        if col < 0 || row < 0 || col >= board_size.0 as i32 || row >= board_size.0 as i32 { return None; }
        tiles.iter().find(|(c, _)| c.col == col as u32 && c.row == row as u32).map(|(_, k)| *k)
    };

    for (mut transform, mut mov, mut target_scale) in &mut bots {
        match mov.phase {
            BotPhase::Stopped => continue,
            BotPhase::Accelerating => {
                mov.speed = (mov.speed + BOT_ACCEL * dt).min(BOT_CRUISE_SPEED);
                mov.progress += mov.speed * dt;
                if mov.speed >= BOT_CRUISE_SPEED { mov.phase = BotPhase::Cruising; }
            }
            BotPhase::Cruising => { mov.progress += BOT_CRUISE_SPEED * dt; }
            BotPhase::Decelerating(_) => {
                mov.speed = (mov.speed - BOT_ACCEL * dt).max(0.0); mov.progress += mov.speed * dt;
                if mov.speed == 0.0 && mov.progress < 0.5 { mov.progress = 0.5; }
                if mov.progress >= 0.5 { mov.progress = 0.5; mov.speed = 0.0; match mov.phase {
                    BotPhase::Decelerating(Some(exit_dir)) => {
                        mov.phase = BotPhase::Rotating { entry_dir: mov.direction, exit_dir, progress: 0.0 };
                    }
                    BotPhase::Decelerating(None) => {
                        let tp = match tile_at(mov.col, mov.row) {
                            Some(TileKind::Teleport(co, num)) => tiles.iter()
                                .find(|(c, k)| matches!(k, TileKind::Teleport(co2, n) if *co2 == co && *n == num)
                                    && (c.col as i32 != mov.col || c.row as i32 != mov.row))
                                .map(|(c, _)| (c.col as i32, c.row as i32)),
                            Some(TileKind::TeleportBut(co, num)) => tiles.iter()
                                .find(|(c, k)| matches!(k, TileKind::TeleportBut(co2, n) if *co2 == co && *n == num)
                                    && (c.col as i32 != mov.col || c.row as i32 != mov.row))
                                .map(|(c, _)| (c.col as i32, c.row as i32)),
                            _ => None,
                        };
                        if let Some((tc, tr)) = tp {
                            target_scale.0 = Vec3::ZERO;
                            mov.phase = BotPhase::TeleportShrink { target_col: tc, target_row: tr };
                        } else { mov.phase = BotPhase::Spinning; }
                    }
                    _ => {} }
                }
            }
            BotPhase::FallingDecel => {
                mov.speed = (mov.speed - BOT_ACCEL * dt).max(0.0); mov.progress += mov.speed * dt;
                if mov.progress >= 0.5 { mov.progress = 0.5; mov.speed = 0.0; mov.phase = BotPhase::FallingPause(FALL_PAUSE); }
            }
            BotPhase::FallingPause(ref mut t) => { *t -= dt; if *t <= 0.0 { mov.phase = BotPhase::Falling(0.0); }
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y, mov.row as f32 - half); continue; }
            BotPhase::Falling(ref mut p) => { *p = (*p + dt / FALL_DURATION).min(1.0); let s = (1.0 - *p).max(0.0);
                transform.translation.y = bot_y - *p * *p * FALL_DISTANCE;
                transform.scale = Vec3::splat(s); target_scale.0 = Vec3::splat(s); continue; }
            BotPhase::Crushing(ref mut p) => { *p = (*p + dt / CRUSH_DURATION).min(1.0);
                let sy = (1.0 - *p).max(0.0); let sxz = 1.0 + *p * (CRUSH_EXPAND - 1.0);
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y * sy, mov.row as f32 - half);
                transform.scale = Vec3::new(sxz, sy, sxz); target_scale.0 = transform.scale; continue; }
            BotPhase::Spinning => { let bounce = (time.elapsed_secs() * BOT_BOUNCE_SPEED).sin().abs() * BOT_BOUNCE_HEIGHT;
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y + bounce, mov.row as f32 - half); continue; }
            BotPhase::TeleportShrink { target_col, target_row } => {
                if transform.scale.x < TELEPORT_SHRINK_DONE {
                    mov.col = target_col; mov.row = target_row; mov.progress = 0.5; mov.speed = 0.0;
                    transform.translation = Vec3::new(target_col as f32 - half, bot_y, target_row as f32 - half);
                    transform.scale = Vec3::ZERO; target_scale.0 = Vec3::ONE; mov.phase = BotPhase::TeleportGrow;
                } continue; }
            BotPhase::TeleportGrow => { if transform.scale.x > TELEPORT_GROW_DONE { transform.scale = Vec3::ONE; mov.phase = BotPhase::Accelerating; } continue; }
            BotPhase::Rotating { entry_dir, exit_dir, ref mut progress } => {
                *progress = (*progress + dt / BOT_TURN_DURATION).min(1.0);
                transform.rotation = Quat::from_rotation_y(entry_dir.rotation())
                    .slerp(Quat::from_rotation_y(exit_dir.rotation()), *progress);
                if *progress >= 1.0 { mov.direction = exit_dir; mov.phase = BotPhase::Accelerating; }
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y, mov.row as f32 - half); continue;
            }
        }

        if mov.progress >= 1.0 { // Tile transition
            let (dc, dr) = mov.direction.grid_delta();
            let (nc, nr) = (mov.col + dc, mov.row + dr);
            macro_rules! advance { () => { mov.col = nc; mov.row = nr; mov.progress -= 1.0; } }
            match tile_at(nc, nr) {
                Some(TileKind::Goal(ci)) if ci == mov.color_index => { advance!(); mov.phase = BotPhase::Decelerating(None); }
                Some(TileKind::Floor) | Some(TileKind::Source(_, _)) | Some(TileKind::Door(true))
                | Some(TileKind::Painter(_)) | Some(TileKind::Goal(_)) => { advance!(); }
                Some(TileKind::Switch) | Some(TileKind::ColorSwitch(_)) | Some(TileKind::ColorSwitchBut(_)) => {
                    advance!();
                    let trigger = match tile_at(mov.col, mov.row) {
                        Some(TileKind::Switch) => true,
                        Some(TileKind::ColorSwitch(ci)) => ci == mov.color_index,
                        Some(TileKind::ColorSwitchBut(ci)) => ci != mov.color_index,
                        _ => false,
                    };
                    if trigger { mov.switch_pending = true; }
                }
                Some(TileKind::Door(false)) => { advance!(); mov.phase = BotPhase::Decelerating(Some(mov.direction.opposite())); }
                Some(TileKind::TurnBut(tci, _)) if tci == mov.color_index => { advance!(); }
                Some(TileKind::BounceBut(bci)) if bci == mov.color_index => { advance!(); }
                Some(TileKind::Bounce(_)) | Some(TileKind::BounceBut(_)) => { advance!(); mov.phase = BotPhase::Decelerating(Some(mov.direction.opposite())); }
                Some(TileKind::Turn(_, tdir)) | Some(TileKind::TurnBut(_, tdir)) => {
                    if let Some(exit_dir) = mov.direction.turn_exit(tdir) {
                        advance!(); mov.phase = BotPhase::Decelerating(Some(exit_dir));
                    } else { advance!(); }
                }
                Some(TileKind::Teleport(co, _)) => {
                    let affects = co == NUM_COLORS || co == mov.color_index;
                    advance!(); if affects { mov.phase = BotPhase::Decelerating(None); }
                }
                Some(TileKind::TeleportBut(co, _)) => {
                    advance!(); if co != mov.color_index { mov.phase = BotPhase::Decelerating(None); }
                }
                Some(TileKind::ArrowBut(aci, _)) if aci == mov.color_index => { advance!(); }
                Some(TileKind::Arrow(_, adir)) | Some(TileKind::ArrowBut(_, adir)) => {
                    advance!(); if adir != mov.direction { mov.phase = BotPhase::Decelerating(Some(adir)); }
                }
                _ => { advance!(); mov.phase = BotPhase::FallingDecel; }
            }
        }
        if mov.switch_pending && mov.progress >= 0.5 { mov.switch_pending = false; door_toggle.0 += 1; }
        let (dc, dr) = mov.direction.grid_delta();
        transform.translation = Vec3::new(
            mov.col as f32 - half + (mov.progress - 0.5) * dc as f32, bot_y,
            mov.row as f32 - half + (mov.progress - 0.5) * dr as f32);
    }
}

pub fn toggle_doors(
    mut toggle: ResMut<DoorToggleCount>,
    mut tiles: Query<(&TileCoord, &mut TileKind, &Children), (With<Tile>, Without<DespawnAtZeroScale>)>,
    mut mat_q: Query<&mut MeshMaterial3d<StandardMaterial>>,
    assets: Res<GameAssets>,
    mut bots: Query<&mut BotMovement, With<Bot>>,
    mut sim_result: ResMut<SimulationResult>,
) {
    if toggle.0 == 0 { return; }
    let do_toggle = toggle.0 % 2 == 1;
    toggle.0 = 0;
    if !do_toggle { return; }
    let mut closed_doors = Vec::new();
    for (coord, mut kind, children) in &mut tiles { if let TileKind::Door(ref mut open) = *kind {
        *open = !*open;
        if !*open { closed_doors.push((coord.col, coord.row)); }
        let mat = if *open { assets.door_open_material.clone() } else { assets.door_closed_material.clone() };
        for child in children.iter() { if let Ok(mut m) = mat_q.get_mut(child) { m.0 = mat.clone(); } }
    }}
    for (col, row) in closed_doors { for mut mov in &mut bots {
        if mov.col == col as i32 && mov.row == row as i32
            && !matches!(mov.phase, BotPhase::Falling(_) | BotPhase::Stopped | BotPhase::Crushing(_))
        {
            mov.phase = BotPhase::Crushing(0.0); mov.speed = 0.0;
            if sim_result.result.is_none() { sim_result.result = Some(SimResult::Error("Bot was crushed by a door!")); }
        }
    }}
}

pub fn check_simulation_result(
    play_mode: Res<PlayMode>, bots: Query<&BotMovement, With<Bot>>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>, mut sim_result: ResMut<SimulationResult>,
) {
    if !(*play_mode == PlayMode::Playing || *play_mode == PlayMode::TestPlaying) || sim_result.result.is_some() { return; }
    let (mut count, mut all_at_goal) = (0, true);
    for mov in &bots { count += 1; match mov.phase {
        BotPhase::Falling(_) | BotPhase::Crushing(_) => {
            if sim_result.result.is_none() {
                let msg = if matches!(mov.phase, BotPhase::Crushing(_)) { "Bot was crushed by a door!" } else { "Bot fell off the board!" };
                sim_result.result = Some(SimResult::Error(msg));
            } return;
        }
        BotPhase::Spinning => {
            let ok = tiles.iter().any(|(c, k)| c.col as i32 == mov.col && c.row as i32 == mov.row
                && matches!(k, TileKind::Goal(ci) if *ci == mov.color_index));
            if !ok { all_at_goal = false; }
        }
        _ => { all_at_goal = false; }
    }}
    if count > 0 && all_at_goal { sim_result.result = Some(SimResult::Success); }
}

pub fn spawn_simulation_overlay(
    mut commands: Commands, mut sim_result: ResMut<SimulationResult>,
    existing: Query<Entity, With<SimulationOverlay>>, font: Res<GameFont>,
    play_mode: Res<PlayMode>, test_inv: Res<TestInventory>,
) {
    if sim_result.result.is_none() || sim_result.overlay_spawned || !existing.is_empty() { return; }
    sim_result.overlay_spawned = true;
    let in_test = matches!(*play_mode, PlayMode::TestPlaying);
    let pieces_left = test_inv.items.iter().map(|(_, c)| *c as usize).sum::<usize>();
    let (msg, color, btn_text) = match &sim_result.result {
        Some(SimResult::Error(s)) => (*s, rgb(SIM_ERROR_COLOR), "Stop"),
        Some(SimResult::Success) if in_test && pieces_left > 0 =>
            ("Solved with pieces to spare!", rgb(SIM_SUCCESS_COLOR), "Continue"),
        Some(SimResult::Success) => ("All bots reached their goals!", rgb(SIM_SUCCESS_COLOR), "Continue"),
        None => return,
    };
    let stats = sim_result.stats_lines.clone();
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        BackgroundColor(rgba(SIM_OVERLAY_BG)), GlobalZIndex(100), SimulationOverlay, Interaction::default(),
    )).with_children(|parent| {
        parent.spawn((
            Node { flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(SIM_CARD_PAD)),
                align_items: AlignItems::Center, row_gap: Val::Px(SIM_CARD_GAP), ..default() },
            BackgroundColor(rgb(SIM_CARD_BG)),
        )).with_children(|card| {
            card.spawn((Text::new(msg), gf(SIM_MSG_FONT, &font.0), TextColor(color)));
            for line in &stats {
                card.spawn((Text::new(line), gf(DIALOG_BODY_FONT, &font.0), TextColor(Color::WHITE)));
            }
            card.spawn((
                Button, SimOverlayButton,
                Node { padding: UiRect::axes(Val::Px(SIM_BTN_PAD.0), Val::Px(SIM_BTN_PAD.1)), ..default() },
                BackgroundColor(rgb(SIM_BTN_BG)),
            )).with_child((Text::new(btn_text), gf(SIM_BTN_FONT, &font.0), TextColor(Color::WHITE)));
        });
    });
}

pub fn overlay_button_interaction(
    q: Query<&Interaction, (With<SimOverlayButton>, Changed<Interaction>)>,
    mut sim_result: ResMut<SimulationResult>, play_mode: Res<PlayMode>,
    hovered: Res<HoveredCell>, mut ghost_cell: ResMut<GhostCell>,
) {
    if !q.iter().any(|i| *i == Interaction::Pressed) { return; }
    suppress_ghost(&hovered, &mut ghost_cell);
    sim_result.stop_requested = true;
    if *play_mode == PlayMode::TestPlaying && matches!(sim_result.result, Some(SimResult::Success)) { sim_result.test_success_exit = true; }
}

pub fn paint_bots(
    mut commands: Commands, time: Res<Time>, play_mode: Res<PlayMode>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut bots: Query<(Entity, &mut BotMovement, &mut MeshMaterial3d<StandardMaterial>,
        Option<&mut BotColorTransition>), With<Bot>>,
    mut materials: ResMut<Assets<StandardMaterial>>, assets: Res<GameAssets>,
) {
    if *play_mode != PlayMode::Playing && *play_mode != PlayMode::TestPlaying { return; }
    let dt = time.delta_secs();
    for (entity, mut mov, mut mat, transition) in &mut bots {
        if let Some(mut tr) = transition {
            tr.progress = (tr.progress + PAINT_TRANSITION_SPEED * dt).min(1.0);
            if let Some(m) = materials.get_mut(&tr.material) {
                let (fr, fg, fb) = SOURCE_COLORS[tr.from_color];
                let (tor, tog, tob) = SOURCE_COLORS[tr.to_color];
                let t = tr.progress;
                m.base_color = Color::srgb(fr + (tor - fr) * t, fg + (tog - fg) * t, fb + (tob - fb) * t);
            }
            if tr.progress >= 1.0 {
                *mat = MeshMaterial3d(assets.bot_materials[tr.to_color].clone());
                commands.entity(entity).remove::<BotColorTransition>();
            }
        } else if matches!(mov.phase, BotPhase::Cruising | BotPhase::Accelerating) {
            if let Some(TileKind::Painter(ci)) = tiles.iter()
                .find(|(c, _)| c.col as i32 == mov.col && c.row as i32 == mov.row).map(|(_, k)| *k)
            { if ci != mov.color_index {
                    let old_color = mov.color_index;
                    let (r, g, b) = SOURCE_COLORS[old_color];
                    let unique = materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() });
                    *mat = MeshMaterial3d(unique.clone());
                    // Update logical color immediately — visual transition is cosmetic only
                    mov.color_index = ci;
                    commands.entity(entity).insert(BotColorTransition {
                        from_color: old_color, to_color: ci, progress: 0.0, material: unique,
                    });
            }}
        }
    }
}
