// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: generated showcase board with looping bots.

use bevy::prelude::*;
use rand::SeedableRng;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::simulation::{BotMovement, BotPhase};
use crate::level_gen_algo::{GenConfig, HolePlacement, generate_attempt};

#[derive(Resource)] pub struct MenuBotsSpawned(pub bool);
#[derive(Component)] pub struct MenuCelebrationTimer(pub f32);

#[derive(Clone, Copy, PartialEq)]
pub enum CamShot {
    Follow,       // close-up following a bot
    Wide,         // pulled-back overview
    Sweep,        // slow pan across the board ("barrido")
}

#[derive(Resource)]
pub struct MenuCamTracker {
    pub bot_idx: usize,
    pub time_on_shot: f32,
    pub last_dir: Direction,
    pub dir_changes: u32,
    pub shot: CamShot,
    pub sweep_angle: f32,    // for Sweep shots
    pub zoom: f32,           // current zoom multiplier
    pub target_zoom: f32,    // target zoom (lerps toward this)
}

const MENU_BOT_SPEED: f32 = 0.35;
const MENU_BOARD_SIZE: u32 = 11;
const CELEBRATION_TIME: f32 = 3.0;
const SHRINK_TIME: f32 = 1.0;      // seconds to shrink before despawn
const CAM_SWITCH_MIN: f32 = 12.0;
const CAM_SWITCH_MAX: f32 = 25.0;
const CAM_LERP_SLOW: f32 = 0.12;
const CAM_LERP_FAST: f32 = 0.25;
const CAM_SETTLE_TIME: f32 = 4.0;

/// Generate a visually rich level using the actual game generator.
fn generate_menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    let size = MENU_BOARD_SIZE;
    let config = GenConfig {
        board_size: size,
        num_bots: 4,
        hole_percent: 12,
        hole_placement: HolePlacement::Both,
        difficulty: 80, // higher = longer, more complex paths
        weights: [10, 6, 10, 6, 8, 5, 8, 5, 4, 4, 4, 4], // heavy on turns+arrows
        unique_solution: false,
        inventory_target: 0,
        door_chains: 2,
        path_sharing: true,
        confusion_tiles: false,
        chapter_idx: 6, // later chapter = more varied colors
        required_tile: None,
    };

    // Try seeds, pick the one with the most mechanic tiles (most interesting)
    let mut best: Option<(Vec<(u32, u32, TileKind, bool)>, u32)> = None;
    for seed in 0..80u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD_CAFE + seed);
        if let Some((tiles, diff)) = generate_attempt(&config, &mut rng) {
            // Score by number of non-floor mechanic tiles (more = more visually interesting)
            let mechanic_count = tiles.iter().filter(|(_, _, k, _)| !matches!(k,
                TileKind::Floor | TileKind::Empty | TileKind::Source(_, _) | TileKind::Goal(_)
            )).count();
            let score = mechanic_count as u32 * 10 + diff;
            if best.as_ref().is_none_or(|(_, s)| score > *s) {
                best = Some((tiles, score));
            }
        }
    }
    if let Some((tiles, _)) = best {
        let level: Vec<_> = tiles.iter().map(|&(c, r, k, _)| (c, r, k)).collect();
        return (size, level);
    }
    // Fallback
    let mut tiles = Vec::new();
    for r in 0..size { for c in 0..size { tiles.push((c, r, TileKind::Floor)); } }
    (size, tiles)
}

pub fn setup_menu_background(
    mut commands: Commands, assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>, _play_mode: ResMut<PlayMode>,
) {
    let (size, tiles) = generate_menu_board();
    board_size.0 = size;

    // Spawn all cells
    let _present: std::collections::HashSet<(u32, u32)> =
        tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    for r in 0..size { for c in 0..size {
        let kind = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r)
            .map(|&(_, _, k)| k).unwrap_or(TileKind::Empty);
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}

    commands.insert_resource(MenuBotsSpawned(false));

    // Camera offsets per bot (generated level has 4 bots)
    commands.insert_resource(MenuCamTracker {
        bot_idx: 0, time_on_shot: 0.0, last_dir: Direction::East, dir_changes: 0,
        shot: CamShot::Wide, sweep_angle: 0.0, zoom: 1.2, target_zoom: 1.2,
    });
}

/// Spawn bot at full size (for initial spawn).
fn spawn_bot(commands: &mut Commands, assets: &GameAssets,
    col: u32, row: u32, size: u32, ci: usize, dir: Direction, si: usize,
) {
    spawn_bot_inner(commands, assets, col, row, size, ci, dir, si, Vec3::ONE);
}

/// Spawn bot at zero scale → grows smoothly via animate_scale.
fn spawn_bot_smooth(commands: &mut Commands, assets: &GameAssets,
    col: u32, row: u32, size: u32, ci: usize, dir: Direction, si: usize,
) {
    spawn_bot_inner(commands, assets, col, row, size, ci, dir, si, Vec3::ZERO);
}

fn spawn_bot_inner(commands: &mut Commands, assets: &GameAssets,
    col: u32, row: u32, size: u32, ci: usize, dir: Direction, si: usize,
    initial_scale: Vec3,
) {
    let pos = tile_world_pos(col, row, size, &TileKind::Source(ci, dir));
    let by = FLOOR_TOP_Y + BOT_SIZE / 2.0;
    commands.spawn((
        Mesh3d(assets.bot_mesh.clone()), MeshMaterial3d(assets.bot_materials[ci].clone()),
        Transform::from_translation(Vec3::new(pos.x, by, pos.z))
            .with_rotation(Quat::from_rotation_y(dir.rotation()))
            .with_scale(initial_scale),
        TargetScale(Vec3::ONE), Bot, BotFormation::default(),
        BotMovement { direction: dir, color_index: ci, col: col as i32, row: row as i32,
            progress: 0.5, speed: MENU_BOT_SPEED, phase: BotPhase::Cruising,
            spawn_index: si, switch_pending: false },
    )).with_children(|p| {
        let ez = -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + OVERLAY_MESH_THICKNESS);
        for ex in [-BOT_EYE_SPACING, BOT_EYE_SPACING] {
            p.spawn((Mesh3d(assets.eye_mesh.clone()), MeshMaterial3d(assets.eye_material.clone()),
                Transform::from_translation(Vec3::new(ex, BOT_EYE_Y_OFFSET, ez))));
        }
    });
}

/// First frame: spawn bots + start sim. Every frame: respawn bots that reach goals or fall.
pub fn menu_sim_loop(
    mut commands: Commands, bots: Query<(Entity, &BotMovement), With<Bot>>,
    assets: Res<GameAssets>, board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut spawned: ResMut<MenuBotsSpawned>, mut play_mode: ResMut<PlayMode>,
    time: Res<Time>, mut cel_q: Query<&mut MenuCelebrationTimer>,
) {
    if !spawned.0 {
        spawned.0 = true;
        *play_mode = PlayMode::TestPlaying;
        commands.insert_resource(crate::simulation::PlayTimer(
            Timer::from_seconds(0.1, TimerMode::Once)));
        let mut si = 0usize;
        for (coord, kind) in tiles.iter() {
            if let TileKind::Source(ci, dir) = *kind {
                spawn_bot(&mut commands, &assets, coord.col, coord.row,
                    board_size.0, ci, dir, si);
                si += 1;
            }
        }
        return;
    }

    let sz = board_size.0 as i32;
    let dt = time.delta_secs();
    for (entity, mov) in bots.iter() {
        let off = mov.col < 0 || mov.row < 0 || mov.col >= sz || mov.row >= sz;
        let on_empty = !off && tiles.iter()
            .find(|(c, _)| c.col == mov.col as u32 && c.row == mov.row as u32)
            .is_none_or(|(_, k)| matches!(*k, TileKind::Empty));

        // Bot reached goal — start celebration timer
        if matches!(mov.phase, BotPhase::Spinning) && cel_q.get(entity).is_err() {
            commands.entity(entity).insert(MenuCelebrationTimer(CELEBRATION_TIME));
        }

        // When celebration nearly done → start shrinking
        let should_shrink = cel_q.get(entity).is_ok_and(|t| t.0 <= SHRINK_TIME && t.0 > 0.0);
        if should_shrink {
            commands.entity(entity).insert(TargetScale(Vec3::ZERO));
        }

        // When fully shrunk or fell off → despawn and respawn
        let celebration_done = cel_q.get(entity).is_ok_and(|t| t.0 <= 0.0);
        if off || on_empty || celebration_done {
            commands.entity(entity).despawn();
            for (coord, kind) in tiles.iter() {
                if let TileKind::Source(ci, dir) = *kind {
                    if ci == mov.color_index {
                        spawn_bot_smooth(&mut commands, &assets, coord.col, coord.row,
                            board_size.0, ci, dir, mov.spawn_index);
                        break;
                    }
                }
            }
        }
    }
    for mut timer in cel_q.iter_mut() { timer.0 -= dt; }
}

/// Cinematic "director" camera — intelligent shot selection, smooth transitions.
pub fn menu_camera(
    time: Res<Time>, mut t: ResMut<MenuCamTracker>,
    bots: Query<(&Transform, &BotMovement), (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
    board_size: Res<BoardSize>,
) {
    let list: Vec<_> = bots.iter().collect();
    let n = list.len();
    if n == 0 { return; }
    let dt = time.delta_secs();
    t.time_on_shot += dt;
    t.sweep_angle += dt * 0.08; // slow sweep rotation

    // Track current bot's activity
    if let Some((_, mov)) = list.get(t.bot_idx) {
        if mov.direction != t.last_dir { t.dir_changes += 1; t.last_dir = mov.direction; }
    }

    // ── Director logic: when to cut ──
    let interest = (t.dir_changes as f32 * 1.0).min(6.0);
    let min_time = match t.shot {
        CamShot::Follow => CAM_SWITCH_MIN + interest,
        CamShot::Wide => 8.0,
        CamShot::Sweep => 12.0,
    };
    let should_cut = t.time_on_shot > min_time.min(CAM_SWITCH_MAX);

    // Don't cut to a bot that's celebrating (about to disappear)
    let bot_is_ending = |idx: usize| -> bool {
        list.get(idx).is_some_and(|(_, m)| matches!(m.phase,
            BotPhase::Spinning | BotPhase::Falling(_) | BotPhase::FallingPause(_)))
    };

    if should_cut && n > 0 {
        t.time_on_shot = 0.0;
        t.dir_changes = 0;

        // Mostly close follow shots, occasional slow sweep — never zoomed out
        let shot_cycle = (time.elapsed_secs() / 20.0) as u32 % 7;
        t.shot = match shot_cycle {
            5 => CamShot::Sweep, // 1 in 7 is a sweep
            _ => CamShot::Follow,
        };

        // Pick a non-ending bot
        if t.shot == CamShot::Follow {
            let mut best = t.bot_idx;
            for i in 0..n {
                let candidate = (t.bot_idx + 1 + i) % n;
                if !bot_is_ending(candidate) { best = candidate; break; }
            }
            t.bot_idx = best;
        }

        // Close zoom with subtle variation — F1 camera style
        t.target_zoom = match t.shot {
            CamShot::Follow => 0.55 + (t.bot_idx as f32 * 0.1) % 0.2, // 0.55-0.75 (very close)
            CamShot::Sweep => 0.9, // slightly further for sweep
            CamShot::Wide => 0.75, // unused but just in case
        };
    }

    // Smooth zoom
    t.zoom += (t.target_zoom - t.zoom) * dt * 0.6;

    // Camera offset: shift camera position to the LEFT so the bot
    // appears centered in the RIGHT 66% of screen (left 34% = menu panel)
    let cam_shift = Vec3::new(-1.5, 0.0, 0.0);

    let idx = t.bot_idx.min(n - 1);
    let target = list[idx].0.translation;

    let (cam_goal, look_goal) = match t.shot {
        CamShot::Follow | CamShot::Wide => {
            let dir = list[idx].1.direction;
            let (fx, fz) = dir.grid_delta();
            let face = Vec3::new(fx as f32 * 0.3, 0.0, fz as f32 * 0.3);
            let h = 2.0 * t.zoom;
            let d = 1.2 * t.zoom;
            // Camera position: behind-ish the bot, offset left for panel
            (target + Vec3::new(d * 0.6, h, d * 0.8) + cam_shift + face,
             target) // look DIRECTLY at the bot — no offset
        }
        CamShot::Sweep => {
            let a = t.sweep_angle;
            let r = 2.0 * t.zoom;
            let h = 2.5 * t.zoom;
            (target + Vec3::new(a.sin() * r, h, a.cos() * r) + cam_shift,
             target) // look DIRECTLY at the bot
        }
    };

    // Ultra-smooth interpolation
    let settle = (t.time_on_shot / CAM_SETTLE_TIME).min(1.0);
    let speed = CAM_LERP_SLOW + settle * (CAM_LERP_FAST - CAM_LERP_SLOW);
    let lerp = (speed * dt).min(0.03);

    for mut tf in cameras.iter_mut() {
        let pos = tf.translation.lerp(cam_goal, lerp);
        let cur_look = tf.forward() * 5.0 + tf.translation;
        let look = cur_look.lerp(look_goal, lerp * 1.1);
        *tf = Transform::from_translation(pos).looking_at(look, Vec3::Y);
    }
}

pub fn cleanup_menu_background(
    mut commands: Commands, bots: Query<Entity, With<Bot>>, mut play_mode: ResMut<PlayMode>,
) {
    for e in bots.iter() { commands.entity(e).despawn(); }
    *play_mode = PlayMode::TestEditing;
    commands.remove_resource::<MenuCamTracker>();
    commands.remove_resource::<MenuBotsSpawned>();
}
