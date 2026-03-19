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
pub mod sound;
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
            match generate_level_seeded(&level.config, attempts, ci, li) {
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
    generate_level_seeded(config, max_attempts, 0, 0)
}
fn generate_level_seeded(config: &GenConfig, max_attempts: usize, ci: usize, li: usize)
    -> Option<(Vec<(u32, u32, TileKind, bool)>, u32, u64)>
{
    use rand::SeedableRng;
    // Try up to 4 seed variations if initial seed fails to produce results
    for seed_try in 0..4u64 {
        let seed: u64 = 0x0C0_2026_CAFE ^ (ci as u64 * 1000 + li as u64 + seed_try * 0xDEAD);
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut best: Option<(Vec<(u32, u32, TileKind, bool)>, u32)> = None;
        for _ in 0..max_attempts {
            if let Some((tiles, rating)) = generate_attempt(config, &mut rng) {
                if let Some(req) = config.required_tile {
                    if !tiles.iter().any(|(_, _, k, _)| req(k)) { continue; }
                }
                let gap = (rating as i32 - config.difficulty as i32).unsigned_abs();
                let prev_gap = best.as_ref()
                    .map(|(_, r)| (*r as i32 - config.difficulty as i32).unsigned_abs())
                    .unwrap_or(u32::MAX);
                if gap < prev_gap { best = Some((tiles, rating)); }
                if gap <= GEN_BEST_TOLERANCE { return best.map(|(t, r)| (t, r, seed)); }
            }
        }
        if best.is_some() { return best.map(|(t, r)| (t, r, seed)); }
    }
    None
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
        else if pos == 0 { (10, 3) }   // intro: new mechanic dominant
        else if pos <= 3 { (8, 5) }     // learning: new dominant
        else if pos <= 7 { (7, 6) }     // challenge: mixed
        else { (6, 7) };               // boss: balanced
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
    // Confusion tiles: intro levels (0-1) never, mid-levels sometimes, late always
    c.confusion_tiles = if pos <= 1 { false } else if pos <= 4 { ch >= 3 } else { true };
    if ch >= 10 { c.door_chains = match pos { 0..=2=>1, 3..=5=>2, 6..=8=>3, _=>4 }.max(1); }
    // Holes: gentle start, ramp up
    let hole_base = if ch <= 2 { 5 } else if ch <= 6 { 12 } else { 18 };
    let pct = hole_base + pos as u32 * 2;
    match pos % 3 {
        0 => { c.hole_placement = HolePlacement::Edges; c.hole_percent = pct; }
        1 => { c.hole_placement = HolePlacement::Middle; c.hole_percent = pct; }
        _ => { c.hole_percent = pct; }
    }
    // Inventory: intro (2-3), learning (3-4), challenge (4-6), boss (5-8)
    // Chapters with complex mechanics (doors/switches) need more intro tiles
    let inv = match pos {
        0 => if ch >= 10 { 3 } else { 2 }, 1 => 3,
        2..=4 => 3 + pos as u32 / 2, // learning: 4
        5..=7 => 4 + pos as u32 / 2, // challenge: 5-6
        _ => 5 + pos as u32 / 3,     // boss: 6-8
    };
    c.inventory_target = inv.min((board - 1).min(8));
    // Unique solutions only for final boss levels — prevents guessing
    c.unique_solution = pos >= 9;
    // ALL levels must contain the chapter's new mechanic (on board or in inventory)
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
        // Ch1: Turns — learn path building (1→5 bots), diff * 1.2
        ch(1, "Turns", &[
            ("First Steps",       3, 1, 18),
            ("Corner to Corner",  4, 2, 36),
            ("The Zigzag",        4, 2, 48),
            ("Around the Block",  5, 2, 55),
            ("Spiral Path",       5, 3, 60),
            ("Double Back",       6, 3, 65),
            ("Winding Road",      6, 3, 70),
            ("Crossroads",        6, 4, 75),
            ("Turn Master I",     7, 4, 80),
            ("Turn Master II",    8, 5, 85),
            ("Turn Master III",   8, 5, 90),
        ]),
        // Ch2: TurnBut — place turns from inventory (2→5 bots), diff * 1.25
        ch(2, "Turn Tiles", &[
            ("Place Your Turn",       4, 2, 31),
            ("Two Turns",             4, 2, 44),
            ("Choose Wisely",         5, 3, 50),
            ("Mixed Turns",           5, 3, 58),
            ("Turn Puzzle",           6, 3, 65),
            ("Inventory Challenge",   6, 4, 72),
            ("Precision Placement",   7, 4, 78),
            ("No Room for Error",     7, 4, 82),
            ("Turn Builder I",        8, 5, 88),
            ("Turn Builder II",       8, 5, 92),
            ("Turn Builder III",      9, 5, 95),
        ]),
        // Ch3: Arrows — forced direction (2→6 bots), diff * 1.25, +1 bot mid
        ch(3, "Arrows", &[
            ("One Way Street",    4, 2, 38),
            ("Follow the Arrow",  5, 3, 50),
            ("Arrow Maze",        5, 3, 63),
            ("Turn and Thrust",   6, 3, 69),
            ("Arrow Chain",       6, 4, 75),
            ("Speed Lines",       7, 4, 81),
            ("The Gauntlet",      7, 5, 88),
            ("Forced March",      8, 5, 94),
            ("Arrow Storm I",     8, 5, 100),
            ("Arrow Storm II",    9, 6, 100),
            ("Arrow Storm III",   9, 6, 100),
        ]),
        // Ch4: ArrowBut — place arrows (2→6 bots), diff * 1.3, +1 board
        ch(4, "Arrow Tiles", &[
            ("Place Your Arrow",  5, 2, 39),
            ("Arrow Setup",       6, 3, 59),
            ("Redirect",          6, 3, 72),
            ("Arrow Architect",   7, 4, 78),
            ("Mixed Signals",     7, 4, 85),
            ("Direction Control", 7, 5, 91),
            ("Arrow Master",      8, 5, 98),
            ("Full Arsenal",      8, 5, 100),
            ("Arrow Crafter I",   9, 6, 100),
            ("Arrow Crafter II",  9, 6, 100),
            ("Arrow Crafter III",10, 6, 100),
        ]),
        // Ch5: Teleports — warp mechanics (3→8 bots), diff * 1.25
        ch(5, "Teleports", &[
            ("Warp Zone",          5, 3, 44),
            ("Portal Hop",         6, 4, 63),
            ("Double Warp",        7, 4, 69),
            ("Teleport Chain",     7, 5, 75),
            ("Warp Tactics",       8, 5, 88),
            ("Portal Network",     8, 6, 94),
            ("Dimensional Shift",  9, 6, 100),
            ("Space Fold",         9, 7, 100),
            ("Warp Master I",     10, 7, 100),
            ("Warp Master II",    10, 7, 100),
            ("Warp Master III",   11, 8, 100),
        ]),
        // Ch6: TeleportBut — place portals (3→8 bots), diff * 1.3
        ch(6, "Teleport Tiles", &[
            ("Place Your Portal",  6, 3, 52),
            ("Warp Builder",       6, 4, 65),
            ("Portal Placement",   7, 5, 78),
            ("Linked Portals",     7, 5, 85),
            ("Warp Circuit",       8, 6, 91),
            ("Teleport Engineer",  8, 6, 98),
            ("Dimension Builder",  9, 7, 100),
            ("Space Architect",    9, 7, 100),
            ("Portal Crafter I",  10, 7, 100),
            ("Portal Crafter II", 10, 8, 100),
            ("Portal Crafter III",11, 8, 100),
        ]),
        // Ch7: Bounce — reflection (3→8 bots), diff * 1.3
        ch(7, "Bounce", &[
            ("First Bounce",       6, 3, 52),
            ("Ricochet",           6, 4, 65),
            ("Bounce Path",        7, 5, 72),
            ("Wall Runner",        7, 5, 85),
            ("Reflection Point",   8, 6, 91),
            ("Bounce House",       8, 6, 98),
            ("Echo Chamber",       9, 7, 100),
            ("Rebound",            9, 7, 100),
            ("Bounce King I",     10, 7, 100),
            ("Bounce King II",    10, 8, 100),
            ("Bounce King III",   11, 8, 100),
        ]),
        // Ch8: BounceBut — place bounces (3→8 bots), diff * 1.3
        ch(8, "Bounce Tiles", &[
            ("Place Your Bounce",  6, 3, 52),
            ("Bounce Setup",       6, 4, 65),
            ("Ricochet Builder",   7, 5, 78),
            ("Bounce Craft",       8, 5, 85),
            ("Reflection Lab",     8, 6, 91),
            ("Bounce Engineer",    9, 7, 98),
            ("Echo Builder",       9, 7, 100),
            ("Rebound Architect", 10, 7, 100),
            ("Bounce Crafter I",  10, 8, 100),
            ("Bounce Crafter II", 11, 8, 100),
            ("Bounce Crafter III",11, 8, 100),
        ]),
        // Ch9: Painters — color changing (3→9 bots), diff * 1.3
        ch(9, "Painters", &[
            ("Color Shift",        6, 3, 52),
            ("Paint the Path",     7, 5, 72),
            ("Color Journey",      7, 5, 78),
            ("Rainbow Road",       8, 6, 85),
            ("Chromatic Path",     8, 6, 98),
            ("Painter's Palette",  9, 7, 100),
            ("Color Cascade",      9, 7, 100),
            ("Hue Shift",         10, 8, 100),
            ("Color Master I",    10, 8, 100),
            ("Color Master II",   11, 9, 100),
            ("Color Master III",  11, 9, 100),
        ]),
        // Ch10: Doors & Switches — toggle timing (3→9 bots), diff * 1.3
        ch(10, "Doors & Switches", &[
            ("Open Sesame",        6, 3, 59),
            ("Locked Path",        7, 5, 72),
            ("Switch Timing",      7, 5, 85),
            ("Door Dance",         8, 6, 91),
            ("Double Lock",        8, 6, 98),
            ("Gate Keeper",        9, 7, 100),
            ("Synchronized",       9, 7, 100),
            ("Chain Reaction",    10, 8, 100),
            ("Lock Master I",     10, 8, 100),
            ("Lock Master II",    11, 9, 100),
            ("Lock Master III",   11, 9, 100),
        ]),
        // Ch11: ColorSwitch — color-gated toggling (3→9 bots), diff * 1.3
        ch(11, "Color Switches", &[
            ("Color Gate",         7, 4, 65),
            ("Chromatic Lock",     7, 5, 85),
            ("Color Timing",       8, 6, 91),
            ("Hue Gate",           8, 6, 98),
            ("Spectrum Lock",      9, 7, 100),
            ("Color Cascade",      9, 7, 100),
            ("Prismatic Path",    10, 8, 100),
            ("Rainbow Gate",      10, 8, 100),
            ("Chroma Master I",   11, 9, 100),
            ("Chroma Master II",  10, 7, 100),
            ("Chroma Master III", 12, 9, 100),
        ]),
        // Ch12: ColorSwitchBut — all tiles placeable (3→9 bots), diff * 1.3
        ch(12, "Color Switch Tiles", &[
            ("Place Your Gate",    7, 4, 72),
            ("Color Builder",      7, 5, 85),
            ("Switch Craft",       8, 6, 91),
            ("Chromatic Builder",  8, 6, 98),
            ("Gate Architect",     9, 7, 100),
            ("Color Engineer",     9, 7, 100),
            ("Prismatic Craft",   10, 8, 100),
            ("Spectrum Builder",  10, 8, 100),
            ("Gate Crafter I",    11, 9, 100),
            ("Gate Crafter II",   12, 9, 100),
            ("Gate Crafter III",  12, 9, 100),
        ]),
        // Ch13: Grand Mastery — all mechanics combined (5→10 bots), diff * 1.3
        ch(13, "Grand Mastery", &[
            ("The Convergence",    9, 5, 85),
            ("All In",             9, 6, 91),
            ("Synthesis",         10, 6, 98),
            ("Full Spectrum",     10, 7, 100),
            ("Mechanic Fusion",   10, 7, 100),
            ("The Crucible",      11, 8, 100),
            ("Quantum Tangle",    11, 8, 100),
            ("Neural Network",    11, 9, 100),
            ("Chaos Theory",      12, 9, 100),
            ("The Architect",     12, 9, 100),
            ("Event Horizon",     12,10, 100),
            ("Singularity",       12,10, 100),
            ("FINAL BOSS I — The Protocol",    12,10, 100),
            ("FINAL BOSS II — The Machine",    12,10, 100),
            ("FINAL BOSS III — Transcendence", 12,10, 100),
            ("SECRET — The Impossible",        11, 8, 100),
            ("SECRET — Protocol Complete",     12,10, 100),
        ]),
    ]
}
