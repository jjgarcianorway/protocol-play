// SPDX-License-Identifier: GPL-3.0-or-later
//
// Player mode: standalone exe that loads level.json and enters test mode directly.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::ui_helpers::*;
use crate::board::spawn_tile;
use crate::test_mode::{group_tiles, spawn_test_inventory};

const LEVEL_FILE: &str = "level.json";

pub fn setup_player(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut board_size: ResMut<BoardSize>,
    mut test_inv: ResMut<TestInventory>,
    icons: Res<InventoryIcons>,
    font: Res<GameFont>,
    mut play_mode: ResMut<PlayMode>,
    mut placed_teleports: ResMut<PlacedTeleports>,
) {
    // Look for level.json next to the executable, then fall back to current directory
    let exe_dir = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));
    info!("Executable dir: {:?}", exe_dir);
    let level_path = exe_dir.as_ref()
        .map(|d| d.join(LEVEL_FILE))
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from(LEVEL_FILE));
    info!("Loading level from: {}", level_path.display());
    let json = match std::fs::read_to_string(&level_path) {
        Ok(j) => j,
        Err(e) => { error!("Failed to read {}: {e}", level_path.display()); return; }
    };
    let level: LevelData = match serde_json::from_str(&json) {
        Ok(l) => l,
        Err(e) => { eprintln!("Failed to parse {LEVEL_FILE}: {e}"); return; }
    };

    // Despawn default board
    for entity in &tiles { commands.entity(entity).despawn_recursive(); }
    board_size.0 = level.board_size.clamp(MIN_BOARD_SIZE, MAX_BOARD_SIZE);

    // Separate marked (inventory) vs unmarked (board) tiles
    let mut board_tiles = Vec::new();
    let mut marked_kinds = Vec::new();
    placed_teleports.0 = [0; 10];
    for &(col, row, kind, is_marked) in &level.tiles {
        if col >= board_size.0 || row >= board_size.0 { continue; }
        if is_marked {
            marked_kinds.push(kind);
            board_tiles.push((col, row, TileKind::Empty));
        } else {
            if let TileKind::Teleport(n) = kind { placed_teleports.0[n] += 1; }
            board_tiles.push((col, row, kind));
        }
    }

    // Fill missing grid positions with Empty
    let mut grid = std::collections::HashSet::new();
    for &(col, row, _) in &board_tiles { grid.insert((col, row)); }
    for row in 0..board_size.0 { for col in 0..board_size.0 {
        if !grid.contains(&(col, row)) { board_tiles.push((col, row, TileKind::Empty)); }
    }}

    // Spawn board
    for &(col, row, kind) in &board_tiles {
        spawn_tile(&mut commands, col, row, board_size.0, kind, &assets);
    }

    // Build test inventory
    test_inv.items = group_tiles(marked_kinds.into_iter());
    test_inv.selected = None;
    test_inv.remove_mode = false;

    // Save initial state for reset
    commands.insert_resource(SavedTestState {
        tiles: board_tiles, inventory: test_inv.items.clone(),
    });

    // Spawn player UI
    spawn_test_inventory(&mut commands, &test_inv, &icons, true, &font.0);
    spawn_player_buttons(&mut commands, &font.0);

    *play_mode = PlayMode::TestEditing;
}

fn spawn_player_buttons(commands: &mut Commands, f: &Handle<Font>) {
    let (tf, tc, br) = (gf(LABEL_FONT, f), TextColor(Color::WHITE), BorderRadius::all(Val::Px(UI_CORNER_RADIUS)));
    let btn = text_btn_node();
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(10.0), top: Val::Px(-50.0),
        flex_direction: FlexDirection::Row, column_gap: Val::Px(4.0), ..default() },
        UiTopAnim { target: TOP_SLIDE_SHOW, despawn_at_target: false }, TestTopButtons,
    )).with_children(|p| {
        p.spawn((Button, ResetTestButton, btn, BackgroundColor(btn_bg()), br))
            .with_child((Text::new("Reset"), tf, tc));
    });
}
