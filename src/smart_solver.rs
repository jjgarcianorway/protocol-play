// SPDX-License-Identifier: GPL-3.0-or-later
// Smart campaign solver — uses bot path tracing + constraint-based placement.
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
mod smart_solver_sim;
mod smart_solver_core;

use types::*;
use smart_solver_core::{solve_level, SolveResult};

fn main() {
    let dir = std::path::PathBuf::from("campaign_levels");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("campaign_levels/ directory not found")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .map(|e| e.path())
        .collect();
    files.sort();

    println!("=== Smart Solver — protocol: play ===\n");

    let mut stats = RunStats::default();
    let mut chapter_stats: Vec<ChapterStats> = Vec::new();
    let mut cur_ch = ChapterStats::default();
    let mut cur_ch_name = String::new();
    let mut difficulty_counts = [0u32; 5]; // trivial, easy, medium, hard, expert

    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let chapter_prefix = &stem[..2];
        let data: LevelData = serde_json::from_str(
            &std::fs::read_to_string(path).unwrap()
        ).unwrap();

        let ch_name = format!("Chapter {}", chapter_prefix.trim_start_matches('0'));
        if ch_name != cur_ch_name {
            if !cur_ch_name.is_empty() {
                cur_ch.name = cur_ch_name.clone();
                chapter_stats.push(cur_ch.clone());
            }
            cur_ch_name = ch_name.clone();
            cur_ch = ChapterStats::default();
            println!("--- {} ---", cur_ch_name);
        }
        cur_ch.total += 1;

        let start = std::time::Instant::now();
        let result = solve_level(&data);
        let elapsed = start.elapsed().as_secs_f64();

        let diff_idx = match result.difficulty {
            "trivial" => 0, "easy" => 1, "medium" => 2, "hard" => 3, _ => 4,
        };
        difficulty_counts[diff_idx] += 1;

        if result.solved {
            stats.solved += 1;
            cur_ch.solved += 1;
            stats.total_attempts += result.attempts;
            cur_ch.attempts += result.attempts;
            stats.total_time += elapsed;
            cur_ch.time += elapsed;
            println!("  OK  {:<42} {:>8} attempts  {:.3}s  [{}] ({})",
                data.name, result.attempts, elapsed,
                result.difficulty, result.strategy);
        } else {
            stats.failed += 1;
            stats.total_time += elapsed;
            cur_ch.time += elapsed;
            println!("  FAIL {:<42} {:>8} attempts  {:.3}s  ({})",
                data.name, result.attempts, elapsed, result.strategy);
        }
    }
    if !cur_ch_name.is_empty() {
        cur_ch.name = cur_ch_name;
        chapter_stats.push(cur_ch);
    }

    // Summary
    println!("\n=== Results ===\n");
    println!("{:<20} {:>7} {:>12} {:>10}", "Chapter", "Solved", "Attempts", "Time");
    println!("{}", "-".repeat(52));
    for ch in &chapter_stats {
        println!("{:<20} {:>3}/{:<3} {:>12} {:>9.3}s",
            ch.name, ch.solved, ch.total, ch.attempts, ch.time);
    }
    println!("{}", "-".repeat(52));
    let total = stats.solved + stats.failed;
    println!("{:<20} {:>3}/{:<3} {:>12} {:>9.3}s",
        "TOTAL", stats.solved, total, stats.total_attempts, stats.total_time);

    // Difficulty report
    println!("\n=== Difficulty Report ===\n");
    let labels = ["trivial", "easy", "medium", "hard", "expert"];
    for (i, label) in labels.iter().enumerate() {
        if difficulty_counts[i] > 0 {
            let bar = "#".repeat(difficulty_counts[i] as usize);
            println!("  {:<8} {:>3}  {}", label, difficulty_counts[i], bar);
        }
    }

    if stats.failed > 0 {
        println!("\n{} levels unsolved!", stats.failed);
        std::process::exit(1);
    } else {
        println!("\nAll {} levels solved in {:.1}s! GG", total, stats.total_time);
    }
}

#[derive(Default)]
struct RunStats {
    solved: u32,
    failed: u32,
    total_attempts: u64,
    total_time: f64,
}

#[derive(Default, Clone)]
struct ChapterStats {
    name: String,
    solved: u32,
    total: u32,
    attempts: u64,
    time: f64,
}
