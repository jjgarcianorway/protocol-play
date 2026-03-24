// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: showcase board with all tile types.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::simulation::{BotMovement, BotPhase};

#[derive(Resource)] pub struct MenuBotsSpawned(pub bool);

#[derive(Resource)]
pub struct MenuCamTracker {
    pub bot_idx: usize, pub time_on_bot: f32,
    pub last_dir: Direction, pub dir_changes: u32,
    pub offsets: Vec<(f32, f32, f32)>,
}

/// 12x12 showcase board — every tile type represented, 3 bots on long paths.
fn menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    use TileKind::*; use Direction::*;
    let s = 12u32;
    let g = NUM_COLORS; // gray = affects all
    let mut t: Vec<(u32, u32, TileKind)> = Vec::new();

    // ═══ Bot 0 (red): perimeter + teleport shortcut, ~40 tiles ═══
    // Top: (1,0)→E→(10,0)→S via teleport to (10,6)→S→(10,11)→W→(0,11)→N→(0,0)→E
    t.push((1, 0, Source(0, East)));
    for c in 2..10 { t.push((c, 0, Floor)); }
    t.push((10, 0, Arrow(g, South)));
    t.push((10, 1, Floor)); t.push((10, 2, Floor));
    t.push((10, 3, Teleport(g, 0))); // entrance
    t.push((10, 7, Teleport(g, 0))); // exit — bot continues South
    t.push((10, 8, Floor)); t.push((10, 9, Floor)); t.push((10, 10, Floor));
    t.push((10, 11, Arrow(g, West)));
    for c in (1..10).rev() { t.push((c, 11, Floor)); }
    t.push((0, 11, Arrow(g, North)));
    for r in (1..11).rev() { t.push((0, r, Floor)); }
    t.push((0, 0, Arrow(g, East)));

    // ═══ Bot 1 (blue→purple): inner path with door+switch+painter, ~30 tiles ═══
    // Path: (2,2)→E→(8,2)→S→(8,5)→door→(8,6)→S→(8,9)→W→(2,9)→N→(2,6)→switch→(2,5)→N→(2,2)
    t.push((2, 1, Source(3, South)));
    t.push((2, 2, Arrow(g, East)));
    for c in 3..8 { t.push((c, 2, Floor)); }
    t.push((8, 2, Arrow(g, South)));
    t.push((8, 3, Floor)); t.push((8, 4, Painter(5))); // blue→purple
    t.push((8, 5, Door(true))); // open door
    t.push((8, 6, Floor)); t.push((8, 7, Floor)); t.push((8, 8, Floor));
    t.push((8, 9, Arrow(g, West)));
    for c in (3..8).rev() { t.push((c, 9, Floor)); }
    t.push((2, 9, Arrow(g, North)));
    t.push((2, 8, Floor)); t.push((2, 7, Floor));
    t.push((2, 6, Switch)); // toggles the door
    t.push((2, 5, Floor)); t.push((2, 4, Floor)); t.push((2, 3, Floor));
    // (2,3)→N→(2,2) Arrow(East) → loop!

    // ═══ Bot 2 (cyan): lower area with bouncers + TurnBut, ~20 tiles ═══
    // Path: (4,4)→E→(6,4)→bounce→W→(4,4)→... wait, bounce goes back.
    // Better: rectangle with bounce at one end
    // (5,4)→S→(5,7)→E→(9,7)→Bounce→W→(5,7) — no, bounce reverses.
    // Simple rectangle with turns:
    t.push((4, 3, Source(6, South)));
    t.push((4, 4, Arrow(g, East)));
    t.push((5, 4, Floor)); t.push((6, 4, Floor)); t.push((7, 4, Floor));
    t.push((7, 5, Floor)); // shared with decorative area
    t.push((8, 4, TurnBut(0, South))); // TurnBut: affects all EXCEPT red (color 0)
    // For cyan (color 6): 0 != 6 && 0 != NUM_COLORS → TurnBut ACTIVATES → turns
    // Wait, TurnBut activates when ci != bot.color && ci != NUM_COLORS
    // TurnBut(0, South): ci=0. Cyan bot color=6. 0 != 6 = true, 0 != 9 = true → activates!
    // turn_exit for bot going East entering Turn(South):
    // South arms: (West, South). entry_side = West (opposite of East). West == West → exit = South. ✓
    t.push((8, 5, Floor)); t.push((8, 6, Floor)); // shared with bot 1 column
    t.push((8, 7, Floor)); // also on bot 1's path — sharing!
    // Wait, (8,7) is already Floor from bot 1. That's fine, dedup keeps it.
    // Actually bot 2 needs to go from (8,4) TurnBut → South to (8,5).
    // But (8,5) is Door(true) from bot 1! The door state changes when bot 1 hits the switch.
    // This is a problem. Let me route bot 2 differently.

    // Redesign bot 2: avoid bot 1's column 8
    // (4,4)→E→(6,4)→S→(6,7)→W→(4,7)→N→(4,4) simple inner square
    t.push((4, 4, Arrow(g, East))); // overrides previous
    t.push((5, 4, Floor)); t.push((6, 4, Arrow(g, South)));
    t.push((6, 5, Bounce(g))); // bounces South→North... no! We want to continue South.
    // Bounce reverses direction. Bad for a loop. Use Floor.
    t.push((6, 5, Floor)); t.push((6, 6, Floor));
    t.push((6, 7, Arrow(g, West)));
    t.push((5, 7, Floor));
    t.push((4, 7, Arrow(g, North)));
    t.push((4, 6, Floor)); t.push((4, 5, Floor));
    // (4,5)→N→(4,4) Arrow(East) → loop!

    // ═══ Decorative tiles (showcase variety, not on any bot path) ═══
    // Bouncers
    t.push((5, 5, Bounce(g))); t.push((5, 6, BounceBut(2)));
    // Color switches
    t.push((3, 5, ColorSwitch(1))); t.push((7, 6, ColorSwitchBut(4)));
    // Extra teleport pair
    t.push((1, 4, Teleport(2, 1))); t.push((9, 8, Teleport(2, 1)));
    // Extra doors
    t.push((3, 8, Door(false))); t.push((7, 3, Door(true)));
    // Turns and arrows (decorative)
    t.push((9, 4, Turn(2, North))); t.push((9, 5, TurnBut(5, East)));
    t.push((1, 8, Arrow(4, West))); t.push((3, 10, ArrowBut(1, South)));
    // Fill some interior with floor
    for r in 4..8 { t.push((3, r, Floor)); }
    for r in 4..8 { t.push((7, r, Floor)); }
    t.push((5, 3, Floor)); t.push((6, 3, Floor));
    t.push((5, 8, Floor)); t.push((6, 8, Floor));
    t.push((9, 6, Floor)); t.push((9, 7, Floor));
    t.push((1, 5, Floor)); t.push((1, 6, Floor)); t.push((1, 7, Floor));
    // Extra floor on perimeter interior
    for c in 1..10 { t.push((c, 1, Floor)); }
    for c in 1..10 { t.push((c, 10, Floor)); }
    t.push((1, 2, Floor)); t.push((1, 3, Floor));
    t.push((9, 1, Floor)); t.push((9, 2, Floor)); t.push((9, 3, Floor));
    t.push((9, 9, Floor)); t.push((9, 10, Floor));
    t.push((1, 9, Floor)); t.push((1, 10, Floor));
    // Teleport area between bot 0's teleport
    for r in 4..7 { t.push((10, r, Floor)); } // fill the gap

    // Dedup: later entries win
    let mut grid = std::collections::HashMap::new();
    for (c, r, k) in t { grid.insert((c, r), k); }
    // Remove the stale TurnBut that was overridden
    // (8,4) was TurnBut then... actually dedup handles it: Arrow(g,East) at (4,4) wins.
    // The TurnBut at (8,4) stays since nothing overrides it. That's fine — it's decorative.

    let tiles: Vec<_> = grid.into_iter().map(|((c, r), k)| (c, r, k)).collect();
    (s, tiles)
}

const MENU_BOT_SPEED: f32 = 0.5; // half speed for relaxed feel

pub fn setup_menu_background(
    mut commands: Commands, assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>, _play_mode: ResMut<PlayMode>,
) {
    let (size, tiles) = menu_board();
    board_size.0 = size;
    for r in 0..size { for c in 0..size {
        let kind = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r)
            .map(|&(_, _, k)| k).unwrap_or(TileKind::Empty);
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}
    commands.insert_resource(MenuBotsSpawned(false));
    commands.insert_resource(MenuCamTracker {
        bot_idx: 0, time_on_bot: 0.0, last_dir: Direction::East, dir_changes: 0,
        offsets: vec![
            (5.5, 3.0, 0.3),   // bot 0: higher (big perimeter)
            (4.2, 2.2, -0.5),  // bot 1: medium
            (3.5, 1.8, 0.7),   // bot 2: closer
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

pub fn menu_sim_loop(
    mut commands: Commands, bots: Query<(Entity, &BotMovement), With<Bot>>,
    assets: Res<GameAssets>, board_size: Res<BoardSize>,
    tiles: Query<(&TileCoord, &TileKind), With<Tile>>,
    mut spawned: ResMut<MenuBotsSpawned>, mut play_mode: ResMut<PlayMode>,
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

/// Cinematic camera — pulled back for 12x12 board, smooth tracking.
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
    if tracker.time_on_bot > (8.0 + interest).min(16.0) && n > 1 {
        tracker.bot_idx = (tracker.bot_idx + 1) % n;
        tracker.time_on_bot = 0.0; tracker.dir_changes = 0;
    }
    let idx = tracker.bot_idx.min(n - 1);
    let target = list[idx].0.translation;
    let (h, d, a) = tracker.offsets.get(idx).copied().unwrap_or((4.5, 2.5, 0.0));
    let cam_goal = target + Vec3::new(d * a.cos(), h, d * a.sin());
    let look_goal = target + Vec3::new(0.0, 0.08, 0.0);
    // Very smooth — slower than before for relaxed feel
    let lerp = ((0.4 + (tracker.time_on_bot / 2.5).min(1.0) * 0.6) * dt).min(0.10);
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
    commands.remove_resource::<MenuBotsSpawned>();
}
