// SPDX-License-Identifier: GPL-3.0-or-later
// Campaign level generator — standalone binary that produces all levels for the game.
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

use constants::*;
use types::*;
use level_gen_algo::*;

fn main() {
    let force = std::env::args().any(|a| a == "--force");
    let chapters = campaign_chapters();
    let out_dir = std::path::PathBuf::from("campaign_levels");
    let _ = std::fs::create_dir_all(&out_dir);
    let (mut total, mut kept, mut failed) = (0u32, 0u32, 0u32);
    for (ci, chapter) in chapters.iter().enumerate() {
        println!("\n=== Chapter {}: {} ===", ci + 1, chapter.name);
        for (li, level) in chapter.levels.iter().enumerate() {
            let (num, ch_num) = (li + 1, ci + 1);
            let filename = format!("{:02}_{:02}_{}", ch_num, num, sanitize(&level.name));
            let path = out_dir.join(format!("{filename}.json"));
            print!("  [{ch_num}-{num:02}] {} ({}x{}, diff={}, bots={})... ",
                level.name, level.config.board_size, level.config.board_size,
                level.config.difficulty, level.config.num_bots);
            if !force && path.exists() { println!("KEPT"); kept += 1; total += 1; continue; }
            let tier = ((level.config.num_bots.saturating_sub(1)) / 2).min(3) as usize;
            let attempts = GEN_CAMPAIGN_ATTEMPTS[tier];
            match generate_level(&level.config, attempts) {
                Some((tiles, rating, seed)) => {
                    let solution: Vec<_> = tiles.iter()
                        .filter(|(_, _, _, m)| *m).map(|(c, r, k, _)| (*c, *r, *k)).collect();
                    let data = LevelData {
                        name: level.name.clone(), board_size: level.config.board_size,
                        tiles, solution, seed: Some(seed), difficulty: Some(rating),
                    };
                    let json = serde_json::to_string_pretty(&data).unwrap();
                    std::fs::write(&path, json).unwrap();
                    println!("OK (diff={rating})");
                    total += 1;
                }
                None => { println!("FAILED"); failed += 1; }
            }
        }
    }
    println!("\n=== Done: {total} levels ({kept} kept, {} new), {failed} failed ===", total - kept);
}

fn generate_level(config: &GenConfig, max_attempts: usize)
    -> Option<(Vec<(u32, u32, TileKind, bool)>, u32, u64)>
{
    let mut rng = rand::thread_rng();
    let seed: u64 = rand::Rng::r#gen(&mut rng);
    let mut best: Option<(Vec<(u32, u32, TileKind, bool)>, u32)> = None;
    for _ in 0..max_attempts {
        if let Some((tiles, rating)) = generate_attempt(config, &mut rng) {
            // If a required tile type is specified, reject attempts without it
            if let Some(req) = config.required_tile {
                if !tiles.iter().any(|(_, _, k, _)| req(k)) { continue; }
            }
            let gap = (rating as i32 - config.difficulty as i32).unsigned_abs();
            let prev_gap = best.as_ref()
                .map(|(_, r)| (*r as i32 - config.difficulty as i32).unsigned_abs())
                .unwrap_or(u32::MAX);
            if gap < prev_gap { best = Some((tiles, rating)); }
            if gap <= GEN_BEST_TOLERANCE { break; }
        }
    }
    best.map(|(t, r)| (t, r, seed))
}

fn sanitize(name: &str) -> String {
    name.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect::<String>().trim_matches('_').to_string()
}

// ===================================================================
// Campaign definition — 13 chapters, 149 levels
// ===================================================================

struct Chapter { name: String, levels: Vec<Level> }
struct Level { name: String, config: GenConfig }

fn cfg(board: u32, bots: u32, diff: u32, weights: [u32; GEN_NUM_WEIGHTS]) -> GenConfig {
    GenConfig {
        board_size: board, num_bots: bots, difficulty: diff, weights,
        hole_percent: 20, hole_placement: HolePlacement::Both,
        unique_solution: false, inventory_target: 0, // set per-level in make_level
        door_chains: 0, path_sharing: false, confusion_tiles: false,
        required_tile: None,
    }
}

/// Cumulative weights: new mechanic emphasized, all previously learned included.
fn ch_w(ch: usize, pos: usize) -> [u32; GEN_NUM_WEIGHTS] {
    // Weight index -> chapter where it's introduced
    let unlock = [1usize, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 9];
    let new_idx: &[usize] = match ch {
        1=>&[0], 2=>&[1], 3=>&[2], 4=>&[3], 5=>&[4], 6=>&[5],
        7=>&[6], 8=>&[7], 9=>&[11], 10=>&[8], 11=>&[9], 12=>&[10], _=>&[],
    };
    let (nw, ow) = if ch >= 13 { (8, 8) }
        else if pos == 0 { (10, 2) }   // intro: new mechanic dominant
        else if pos <= 3 { (8, 4) }     // learning: new dominant
        else if pos <= 7 { (7, 5) }     // challenge: mixed
        else { (6, 6) };               // boss: balanced
    let mut w = [0u32; GEN_NUM_WEIGHTS];
    for i in 0..GEN_NUM_WEIGHTS {
        if new_idx.contains(&i) { w[i] = nw; }
        else if ch >= unlock[i] { w[i] = ow; }
    }
    w
}

/// Build a level with chapter-appropriate modifiers applied automatically.
fn make_level(ch: usize, pos: usize, name: &str, board: u32, bots: u32, diff: u32) -> Level {
    let mut c = cfg(board, bots, diff, ch_w(ch, pos));
    if ch >= 2 { c.path_sharing = true; }
    c.confusion_tiles = pos >= 3; // confusion tiles in all chapters from level 4 onwards
    if ch >= 10 { c.door_chains = match pos { 0..=2=>1, 3..=5=>2, 6..=8=>3, _=>4 }; }
    let hole_base = if ch <= 2 { 8 } else if ch <= 6 { 15 } else { 20 };
    let pct = hole_base + pos as u32 * 2;
    match pos % 3 {
        0 => { c.hole_placement = HolePlacement::Edges; c.hole_percent = pct; }
        1 => { c.hole_placement = HolePlacement::Middle; c.hole_percent = pct; }
        _ => { c.hole_percent = pct; }
    }
    // More inventory tiles to place — scales with position and board size
    let inv_min = 2 + (pos as u32 / 2); // 2-6 based on position
    let inv_max = board.min(10);
    c.inventory_target = inv_min.max(3).min(inv_max);
    // Intro levels must contain the chapter's new mechanic
    if pos == 0 {
        c.required_tile = match ch {
            1 => Some(|k: &TileKind| matches!(k, TileKind::Turn(..))),
            2 => Some(|k: &TileKind| matches!(k, TileKind::TurnBut(..))),
            3 => Some(|k: &TileKind| matches!(k, TileKind::Arrow(..))),
            4 => Some(|k: &TileKind| matches!(k, TileKind::ArrowBut(..))),
            5 => Some(|k: &TileKind| matches!(k, TileKind::Teleport(..))),
            6 => Some(|k: &TileKind| matches!(k, TileKind::TeleportBut(..))),
            7 => Some(|k: &TileKind| matches!(k, TileKind::Bounce(..))),
            8 => Some(|k: &TileKind| matches!(k, TileKind::BounceBut(..))),
            9 => Some(|k: &TileKind| matches!(k, TileKind::Painter(..))),
            10 => Some(|k: &TileKind| matches!(k, TileKind::Door(..) | TileKind::Switch)),
            11 => Some(|k: &TileKind| matches!(k, TileKind::ColorSwitch(..))),
            12 => Some(|k: &TileKind| matches!(k, TileKind::ColorSwitchBut(..))),
            _ => None,
        };
    }
    Level { name: name.into(), config: c }
}

fn ch(num: usize, name: &str, specs: &[(&str, u32, u32, u32)]) -> Chapter {
    Chapter {
        name: name.into(),
        levels: specs.iter().enumerate()
            .map(|(pos, &(n, b, bt, d))| make_level(num, pos, n, b, bt, d)).collect(),
    }
}

fn campaign_chapters() -> Vec<Chapter> {
    vec![
        // Ch1: Turns — learn path building (1→4 bots)
        ch(1, "Turns", &[
            ("First Steps",       3, 1, 15),
            ("Corner to Corner",  4, 2, 30),
            ("The Zigzag",        4, 2, 40),
            ("Around the Block",  5, 2, 50),
            ("Spiral Path",       5, 3, 55),
            ("Double Back",       6, 3, 60),
            ("Winding Road",      6, 3, 65),
            ("Crossroads",        6, 3, 70),
            ("Turn Master I",     7, 4, 80),
            ("Turn Master II",    8, 4, 85),
            ("Turn Master III",   8, 4, 90),
        ]),
        // Ch2: TurnBut — place turns from inventory (2→4 bots, uses Turn+TurnBut)
        ch(2, "Turn Tiles", &[
            ("Place Your Turn",       4, 2, 25),
            ("Two Turns",             4, 2, 35),
            ("Choose Wisely",         5, 2, 45),
            ("Mixed Turns",           5, 3, 55),
            ("Turn Puzzle",           6, 3, 60),
            ("Inventory Challenge",   6, 3, 65),
            ("Precision Placement",   6, 4, 70),
            ("No Room for Error",     7, 4, 75),
            ("Turn Builder I",        7, 4, 80),
            ("Turn Builder II",       8, 4, 85),
            ("Turn Builder III",      8, 4, 90),
        ]),
        // Ch3: Arrows — forced direction (2→5 bots, uses Turn+TurnBut+Arrow)
        ch(3, "Arrows", &[
            ("One Way Street",    4, 2, 30),
            ("Follow the Arrow",  5, 2, 40),
            ("Arrow Maze",        5, 3, 50),
            ("Turn and Thrust",   5, 3, 55),
            ("Arrow Chain",       6, 3, 60),
            ("Speed Lines",       6, 4, 65),
            ("The Gauntlet",      7, 4, 70),
            ("Forced March",      7, 4, 75),
            ("Arrow Storm I",     8, 5, 85),
            ("Arrow Storm II",    8, 5, 90),
            ("Arrow Storm III",   9, 5, 95),
        ]),
        // Ch4: ArrowBut — place arrows (2→5 bots, uses all Turn/Arrow variants)
        ch(4, "Arrow Tiles", &[
            ("Place Your Arrow",  4, 2, 30),
            ("Arrow Setup",       5, 2, 45),
            ("Redirect",          5, 3, 55),
            ("Arrow Architect",   6, 3, 60),
            ("Mixed Signals",     6, 4, 65),
            ("Direction Control", 6, 4, 70),
            ("Arrow Master",      7, 4, 75),
            ("Full Arsenal",      7, 5, 80),
            ("Arrow Crafter I",   8, 5, 85),
            ("Arrow Crafter II",  8, 5, 90),
            ("Arrow Crafter III", 9, 5, 95),
        ]),
        // Ch5: Teleports — warp mechanics (3→7 bots, uses all previous)
        ch(5, "Teleports", &[
            ("Warp Zone",          5, 3, 35),
            ("Portal Hop",         6, 3, 50),
            ("Double Warp",        6, 4, 55),
            ("Teleport Chain",     7, 4, 60),
            ("Warp Tactics",       7, 5, 70),
            ("Portal Network",     8, 5, 75),
            ("Dimensional Shift",  8, 5, 80),
            ("Space Fold",         9, 6, 85),
            ("Warp Master I",      9, 6, 90),
            ("Warp Master II",    10, 6, 95),
            ("Warp Master III",   10, 7, 100),
        ]),
        // Ch6: TeleportBut — place portals (3→7 bots, all previous)
        ch(6, "Teleport Tiles", &[
            ("Place Your Portal",  5, 3, 40),
            ("Warp Builder",       6, 3, 50),
            ("Portal Placement",   6, 4, 60),
            ("Linked Portals",     7, 5, 65),
            ("Warp Circuit",       7, 5, 70),
            ("Teleport Engineer",  8, 5, 75),
            ("Dimension Builder",  8, 6, 80),
            ("Space Architect",    9, 6, 85),
            ("Portal Crafter I",   9, 6, 90),
            ("Portal Crafter II", 10, 7, 95),
            ("Portal Crafter III",10, 7, 100),
        ]),
        // Ch7: Bounce — reflection (3→7 bots, all previous)
        ch(7, "Bounce", &[
            ("First Bounce",       5, 3, 40),
            ("Ricochet",           6, 4, 50),
            ("Bounce Path",        6, 4, 55),
            ("Wall Runner",        7, 5, 65),
            ("Reflection Point",   7, 5, 70),
            ("Bounce House",       8, 5, 75),
            ("Echo Chamber",       8, 6, 80),
            ("Rebound",            9, 6, 85),
            ("Bounce King I",      9, 6, 90),
            ("Bounce King II",    10, 7, 95),
            ("Bounce King III",   10, 7, 100),
        ]),
        // Ch8: BounceBut — place bounces (3→7 bots, all previous)
        ch(8, "Bounce Tiles", &[
            ("Place Your Bounce",  5, 3, 40),
            ("Bounce Setup",       6, 4, 50),
            ("Ricochet Builder",   7, 4, 60),
            ("Bounce Craft",       7, 5, 65),
            ("Reflection Lab",     8, 5, 70),
            ("Bounce Engineer",    8, 6, 75),
            ("Echo Builder",       9, 6, 80),
            ("Rebound Architect",  9, 6, 85),
            ("Bounce Crafter I",  10, 7, 90),
            ("Bounce Crafter II", 10, 7, 95),
            ("Bounce Crafter III",11, 7, 100),
        ]),
        // Ch9: Painters — color changing (3→8 bots, all previous)
        ch(9, "Painters", &[
            ("Color Shift",        5, 3, 40),
            ("Paint the Path",     6, 4, 55),
            ("Color Journey",      7, 5, 60),
            ("Rainbow Road",       7, 5, 65),
            ("Chromatic Path",     8, 6, 75),
            ("Painter's Palette",  8, 6, 80),
            ("Color Cascade",      9, 7, 85),
            ("Hue Shift",          9, 7, 90),
            ("Color Master I",    10, 7, 92),
            ("Color Master II",   10, 8, 95),
            ("Color Master III",  11, 8, 100),
        ]),
        // Ch10: Doors & Switches — toggle timing (3→8 bots, all previous + door chains)
        ch(10, "Doors & Switches", &[
            ("Open Sesame",        5, 3, 45),
            ("Locked Path",        6, 4, 55),
            ("Switch Timing",      7, 5, 65),
            ("Door Dance",         7, 5, 70),
            ("Double Lock",        8, 6, 75),
            ("Gate Keeper",        8, 6, 80),
            ("Synchronized",       9, 7, 85),
            ("Chain Reaction",     9, 7, 90),
            ("Lock Master I",     10, 7, 95),
            ("Lock Master II",    10, 8, 97),
            ("Lock Master III",   11, 8, 100),
        ]),
        // Ch11: ColorSwitch — color-gated toggling (3→8 bots, all previous)
        ch(11, "Color Switches", &[
            ("Color Gate",         6, 3, 50),
            ("Chromatic Lock",     7, 5, 65),
            ("Color Timing",       7, 5, 70),
            ("Hue Gate",           8, 6, 75),
            ("Spectrum Lock",      8, 6, 80),
            ("Color Cascade",      9, 7, 85),
            ("Prismatic Path",     9, 7, 90),
            ("Rainbow Gate",      10, 7, 95),
            ("Chroma Master I",   10, 8, 97),
            ("Chroma Master II",  11, 8, 100),
            ("Chroma Master III", 11, 8, 100),
        ]),
        // Ch12: ColorSwitchBut — all tiles placeable (3→8 bots, everything)
        ch(12, "Color Switch Tiles", &[
            ("Place Your Gate",    6, 3, 55),
            ("Color Builder",      7, 5, 65),
            ("Switch Craft",       7, 5, 70),
            ("Chromatic Builder",  8, 6, 75),
            ("Gate Architect",     8, 6, 80),
            ("Color Engineer",     9, 7, 85),
            ("Prismatic Craft",   10, 7, 90),
            ("Spectrum Builder",  10, 7, 95),
            ("Gate Crafter I",    11, 8, 97),
            ("Gate Crafter II",   11, 8, 100),
            ("Gate Crafter III",  12, 8, 100),
        ]),
        // Ch13: Grand Mastery — all mechanics combined (5→10 bots)
        ch(13, "Grand Mastery", &[
            ("The Convergence",    8, 5, 65),
            ("All In",             8, 5, 70),
            ("Synthesis",          9, 6, 75),
            ("Full Spectrum",      9, 6, 80),
            ("Mechanic Fusion",    9, 7, 85),
            ("The Crucible",      10, 7, 88),
            ("Quantum Tangle",    10, 8, 90),
            ("Neural Network",    11, 8, 92),
            ("Chaos Theory",      11, 8, 95),
            ("The Architect",     12, 9, 97),
            ("Event Horizon",     12, 9, 100),
            ("Singularity",       12, 9, 100),
            ("FINAL BOSS I — The Protocol",    12, 9, 100),
            ("FINAL BOSS II — The Machine",    12, 9, 100),
            ("FINAL BOSS III — Transcendence", 12, 9, 100),
            ("SECRET — The Impossible",        12, 9, 100),
            ("SECRET — Protocol Complete",     12, 9, 100),
        ]),
    ]
}
