// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D board background for the main menu.

use bevy::prelude::*;
use rand::SeedableRng;
use crate::constants::*;
use crate::types::*;
use crate::board::spawn_tile;
use crate::level_gen_algo::{GenConfig, HolePlacement, generate_attempt};

#[derive(Component)] pub struct MenuBgEntity;
#[derive(Resource)] pub struct MenuCameraOrbit { pub angle: f32 }

/// Startup: generate a level and spawn it as the menu background.
/// Also despawns the default 3x3 board from setup_scene and updates BoardSize.
pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    existing_tiles: Query<Entity, With<Tile>>,
    mut board_size: ResMut<BoardSize>,
) {
    // Despawn the initial 3x3 board from setup_scene
    for e in existing_tiles.iter() { commands.entity(e).despawn(); }

    let size = MENU_BG_BOARD_SIZE;
    board_size.0 = size;

    let config = GenConfig {
        board_size: size, num_bots: 3, hole_percent: 12,
        hole_placement: HolePlacement::Both, difficulty: 40,
        weights: [5; GEN_NUM_WEIGHTS], unique_solution: false,
        inventory_target: 0, door_chains: 0, path_sharing: true,
        confusion_tiles: false, chapter_idx: 2, required_tile: None,
    };

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    // Try a few seeds to find a nice-looking level
    let tile_data = (0..20u64).find_map(|seed_offset| {
        let mut r = rand::rngs::StdRng::seed_from_u64(42 + seed_offset);
        generate_attempt(&config, &mut r)
    });

    if let Some((tiles, _)) = tile_data {
        let present: std::collections::HashSet<(u32, u32)> =
            tiles.iter().map(|&(c, r, _, _)| (c, r)).collect();
        // Spawn empty cells for holes
        for row in 0..size { for col in 0..size {
            if !present.contains(&(col, row)) {
                let e = spawn_tile(&mut commands, col, row, size, TileKind::Empty, &assets);
                commands.entity(e).insert(MenuBgEntity);
            }
        }}
        // Spawn level tiles
        for &(col, row, kind, _) in &tiles {
            let e = spawn_tile(&mut commands, col, row, size, kind, &assets);
            commands.entity(e).insert(MenuBgEntity);
        }
    } else {
        // Fallback: empty board
        for row in 0..size { for col in 0..size {
            let e = spawn_tile(&mut commands, col, row, size, TileKind::Floor, &assets);
            commands.entity(e).insert(MenuBgEntity);
        }}
    }

    commands.insert_resource(MenuCameraOrbit { angle: 0.0 });
}

/// Slowly orbit the camera around the board.
pub fn animate_menu_camera(
    time: Res<Time>,
    mut orbit: ResMut<MenuCameraOrbit>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<IconCamera>)>,
) {
    orbit.angle += MENU_CAMERA_ORBIT_SPEED * time.delta_secs();
    let elev = MENU_CAMERA_ELEVATION.to_radians();
    let dist = MENU_CAMERA_DISTANCE;
    let pos = Vec3::new(
        elev.cos() * orbit.angle.sin() * dist,
        elev.sin() * dist,
        elev.cos() * orbit.angle.cos() * dist,
    );
    for mut tf in cameras.iter_mut() {
        *tf = Transform::from_translation(pos).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

/// OnExit(MainMenu): despawn background entities.
pub fn cleanup_menu_background(
    mut commands: Commands,
    q: Query<Entity, With<MenuBgEntity>>,
) {
    for e in q.iter() { commands.entity(e).despawn(); }
    commands.remove_resource::<MenuCameraOrbit>();
}
