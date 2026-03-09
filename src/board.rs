// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

// === Helpers ===
pub fn camera_direction() -> Vec3 {
    let elev = CAMERA_ELEVATION.to_radians();
    let azim = CAMERA_AZIMUTH.to_radians();
    Vec3::new(elev.cos() * azim.sin(), elev.sin(), elev.cos() * azim.cos())
}

pub fn board_bounding_radius(size: u32) -> f32 {
    let half = size as f32 / 2.0;
    (half * half + 0.35 * 0.35 + half * half).sqrt()
}

pub fn tile_world_pos(col: u32, row: u32, board_size: u32, kind: &TileKind) -> Vec3 {
    let offset = (board_size as f32 - 1.0) / 2.0;
    let y = match kind {
        TileKind::Empty => EMPTY_MARKER_Y,
        TileKind::Floor | TileKind::Source(_, _) | TileKind::Goal(_)
        | TileKind::Turn(_, _) | TileKind::TurnBut(_, _) | TileKind::Teleport(_)
        | TileKind::Bounce(_) | TileKind::BounceBut(_) => 0.0,
    };
    Vec3::new(col as f32 - offset, y, row as f32 - offset)
}

// === Spawning ===
pub fn spawn_tile(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets,
) {
    spawn_tile_at_scale(commands, col, row, board_size, kind, assets, Vec3::ZERO);
}

pub fn spawn_tile_at_scale(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets, initial_scale: Vec3,
) {
    let pos = tile_world_pos(col, row, board_size, &kind);
    match kind {
        TileKind::Empty => {
            commands.spawn((
                Mesh3d(assets.empty_mesh.clone()), MeshMaterial3d(assets.empty_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
        }
        TileKind::Floor => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
        }
        TileKind::Source(ci, dir) => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.source_symbol_mesh.clone()),
                    MeshMaterial3d(assets.source_symbol_materials[ci].clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            });
        }
        TileKind::Goal(ci) | TileKind::Teleport(ci) | TileKind::Bounce(ci) | TileKind::BounceBut(ci) => {
            let mat = match kind {
                TileKind::Goal(_) => assets.goal_symbol_materials[ci].clone(),
                TileKind::Teleport(_) => assets.teleport_symbol_materials[ci].clone(),
                TileKind::Bounce(_) => assets.bounce_symbol_materials[ci].clone(),
                _ => assets.bouncebot_symbol_materials[ci].clone(),
            };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.goal_symbol_mesh.clone()), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            });
        }
        TileKind::Turn(ci, dir) | TileKind::TurnBut(ci, dir) => {
            let (mesh, mat) = if matches!(kind, TileKind::Turn(_, _)) {
                (assets.turn_symbol_mesh.clone(), assets.turn_symbol_materials[ci].clone())
            } else {
                (assets.turnbut_symbol_mesh.clone(), assets.turnbut_symbol_materials[ci].clone())
            };
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(mesh), MeshMaterial3d(mat),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
                ));
            });
        }
    }
}

pub fn spawn_board(commands: &mut Commands, size: u32, assets: &GameAssets) {
    for row in 0..size {
        for col in 0..size {
            spawn_tile(commands, col, row, size, TileKind::Empty, assets);
        }
    }
}

// === Board Size Buttons ===
pub fn button_interaction(
    mut commands: Commands,
    mut board_size: ResMut<BoardSize>,
    interaction_query: Query<(&Interaction, &BoardButton), Changed<Interaction>>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut size_text: Query<&mut Text, With<BoardSizeText>>,
    mut placed_sources: ResMut<PlacedSources>,
    mut placed_goals: ResMut<PlacedGoals>,
    mut placed_teleports: ResMut<PlacedTeleports>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let new_size = match button {
            BoardButton::Increase => (board_size.0 + 1).min(MAX_BOARD_SIZE),
            BoardButton::Decrease => board_size.0.saturating_sub(1).max(MIN_BOARD_SIZE),
        };
        if new_size == board_size.0 { continue; }
        board_size.0 = new_size;
        placed_sources.0.clear();
        placed_goals.0.clear();
        placed_teleports.0 = [0; 10];
        for entity in &tiles { commands.entity(entity).despawn(); }
        spawn_board(&mut commands, new_size, &assets);
        let mut text = size_text.single_mut();
        **text = format!("{}x{}", new_size, new_size);
    }
}

// === Play/Stop ===
pub fn play_stop_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<PlayStopButton>, Changed<Interaction>)>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    bots: Query<Entity, With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut button_image: Query<&mut ImageNode, With<PlayButtonImage>>,
    play_icons: Res<PlayIcons>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        match *play_mode {
            PlayMode::Editing => {
                *play_mode = PlayMode::Playing;
                commands.insert_resource(PlayTimer(Timer::from_seconds(BOT_START_DELAY, TimerMode::Once)));
                let mut img = button_image.single_mut();
                img.image = play_icons.stop.clone();
                for (coord, kind) in &tiles {
                    if let TileKind::Source(ci, dir) = kind {
                        let pos = tile_world_pos(coord.col, coord.row, board_size.0, kind);
                        let bot_y = FLOOR_TOP_Y + BOT_SIZE / 2.0;
                        commands.spawn((
                            Mesh3d(assets.bot_mesh.clone()),
                            MeshMaterial3d(assets.bot_materials[*ci].clone()),
                            Transform::from_translation(Vec3::new(pos.x, bot_y, pos.z))
                                .with_rotation(Quat::from_rotation_y(dir.rotation()))
                                .with_scale(Vec3::ZERO),
                            TargetScale(Vec3::ONE),
                            Bot,
                            BotMovement {
                                direction: *dir,
                                color_index: *ci,
                                col: coord.col as i32,
                                row: coord.row as i32,
                                progress: 0.5,
                                speed: 0.0,
                                phase: BotPhase::Accelerating,
                            },
                        )).with_children(|parent| {
                            let ez = -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + 0.001);
                            for ex in [-0.07, 0.07] {
                                parent.spawn((Mesh3d(assets.eye_mesh.clone()),
                                    MeshMaterial3d(assets.eye_material.clone()),
                                    Transform::from_translation(Vec3::new(ex, 0.04, ez))));
                            }
                        });
                    }
                }
            }
            PlayMode::Playing => {
                *play_mode = PlayMode::Editing;
                let mut img = button_image.single_mut();
                img.image = play_icons.play.clone();
                for entity in &bots {
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                }
            }
        }
    }
}

// === Bot Movement ===
// Progress model: 0.0 = entry edge, 0.5 = center, 1.0 = exit edge
// Position = tile_center + (progress - 0.5) * direction_delta
pub fn move_bots(
    time: Res<Time>,
    play_mode: Res<PlayMode>,
    mut play_timer: Option<ResMut<PlayTimer>>,
    board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut bots: Query<(&mut Transform, &mut BotMovement, &mut TargetScale), With<Bot>>,
) {
    if *play_mode != PlayMode::Playing { return; }
    if let Some(ref mut timer) = play_timer {
        timer.0.tick(time.delta());
        if !timer.0.finished() { return; }
    }

    let dt = time.delta_secs();
    let half = (board_size.0 as f32 - 1.0) / 2.0;
    let bot_y = FLOOR_TOP_Y + BOT_SIZE / 2.0;

    let tile_at = |col: i32, row: i32| -> Option<TileKind> {
        if col < 0 || row < 0 || col >= board_size.0 as i32 || row >= board_size.0 as i32 {
            return None;
        }
        tiles.iter()
            .find(|(c, _)| c.col == col as u32 && c.row == row as u32)
            .map(|(_, k)| *k)
    };

    for (mut transform, mut mov, mut target_scale) in &mut bots {
        // Update speed and progress based on phase
        match mov.phase {
            BotPhase::Stopped => continue,
            BotPhase::Accelerating => {
                mov.speed = (mov.speed + BOT_ACCEL * dt).min(BOT_CRUISE_SPEED);
                mov.progress += mov.speed * dt;
                if mov.speed >= BOT_CRUISE_SPEED { mov.phase = BotPhase::Cruising; }
            }
            BotPhase::Cruising => {
                mov.progress += BOT_CRUISE_SPEED * dt;
            }
            BotPhase::Decelerating(_) => {
                mov.speed = (mov.speed - BOT_ACCEL * dt).max(0.0);
                mov.progress += mov.speed * dt;
                if mov.progress >= 0.5 {
                    mov.progress = 0.5;
                    mov.speed = 0.0;
                    match mov.phase {
                        BotPhase::Decelerating(Some(exit_dir)) => {
                            mov.phase = BotPhase::Rotating {
                                entry_dir: mov.direction,
                                exit_dir,
                                progress: 0.0,
                            };
                        }
                        BotPhase::Decelerating(None) => {
                            let tp = if let Some(TileKind::Teleport(num)) = tile_at(mov.col, mov.row) {
                                tiles.iter().find(|(c, k)| matches!(k, TileKind::Teleport(n) if *n == num)
                                    && (c.col as i32 != mov.col || c.row as i32 != mov.row))
                                    .map(|(c, _)| (c.col as i32, c.row as i32))
                            } else { None };
                            if let Some((tc, tr)) = tp {
                                target_scale.0 = Vec3::ZERO;
                                mov.phase = BotPhase::TeleportShrink { target_col: tc, target_row: tr };
                            } else { mov.phase = BotPhase::Spinning; }
                        }
                        _ => {}
                    }
                }
            }
            BotPhase::Spinning => {
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y, mov.row as f32 - half);
                transform.rotation = Quat::from_rotation_y(time.elapsed_secs() * BOT_SPIN_SPEED);
                continue;
            }
            BotPhase::TeleportShrink { target_col, target_row } => {
                if transform.scale.x < 0.03 {
                    mov.col = target_col; mov.row = target_row;
                    mov.progress = 0.5; mov.speed = 0.0;
                    transform.translation = Vec3::new(target_col as f32 - half, bot_y, target_row as f32 - half);
                    transform.scale = Vec3::ZERO; target_scale.0 = Vec3::ONE;
                    mov.phase = BotPhase::TeleportGrow;
                }
                continue;
            }
            BotPhase::TeleportGrow => {
                if transform.scale.x > 0.97 { transform.scale = Vec3::ONE; mov.phase = BotPhase::Accelerating; }
                continue;
            }
            BotPhase::Rotating { entry_dir, exit_dir, ref mut progress } => {
                *progress = (*progress + dt / BOT_TURN_DURATION).min(1.0);
                transform.rotation = Quat::from_rotation_y(entry_dir.rotation())
                    .slerp(Quat::from_rotation_y(exit_dir.rotation()), *progress);
                if *progress >= 1.0 { mov.direction = exit_dir; mov.phase = BotPhase::Accelerating; }
                transform.translation = Vec3::new(mov.col as f32 - half, bot_y, mov.row as f32 - half);
                continue;
            }
        }

        // Tile transition: check when exiting current tile
        if mov.progress >= 1.0 {
            let (dc, dr) = mov.direction.grid_delta();
            let next_col = mov.col + dc;
            let next_row = mov.row + dr;
            match tile_at(next_col, next_row) {
                Some(TileKind::Floor) | Some(TileKind::Source(_, _)) => {
                    mov.col = next_col;
                    mov.row = next_row;
                    mov.progress -= 1.0;
                }
                Some(TileKind::TurnBut(tci, _)) if tci == mov.color_index => {
                    mov.col = next_col; mov.row = next_row; mov.progress -= 1.0;
                }
                Some(TileKind::BounceBut(bci)) if bci == mov.color_index => {
                    mov.col = next_col; mov.row = next_row; mov.progress -= 1.0;
                }
                Some(TileKind::Bounce(_)) | Some(TileKind::BounceBut(_)) => {
                    mov.col = next_col; mov.row = next_row; mov.progress -= 1.0;
                    mov.phase = BotPhase::Decelerating(Some(mov.direction.opposite()));
                }
                Some(TileKind::Turn(_, tdir)) | Some(TileKind::TurnBut(_, tdir)) => {
                    if let Some(exit_dir) = mov.direction.turn_exit(tdir) {
                        mov.col = next_col;
                        mov.row = next_row;
                        mov.progress -= 1.0;
                        mov.phase = BotPhase::Decelerating(Some(exit_dir));
                    } else {
                        mov.progress = 1.0;
                        mov.speed = 0.0;
                        mov.phase = BotPhase::Stopped;
                    }
                }
                Some(TileKind::Goal(_)) | Some(TileKind::Teleport(_)) => {
                    mov.col = next_col;
                    mov.row = next_row;
                    mov.progress -= 1.0;
                    mov.phase = BotPhase::Decelerating(None);
                }
                _ => {
                    mov.progress = 1.0;
                    mov.speed = 0.0;
                    mov.phase = BotPhase::Stopped;
                }
            }
        }

        // Update world position
        let (dc, dr) = mov.direction.grid_delta();
        let cx = mov.col as f32 - half;
        let cz = mov.row as f32 - half;
        transform.translation = Vec3::new(
            cx + (mov.progress - 0.5) * dc as f32,
            bot_y,
            cz + (mov.progress - 0.5) * dr as f32,
        );
    }
}

// === Camera ===
pub fn adapt_camera(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection), With<Camera3d>>,
    board_size: Res<BoardSize>,
) {
    let window = windows.single();
    let (mut transform, projection) = cameras.single_mut();
    let aspect = window.width() / window.height();
    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => return,
    };
    let radius = board_bounding_radius(board_size.0);
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let dist_v = radius / half_fov_v.sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;
    let dir = camera_direction();
    *transform = Transform::from_translation(dir * distance).looking_at(Vec3::ZERO, Vec3::Y);
}
