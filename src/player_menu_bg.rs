// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: bots looping on a handcrafted showcase board.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::simulation::{BotMovement, BotPhase};

#[derive(Resource)]
pub struct MenuCamTracker {
    pub bot_idx: usize,
    pub time_on_bot: f32,
    pub last_dir: Direction,
    pub dir_changes: u32,
    pub offsets: Vec<(f32, f32, f32)>,
}

/// Build a 9x9 showcase board. All bot paths are closed loops — no dead ends.
fn menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    use TileKind::*; use Direction::*;
    let size = 9u32;
    let g = NUM_COLORS; // gray = affects all bots

    let mut t: Vec<(u32, u32, TileKind)> = Vec::new();

    // ── Bot 0 (red): outer perimeter loop (32 tiles) ──
    t.push((1, 0, Source(0, East)));
    for c in 2..8 { t.push((c, 0, Floor)); }
    t.push((8, 0, Arrow(g, South)));
    for r in 1..8 { t.push((8, r, Floor)); }
    t.push((8, 8, Arrow(g, West)));
    for c in (1..8).rev() { t.push((c, 8, Floor)); }
    t.push((0, 8, Arrow(g, North)));
    for r in (1..8).rev() { t.push((0, r, Floor)); }
    t.push((0, 0, Arrow(g, East)));

    // ── Bot 1 (blue→purple): inner rectangle with painter (16 tiles) ──
    t.push((2, 1, Source(3, East)));
    t.push((3, 1, Floor)); t.push((4, 1, Floor)); t.push((5, 1, Floor));
    t.push((6, 1, Arrow(g, South)));
    t.push((6, 2, Painter(5))); // blue → purple
    t.push((6, 3, Floor)); t.push((6, 4, Floor)); t.push((6, 5, Floor));
    t.push((6, 6, Arrow(g, West)));
    t.push((5, 6, Floor)); t.push((4, 6, Floor)); t.push((3, 6, Floor));
    t.push((2, 6, Arrow(g, North)));
    t.push((2, 5, Floor)); t.push((2, 4, Floor)); t.push((2, 3, Floor));
    t.push((2, 2, Arrow(g, East)));
    // Bot loops: (2,2)→E→(2,1) Source (no effect)→(3,1)→...→(6,1)→S→...

    // ── Bot 2 (cyan): zigzag path with teleport (long, interesting) ──
    t.push((1, 3, Source(6, East)));
    t.push((1, 4, Floor)); // decorative neighbor
    // East across middle
    t.push((3, 3, Teleport(g, 0))); // teleport entrance
    // Teleport exit at opposite side
    t.push((5, 5, Teleport(g, 0))); // teleport exit, bot continues East
    t.push((6, 5, Floor)); t.push((7, 5, Arrow(g, North)));
    t.push((7, 4, Floor)); t.push((7, 3, Floor));
    t.push((7, 2, Arrow(g, West)));
    t.push((5, 2, Floor)); t.push((4, 2, Floor)); t.push((3, 2, Floor));
    t.push((1, 2, Arrow(g, South)));
    // Back to source → loop!

    // ── Decorative tiles (not on any path) — showcase tile variety ──
    t.push((4, 4, Floor)); t.push((5, 4, Floor));
    t.push((3, 4, Floor)); t.push((3, 5, Floor));
    t.push((4, 5, Floor)); t.push((4, 3, Floor));
    t.push((5, 3, Floor)); t.push((1, 5, Floor));
    t.push((1, 6, Floor)); t.push((1, 7, Floor));
    t.push((7, 6, Floor)); t.push((7, 7, Floor));

    // Dedup: later entries win
    let mut grid = std::collections::HashMap::new();
    for (c, r, k) in t { grid.insert((c, r), k); }

    let final_tiles: Vec<(u32, u32, TileKind)> = grid.into_iter()
        .map(|((c, r), k)| (c, r, k)).collect();
    (size, final_tiles)
}

pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    mut play_mode: ResMut<PlayMode>,
) {
    let (size, tiles) = menu_board();
    board_size.0 = size;

    for r in 0..size { for c in 0..size {
        let kind = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r)
            .map(|&(_, _, k)| k).unwrap_or(TileKind::Empty);
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}

    let mut si = 0usize;
    for &(col, row, kind) in &tiles {
        if let TileKind::Source(ci, dir) = kind {
            spawn_bot(&mut commands, &assets, col, row, size, ci, dir, si);
            si += 1;
        }
    }

    *play_mode = PlayMode::TestPlaying;
    commands.insert_resource(crate::simulation::PlayTimer(
        Timer::from_seconds(0.3, TimerMode::Once)));
    commands.insert_resource(MenuCamTracker {
        bot_idx: 0, time_on_bot: 0.0, last_dir: Direction::East, dir_changes: 0,
        offsets: vec![
            (3.8, 2.2, 0.3),
            (2.8, 1.6, -0.4),
            (3.2, 1.8, 0.8),
        ],
    });
}

fn spawn_bot(
    commands: &mut Commands, assets: &GameAssets,
    col: u32, row: u32, size: u32, ci: usize, dir: Direction, si: usize,
) {
    let kind = TileKind::Source(ci, dir);
    let pos = tile_world_pos(col, row, size, &kind);
    let by = FLOOR_TOP_Y + BOT_SIZE / 2.0;
    commands.spawn((
        Mesh3d(assets.bot_mesh.clone()),
        MeshMaterial3d(assets.bot_materials[ci].clone()),
        Transform::from_translation(Vec3::new(pos.x, by, pos.z))
            .with_rotation(Quat::from_rotation_y(dir.rotation())),
        TargetScale(Vec3::ONE), Bot, BotFormation::default(),
        BotMovement {
            direction: dir, color_index: ci,
            col: col as i32, row: row as i32,
            progress: 0.5, speed: 0.0,
            phase: BotPhase::Accelerating, spawn_index: si,
            switch_pending: false,
        },
    )).with_children(|p| {
        let ez = -(BOT_SIZE / 2.0 + BOT_EYE_D / 2.0 + OVERLAY_MESH_THICKNESS);
        for ex in [-BOT_EYE_SPACING, BOT_EYE_SPACING] {
            p.spawn((
                Mesh3d(assets.eye_mesh.clone()),
                MeshMaterial3d(assets.eye_material.clone()),
                Transform::from_translation(Vec3::new(ex, BOT_EYE_Y_OFFSET, ez)),
            ));
        }
    });
}

/// Respawn any bot that falls off the board.
pub fn menu_sim_loop(
    mut commands: Commands,
    bots: Query<(Entity, &BotMovement), With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
) {
    let size = board_size.0 as i32;
    for (entity, mov) in bots.iter() {
        let off = mov.col < 0 || mov.row < 0 || mov.col >= size || mov.row >= size;
        let on_empty = !off && tiles.iter()
            .find(|(c, _)| c.col == mov.col as u32 && c.row == mov.row as u32)
            .is_none_or(|(_, k)| matches!(*k, TileKind::Empty));
        if off || on_empty {
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
    time: Res<Time>,
    mut tracker: ResMut<MenuCamTracker>,
    bots: Query<(&Transform, &BotMovement), (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
) {
    let bot_list: Vec<_> = bots.iter().collect();
    let n = bot_list.len();
    if n == 0 { return; }

    let dt = time.delta_secs();
    tracker.time_on_bot += dt;

    if let Some((_, mov)) = bot_list.get(tracker.bot_idx) {
        if mov.direction != tracker.last_dir {
            tracker.dir_changes += 1;
            tracker.last_dir = mov.direction;
        }
    }

    let interest = (tracker.dir_changes as f32 * 0.8).min(4.0);
    let switch_time = (6.0 + interest).min(14.0);
    if tracker.time_on_bot > switch_time && n > 1 {
        tracker.bot_idx = (tracker.bot_idx + 1) % n;
        tracker.time_on_bot = 0.0;
        tracker.dir_changes = 0;
    }

    let idx = tracker.bot_idx.min(n - 1);
    let (bot_tf, _) = bot_list[idx];
    let target = bot_tf.translation;

    let (h, d, angle) = tracker.offsets.get(idx).copied().unwrap_or((3.2, 1.8, 0.0));
    let offset = Vec3::new(d * angle.cos(), h, d * angle.sin());
    let cam_goal = target + offset;
    let look_goal = target + Vec3::new(0.0, 0.08, 0.0);

    let t_factor = (tracker.time_on_bot / 2.0).min(1.0);
    let speed = (0.6 + t_factor * 0.8) * dt;
    let lerp = speed.min(0.15);

    for mut tf in cameras.iter_mut() {
        let new_pos = tf.translation.lerp(cam_goal, lerp);
        let cur_look = tf.forward() * 5.0 + tf.translation;
        let new_look = cur_look.lerp(look_goal, lerp * 1.2);
        *tf = Transform::from_translation(new_pos).looking_at(new_look, Vec3::Y);
    }
}

pub fn cleanup_menu_background(
    mut commands: Commands,
    bots: Query<Entity, With<Bot>>,
    mut play_mode: ResMut<PlayMode>,
) {
    for e in bots.iter() { commands.entity(e).despawn(); }
    *play_mode = PlayMode::TestEditing;
    commands.remove_resource::<MenuCamTracker>();
}
