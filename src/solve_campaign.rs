// SPDX-License-Identifier: GPL-3.0-or-later
// Headless campaign solver — Claude plays all 149 levels without peeking at solutions.
#![allow(dead_code, unused_imports)]

mod constants;
mod types;
mod textures;
mod gen_textures;
mod board;
mod ui_helpers;
mod slot_ui;
mod inventory;
mod systems;
mod simulation;
mod messages;
mod bot_formation;
mod mat_helpers;
mod test_mode;
mod level_io;
mod save_dialog;
mod level_gen_sim;
mod level_gen_tiles;
mod level_gen_algo;
mod level_gen_ui;
mod level_gen_interact;

use types::*;
use level_gen_sim::simulate_headless;

fn main() {
    let dir = std::path::PathBuf::from("campaign_levels");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("campaign_levels/ directory not found")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .map(|e| e.path())
        .collect();
    files.sort();

    println!("=== Claude plays protocol: play ===\n");
    let (mut solved, mut failed, mut total_attempts) = (0u32, 0u32, 0u64);
    let mut total_time = 0.0f64;
    let mut chapter_stats: Vec<(String, u32, u32, u64, f64)> = Vec::new(); // (name, solved, total, attempts, time)
    let mut current_chapter = String::new();
    let mut ch_solved = 0u32;
    let mut ch_total = 0u32;
    let mut ch_attempts = 0u64;
    let mut ch_time = 0.0f64;

    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let chapter_prefix = &stem[..2]; // "01", "02", etc.
        let data: LevelData = serde_json::from_str(
            &std::fs::read_to_string(path).unwrap()
        ).unwrap();

        // Detect chapter change
        let ch_name = format!("Chapter {}", chapter_prefix.trim_start_matches('0'));
        if ch_name != current_chapter {
            if !current_chapter.is_empty() {
                chapter_stats.push((current_chapter.clone(), ch_solved, ch_total, ch_attempts, ch_time));
            }
            current_chapter = ch_name.clone();
            ch_solved = 0; ch_total = 0; ch_attempts = 0; ch_time = 0.0;
            println!("--- {} ---", current_chapter);
        }
        ch_total += 1;

        let start = std::time::Instant::now();
        let result = solve_level(&data);
        let elapsed = start.elapsed().as_secs_f64();

        match result {
            Some(attempts) => {
                solved += 1; ch_solved += 1;
                total_attempts += attempts; ch_attempts += attempts;
                total_time += elapsed; ch_time += elapsed;
                let tag = if attempts == 1 { "instant" }
                    else if attempts <= 10 { "quick" }
                    else if attempts <= 100 { "thought" }
                    else if attempts <= 1000 { "tricky" }
                    else { "hard" };
                println!("  OK  {:<42} {:>8} attempts  {:.3}s  ({})",
                    data.name, attempts, elapsed, tag);
            }
            None => {
                failed += 1;
                total_time += elapsed; ch_time += elapsed;
                println!("  FAIL {:<42} {:.3}s  !!!", data.name, elapsed);
            }
        }
    }
    if !current_chapter.is_empty() {
        chapter_stats.push((current_chapter, ch_solved, ch_total, ch_attempts, ch_time));
    }

    // Summary
    println!("\n=== Results ===\n");
    println!("{:<20} {:>7} {:>12} {:>10}", "Chapter", "Solved", "Attempts", "Time");
    println!("{}", "-".repeat(52));
    for (name, s, t, a, time) in &chapter_stats {
        println!("{:<20} {:>3}/{:<3} {:>12} {:>9.3}s", name, s, t, a, time);
    }
    println!("{}", "-".repeat(52));
    println!("{:<20} {:>3}/{:<3} {:>12} {:>9.3}s",
        "TOTAL", solved, solved + failed, total_attempts, total_time);
    if failed > 0 {
        println!("\n{} levels stumped me!", failed);
    } else {
        println!("\nAll levels solved! GG");
    }
}

fn solve_level(data: &LevelData) -> Option<u64> {
    let size = data.board_size;

    // Separate board tiles from inventory (marked tiles become Floor on the board)
    let mut board_tiles: Vec<(u32, u32, TileKind)> = Vec::new();
    let mut inventory: Vec<TileKind> = Vec::new();

    for &(c, r, kind, marked) in &data.tiles {
        if marked {
            inventory.push(kind);
            board_tiles.push((c, r, TileKind::Floor));
        } else {
            board_tiles.push((c, r, kind));
        }
    }

    // No inventory — just check if it's already solved
    if inventory.is_empty() {
        return if simulate_headless(size, &board_tiles) { Some(0) } else { None };
    }

    // Find all Floor positions where we could place tiles
    let floor_positions: Vec<(u32, u32)> = board_tiles.iter()
        .filter(|(_, _, k)| matches!(k, TileKind::Floor))
        .map(|(c, r, _)| (*c, *r))
        .collect();

    // Sort inventory: most constrained tiles first for better pruning
    inventory.sort_by_key(|k| constraint_priority(k));

    // Build position index map for fast tile placement
    let pos_to_idx: std::collections::HashMap<(u32, u32), usize> = board_tiles.iter()
        .enumerate().map(|(i, (c, r, _))| ((*c, *r), i)).collect();

    let mut attempts = 0u64;
    let mut used = vec![false; floor_positions.len()];
    let mut working_tiles = board_tiles.clone();

    if backtrack(
        &inventory, 0, &floor_positions, &mut used,
        &mut working_tiles, &pos_to_idx, size, &mut attempts,
    ) {
        Some(attempts)
    } else {
        None
    }
}

fn backtrack(
    inventory: &[TileKind],
    depth: usize,
    positions: &[(u32, u32)],
    used: &mut Vec<bool>,
    tiles: &mut Vec<(u32, u32, TileKind)>,
    pos_idx: &std::collections::HashMap<(u32, u32), usize>,
    size: u32,
    attempts: &mut u64,
) -> bool {
    if depth == inventory.len() {
        *attempts += 1;
        return simulate_headless(size, tiles);
    }

    let kind = inventory[depth];

    // Symmetry breaking: if same kind as previous, only try positions after previous
    let min_pos = if depth > 0 && inventory[depth] == inventory[depth - 1] {
        // Find position index of last placed tile
        (0..positions.len()).find(|&i| used[i] && {
            let idx = pos_idx[&positions[i]];
            tiles[idx].2 == inventory[depth - 1]
        }).map_or(0, |i| i + 1)
    } else {
        0
    };

    for i in min_pos..positions.len() {
        if used[i] { continue; }

        let pos = positions[i];
        let tile_idx = pos_idx[&pos];

        // Place tile
        used[i] = true;
        tiles[tile_idx].2 = kind;

        if backtrack(inventory, depth + 1, positions, used, tiles, pos_idx, size, attempts) {
            return true;
        }

        // Undo placement
        tiles[tile_idx].2 = TileKind::Floor;
        used[i] = false;
    }
    false
}

fn constraint_priority(kind: &TileKind) -> u8 {
    match kind {
        TileKind::Teleport(..) | TileKind::TeleportBut(..) => 0,
        TileKind::Source(..) => 1,
        TileKind::Turn(..) | TileKind::TurnBut(..) => 2,
        TileKind::Arrow(..) | TileKind::ArrowBut(..) => 2,
        TileKind::Bounce(..) | TileKind::BounceBut(..) => 3,
        TileKind::ColorSwitch(..) | TileKind::ColorSwitchBut(..) => 3,
        TileKind::Switch | TileKind::Door(..) => 4,
        TileKind::Painter(..) => 4,
        TileKind::Goal(..) => 5,
        _ => 6,
    }
}
