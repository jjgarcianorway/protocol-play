// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D menu background: handcrafted loop board with bots circling forever.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::simulation::{BotMovement, BotPhase};

#[derive(Resource)]
pub struct MenuCamTracker { pub switch_timer: f32, pub bot_idx: usize }

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
    tiles.push((6, 5, Door(true)));         // open door on the path
    tiles.push((6, 6, Arrow(g, West)));
    tiles.push((5, 6, Floor));
    tiles.push((4, 6, Bounce(g)));          // bounces west→east... no! We want to continue west.
    // Actually Bounce reverses direction. That would break the loop. Use Floor instead.
    tiles.push((3, 6, Floor));
    tiles.push((2, 6, Arrow(g, North)));
    tiles.push((2, 5, Floor));
    tiles.push((2, 4, Switch));             // switch toggles the door
    tiles.push((2, 3, Arrow(g, East)));     // completes the loop → (3,3) Arrow East

    // Fix: replace (4,6) Bounce with Floor — Bounce would reverse direction
    // (Already pushed Bounce, need to override it in the dedup step)
    tiles.push((4, 6, Floor));

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

    // ── Decorative tiles (visual interest, not on paths) ──
    tiles.push((4, 4, Floor)); tiles.push((5, 4, Floor));
    tiles.push((4, 5, Floor)); tiles.push((5, 5, Floor));
    tiles.push((3, 4, Floor)); tiles.push((3, 5, Floor));
    // Some teleport pairs for visual flair (not on bot paths)
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
    commands.insert_resource(MenuCamTracker { switch_timer: 0.0, bot_idx: 0 });
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

/// No-op — bots loop forever on the handcrafted board.
pub fn menu_sim_loop() {}

/// Camera: smooth drone view, switches between bots every ~8 seconds.
pub fn menu_camera(
    time: Res<Time>,
    mut tracker: ResMut<MenuCamTracker>,
    bots: Query<&Transform, (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
) {
    let bot_count = bots.iter().count();
    if bot_count == 0 { return; }

    // Switch tracked bot periodically
    tracker.switch_timer += time.delta_secs();
    if tracker.switch_timer > 8.0 {
        tracker.switch_timer = 0.0;
        tracker.bot_idx = (tracker.bot_idx + 1) % bot_count;
    }

    let target = bots.iter().nth(tracker.bot_idx)
        .map(|t| t.translation)
        .unwrap_or(Vec3::ZERO);

    // Smooth close-up drone offset
    let offset = Vec3::new(1.5, 3.2, 2.0);
    let cam_target = target + offset;
    let look_at = target + Vec3::new(0.0, 0.1, 0.0);

    for mut tf in cameras.iter_mut() {
        let speed = 1.5 * time.delta_secs();
        tf.translation = tf.translation.lerp(cam_target, speed.min(1.0));
        *tf = Transform::from_translation(tf.translation).looking_at(look_at, Vec3::Y);
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
