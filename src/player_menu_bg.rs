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

#[derive(Resource)]
pub struct MenuCamTracker {
    pub bot_idx: usize, pub time_on_bot: f32,
    pub last_dir: Direction, pub dir_changes: u32,
    pub offsets: Vec<(f32, f32, f32)>,
}

const MENU_BOT_SPEED: f32 = 0.35; // very slow, meditative
const MENU_BOARD_SIZE: u32 = 11;
const CELEBRATION_TIME: f32 = 4.0; // seconds of spinning at goal
const CAM_SWITCH_MIN: f32 = 10.0;  // minimum seconds before switching bot
const CAM_SWITCH_MAX: f32 = 20.0;  // maximum seconds on one bot
const CAM_LERP_SLOW: f32 = 0.25;   // lerp speed during transitions
const CAM_LERP_FAST: f32 = 0.50;   // lerp speed when settled
const CAM_SETTLE_TIME: f32 = 3.0;  // seconds to reach full speed after switch

/// Generate a visually rich level using the actual game generator.
fn generate_menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    let size = MENU_BOARD_SIZE;
    let config = GenConfig {
        board_size: size,
        num_bots: 4,
        hole_percent: 8,
        hole_placement: HolePlacement::Edges,
        difficulty: 60,
        weights: [8, 5, 8, 5, 6, 4, 6, 4, 3, 3, 3, 3], // all tile types enabled
        unique_solution: false,
        inventory_target: 0,
        door_chains: 1,
        path_sharing: true,
        confusion_tiles: false,
        chapter_idx: 4, // mid-game palette for color variety
        required_tile: None,
    };

    // Try multiple seeds for a good-looking level
    for seed in 0..100u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xCAFE_BABE + seed);
        if let Some((tiles, _)) = generate_attempt(&config, &mut rng) {
            let level: Vec<(u32, u32, TileKind)> = tiles.iter()
                .map(|&(c, r, k, _)| (c, r, k)).collect();
            return (size, level);
        }
    }
    // Fallback: floor grid (should never happen)
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
    let present: std::collections::HashSet<(u32, u32)> =
        tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    for r in 0..size { for c in 0..size {
        let kind = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r)
            .map(|&(_, _, k)| k).unwrap_or(TileKind::Empty);
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}

    commands.insert_resource(MenuBotsSpawned(false));

    // Camera offsets per bot (generated level has 4 bots)
    commands.insert_resource(MenuCamTracker {
        bot_idx: 0, time_on_bot: 0.0, last_dir: Direction::East, dir_changes: 0,
        offsets: vec![
            (5.0, 2.8, 0.3),
            (4.0, 2.0, -0.5),
            (4.5, 2.4, 0.8),
            (3.8, 1.8, -0.2),
        ],
    });
}

fn spawn_bot(commands: &mut Commands, assets: &GameAssets,
    col: u32, row: u32, size: u32, ci: usize, dir: Direction, si: usize,
) {
    let pos = tile_world_pos(col, row, size, &TileKind::Source(ci, dir));
    let by = FLOOR_TOP_Y + BOT_SIZE / 2.0;
    commands.spawn((
        Mesh3d(assets.bot_mesh.clone()), MeshMaterial3d(assets.bot_materials[ci].clone()),
        Transform::from_translation(Vec3::new(pos.x, by, pos.z))
            .with_rotation(Quat::from_rotation_y(dir.rotation())),
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

        // Bot reached goal — celebrate then quietly respawn
        if matches!(mov.phase, BotPhase::Spinning) && cel_q.get(entity).is_err() {
            commands.entity(entity).insert(MenuCelebrationTimer(CELEBRATION_TIME));
        }
        let celebration_done = cel_q.get(entity).is_ok_and(|t| t.0 <= 0.0);

        if off || on_empty || celebration_done {
            commands.entity(entity).despawn();
            // Respawn at source — bot starts small and grows via TargetScale
            for (coord, kind) in tiles.iter() {
                if let TileKind::Source(ci, dir) = *kind {
                    if ci == mov.color_index {
                        spawn_bot(&mut commands, &assets, coord.col, coord.row,
                            board_size.0, ci, dir, mov.spawn_index);
                        break;
                    }
                }
            }
        }
    }
    for mut timer in cel_q.iter_mut() { timer.0 -= dt; }
}

/// Cinematic camera — very slow, smooth, meditative. Offset right for panel.
pub fn menu_camera(
    time: Res<Time>, mut tracker: ResMut<MenuCamTracker>,
    bots: Query<(&Transform, &BotMovement), (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
) {
    let list: Vec<_> = bots.iter().collect();
    let n = list.len();
    if n == 0 { return; }
    let dt = time.delta_secs();
    tracker.time_on_bot += dt;

    // Track interest level of current bot
    if let Some((_, mov)) = list.get(tracker.bot_idx) {
        if mov.direction != tracker.last_dir {
            tracker.dir_changes += 1;
            tracker.last_dir = mov.direction;
        }
    }

    // Smart switching: stay longer when bot is interesting
    let interest = (tracker.dir_changes as f32 * 1.0).min(6.0);
    let switch_time = (CAM_SWITCH_MIN + interest).min(CAM_SWITCH_MAX);
    if tracker.time_on_bot > switch_time && n > 1 {
        tracker.bot_idx = (tracker.bot_idx + 1) % n;
        tracker.time_on_bot = 0.0;
        tracker.dir_changes = 0;
    }

    let idx = tracker.bot_idx.min(n - 1);
    let target = list[idx].0.translation;
    let (h, d, a) = tracker.offsets.get(idx).copied().unwrap_or((4.5, 2.5, 0.0));

    // Camera positioned to see bot's face (slightly in front)
    let bot_dir = list[idx].1.direction;
    let (fwd_x, fwd_z) = bot_dir.grid_delta();
    let face_offset = Vec3::new(fwd_x as f32 * 0.8, 0.0, fwd_z as f32 * 0.8);

    // Offset right so board fills the right 66% of screen
    let right_offset = Vec3::new(-3.0, 0.0, 0.5);
    let cam_goal = target + Vec3::new(d * a.cos(), h, d * a.sin()) + right_offset + face_offset;
    let look_goal = target + Vec3::new(0.0, 0.1, 0.0) + right_offset * 0.3;

    // Very smooth interpolation — slower during transitions, buttery when settled
    let settle = (tracker.time_on_bot / CAM_SETTLE_TIME).min(1.0);
    let base_speed = CAM_LERP_SLOW + settle * (CAM_LERP_FAST - CAM_LERP_SLOW);
    let lerp = (base_speed * dt).min(0.06); // hard cap for smoothness

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
