// SPDX-License-Identifier: GPL-3.0-or-later
//! Animated 3D board background for the main menu.
//! Generates a small puzzle level and slowly orbits the camera around it.

use bevy::prelude::*;
use rand::SeedableRng;
use crate::constants::*;
use crate::types::*;
use crate::board::{spawn_tile, camera_direction, board_bounding_radius};
use crate::level_gen_algo::{GenConfig, HolePlacement, generate_attempt};

/// Tag for all entities spawned as part of the menu background.
#[derive(Component)]
pub struct MenuBgEntity;

/// Tracks the orbit angle of the menu camera.
#[derive(Resource)]
pub struct MenuCameraOrbit {
    pub angle: f32,
}

/// Startup system: generate a small level and spawn its tiles as the menu background.
pub fn setup_menu_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let config = GenConfig {
        board_size: MENU_BG_BOARD_SIZE,
        num_bots: 2,
        hole_percent: 15,
        hole_placement: HolePlacement::Edges,
        difficulty: 30,
        weights: [5; GEN_NUM_WEIGHTS],
        unique_solution: false,
        inventory_target: 0,
        door_chains: 0,
        path_sharing: false,
        confusion_tiles: false,
        chapter_idx: 0,
        required_tile: None,
    };

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let tile_data = generate_attempt(&config, &mut rng);

    let size = MENU_BG_BOARD_SIZE;
    if let Some((tiles, _diff)) = tile_data {
        // Spawn solution tiles (the full board)
        let mut present = std::collections::HashSet::new();
        for &(col, row, _, _) in &tiles {
            present.insert((col, row));
        }
        // First spawn empty tiles for cells not in the solution
        for row in 0..size {
            for col in 0..size {
                if !present.contains(&(col, row)) {
                    let e = spawn_tile(&mut commands, col, row, size, TileKind::Empty, &assets);
                    commands.entity(e).insert(MenuBgEntity);
                }
            }
        }
        // Then spawn the solution tiles
        for &(col, row, ref kind, _is_inv) in &tiles {
            let e = spawn_tile(&mut commands, col, row, size, kind.clone(), &assets);
            commands.entity(e).insert(MenuBgEntity);
        }
    } else {
        // Fallback: just spawn a plain empty board
        for row in 0..size {
            for col in 0..size {
                let e = spawn_tile(&mut commands, col, row, size, TileKind::Empty, &assets);
                commands.entity(e).insert(MenuBgEntity);
            }
        }
    }

    commands.insert_resource(MenuCameraOrbit { angle: 0.0 });
}

/// Update system: slowly orbit the camera around the board center.
pub fn animate_menu_camera(
    time: Res<Time>,
    mut orbit: ResMut<MenuCameraOrbit>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<IconCamera>)>,
) {
    orbit.angle += MENU_CAMERA_ORBIT_SPEED * time.delta_secs();

    let elev = MENU_CAMERA_ELEVATION.to_radians();
    let azim = orbit.angle;

    let dir = Vec3::new(
        elev.cos() * azim.sin(),
        elev.sin(),
        elev.cos() * azim.cos(),
    );

    let distance = MENU_CAMERA_DISTANCE;

    for mut transform in cameras.iter_mut() {
        *transform = Transform::from_translation(dir * distance)
            .looking_at(Vec3::ZERO, Vec3::Y);
    }
}

/// OnExit(MainMenu): despawn all menu background entities.
pub fn cleanup_menu_background(
    mut commands: Commands,
    q: Query<Entity, With<MenuBgEntity>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<MenuCameraOrbit>();
}
