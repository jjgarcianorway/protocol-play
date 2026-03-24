// SPDX-License-Identifier: GPL-3.0-or-later
// count-solutions: Enumerate distinct solutions for each campaign level and embed the count
// in the level JSON as `solution_count`. Run once after campaign generation.
// Usage: cargo run --bin count-solutions [-- --dir campaign_levels]
#![allow(dead_code, unused_imports)]

mod constants;
mod types;
mod textures;
mod gen_textures;
mod board;
mod ui_helpers;
mod slot_ui;
mod inventory;
pub mod sound;
mod systems;
mod simulation;
mod messages;
mod i18n;
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
use smart_solver_core::count_solutions;

fn main() {
    let dir_arg = std::env::args().skip(1).find(|a| !a.starts_with("--"))
        .unwrap_or_else(|| "campaign_levels".into());
    let dir = std::path::PathBuf::from(&dir_arg);
    let max_solutions: u32 = 10; // cap at 10 — if we hit this, we know it's "many"
    let fast_only = std::env::args().any(|a| a == "--fast");

    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("Directory not found: {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .map(|e| e.path())
        .collect();
    files.sort();

    println!("Counting solutions for {} levels (cap: {max_solutions})...\n", files.len());

    let mut exactly_one = 0usize;
    let mut multiple = 0usize;
    let mut capped = 0usize;
    let mut unsolvable = 0usize;

    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();
        let json_str = std::fs::read_to_string(path).unwrap();
        let mut data: LevelData = match serde_json::from_str(&json_str) {
            Ok(d) => d,
            Err(e) => { eprintln!("SKIP {stem}: {e}"); continue; }
        };

        let start = std::time::Instant::now();
        let (count, exact) = count_solutions(&data, max_solutions);
        let elapsed = start.elapsed().as_secs_f64();

        let display = if exact { format!("{count}") } else { format!("{count}+") };
        let flag = if count == 0 { "✗ UNSOLVABLE" }
            else if !exact { "~ capped" }
            else if count == 1 { "✓ unique" }
            else { "! multiple" };
        println!("{stem}: {display} solutions [{flag}] ({elapsed:.2}s)");

        if count == 0 { unsolvable += 1; }
        else if !exact { capped += 1; }
        else if count == 1 { exactly_one += 1; }
        else { multiple += 1; }

        data.solution_count = Some(if exact { count } else { count }); // store what we found (capped value)
        let new_json = serde_json::to_string_pretty(&data).unwrap();
        std::fs::write(path, new_json).unwrap();
    }

    println!("\n=== Summary ===");
    println!("Unique (1 solution):  {exactly_one}");
    println!("Multiple solutions:   {multiple}");
    println!("Capped (≥{max_solutions}):          {capped}");
    println!("Unsolvable:           {unsolvable}");
    println!("Total:                {}", files.len());
}
