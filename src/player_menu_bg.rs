// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: handcrafted loop board with bots circling forever.

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
    /// Per-bot camera personality: (height_offset, distance_offset, angle_offset)
    pub offsets: Vec<(f32, f32, f32)>,
}

/// Build the handcrafted loop board. Bots never reach goals — they loop forever.
fn menu_board() -> (u32, Vec<(u32, u32, TileKind)>) {
    use TileKind::*; use Direction::*;
    let size = 9u32;
    let g = NUM_COLORS; // gray = affects all bots
    let mut tiles: Vec<(u32, u32, TileKind)> = Vec::new();

    // ── Bot 0 (color 0, red): outer perimeter loop ──
    // Turn directions: bot_dir.turn_exit(turn_dir) must return Some(exit)
    // NW corner: arrives North → need East. Turn(East): entry=South? No. Let me use Arrow.
    tiles.push((1, 0, Source(0, East)));
    // Top row: (2,0)..(7,0)
    for c in 2..8 { tiles.push((c, 0, Floor)); }
    tiles.push((8, 0, Arrow(g, South)));    // East→South at NE corner
    // Right column: (8,1)..(8,7)
    for r in 1..8 { tiles.push((8, r, Floor)); }
    tiles.push((8, 8, Arrow(g, West)));     // South→West at SE corner
    // Bottom row: (7,8)..(1,8)
    for c in (1..8).rev() { tiles.push((c, 8, Floor)); }
    tiles.push((0, 8, Arrow(g, North)));    // West→North at SW corner
    // Left column: (0,7)..(0,1)
    for r in (1..8).rev() { tiles.push((0, r, Floor)); }
    tiles.push((0, 0, Arrow(g, East)));     // North→East at NW corner
    // Bot passes through Source(1,0) → continues East → infinite loop!

    // ── Bot 1 (color 3, blue): inner loop with arrows + door ──
    // Source enters the loop from above
    tiles.push((3, 1, Source(3, South)));
    tiles.push((3, 2, Floor));
    // Loop: (3,3)→E→(6,3)→S→(6,6)→W→(2,6)→N→(2,3)→E→(3,3)
    tiles.push((3, 3, Arrow(g, East)));
    tiles.push((4, 3, Floor));
    tiles.push((5, 3, Painter(5)));         // changes blue→purple for visual flair
    tiles.push((6, 3, Arrow(g, South)));
    tiles.push((6, 4, Floor));
    tiles.push((6, 5, Floor));              // was Door+Switch — broke on second loop
    tiles.push((6, 6, Arrow(g, West)));
    tiles.push((5, 6, Floor));
    tiles.push((4, 6, Floor));
    tiles.push((3, 6, Floor));
    tiles.push((2, 6, Arrow(g, North)));
    tiles.push((2, 5, Floor));
    tiles.push((2, 4, Floor));
    tiles.push((2, 3, Arrow(g, East)));     // completes the loop → (3,3) Arrow East

    // ── Bot 2 (color 6, cyan): small bounce corridor ──
    // Bounces back and forth between two bouncers
    tiles.push((4, 1, Source(6, East)));
    tiles.push((5, 1, Floor));
    tiles.push((6, 1, Floor));
    tiles.push((7, 1, Bounce(g)));          // bounces east→west
    tiles.push((7, 0, Floor));              // decorative
    // When bot bounces west from (7,1), goes to (6,1), (5,1), (4,1) Source (no effect),
    // continues west to... (3,1) is Bot 1's source! Need to stop before that.
    // Put a bouncer at the west end:
    tiles.push((3, 1, Source(3, South)));    // already placed, bot 2 would pass through
    // Hmm, Source doesn't bounce. Let me shift bot 2's corridor:
    tiles.push((5, 7, Source(6, East)));
    tiles.push((6, 7, Floor));
    tiles.push((7, 7, Bounce(g)));          // bounces east→west
    tiles.push((4, 7, Bounce(g)));          // bounces west→east
    // Bot bounces between (4,7) and (7,7) forever!
    // Remove the original source at (4,1)

    // ── Decorative tiles (visual interest, NOT on bot paths) ──
    tiles.push((4, 4, Door(true)));  tiles.push((5, 4, Switch));
    tiles.push((4, 5, Floor)); tiles.push((5, 5, Floor));
    tiles.push((3, 4, Floor)); tiles.push((3, 5, Floor));
    tiles.push((1, 2, Teleport(2, 0)));
    tiles.push((7, 6, Teleport(2, 0)));
    tiles.push((1, 4, ColorSwitch(1)));
    tiles.push((7, 4, Floor));

    // Deduplicate: later entries win
    let mut grid = std::collections::HashMap::new();
    for (c, r, k) in tiles { grid.insert((c, r), k); }
    // Remove the misplaced bot 2 source at (4,1)
    grid.remove(&(4, 1));
    // Make sure bot 2's source is at (5,7)

    let final_tiles: Vec<(u32, u32, TileKind)> = grid.into_iter()
        .map(|((c, r), k)| (c, r, k)).collect();
    (size, final_tiles)
}

/// Startup: spawn the loop board and bots.
pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    mut play_mode: ResMut<PlayMode>,
) {
    let (size, tiles) = menu_board();
    board_size.0 = size;

    let present: std::collections::HashSet<(u32, u32)> =
        tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    for r in 0..size { for c in 0..size {
        let kind = if let Some(&(_, _, k)) = tiles.iter().find(|&&(tc, tr, _)| tc == c && tr == r) {
            k
        } else { TileKind::Empty };
        spawn_tile_at_scale(&mut commands, c, r, size, kind, &assets, Vec3::ONE);
    }}

    // Spawn bots at source positions
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
        // Each bot gets a slightly different camera angle for variety
        offsets: vec![
            (3.8, 2.2, 0.3),   // bot 0: higher, further back
            (2.8, 1.6, -0.4),  // bot 1: closer, lower
            (3.2, 1.8, 0.8),   // bot 2: medium, side angle
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

/// Safety: respawn any bot that falls off the board.
pub fn menu_sim_loop(
    mut commands: Commands,
    bots: Query<(Entity, &BotMovement), With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    tiles: Query<(&crate::types::TileCoord, &crate::types::TileKind), With<crate::types::Tile>>,
) {
    let size = board_size.0 as i32;
    for (entity, mov) in bots.iter() {
        let off_board = mov.col < 0 || mov.row < 0 || mov.col >= size || mov.row >= size;
        let on_empty = !off_board && tiles.iter()
            .find(|(c, _)| c.col == mov.col as u32 && c.row == mov.row as u32)
            .is_none_or(|(_, k)| matches!(*k, TileKind::Empty));
        if off_board || on_empty {
            // Despawn fallen bot and respawn at its source
            commands.entity(entity).despawn();
            // Find the source tile for this bot's color
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

/// Cinematic screensaver camera: smooth drone view with intelligent bot switching.
pub fn menu_camera(
    time: Res<Time>,
    mut tracker: ResMut<MenuCamTracker>,
    bots: Query<(&Transform, &BotMovement), (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
) {
    let bot_list: Vec<_> = bots.iter().collect();
    let bot_count = bot_list.len();
    if bot_count == 0 { return; }

    let dt = time.delta_secs();
    tracker.time_on_bot += dt;

    // Track direction changes of current bot (more changes = more interesting)
    if let Some((_, mov)) = bot_list.get(tracker.bot_idx) {
        if mov.direction != tracker.last_dir {
            tracker.dir_changes += 1;
            tracker.last_dir = mov.direction;
        }
    }

    // Smart switching: stay longer when bot is doing interesting things
    let min_time = 6.0;
    let max_time = 14.0;
    // If bot has been turning/bouncing a lot, stay longer
    let interest = (tracker.dir_changes as f32 * 0.8).min(4.0);
    let switch_time = min_time + interest;
    let should_switch = tracker.time_on_bot > switch_time.min(max_time);

    if should_switch && bot_count > 1 {
        tracker.bot_idx = (tracker.bot_idx + 1) % bot_count;
        tracker.time_on_bot = 0.0;
        tracker.dir_changes = 0;
    }

    let idx = tracker.bot_idx.min(bot_count - 1);
    let (bot_tf, _) = bot_list[idx];
    let target = bot_tf.translation;

    // Per-bot camera personality
    let (h, d, angle) = tracker.offsets.get(idx).copied().unwrap_or((3.2, 1.8, 0.0));
    let offset = Vec3::new(d * angle.cos(), h, d * angle.sin());
    let cam_goal = target + offset;
    let look_goal = target + Vec3::new(0.0, 0.08, 0.0);

    // Very smooth interpolation — slow, deliberate, screensaver-like
    // Slower when transitioning between bots (first 2 seconds after switch)
    let transition_factor = (tracker.time_on_bot / 2.0).min(1.0);
    let base_speed = 0.6 + transition_factor * 0.8; // 0.6 → 1.4
    let lerp_speed = (base_speed * dt).min(0.15);

    for mut tf in cameras.iter_mut() {
        let new_pos = tf.translation.lerp(cam_goal, lerp_speed);
        let current_look = tf.forward() * 5.0 + tf.translation;
        let new_look = current_look.lerp(look_goal, lerp_speed * 1.2);
        *tf = Transform::from_translation(new_pos).looking_at(new_look, Vec3::Y);
    }
}

/// OnExit: despawn bots, clean up.
pub fn cleanup_menu_background(
    mut commands: Commands,
    bots: Query<Entity, With<Bot>>,
    mut play_mode: ResMut<PlayMode>,
) {
    for e in bots.iter() { commands.entity(e).despawn(); }
    *play_mode = PlayMode::TestEditing;
    commands.remove_resource::<MenuCamTracker>();
}
