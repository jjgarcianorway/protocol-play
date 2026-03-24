// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: bots looping on a handcrafted showcase board.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::simulation::{BotMovement, BotPhase};

#[derive(Resource)]
pub struct MenuBotsSpawned(pub bool);

#[derive(Resource)]
pub struct MenuCamTracker {
    pub bot_idx: usize, pub time_on_bot: f32,
    pub last_dir: Direction, pub dir_changes: u32,
    pub offsets: Vec<(f32, f32, f32)>,
}

/// 9x9 board with 3 bots on verified closed-loop paths.
/// EVERY cell a bot traverses is explicitly listed.
fn menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    use TileKind::*; use Direction::*;
    let s = 9u32;
    let g = NUM_COLORS;
    let mut t: Vec<(u32, u32, TileKind)> = Vec::new();

    // ═══ Bot 0 (red): outer perimeter, 32 tiles ═══
    // Path: (1,0)→E→(8,0)→S→(8,8)→W→(0,8)→N→(0,0)→E→(1,0) loop
    t.push((1, 0, Source(0, East)));
    t.push((2, 0, Floor)); t.push((3, 0, Floor)); t.push((4, 0, Floor));
    t.push((5, 0, Floor)); t.push((6, 0, Floor)); t.push((7, 0, Floor));
    t.push((8, 0, Arrow(g, South)));
    t.push((8, 1, Floor)); t.push((8, 2, Floor)); t.push((8, 3, Floor));
    t.push((8, 4, Floor)); t.push((8, 5, Floor)); t.push((8, 6, Floor));
    t.push((8, 7, Floor));
    t.push((8, 8, Arrow(g, West)));
    t.push((7, 8, Floor)); t.push((6, 8, Floor)); t.push((5, 8, Floor));
    t.push((4, 8, Floor)); t.push((3, 8, Floor)); t.push((2, 8, Floor));
    t.push((1, 8, Floor));
    t.push((0, 8, Arrow(g, North)));
    t.push((0, 7, Floor)); t.push((0, 6, Floor)); t.push((0, 5, Floor));
    t.push((0, 4, Floor)); t.push((0, 3, Floor)); t.push((0, 2, Floor));
    t.push((0, 1, Floor));
    t.push((0, 0, Arrow(g, East)));

    // ═══ Bot 1 (blue→purple): inner rectangle, 20 tiles ═══
    // Path: (2,2)→E→(6,2)→S→(6,6)→W→(2,6)→N→(2,2) loop
    // Source at (2,1) feeds into the loop
    t.push((2, 1, Source(3, South)));
    t.push((2, 2, Arrow(g, East)));
    t.push((3, 2, Floor)); t.push((4, 2, Floor)); t.push((5, 2, Floor));
    t.push((6, 2, Arrow(g, South)));
    t.push((6, 3, Painter(5))); // blue → purple!
    t.push((6, 4, Floor)); t.push((6, 5, Floor));
    t.push((6, 6, Arrow(g, West)));
    t.push((5, 6, Floor)); t.push((4, 6, Floor)); t.push((3, 6, Floor));
    t.push((2, 6, Arrow(g, North)));
    t.push((2, 5, Floor)); t.push((2, 4, Floor)); t.push((2, 3, Floor));
    // (2,3)→N→(2,2) Arrow(East) → loop resumes

    // ═══ Bot 2 (cyan): lower-left rectangle, 12 tiles ═══
    // Path: (1,5)→E→(4,5)→S→(4,7)→W→(1,7)→N→(1,5) loop
    t.push((1, 4, Source(6, South)));
    t.push((1, 5, Arrow(g, East)));
    t.push((2, 5, Floor)); t.push((3, 5, Floor));
    t.push((4, 5, Arrow(g, South)));
    t.push((4, 6, Floor));
    t.push((4, 7, Arrow(g, West)));
    t.push((3, 7, Floor)); t.push((2, 7, Floor));
    t.push((1, 7, Arrow(g, North)));
    t.push((1, 6, Floor));
    // (1,6)→N→(1,5) Arrow(East) → loop resumes

    // ═══ Decorative tiles (not on paths, just visual interest) ═══
    t.push((4, 3, Floor)); t.push((5, 3, Floor));
    t.push((4, 4, Floor)); t.push((5, 4, Floor));
    t.push((3, 3, Floor)); t.push((3, 4, Floor));
    t.push((3, 5, Floor)); // already exists
    t.push((5, 7, Floor)); t.push((6, 7, Floor)); t.push((7, 7, Floor));
    t.push((7, 5, Floor)); t.push((7, 6, Floor));
    // Teleport pair for visual flair (not on any path)
    t.push((5, 3, Teleport(2, 0)));
    t.push((7, 6, Teleport(2, 0)));

    // Dedup: later entries win
    let mut grid = std::collections::HashMap::new();
    for (c, r, k) in t { grid.insert((c, r), k); }

    let tiles: Vec<_> = grid.into_iter().map(|((c, r), k)| (c, r, k)).collect();
    (s, tiles)
}

pub fn setup_menu_background(
    mut commands: Commands, assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>, mut play_mode: ResMut<PlayMode>,
) {
    let (size, tiles) = menu_board();
    board_size.0 = size;
    for r in 0..size { for c in 0..size {
        let kind = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r)
            .map(|&(_, _, k)| k).unwrap_or(TileKind::Empty);
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}
    // Don't spawn bots here — tiles are deferred. menu_sim_loop spawns them on first Update.
    commands.insert_resource(MenuBotsSpawned(false));
    commands.insert_resource(MenuCamTracker {
        bot_idx: 0, time_on_bot: 0.0, last_dir: Direction::East, dir_changes: 0,
        offsets: vec![(3.8, 2.2, 0.3), (2.8, 1.6, -0.4), (3.2, 1.8, 0.8)],
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
            progress: 0.5, speed: BOT_CRUISE_SPEED, phase: BotPhase::Cruising,
            spawn_index: si, switch_pending: false },
    )).with_children(|p| {
        let ez = -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + OVERLAY_MESH_THICKNESS);
        for ex in [-BOT_EYE_SPACING, BOT_EYE_SPACING] {
            p.spawn((Mesh3d(assets.eye_mesh.clone()), MeshMaterial3d(assets.eye_material.clone()),
                Transform::from_translation(Vec3::new(ex, BOT_EYE_Y_OFFSET, ez))));
        }
    });
}

/// First frame: spawn bots at source tiles. Every frame: respawn fallen bots.
pub fn menu_sim_loop(
    mut commands: Commands,
    bots: Query<(Entity, &BotMovement), With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut spawned: ResMut<MenuBotsSpawned>,
    mut play_mode: ResMut<PlayMode>,
) {
    // First frame: spawn bots now that tiles are committed
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

    // Respawn fallen bots
    let sz = board_size.0 as i32;
    for (entity, mov) in bots.iter() {
        let off = mov.col < 0 || mov.row < 0 || mov.col >= sz || mov.row >= sz;
        let empty = !off && tiles.iter()
            .find(|(c, _)| c.col == mov.col as u32 && c.row == mov.row as u32)
            .is_none_or(|(_, k)| matches!(*k, TileKind::Empty));
        if off || empty {
            commands.entity(entity).despawn();
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
}

/// Cinematic screensaver camera.
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
    if let Some((_, mov)) = list.get(tracker.bot_idx) {
        if mov.direction != tracker.last_dir { tracker.dir_changes += 1; tracker.last_dir = mov.direction; }
    }
    let interest = (tracker.dir_changes as f32 * 0.8).min(4.0);
    if tracker.time_on_bot > (6.0 + interest).min(14.0) && n > 1 {
        tracker.bot_idx = (tracker.bot_idx + 1) % n;
        tracker.time_on_bot = 0.0; tracker.dir_changes = 0;
    }
    let idx = tracker.bot_idx.min(n - 1);
    let target = list[idx].0.translation;
    let (h, d, a) = tracker.offsets.get(idx).copied().unwrap_or((3.2, 1.8, 0.0));
    let cam_goal = target + Vec3::new(d * a.cos(), h, d * a.sin());
    let look_goal = target + Vec3::new(0.0, 0.08, 0.0);
    let lerp = ((0.6 + (tracker.time_on_bot / 2.0).min(1.0) * 0.8) * dt).min(0.15);
    for mut tf in cameras.iter_mut() {
        let pos = tf.translation.lerp(cam_goal, lerp);
        let look = (tf.forward() * 5.0 + tf.translation).lerp(look_goal, lerp * 1.2);
        *tf = Transform::from_translation(pos).looking_at(look, Vec3::Y);
    }
}

pub fn cleanup_menu_background(
    mut commands: Commands, bots: Query<Entity, With<Bot>>, mut play_mode: ResMut<PlayMode>,
) {
    for e in bots.iter() { commands.entity(e).despawn(); }
    *play_mode = PlayMode::TestEditing;
    commands.remove_resource::<MenuCamTracker>();
}
