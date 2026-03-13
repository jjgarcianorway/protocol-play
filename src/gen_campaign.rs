// SPDX-License-Identifier: GPL-3.0-or-later
// Campaign level generator — standalone binary that produces all levels for the game.

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
    let chapters = campaign_chapters();
    let out_dir = std::path::PathBuf::from("campaign_levels");
    let _ = std::fs::create_dir_all(&out_dir);

    let mut total = 0u32;
    let mut failed = 0u32;

    for (ci, chapter) in chapters.iter().enumerate() {
        println!("\n=== Chapter {}: {} ===", ci + 1, chapter.name);
        for (li, level) in chapter.levels.iter().enumerate() {
            let num = li + 1;
            let ch_num = ci + 1;
            let filename = format!("{:02}_{:02}_{}", ch_num, num,
                sanitize(&level.display_name));
            print!("  [{ch_num}-{num:02}] {} ({}x{}, diff={}, bots={})... ",
                level.display_name, level.config.board_size, level.config.board_size,
                level.config.difficulty, level.config.num_bots);

            let attempts = if level.config.num_bots >= 8 { 200000 }
                else if level.config.num_bots >= 6 { 100000 }
                else if level.config.num_bots >= 4 { 50000 } else { 20000 };
            match generate_level(&level.config, attempts) {
                Some((tiles, rating, seed)) => {
                    let data = LevelData {
                        name: level.display_name.clone(),
                        board_size: level.config.board_size,
                        tiles, solution: vec![],
                        seed: Some(seed), difficulty: Some(rating),
                    };
                    // Fill in solution from marked tiles
                    let solution: Vec<_> = data.tiles.iter()
                        .filter(|(_, _, _, m)| *m).map(|(c, r, k, _)| (*c, *r, *k)).collect();
                    let data = LevelData { solution, ..data };
                    let path = out_dir.join(format!("{filename}.json"));
                    let json = serde_json::to_string_pretty(&data).unwrap();
                    std::fs::write(&path, json).unwrap();
                    println!("OK (diff={rating})");
                    total += 1;
                }
                None => {
                    println!("FAILED");
                    failed += 1;
                }
            }
        }
    }
    println!("\n=== Done: {total} levels generated, {failed} failed ===");
}

fn generate_level(config: &GenConfig, max_attempts: usize) -> Option<(Vec<(u32, u32, TileKind, bool)>, u32, u64)> {
    let mut rng = rand::thread_rng();
    let seed: u64 = rand::Rng::r#gen(&mut rng);
    let mut best: Option<(Vec<(u32, u32, TileKind, bool)>, u32)> = None;

    for _ in 0..max_attempts {
        if let Some((tiles, rating)) = generate_attempt(config, &mut rng) {
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

// Campaign definition — 13 chapters, 149 levels
struct Chapter {
    name: String,
    levels: Vec<CampaignLevel>,
}

struct CampaignLevel {
    display_name: String,
    config: GenConfig,
}

fn cfg(board_size: u32, num_bots: u32, difficulty: u32, weights: [u32; GEN_NUM_WEIGHTS]) -> GenConfig {
    GenConfig {
        board_size, num_bots, difficulty, weights,
        hole_percent: 20, hole_placement: HolePlacement::Both,
        unique_solution: false, inventory_target: 0,
        door_chains: 0, path_sharing: false, confusion_tiles: false,
    }
}

fn campaign_chapters() -> Vec<Chapter> {
    vec![
        // Chapter 1: Turns — 2 bots early, edge/mid holes for variety
        Chapter { name: "Turns".into(), levels: vec![
            cl("First Steps", cfg(3, 1, 20, w(&[(0, 8)])).edge_holes(15)),
            cl("Corner to Corner", cfg(4, 2, 30, w(&[(0, 9)])).mid_holes(15).share()),
            cl("The Zigzag", cfg(4, 2, 40, w(&[(0, 10)])).edge_holes(20).share()),
            cl("Around the Block", cfg(5, 2, 45, w(&[(0, 10)])).holes(20).share()),
            cl("Spiral Path", cfg(5, 3, 50, w(&[(0, 11)])).mid_holes(20).share()),
            cl("Double Back", cfg(5, 3, 55, w(&[(0, 12)])).edge_holes(25).share()),
            cl("Winding Road", cfg(6, 3, 60, w(&[(0, 12)])).holes(25).share()),
            cl("Crossroads", cfg(6, 3, 65, w(&[(0, 12)])).mid_holes(25).share()),
            cl("Turn Master I", cfg(6, 4, 70, w(&[(0, 12)])).edge_holes(20).share()),
            cl("Turn Master II", cfg(7, 4, 75, w(&[(0, 13)])).holes(25).share()),
            cl("Turn Master III", cfg(8, 4, 80, w(&[(0, 14)])).mid_holes(25).share()),
        ]},
        // Chapter 2: TurnBut — 2-4 bots
        Chapter { name: "Turn Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Turn", cfg(4, 2, 30, w(&[(0, 5), (1, 6)])).inv(2).edge_holes(15)),
                cl("Two Turns", cfg(4, 2, 40, w(&[(0, 5), (1, 7)])).inv(3).mid_holes(15).share()),
                cl("Choose Wisely", cfg(5, 3, 50, w(&[(0, 6), (1, 7)])).inv(3).holes(20).share().confuse()),
                cl("Mixed Turns", cfg(5, 3, 55, w(&[(0, 6), (1, 7)])).inv(3).edge_holes(20).share()),
                cl("Turn Puzzle", cfg(6, 3, 60, w(&[(0, 6), (1, 8)])).inv(4).mid_holes(20).share().confuse()),
                cl("Inventory Challenge", cfg(6, 3, 65, w(&[(0, 7), (1, 8)])).inv(4).holes(25).share()),
                cl("Precision Placement", cfg(6, 4, 70, w(&[(0, 7), (1, 8)])).inv(5).edge_holes(25).share().confuse()),
                cl("No Room for Error", cfg(7, 4, 75, w(&[(0, 7), (1, 9)])).inv(5).holes(25).share()),
            ];
            lvls.push(cl("Turn Builder I", cfg(7, 4, 80, w(&[(0, 8), (1, 9)])).inv(6).mid_holes(25).share().confuse()));
            lvls.push(cl("Turn Builder II", cfg(7, 4, 85, w(&[(0, 8), (1, 10)])).inv(6).holes(25).share().confuse()));
            lvls.push(cl("Turn Builder III", cfg(8, 4, 90, w(&[(0, 9), (1, 10)])).inv(7).edge_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 3: Arrows — 2-4 bots
        Chapter { name: "Arrows".into(), levels: vec![
            cl("One Way Street", cfg(4, 2, 30, w(&[(0, 4), (2, 8)])).edge_holes(15).share()),
            cl("Follow the Arrow", cfg(5, 2, 40, w(&[(0, 5), (2, 8)])).mid_holes(15).share()),
            cl("Arrow Maze", cfg(5, 3, 50, w(&[(0, 5), (2, 8)])).holes(20).share()),
            cl("Turn and Thrust", cfg(5, 3, 55, w(&[(0, 6), (2, 8)])).edge_holes(20).share()),
            cl("Arrow Chain", cfg(6, 3, 60, w(&[(0, 6), (2, 9)])).mid_holes(20).share().confuse()),
            cl("Speed Lines", cfg(6, 3, 65, w(&[(0, 6), (2, 9)])).holes(25).share()),
            cl("The Gauntlet", cfg(6, 4, 70, w(&[(0, 7), (2, 10)])).edge_holes(25).share().confuse()),
            cl("Forced March", cfg(7, 4, 75, w(&[(0, 7), (2, 10)])).holes(25).share()),
            cl("Arrow Storm I", cfg(7, 4, 80, w(&[(0, 8), (2, 10)])).mid_holes(25).share().confuse()),
            cl("Arrow Storm II", cfg(8, 4, 85, w(&[(0, 8), (2, 11)])).holes(25).share().confuse()),
            cl("Arrow Storm III", cfg(8, 4, 90, w(&[(0, 9), (2, 11)])).edge_holes(30).share().confuse()),
        ]},
        // Chapter 4: ArrowBut — 2-4 bots
        Chapter { name: "Arrow Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Arrow", cfg(4, 2, 35, w(&[(0, 4), (2, 4), (3, 7)])).inv(2).mid_holes(15)),
                cl("Arrow Setup", cfg(5, 2, 45, w(&[(0, 5), (1, 3), (2, 4), (3, 7)])).inv(3).holes(20).share()),
                cl("Redirect", cfg(5, 3, 55, w(&[(0, 5), (1, 3), (2, 5), (3, 7)])).inv(3).edge_holes(20).share().confuse()),
                cl("Arrow Architect", cfg(6, 3, 60, w(&[(0, 6), (2, 5), (3, 7)])).inv(4).holes(20).share()),
                cl("Mixed Signals", cfg(6, 3, 65, w(&[(0, 5), (1, 4), (2, 5), (3, 8)])).inv(4).mid_holes(25).share().confuse()),
                cl("Direction Control", cfg(6, 4, 70, w(&[(0, 6), (1, 4), (2, 5), (3, 8)])).inv(5).holes(25).share()),
                cl("Arrow Master", cfg(7, 4, 75, w(&[(0, 6), (2, 6), (3, 9)])).inv(5).edge_holes(25).share().confuse()),
                cl("Full Arsenal", cfg(7, 4, 80, w(&[(0, 7), (1, 4), (2, 6), (3, 9)])).inv(5).holes(25).share().confuse()),
            ];
            lvls.push(cl("Arrow Crafter I", cfg(7, 4, 85, w(&[(0, 7), (1, 5), (2, 6), (3, 9)])).inv(6).mid_holes(25).share().confuse()));
            lvls.push(cl("Arrow Crafter II", cfg(8, 4, 90, w(&[(0, 7), (1, 5), (2, 6), (3, 10)])).inv(6).holes(30).share().confuse()));
            lvls.push(cl("Arrow Crafter III", cfg(8, 4, 95, w(&[(0, 8), (1, 5), (2, 7), (3, 10)])).inv(7).edge_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 5: Teleports — 3-6 bots
        Chapter { name: "Teleports".into(), levels: vec![
            cl("Warp Zone", cfg(5, 3, 40, w(&[(0, 5), (2, 3), (4, 8)])).mid_holes(20).share()),
            cl("Portal Hop", cfg(5, 3, 50, w(&[(0, 5), (2, 4), (4, 8)])).edge_holes(20).share()),
            cl("Double Warp", cfg(6, 4, 55, w(&[(0, 6), (2, 4), (4, 9)])).holes(20).share()),
            cl("Teleport Chain", cfg(6, 4, 60, w(&[(0, 6), (2, 4), (4, 9)])).mid_holes(25).share()),
            cl("Warp Tactics", cfg(7, 5, 65, w(&[(0, 6), (2, 5), (4, 9)])).holes(25).share().confuse()),
            cl("Portal Network", cfg(7, 5, 70, w(&[(0, 7), (2, 5), (4, 10)])).edge_holes(25).share()),
            cl("Dimensional Shift", cfg(8, 5, 75, w(&[(0, 7), (2, 5), (4, 10)])).holes(25).share().confuse()),
            cl("Space Fold", cfg(8, 5, 80, w(&[(0, 7), (2, 6), (4, 10)])).mid_holes(25).share()),
            cl("Warp Master I", cfg(9, 6, 85, w(&[(0, 8), (2, 6), (4, 10)])).holes(25).share().confuse()),
            cl("Warp Master II", cfg(9, 6, 90, w(&[(0, 8), (2, 6), (4, 11)])).edge_holes(30).share().confuse()),
            cl("Warp Master III", cfg(10, 6, 95, w(&[(0, 9), (2, 7), (4, 11)])).holes(30).share().confuse()),
        ]},
        // Chapter 6: TeleportBut — 3-6 bots
        Chapter { name: "Teleport Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Portal", cfg(5, 3, 45, w(&[(0, 5), (4, 4), (5, 7)])).inv(2).mid_holes(20).share()),
                cl("Warp Builder", cfg(6, 4, 55, w(&[(0, 5), (2, 3), (4, 4), (5, 7)])).inv(3).holes(20).share()),
                cl("Portal Placement", cfg(6, 4, 60, w(&[(0, 5), (1, 3), (2, 4), (5, 8)])).inv(3).edge_holes(25).share().confuse()),
                cl("Linked Portals", cfg(7, 5, 65, w(&[(0, 6), (2, 4), (4, 5), (5, 8)])).inv(4).holes(25).share()),
                cl("Warp Circuit", cfg(7, 5, 70, w(&[(0, 6), (1, 3), (2, 4), (4, 5), (5, 8)])).inv(4).mid_holes(25).share().confuse()),
                cl("Teleport Engineer", cfg(8, 5, 75, w(&[(0, 6), (2, 5), (3, 3), (4, 5), (5, 9)])).inv(5).holes(25).share()),
                cl("Dimension Builder", cfg(8, 5, 80, w(&[(0, 7), (2, 5), (4, 5), (5, 9)])).inv(5).edge_holes(25).share().confuse()),
                cl("Space Architect", cfg(9, 6, 85, w(&[(0, 7), (1, 4), (2, 5), (4, 5), (5, 9)])).inv(5).holes(25).share().confuse()),
            ];
            lvls.push(cl("Portal Crafter I", cfg(9, 6, 90, w(&[(0, 7), (2, 5), (3, 3), (4, 5), (5, 10)])).inv(6).mid_holes(30).share().confuse()));
            lvls.push(cl("Portal Crafter II", cfg(10, 6, 95, w(&[(0, 8), (1, 4), (2, 6), (4, 6), (5, 10)])).inv(6).holes(30).share().confuse()));
            lvls.push(cl("Portal Crafter III", cfg(10, 6, 100, w(&[(0, 8), (2, 6), (3, 4), (4, 6), (5, 11)])).inv(7).edge_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 7: Bounce — 3-7 bots
        Chapter { name: "Bounce".into(), levels: vec![
            cl("First Bounce", cfg(5, 3, 40, w(&[(0, 5), (2, 4), (6, 8)])).edge_holes(20).share()),
            cl("Ricochet", cfg(6, 4, 50, w(&[(0, 5), (2, 4), (6, 9)])).mid_holes(20).share()),
            cl("Bounce Path", cfg(6, 4, 55, w(&[(0, 6), (2, 5), (6, 9)])).holes(20).share()),
            cl("Wall Runner", cfg(7, 5, 60, w(&[(0, 6), (4, 3), (6, 9)])).edge_holes(25).share()),
            cl("Reflection Point", cfg(7, 5, 65, w(&[(0, 6), (2, 5), (4, 3), (6, 9)])).holes(25).share().confuse()),
            cl("Bounce House", cfg(8, 5, 70, w(&[(0, 7), (2, 5), (4, 4), (6, 10)])).mid_holes(25).share()),
            cl("Echo Chamber", cfg(8, 6, 75, w(&[(0, 7), (2, 5), (4, 4), (6, 10)])).holes(25).share().confuse()),
            cl("Rebound", cfg(9, 6, 80, w(&[(0, 7), (2, 6), (4, 4), (6, 10)])).edge_holes(25).share()),
            cl("Bounce King I", cfg(9, 6, 85, w(&[(0, 8), (2, 6), (4, 5), (6, 10)])).holes(25).share().confuse()),
            cl("Bounce King II", cfg(10, 7, 90, w(&[(0, 8), (2, 6), (4, 5), (6, 11)])).mid_holes(30).share().confuse()),
            cl("Bounce King III", cfg(10, 7, 95, w(&[(0, 9), (2, 7), (4, 5), (6, 11)])).holes(30).share().confuse()),
        ]},
        // Chapter 8: BounceBut — 3-7 bots
        Chapter { name: "Bounce Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Bounce", cfg(5, 3, 45, w(&[(0, 5), (6, 4), (7, 7)])).inv(2).edge_holes(20).share()),
                cl("Bounce Setup", cfg(6, 4, 55, w(&[(0, 5), (2, 3), (6, 4), (7, 8)])).inv(3).holes(20).share()),
                cl("Ricochet Builder", cfg(7, 5, 60, w(&[(0, 6), (2, 4), (6, 4), (7, 8)])).inv(3).mid_holes(25).share().confuse()),
                cl("Bounce Craft", cfg(7, 5, 65, w(&[(0, 6), (4, 3), (6, 5), (7, 8)])).inv(4).holes(25).share()),
                cl("Reflection Lab", cfg(8, 5, 70, w(&[(0, 6), (2, 4), (4, 3), (6, 5), (7, 9)])).inv(4).edge_holes(25).share().confuse()),
                cl("Bounce Engineer", cfg(8, 6, 75, w(&[(0, 7), (1, 3), (2, 4), (6, 5), (7, 9)])).inv(5).holes(25).share()),
                cl("Echo Builder", cfg(9, 6, 80, w(&[(0, 7), (2, 5), (4, 4), (6, 5), (7, 9)])).inv(5).mid_holes(25).share().confuse()),
                cl("Rebound Architect", cfg(9, 6, 85, w(&[(0, 7), (2, 5), (3, 3), (4, 4), (6, 5), (7, 10)])).inv(5).holes(25).share().confuse()),
            ];
            lvls.push(cl("Bounce Crafter I", cfg(10, 7, 90, w(&[(0, 8), (2, 5), (4, 4), (6, 5), (7, 10)])).inv(6).edge_holes(30).share().confuse()));
            lvls.push(cl("Bounce Crafter II", cfg(10, 7, 95, w(&[(0, 8), (1, 4), (2, 6), (6, 6), (7, 10)])).inv(6).holes(30).share().confuse()));
            lvls.push(cl("Bounce Crafter III", cfg(11, 7, 100, w(&[(0, 9), (2, 6), (4, 5), (6, 6), (7, 11)])).inv(7).mid_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 9: Painters — 3-8 bots
        Chapter { name: "Painters".into(), levels: vec![
            cl("Color Shift", cfg(5, 3, 45, w(&[(0, 5), (2, 4), (11, 8)])).edge_holes(20).share()),
            cl("Paint the Path", cfg(6, 4, 55, w(&[(0, 6), (2, 4), (11, 8)])).holes(20).share()),
            cl("Color Journey", cfg(7, 5, 60, w(&[(0, 6), (2, 5), (4, 3), (11, 9)])).mid_holes(25).share()),
            cl("Rainbow Road", cfg(7, 5, 65, w(&[(0, 6), (2, 5), (6, 3), (11, 9)])).holes(25).share().confuse()),
            cl("Chromatic Path", cfg(8, 6, 70, w(&[(0, 7), (2, 5), (4, 4), (11, 9)])).edge_holes(25).share()),
            cl("Painter's Palette", cfg(8, 6, 75, w(&[(0, 7), (2, 5), (6, 4), (11, 10)])).holes(25).share().confuse()),
            cl("Color Cascade", cfg(9, 7, 80, w(&[(0, 7), (2, 6), (4, 4), (6, 4), (11, 10)])).mid_holes(25).share()),
            cl("Hue Shift", cfg(9, 7, 85, w(&[(0, 8), (2, 6), (4, 4), (6, 4), (11, 10)])).holes(25).share().confuse()),
            cl("Color Master I", cfg(10, 7, 90, w(&[(0, 8), (2, 6), (4, 5), (6, 4), (11, 11)])).edge_holes(30).share().confuse()),
            cl("Color Master II", cfg(10, 8, 95, w(&[(0, 9), (2, 7), (4, 5), (6, 5), (11, 11)])).holes(30).share().confuse()),
            cl("Color Master III", cfg(11, 8, 100, w(&[(0, 9), (2, 7), (4, 5), (6, 5), (11, 12)])).mid_holes(30).share().confuse()),
        ]},
        // Chapter 10: Doors & Switches — 3-8 bots
        Chapter { name: "Doors & Switches".into(), levels: {
            let mut lvls = vec![
                cl("Open Sesame", cfg(5, 3, 50, w(&[(0, 6), (2, 4), (8, 8)])).chains(1).edge_holes(20).share()),
                cl("Locked Path", cfg(6, 4, 60, w(&[(0, 6), (2, 5), (8, 8)])).chains(1).holes(20).share()),
                cl("Switch Timing", cfg(7, 5, 65, w(&[(0, 7), (2, 5), (4, 3), (8, 9)])).chains(1).mid_holes(25).share()),
                cl("Door Dance", cfg(7, 5, 70, w(&[(0, 7), (2, 5), (6, 3), (8, 9)])).chains(1).holes(25).share().confuse()),
                cl("Double Lock", cfg(8, 6, 75, w(&[(0, 7), (2, 5), (4, 4), (8, 9)])).chains(2).edge_holes(25).share()),
                cl("Gate Keeper", cfg(8, 6, 80, w(&[(0, 7), (2, 6), (4, 4), (8, 10)])).chains(2).holes(25).share().confuse()),
                cl("Synchronized", cfg(9, 7, 85, w(&[(0, 8), (2, 6), (4, 4), (6, 3), (8, 10)])).chains(2).mid_holes(25).share()),
                cl("Chain Reaction", cfg(9, 7, 90, w(&[(0, 8), (2, 6), (4, 5), (8, 10)])).chains(3).holes(30).share().confuse()),
            ];
            lvls.push(cl("Lock Master I", cfg(10, 7, 95, w(&[(0, 9), (2, 7), (4, 5), (6, 4), (8, 11)])).chains(3).edge_holes(30).share().confuse()));
            lvls.push(cl("Lock Master II", cfg(10, 8, 100, w(&[(0, 9), (2, 7), (4, 5), (8, 11)])).chains(3).holes(30).share().confuse()));
            lvls.push(cl("Lock Master III", cfg(11, 8, 100, w(&[(0, 10), (2, 7), (4, 5), (6, 5), (8, 12)])).chains(4).mid_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 11: ColorSwitch — 3-8 bots
        Chapter { name: "Color Switches".into(), levels: {
            let mut lvls = vec![
                cl("Color Gate", cfg(6, 3, 55, w(&[(0, 6), (2, 4), (9, 8), (11, 4)])).chains(1).edge_holes(20).share()),
                cl("Chromatic Lock", cfg(7, 5, 65, w(&[(0, 6), (2, 4), (8, 4), (9, 8), (11, 4)])).chains(1).holes(25).share()),
                cl("Color Timing", cfg(7, 5, 70, w(&[(0, 7), (2, 5), (9, 9), (11, 5)])).chains(1).mid_holes(25).share().confuse()),
                cl("Hue Gate", cfg(8, 6, 75, w(&[(0, 7), (2, 5), (4, 3), (8, 4), (9, 9)])).chains(1).holes(25).share()),
                cl("Spectrum Lock", cfg(8, 6, 80, w(&[(0, 7), (2, 5), (4, 4), (9, 9)])).chains(2).edge_holes(25).share().confuse()),
                cl("Color Cascade", cfg(9, 7, 85, w(&[(0, 8), (2, 6), (6, 3), (8, 5), (9, 10)])).chains(2).holes(25).share()),
                cl("Prismatic Path", cfg(9, 7, 90, w(&[(0, 8), (2, 6), (4, 4), (9, 10), (11, 5)])).chains(2).mid_holes(30).share().confuse()),
                cl("Rainbow Gate", cfg(10, 7, 95, w(&[(0, 8), (2, 6), (4, 4), (8, 5), (9, 10)])).chains(3).holes(30).share().confuse()),
            ];
            lvls.push(cl("Chroma Master I", cfg(10, 8, 100, w(&[(0, 9), (2, 7), (4, 5), (8, 5), (9, 11)])).chains(3).edge_holes(30).share().confuse()));
            lvls.push(cl("Chroma Master II", cfg(11, 8, 100, w(&[(0, 9), (2, 7), (6, 4), (8, 6), (9, 11), (11, 5)])).chains(3).holes(30).share().confuse()));
            lvls.push(cl("Chroma Master III", cfg(11, 8, 100, w(&[(0, 10), (2, 7), (4, 5), (6, 5), (8, 6), (9, 12)])).chains(4).mid_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 12: ColorSwitchBut — 3-8 bots
        Chapter { name: "Color Switch Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Gate", cfg(6, 3, 60, w(&[(0, 6), (2, 4), (9, 4), (10, 8)])).inv(2).chains(1).edge_holes(25).share()),
                cl("Color Builder", cfg(7, 5, 65, w(&[(0, 6), (2, 4), (8, 3), (9, 4), (10, 8)])).inv(3).chains(1).holes(25).share()),
                cl("Switch Craft", cfg(7, 5, 70, w(&[(0, 7), (2, 5), (4, 3), (10, 9)])).inv(3).chains(1).mid_holes(25).share().confuse()),
                cl("Chromatic Builder", cfg(8, 6, 75, w(&[(0, 7), (2, 5), (8, 4), (9, 4), (10, 9)])).inv(4).chains(2).holes(25).share()),
                cl("Gate Architect", cfg(8, 6, 80, w(&[(0, 7), (2, 5), (4, 4), (9, 4), (10, 9)])).inv(4).chains(2).edge_holes(25).share().confuse()),
                cl("Color Engineer", cfg(9, 7, 85, w(&[(0, 8), (2, 6), (6, 3), (8, 4), (10, 10)])).inv(5).chains(2).holes(25).share()),
                cl("Prismatic Craft", cfg(10, 7, 90, w(&[(0, 8), (2, 6), (4, 4), (9, 5), (10, 10)])).inv(5).chains(2).mid_holes(30).share().confuse()),
                cl("Spectrum Builder", cfg(10, 7, 95, w(&[(0, 8), (2, 6), (4, 4), (8, 5), (9, 5), (10, 10)])).inv(5).chains(3).holes(30).share().confuse()),
            ];
            lvls.push(cl("Gate Crafter I", cfg(11, 8, 100, w(&[(0, 9), (2, 7), (4, 5), (9, 5), (10, 11)])).inv(6).chains(3).edge_holes(30).share().confuse()));
            lvls.push(cl("Gate Crafter II", cfg(11, 8, 100, w(&[(0, 9), (2, 7), (6, 4), (8, 5), (9, 6), (10, 11)])).inv(6).chains(3).holes(30).share().confuse()));
            lvls.push(cl("Gate Crafter III", cfg(12, 8, 100, w(&[(0, 10), (1, 5), (2, 7), (4, 5), (9, 6), (10, 12)])).inv(7).chains(4).mid_holes(30).share().confuse()));
            lvls
        }},
        // Chapter 13: Grand Mastery — 5-10 bots, big boards, everything combined
        Chapter { name: "Grand Mastery".into(), levels: {
            let all_w = |d: u32| -> [u32; GEN_NUM_WEIGHTS] {
                let s = d as f32 / 100.0;
                let v = |base: u32| (base as f32 * (0.8 + s * 1.0)) as u32;
                [v(6), v(4), v(6), v(4), v(5), v(3), v(5), v(3), v(5), v(4), v(3), v(4)]
            };
            let mut lvls = vec![
                cl("The Convergence", cfg(8, 5, 65, all_w(65)).chains(1).edge_holes(25).share().confuse()),
                cl("All In", cfg(8, 5, 70, all_w(70)).chains(1).holes(25).share().confuse()),
                cl("Synthesis", cfg(9, 6, 75, all_w(75)).chains(2).mid_holes(25).share().confuse()),
                cl("Full Spectrum", cfg(9, 6, 80, all_w(80)).chains(2).holes(25).share().confuse()),
                cl("Mechanic Fusion", cfg(9, 7, 85, all_w(85)).inv(4).chains(2).edge_holes(25).share().confuse()),
                cl("The Crucible", cfg(10, 7, 88, all_w(88)).inv(5).chains(3).holes(30).share().confuse()),
                cl("Quantum Tangle", cfg(10, 8, 90, all_w(90)).inv(5).chains(3).mid_holes(30).share().confuse()),
                cl("Neural Network", cfg(11, 8, 92, all_w(92)).inv(6).chains(3).holes(30).share().confuse()),
                cl("Chaos Theory", cfg(11, 8, 95, all_w(95)).inv(6).chains(4).edge_holes(30).share().confuse()),
                cl("The Architect", cfg(12, 9, 97, all_w(97)).inv(7).chains(4).holes(30).share().confuse()),
                cl("Event Horizon", cfg(12, 9, 100, all_w(100)).inv(7).chains(4).mid_holes(30).share().confuse()),
                cl("Singularity", cfg(12, 10, 100, all_w(100)).inv(7).chains(4).holes(30).share().confuse()),
            ];
            lvls.push(cl("FINAL BOSS I — The Protocol", cfg(12, 10, 100, [12; GEN_NUM_WEIGHTS]).inv(8).chains(5).edge_holes(30).share().confuse()));
            lvls.push(cl("FINAL BOSS II — The Machine", cfg(12, 10, 100, [12; GEN_NUM_WEIGHTS]).inv(8).chains(5).holes(30).share().confuse()));
            lvls.push(cl("FINAL BOSS III — Transcendence", cfg(12, 10, 100, [14; GEN_NUM_WEIGHTS]).inv(9).chains(5).mid_holes(30).share().confuse()));
            lvls.push(cl("SECRET — The Impossible", cfg(12, 10, 100, [14; GEN_NUM_WEIGHTS]).inv(8).chains(5).holes(30).share().confuse()));
            lvls.push(cl("SECRET — Protocol Complete", cfg(12, 10, 100, [14; GEN_NUM_WEIGHTS]).inv(8).chains(5).edge_holes(30).share().confuse()));
            lvls
        }},
    ]
}

// ============================================================
// Builder helpers
// ============================================================

fn w(pairs: &[(usize, u32)]) -> [u32; GEN_NUM_WEIGHTS] {
    let mut weights = [0u32; GEN_NUM_WEIGHTS];
    for &(idx, val) in pairs { weights[idx] = val; }
    weights
}

fn cl(name: &str, config: GenConfig) -> CampaignLevel {
    CampaignLevel { display_name: name.into(), config }
}

trait ConfigExt {
    fn inv(self, target: u32) -> Self;
    fn chains(self, n: u32) -> Self;
    fn share(self) -> Self;
    fn confuse(self) -> Self;
    fn unique(self) -> Self;
    fn holes(self, pct: u32) -> Self;
    fn edge_holes(self, pct: u32) -> Self;
    fn mid_holes(self, pct: u32) -> Self;
}

impl ConfigExt for GenConfig {
    fn inv(mut self, target: u32) -> Self { self.inventory_target = target; self }
    fn chains(mut self, n: u32) -> Self { self.door_chains = n; self }
    fn share(mut self) -> Self { self.path_sharing = true; self }
    fn confuse(mut self) -> Self { self.confusion_tiles = true; self }
    fn unique(mut self) -> Self { self.unique_solution = true; self }
    fn holes(mut self, pct: u32) -> Self { self.hole_percent = pct; self }
    fn edge_holes(mut self, pct: u32) -> Self { self.hole_percent = pct; self.hole_placement = HolePlacement::Edges; self }
    fn mid_holes(mut self, pct: u32) -> Self { self.hole_percent = pct; self.hole_placement = HolePlacement::Middle; self }
}

impl ConfigExt for CampaignLevel {
    fn inv(mut self, target: u32) -> Self { self.config.inventory_target = target; self }
    fn chains(mut self, n: u32) -> Self { self.config.door_chains = n; self }
    fn share(mut self) -> Self { self.config.path_sharing = true; self }
    fn confuse(mut self) -> Self { self.config.confusion_tiles = true; self }
    fn unique(mut self) -> Self { self.config.unique_solution = true; self }
    fn holes(mut self, pct: u32) -> Self { self.config.hole_percent = pct; self }
    fn edge_holes(mut self, pct: u32) -> Self { self.config.hole_percent = pct; self.config.hole_placement = HolePlacement::Edges; self }
    fn mid_holes(mut self, pct: u32) -> Self { self.config.hole_percent = pct; self.config.hole_placement = HolePlacement::Middle; self }
}
