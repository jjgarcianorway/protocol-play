// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D board background with bot simulation for the main menu.

use bevy::prelude::*;
use rand::SeedableRng;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile_at_scale, tile_world_pos};
use crate::level_gen_algo::{GenConfig, HolePlacement, generate_attempt};
use crate::simulation::{BotMovement, BotPhase};

#[derive(Resource)]
pub struct MenuLevel {
    pub tiles: Vec<(u32, u32, TileKind)>,
    pub size: u32,
}

#[derive(Resource)]
pub struct MenuSimState {
    pub running: bool,
    pub restart_timer: f32,
}

/// Call during app build to generate the background level.
pub fn generate_menu_level() -> MenuLevel {
    let size = MENU_BG_BOARD_SIZE;
    let config = GenConfig {
        board_size: size, num_bots: 3, hole_percent: 10,
        hole_placement: HolePlacement::Both, difficulty: 35,
        weights: [5; GEN_NUM_WEIGHTS], unique_solution: false,
        inventory_target: 0, door_chains: 0, path_sharing: true,
        confusion_tiles: false, chapter_idx: 2, required_tile: None,
    };
    for seed in 0..50u64 {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xBEEF_CAFE + seed);
        if let Some((tiles, _)) = generate_attempt(&config, &mut rng) {
            return MenuLevel {
                tiles: tiles.iter().map(|&(c, r, k, _)| (c, r, k)).collect(), size,
            };
        }
    }
    let mut tiles = Vec::new();
    for r in 0..size { for c in 0..size { tiles.push((c, r, TileKind::Floor)); } }
    MenuLevel { tiles, size }
}

/// Startup: spawn the menu level and start the bot simulation.
pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    menu_level: Res<MenuLevel>,
    mut board_size: ResMut<BoardSize>,
    mut play_mode: ResMut<PlayMode>,
) {
    board_size.0 = menu_level.size;

    // Spawn tiles at full scale
    let present: std::collections::HashSet<(u32, u32)> =
        menu_level.tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    for r in 0..menu_level.size { for c in 0..menu_level.size {
        if !present.contains(&(c, r)) {
            spawn_tile_at_scale(&mut commands, c, r, menu_level.size, TileKind::Empty, &assets, Vec3::ONE);
        }
    }}
    for &(c, r, kind) in &menu_level.tiles {
        spawn_tile_at_scale(&mut commands, c, r, menu_level.size, kind, &assets, Vec3::ONE);
    }

    // Spawn bots at source positions (full scale, no animation delay)
    spawn_menu_bots(&mut commands, &assets, &menu_level);

    // Start simulation
    *play_mode = PlayMode::TestPlaying;
    commands.insert_resource(crate::simulation::PlayTimer(
        Timer::from_seconds(0.1, TimerMode::Once)));
    commands.insert_resource(MenuSimState { running: true, restart_timer: 0.0 });
}

fn spawn_menu_bots(commands: &mut Commands, assets: &GameAssets, level: &MenuLevel) {
    let mut si = 0usize;
    for &(col, row, kind) in &level.tiles {
        if let TileKind::Source(ci, dir) = kind {
            let pos = tile_world_pos(col, row, level.size, &kind);
            let by = FLOOR_TOP_Y + BOT_SIZE / 2.0;
            commands.spawn((
                Mesh3d(assets.bot_mesh.clone()),
                MeshMaterial3d(assets.bot_materials[ci].clone()),
                Transform::from_translation(Vec3::new(pos.x, by, pos.z))
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(Vec3::splat(BOT_SIZE / 0.35)), // full size
                Bot, BotFormation::default(),
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
            si += 1;
        }
    }
}

/// Detect when simulation ends; after a short pause, reset bots and restart.
pub fn menu_sim_loop(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<MenuSimState>,
    bots: Query<(Entity, &BotMovement)>,
    assets: Res<GameAssets>,
    menu_level: Res<MenuLevel>,
    mut play_mode: ResMut<PlayMode>,
) {
    if !state.running {
        // Waiting to restart
        state.restart_timer -= time.delta_secs();
        if state.restart_timer <= 0.0 {
            // Respawn bots and restart
            spawn_menu_bots(&mut commands, &assets, &menu_level);
            *play_mode = PlayMode::TestPlaying;
            commands.insert_resource(crate::simulation::PlayTimer(
                Timer::from_seconds(0.1, TimerMode::Once)));
            state.running = true;
        }
        return;
    }

    // Check if all bots are done (at_goal or stuck for too long)
    let all_done = !bots.is_empty() && bots.iter().all(|(_, m)| m.speed < 0.01 && m.progress > 0.9);
    if all_done {
        // Despawn bots, pause briefly, then restart
        for (e, _) in bots.iter() { commands.entity(e).despawn(); }
        *play_mode = PlayMode::TestEditing;
        state.running = false;
        state.restart_timer = 1.5; // pause before restarting
    }
}

/// Camera: follow the first bot from a close drone view.
pub fn menu_camera(
    time: Res<Time>,
    bots: Query<&Transform, (With<Bot>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Bot>)>,
    board_size: Res<BoardSize>,
) {
    let target_pos = if let Some(bot_tf) = bots.iter().next() {
        // Follow first bot from above and slightly behind
        bot_tf.translation
    } else {
        Vec3::ZERO
    };

    let offset = Vec3::new(1.5, 4.0, 2.5);
    let cam_target = target_pos + offset;

    for mut tf in cameras.iter_mut() {
        // Smooth follow
        let speed = 2.0 * time.delta_secs();
        tf.translation = tf.translation.lerp(cam_target, speed.min(1.0));
        let look = target_pos + Vec3::new(0.0, 0.2, 0.0);
        *tf = Transform::from_translation(tf.translation).looking_at(look, Vec3::Y);
    }
}

/// OnExit: despawn bots and clean up resources.
pub fn cleanup_menu_background(
    mut commands: Commands,
    bots: Query<Entity, With<Bot>>,
    mut play_mode: ResMut<PlayMode>,
) {
    for e in bots.iter() { commands.entity(e).despawn(); }
    *play_mode = PlayMode::TestEditing;
    commands.remove_resource::<MenuSimState>();
}
