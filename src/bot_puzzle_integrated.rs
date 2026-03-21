// SPDX-License-Identifier: GPL-3.0-or-later
//! Integrated bot puzzle — system registration + handlers for GameScene::BotPuzzle.
//! Scene enter/exit logic lives in bot_puzzle_scene.rs.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::board::*;
use crate::mission::types::{GameScene, MissionCamera};
use crate::save_state::{load_game_state, save_game_state};
use crate::test_mode::spawn_test_inventory;

/// Marker: every entity spawned by the bot puzzle scene.
#[derive(Component)]
pub struct BotPuzzleEntity;

/// Data loaded for the current puzzle level.
#[derive(Resource)]
pub struct BotPuzzleLevel {
    pub level: LevelData,
    pub filename: String,
}

/// Register all bot-puzzle systems for integrated (full) mode.
pub fn register_integrated_systems(app: &mut App) {
    use crate::bot_puzzle_scene::{enter_bot_puzzle, exit_bot_puzzle};

    app.add_systems(OnEnter(GameScene::BotPuzzle), enter_bot_puzzle)
        .add_systems(OnExit(GameScene::BotPuzzle), exit_bot_puzzle)
        // Core animation + hover systems
        .add_systems(Update, (
            crate::systems::update_hovered_cell,
            crate::systems::update_ghost_and_highlight
                .after(crate::systems::update_hovered_cell),
            crate::systems::animate_scale
                .after(crate::systems::update_ghost_and_highlight)
                .after(crate::simulation::move_bots)
                .after(crate::bot_formation::apply_bot_formation),
            crate::systems::animate_node_width,
            crate::systems::animate_ui_slides,
            crate::systems::animate_border_fade,
            crate::systems::cleanup_despawned
                .after(crate::systems::animate_scale),
        ).run_if(in_state(GameScene::BotPuzzle)))
        // Simulation + play/stop + camera
        .add_systems(Update, (
            crate::simulation::overlay_button_interaction,
            crate::simulation::play_stop_interaction
                .after(crate::simulation::overlay_button_interaction),
            crate::simulation::move_bots
                .after(crate::simulation::play_stop_interaction),
            crate::bot_formation::update_bot_formation
                .after(crate::simulation::move_bots),
            crate::bot_formation::apply_bot_formation
                .after(crate::bot_formation::update_bot_formation),
            crate::bot_formation::animate_merge_flashes,
            crate::simulation::paint_bots
                .after(crate::simulation::move_bots),
            crate::simulation::toggle_doors
                .after(crate::simulation::move_bots),
            crate::simulation::check_simulation_result
                .after(crate::simulation::move_bots),
            crate::simulation::spawn_simulation_overlay
                .after(crate::simulation::check_simulation_result),
            crate::simulation::animate_sim_overlay_fade,
            adapt_camera_bot_puzzle,
            crate::systems_ui::sync_ui_play_mode,
        ).run_if(in_state(GameScene::BotPuzzle)))
        // Test mode tile placement
        .add_systems(Update, (
            crate::test_mode::handle_test_tile_click
                .after(crate::systems::update_hovered_cell),
            crate::test_mode::test_inventory_interaction,
            crate::test_mode::reset_test_interaction,
            crate::inventory::update_status_bar,
            crate::test_mode::test_tile_sound
                .after(crate::test_mode::handle_test_tile_click),
        ).run_if(in_state(GameScene::BotPuzzle)))
        // Level complete + inventory spawn
        .add_systems(Update, (
            handle_bot_level_complete,
            spawn_bot_inventory_once,
        ).run_if(in_state(GameScene::BotPuzzle)));
}

/// Spawn the test inventory UI one frame after enter (resources now exist).
fn spawn_bot_inventory_once(
    mut commands: Commands,
    test_inv: Option<Res<TestInventory>>,
    icons: Option<Res<InventoryIcons>>,
    font: Option<Res<GameFont>>,
    existing: Query<Entity, With<TestInventoryContainer>>,
    level: Option<Res<BotPuzzleLevel>>,
) {
    if !existing.is_empty() || level.is_none() { return; }
    let (Some(inv), Some(ic), Some(f)) = (test_inv, icons, font) else { return };
    spawn_test_inventory(&mut commands, &inv, &ic, true, &f.0);
}

/// Handle level complete: increment bot_level, save, return to Dashboard.
fn handle_bot_level_complete(
    mut validated: ResMut<LevelValidated>,
    level: Option<Res<BotPuzzleLevel>>,
    mut next_scene: ResMut<NextState<GameScene>>,
) {
    if !validated.is_changed() || !validated.0 { return; }
    validated.0 = false;
    let Some(_level) = level else { return };

    // Increment bot_level and save
    let mut gs = load_game_state();
    gs.bot_level += 1;
    save_game_state(&gs);

    // Return to Dashboard
    next_scene.set(GameScene::Dashboard);
}

/// Camera system for bot puzzle (no expansion container or player nav).
fn adapt_camera_bot_puzzle(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection),
        (With<Camera3d>, Without<IconCamera>, Without<MissionCamera>)>,
    board_size: Res<BoardSize>,
    play_mode: Res<PlayMode>,
    time: Res<Time>,
) -> Result {
    let window = windows.single()?;
    let (mut transform, projection) = cameras.single_mut()?;
    let (w, h) = (window.width(), window.height());
    let aspect = w / h;
    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => return Ok(()),
    };
    let playing = matches!(*play_mode, PlayMode::Playing | PlayMode::TestPlaying);
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let radius = board_bounding_radius(board_size.0);
    let vw = w / 100.0;

    let (top_px, bot_px) = if playing {
        (0.0, 0.0)
    } else {
        let inv = SLOT_HEIGHT_VW * vw + INVENTORY_PAD_VW * 2.0 * vw + INV_SLIDE_SHOW;
        (h * 0.06, inv + h * 0.12)
    };

    let usable_h = (h - top_px - bot_px).max(100.0);
    let rv = if playing { radius * 0.85 } else { radius * 0.7 };
    let usable_fov_v = half_fov_v * (usable_h / h);
    let dist_v = rv / usable_fov_v.sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;
    let look_y = if playing { 0.0 } else { -0.09 * distance };
    let look_at = Vec3::new(0.0, look_y, 0.0);
    let dir = camera_direction();
    let target = Transform::from_translation(look_at + dir * distance)
        .looking_at(look_at, Vec3::Y);
    let speed = CAMERA_ZOOM_SPEED * time.delta_secs();
    transform.translation = transform.translation.lerp(target.translation, speed);
    transform.rotation = transform.rotation.slerp(target.rotation, speed);
    Ok(())
}
