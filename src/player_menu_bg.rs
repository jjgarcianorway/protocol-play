// SPDX-License-Identifier: GPL-3.0-or-later
//! 3D board background for the main menu.

use bevy::prelude::*;
use rand::SeedableRng;
use crate::constants::*;
use crate::types::*;
use crate::board::spawn_tile;
use crate::level_gen_algo::{GenConfig, HolePlacement, generate_attempt};

#[derive(Resource)]
pub struct MenuLevel {
    pub tiles: Vec<(u32, u32, TileKind)>,
    pub size: u32,
}

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
                tiles: tiles.iter().map(|&(c, r, k, _)| (c, r, k)).collect(),
                size,
            };
        }
    }
    let mut tiles = Vec::new();
    for r in 0..size { for c in 0..size { tiles.push((c, r, TileKind::Floor)); } }
    MenuLevel { tiles, size }
}

/// Startup: spawn menu level tiles.
pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    menu_level: Res<MenuLevel>,
    mut board_size: ResMut<BoardSize>,
) {
    board_size.0 = menu_level.size;
    let present: std::collections::HashSet<(u32, u32)> =
        menu_level.tiles.iter().map(|&(c, r, _)| (c, r)).collect();
    // Use spawn_tile_at_scale with Vec3::ONE — tiles must be full size immediately
    // because animate_scale is gated to Playing state.
    use crate::board::spawn_tile_at_scale;
    for r in 0..menu_level.size { for c in 0..menu_level.size {
        if !present.contains(&(c, r)) {
            spawn_tile_at_scale(&mut commands, c, r, menu_level.size, TileKind::Empty, &assets, Vec3::ONE);
        }
    }}
    for &(c, r, kind) in &menu_level.tiles {
        spawn_tile_at_scale(&mut commands, c, r, menu_level.size, kind, &assets, Vec3::ONE);
    }
}

/// Update: point camera at the board. Hardcoded position that definitely works.
pub fn menu_camera(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    // Same math as camera_direction() with elevation=38, azimuth=45, distance=8
    let pos = Vec3::new(4.3, 4.9, 4.3); // ~8 units from origin at 38° elevation
    for mut tf in cameras.iter_mut() {
        *tf = Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

pub fn cleanup_menu_background() {}
