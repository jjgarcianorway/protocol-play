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

            match generate_level(&level.config, 20000) {
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

// ============================================================
// Campaign definition — 13 chapters, ~149 levels total
// ============================================================

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
        hole_percent: 0, hole_placement: HolePlacement::Both,
        unique_solution: false, inventory_target: 0,
        door_chains: 0, path_sharing: false, confusion_tiles: false,
    }
}

fn campaign_chapters() -> Vec<Chapter> {
    vec![
        // =====================================================
        // Chapter 1: Turn — Basic path building
        // =====================================================
        Chapter { name: "Turns".into(), levels: vec![
            cl("First Steps", cfg(3, 1, 15, w(&[(0, 5)]))),
            cl("Corner to Corner", cfg(3, 1, 25, w(&[(0, 6)]))),
            cl("The Zigzag", cfg(3, 1, 35, w(&[(0, 7)]))),
            cl("Around the Block", cfg(4, 1, 40, w(&[(0, 7)]))),
            cl("Spiral Path", cfg(4, 1, 45, w(&[(0, 8)]))),
            cl("Double Back", cfg(4, 1, 50, w(&[(0, 8)]))),
            cl("Winding Road", cfg(5, 1, 55, w(&[(0, 8)]))),
            cl("Crossroads", cfg(5, 1, 60, w(&[(0, 9)]))),
            // Boss levels
            cl("Turn Master I", cfg(5, 1, 65, w(&[(0, 10)]))),
            cl("Turn Master II", cfg(6, 1, 70, w(&[(0, 10)]))),
            cl("Turn Master III", cfg(6, 1, 75, w(&[(0, 11)]))),
        ]},

        // =====================================================
        // Chapter 2: TurnBut — Inventory placement
        // =====================================================
        Chapter { name: "Turn Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Turn", cfg(3, 1, 20, w(&[(1, 5)])).inv(2)),
                cl("Two Turns", cfg(3, 1, 30, w(&[(1, 6)])).inv(3)),
                cl("Choose Wisely", cfg(4, 1, 40, w(&[(0, 3), (1, 5)])).inv(3)),
                cl("Mixed Turns", cfg(4, 1, 45, w(&[(0, 4), (1, 5)])).inv(4)),
                cl("Turn Puzzle", cfg(4, 1, 50, w(&[(0, 3), (1, 6)])).inv(4)),
                cl("Inventory Challenge", cfg(5, 1, 55, w(&[(0, 3), (1, 7)])).inv(5)),
                cl("Precision Placement", cfg(5, 1, 60, w(&[(0, 4), (1, 7)])).inv(5)),
                cl("No Room for Error", cfg(5, 1, 65, w(&[(0, 3), (1, 8)])).inv(5)),
            ];
            // Boss levels
            lvls.push(cl("Turn Builder I", cfg(5, 1, 70, w(&[(0, 4), (1, 8)])).inv(6)));
            lvls.push(cl("Turn Builder II", cfg(6, 1, 75, w(&[(0, 4), (1, 8)])).inv(6)));
            lvls.push(cl("Turn Builder III", cfg(6, 1, 80, w(&[(0, 5), (1, 9)])).inv(7)));
            lvls
        }},

        // =====================================================
        // Chapter 3: Arrow — Forced direction
        // =====================================================
        Chapter { name: "Arrows".into(), levels: vec![
            cl("One Way Street", cfg(3, 1, 25, w(&[(2, 5)]))),
            cl("Follow the Arrow", cfg(3, 1, 35, w(&[(2, 6)]))),
            cl("Arrow Maze", cfg(4, 1, 40, w(&[(0, 3), (2, 5)]))),
            cl("Turn and Thrust", cfg(4, 1, 45, w(&[(0, 4), (2, 5)]))),
            cl("Arrow Chain", cfg(4, 1, 50, w(&[(0, 3), (2, 6)]))),
            cl("Speed Lines", cfg(5, 1, 55, w(&[(0, 4), (2, 7)]))),
            cl("The Gauntlet", cfg(5, 1, 60, w(&[(0, 4), (2, 7)]))),
            cl("Forced March", cfg(5, 1, 65, w(&[(0, 5), (2, 8)]))),
            // Boss
            cl("Arrow Storm I", cfg(6, 1, 70, w(&[(0, 5), (2, 8)]))),
            cl("Arrow Storm II", cfg(6, 1, 75, w(&[(0, 5), (2, 9)]))),
            cl("Arrow Storm III", cfg(7, 1, 80, w(&[(0, 6), (2, 9)]))),
        ]},

        // =====================================================
        // Chapter 4: ArrowBut — Arrow inventory
        // =====================================================
        Chapter { name: "Arrow Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Arrow", cfg(3, 1, 30, w(&[(3, 5)])).inv(2)),
                cl("Arrow Setup", cfg(3, 1, 40, w(&[(0, 3), (3, 5)])).inv(3)),
                cl("Redirect", cfg(4, 1, 45, w(&[(0, 3), (2, 3), (3, 5)])).inv(3)),
                cl("Arrow Architect", cfg(4, 1, 50, w(&[(0, 4), (2, 3), (3, 5)])).inv(4)),
                cl("Mixed Signals", cfg(4, 1, 55, w(&[(0, 3), (1, 2), (2, 3), (3, 5)])).inv(4)),
                cl("Direction Control", cfg(5, 1, 60, w(&[(0, 4), (1, 3), (2, 3), (3, 6)])).inv(5)),
                cl("Arrow Master", cfg(5, 1, 65, w(&[(0, 4), (2, 4), (3, 6)])).inv(5)),
                cl("Full Arsenal", cfg(5, 1, 70, w(&[(0, 4), (1, 3), (2, 4), (3, 6)])).inv(5)),
            ];
            lvls.push(cl("Arrow Crafter I", cfg(6, 1, 75, w(&[(0, 5), (1, 3), (2, 4), (3, 7)])).inv(6)));
            lvls.push(cl("Arrow Crafter II", cfg(6, 1, 80, w(&[(0, 5), (1, 3), (2, 5), (3, 7)])).inv(6)));
            lvls.push(cl("Arrow Crafter III", cfg(7, 1, 85, w(&[(0, 5), (1, 4), (2, 5), (3, 8)])).inv(7)));
            lvls
        }},

        // =====================================================
        // Chapter 5: Teleport — Warp mechanics
        // =====================================================
        Chapter { name: "Teleports".into(), levels: vec![
            cl("Warp Zone", cfg(3, 1, 30, w(&[(0, 3), (4, 5)]))),
            cl("Portal Hop", cfg(4, 1, 40, w(&[(0, 4), (4, 6)]))),
            cl("Double Warp", cfg(4, 1, 45, w(&[(0, 4), (2, 3), (4, 5)]))),
            cl("Teleport Chain", cfg(4, 1, 50, w(&[(0, 4), (4, 7)]))),
            cl("Warp Tactics", cfg(5, 1, 55, w(&[(0, 4), (2, 3), (4, 6)]))),
            cl("Portal Network", cfg(5, 1, 60, w(&[(0, 5), (2, 4), (4, 6)]))),
            cl("Dimensional Shift", cfg(5, 1, 65, w(&[(0, 5), (2, 4), (4, 7)]))),
            cl("Space Fold", cfg(6, 1, 70, w(&[(0, 5), (2, 5), (4, 7)]))),
            // Boss
            cl("Warp Master I", cfg(6, 1, 75, w(&[(0, 5), (2, 5), (4, 8)]))),
            cl("Warp Master II", cfg(7, 1, 80, w(&[(0, 6), (2, 5), (4, 9)]))),
            cl("Warp Master III", cfg(7, 1, 85, w(&[(0, 6), (2, 5), (4, 9)]))),
        ]},

        // =====================================================
        // Chapter 6: TeleportBut — Teleport inventory
        // =====================================================
        Chapter { name: "Teleport Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Portal", cfg(3, 1, 35, w(&[(0, 3), (5, 5)])).inv(2)),
                cl("Warp Builder", cfg(4, 1, 45, w(&[(0, 3), (4, 3), (5, 5)])).inv(3)),
                cl("Portal Placement", cfg(4, 1, 50, w(&[(0, 4), (2, 3), (5, 5)])).inv(3)),
                cl("Linked Portals", cfg(4, 1, 55, w(&[(0, 4), (4, 3), (5, 6)])).inv(4)),
                cl("Warp Circuit", cfg(5, 1, 60, w(&[(0, 4), (2, 3), (4, 3), (5, 6)])).inv(4)),
                cl("Teleport Engineer", cfg(5, 1, 65, w(&[(0, 4), (2, 3), (4, 4), (5, 6)])).inv(5)),
                cl("Dimension Builder", cfg(5, 1, 70, w(&[(0, 5), (2, 4), (4, 4), (5, 7)])).inv(5)),
                cl("Space Architect", cfg(6, 1, 75, w(&[(0, 5), (1, 3), (4, 4), (5, 7)])).inv(5)),
            ];
            lvls.push(cl("Portal Crafter I", cfg(6, 1, 80, w(&[(0, 5), (2, 4), (4, 4), (5, 8)])).inv(6)));
            lvls.push(cl("Portal Crafter II", cfg(7, 1, 85, w(&[(0, 5), (1, 3), (2, 4), (4, 4), (5, 8)])).inv(6)));
            lvls.push(cl("Portal Crafter III", cfg(7, 1, 90, w(&[(0, 6), (2, 5), (4, 5), (5, 9)])).inv(7)));
            lvls
        }},

        // =====================================================
        // Chapter 7: Bounce — Reflection mechanics
        // =====================================================
        Chapter { name: "Bounce".into(), levels: vec![
            cl("First Bounce", cfg(3, 1, 30, w(&[(0, 4), (6, 5)]))),
            cl("Ricochet", cfg(4, 1, 40, w(&[(0, 4), (6, 6)]))),
            cl("Bounce Path", cfg(4, 1, 45, w(&[(0, 4), (2, 3), (6, 5)]))),
            cl("Wall Runner", cfg(4, 1, 50, w(&[(0, 4), (6, 7)]))),
            cl("Reflection Point", cfg(5, 1, 55, w(&[(0, 4), (2, 3), (6, 6)]))),
            cl("Bounce House", cfg(5, 1, 60, w(&[(0, 5), (4, 3), (6, 6)]))),
            cl("Echo Chamber", cfg(5, 1, 65, w(&[(0, 5), (2, 4), (6, 7)]))),
            cl("Rebound", cfg(6, 1, 70, w(&[(0, 5), (2, 4), (4, 3), (6, 7)]))),
            // Boss
            cl("Bounce King I", cfg(6, 1, 75, w(&[(0, 5), (2, 5), (4, 3), (6, 8)]))),
            cl("Bounce King II", cfg(7, 1, 80, w(&[(0, 6), (2, 5), (4, 4), (6, 8)]))),
            cl("Bounce King III", cfg(7, 1, 85, w(&[(0, 6), (2, 5), (4, 4), (6, 9)]))),
        ]},

        // =====================================================
        // Chapter 8: BounceBut — Bounce inventory
        // =====================================================
        Chapter { name: "Bounce Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Bounce", cfg(3, 1, 35, w(&[(0, 3), (7, 5)])).inv(2)),
                cl("Bounce Setup", cfg(4, 1, 45, w(&[(0, 3), (6, 3), (7, 5)])).inv(3)),
                cl("Ricochet Builder", cfg(4, 1, 50, w(&[(0, 4), (2, 3), (7, 5)])).inv(3)),
                cl("Bounce Craft", cfg(4, 1, 55, w(&[(0, 4), (6, 3), (7, 6)])).inv(4)),
                cl("Reflection Lab", cfg(5, 1, 60, w(&[(0, 4), (2, 3), (6, 3), (7, 6)])).inv(4)),
                cl("Bounce Engineer", cfg(5, 1, 65, w(&[(0, 4), (2, 3), (6, 4), (7, 6)])).inv(5)),
                cl("Echo Builder", cfg(5, 1, 70, w(&[(0, 5), (4, 3), (6, 4), (7, 7)])).inv(5)),
                cl("Rebound Architect", cfg(6, 1, 75, w(&[(0, 5), (2, 4), (4, 3), (6, 4), (7, 7)])).inv(5)),
            ];
            lvls.push(cl("Bounce Crafter I", cfg(6, 1, 80, w(&[(0, 5), (2, 4), (6, 4), (7, 8)])).inv(6)));
            lvls.push(cl("Bounce Crafter II", cfg(7, 1, 85, w(&[(0, 5), (1, 3), (6, 4), (7, 8)])).inv(6)));
            lvls.push(cl("Bounce Crafter III", cfg(7, 2, 90, w(&[(0, 6), (2, 5), (6, 5), (7, 9)])).inv(7)));
            lvls
        }},

        // =====================================================
        // Chapter 9: Painter — Color changing
        // =====================================================
        Chapter { name: "Painters".into(), levels: vec![
            cl("Color Shift", cfg(3, 1, 30, w(&[(0, 4), (11, 5)]))),
            cl("Paint the Path", cfg(4, 1, 40, w(&[(0, 4), (2, 3), (11, 5)]))),
            cl("Color Journey", cfg(4, 1, 50, w(&[(0, 4), (11, 6)]))),
            cl("Rainbow Road", cfg(5, 2, 55, w(&[(0, 4), (2, 3), (11, 6)]))),
            cl("Chromatic Path", cfg(5, 2, 60, w(&[(0, 5), (4, 3), (11, 6)]))),
            cl("Painter's Palette", cfg(5, 2, 65, w(&[(0, 5), (2, 4), (11, 7)]))),
            cl("Color Cascade", cfg(6, 2, 70, w(&[(0, 5), (2, 4), (4, 3), (11, 7)]))),
            cl("Hue Shift", cfg(6, 2, 75, w(&[(0, 5), (2, 4), (6, 3), (11, 7)]))),
            // Boss
            cl("Color Master I", cfg(6, 2, 80, w(&[(0, 6), (2, 5), (4, 4), (11, 8)]))),
            cl("Color Master II", cfg(7, 2, 85, w(&[(0, 6), (2, 5), (6, 4), (11, 9)]))),
            cl("Color Master III", cfg(7, 2, 90, w(&[(0, 6), (2, 5), (4, 4), (6, 4), (11, 9)]))),
        ]},

        // =====================================================
        // Chapter 10: Door + Switch — Toggle timing
        // =====================================================
        Chapter { name: "Doors & Switches".into(), levels: {
            let mut lvls = vec![
                cl("Open Sesame", cfg(4, 2, 40, w(&[(0, 5), (8, 5)])).chains(1)),
                cl("Locked Path", cfg(4, 2, 50, w(&[(0, 5), (2, 3), (8, 5)])).chains(1)),
                cl("Switch Timing", cfg(4, 2, 55, w(&[(0, 5), (8, 6)])).chains(1).share()),
                cl("Door Dance", cfg(5, 2, 60, w(&[(0, 5), (2, 3), (8, 6)])).chains(1).share()),
                cl("Double Lock", cfg(5, 2, 65, w(&[(0, 5), (2, 4), (8, 7)])).chains(2).share()),
                cl("Gate Keeper", cfg(5, 2, 70, w(&[(0, 5), (4, 3), (8, 7)])).chains(2).share()),
                cl("Synchronized", cfg(6, 2, 75, w(&[(0, 5), (2, 4), (4, 3), (8, 7)])).chains(2).share()),
                cl("Chain Reaction", cfg(6, 2, 80, w(&[(0, 6), (2, 4), (8, 8)])).chains(3).share()),
            ];
            lvls.push(cl("Lock Master I", cfg(6, 2, 85, w(&[(0, 6), (2, 5), (4, 4), (8, 8)])).chains(3).share()));
            lvls.push(cl("Lock Master II", cfg(7, 3, 90, w(&[(0, 6), (2, 5), (8, 9)])).chains(3).share()));
            lvls.push(cl("Lock Master III", cfg(7, 3, 95, w(&[(0, 7), (2, 5), (4, 4), (8, 9)])).chains(4).share()));
            lvls
        }},

        // =====================================================
        // Chapter 11: ColorSwitch — Color-gated toggling
        // =====================================================
        Chapter { name: "Color Switches".into(), levels: {
            let mut lvls = vec![
                cl("Color Gate", cfg(4, 2, 45, w(&[(0, 4), (9, 5)])).chains(1)),
                cl("Chromatic Lock", cfg(4, 2, 55, w(&[(0, 4), (2, 3), (9, 5)])).chains(1).share()),
                cl("Color Timing", cfg(5, 2, 60, w(&[(0, 5), (9, 6), (11, 3)])).chains(1).share()),
                cl("Hue Gate", cfg(5, 2, 65, w(&[(0, 5), (2, 3), (8, 3), (9, 6)])).chains(1).share()),
                cl("Spectrum Lock", cfg(5, 2, 70, w(&[(0, 5), (4, 3), (9, 6)])).chains(2).share()),
                cl("Color Cascade", cfg(6, 2, 75, w(&[(0, 5), (2, 4), (8, 3), (9, 7)])).chains(2).share()),
                cl("Prismatic Path", cfg(6, 2, 80, w(&[(0, 6), (4, 3), (9, 7), (11, 4)])).chains(2).share()),
                cl("Rainbow Gate", cfg(6, 2, 85, w(&[(0, 6), (2, 4), (8, 4), (9, 7)])).chains(3).share()),
            ];
            lvls.push(cl("Chroma Master I", cfg(7, 2, 90, w(&[(0, 6), (2, 5), (4, 4), (8, 4), (9, 8)])).chains(3).share()));
            lvls.push(cl("Chroma Master II", cfg(7, 3, 95, w(&[(0, 6), (2, 5), (8, 5), (9, 8), (11, 4)])).chains(3).share()));
            lvls.push(cl("Chroma Master III", cfg(8, 3, 100, w(&[(0, 7), (2, 5), (4, 4), (8, 5), (9, 9)])).chains(4).share()));
            lvls
        }},

        // =====================================================
        // Chapter 12: ColorSwitchBut — Color switch inventory
        // =====================================================
        Chapter { name: "Color Switch Tiles".into(), levels: {
            let mut lvls = vec![
                cl("Place Your Gate", cfg(4, 2, 50, w(&[(0, 4), (10, 5)])).inv(2).chains(1).share()),
                cl("Color Builder", cfg(5, 2, 55, w(&[(0, 4), (9, 3), (10, 5)])).inv(3).chains(1).share()),
                cl("Switch Craft", cfg(5, 2, 60, w(&[(0, 5), (2, 3), (10, 6)])).inv(3).chains(1).share()),
                cl("Chromatic Builder", cfg(5, 2, 65, w(&[(0, 5), (8, 3), (9, 3), (10, 6)])).inv(4).chains(2).share()),
                cl("Gate Architect", cfg(6, 2, 70, w(&[(0, 5), (2, 4), (9, 3), (10, 6)])).inv(4).chains(2).share()),
                cl("Color Engineer", cfg(6, 2, 75, w(&[(0, 5), (4, 3), (8, 3), (10, 7)])).inv(5).chains(2).share()),
                cl("Prismatic Craft", cfg(6, 2, 80, w(&[(0, 6), (2, 4), (9, 4), (10, 7)])).inv(5).chains(2).share()),
                cl("Spectrum Builder", cfg(7, 3, 85, w(&[(0, 6), (2, 4), (8, 4), (9, 4), (10, 7)])).inv(5).chains(3).share()),
            ];
            lvls.push(cl("Gate Crafter I", cfg(7, 3, 90, w(&[(0, 6), (2, 5), (4, 4), (9, 4), (10, 8)])).inv(6).chains(3).share()));
            lvls.push(cl("Gate Crafter II", cfg(7, 3, 95, w(&[(0, 7), (2, 5), (8, 5), (9, 5), (10, 8)])).inv(6).chains(3).share()));
            lvls.push(cl("Gate Crafter III", cfg(8, 3, 100, w(&[(0, 7), (1, 4), (2, 5), (4, 4), (9, 5), (10, 9)])).inv(7).chains(4).share()));
            lvls
        }},

        // =====================================================
        // Chapter 13: Grand Mastery — All mechanics combined
        // =====================================================
        Chapter { name: "Grand Mastery".into(), levels: {
            let all_w = |d: u32| -> [u32; GEN_NUM_WEIGHTS] {
                let s = d as f32 / 100.0;
                let v = |base: u32| (base as f32 * (0.5 + s * 0.8)) as u32;
                [v(5), v(3), v(5), v(3), v(4), v(2), v(4), v(2), v(4), v(3), v(2), v(3)]
            };
            let mut lvls = vec![
                cl("The Convergence", cfg(5, 2, 45, all_w(45)).chains(1).share()),
                cl("All In", cfg(5, 2, 50, all_w(50)).chains(1).share().confuse()),
                cl("Synthesis", cfg(6, 2, 55, all_w(55)).chains(2).share()),
                cl("Full Spectrum", cfg(6, 2, 60, all_w(60)).chains(2).share().confuse()),
                cl("Mechanic Fusion", cfg(6, 3, 65, all_w(65)).inv(4).chains(2).share()),
                cl("The Crucible", cfg(7, 3, 70, all_w(70)).inv(5).chains(3).share().confuse()),
                cl("Quantum Tangle", cfg(7, 3, 75, all_w(75)).inv(5).chains(3).share()),
                cl("Neural Network", cfg(7, 3, 80, all_w(80)).inv(6).chains(3).share().confuse()),
                cl("Chaos Theory", cfg(8, 3, 85, all_w(85)).inv(6).chains(4).share().confuse()),
                cl("The Architect", cfg(8, 4, 88, all_w(88)).inv(7).chains(4).share()),
                cl("Event Horizon", cfg(8, 4, 90, all_w(90)).inv(7).chains(4).share().confuse()),
                cl("Singularity", cfg(9, 4, 92, all_w(92)).inv(7).chains(4).share().confuse()),
            ];
            // Grand Boss levels
            lvls.push(cl("FINAL BOSS I — The Protocol", cfg(9, 4, 95, all_w(95)).inv(8).chains(5).share().confuse()));
            lvls.push(cl("FINAL BOSS II — The Machine", cfg(10, 4, 97, all_w(97)).inv(8).chains(5).share().confuse()));
            lvls.push(cl("FINAL BOSS III — Transcendence", cfg(10, 4, 100, [10; GEN_NUM_WEIGHTS]).inv(9).chains(5).share().confuse()));
            lvls.push(cl("SECRET — The Impossible", cfg(11, 4, 100, [10; GEN_NUM_WEIGHTS]).inv(8).chains(5).share().confuse()));
            lvls.push(cl("SECRET — Protocol Complete", cfg(12, 4, 100, [10; GEN_NUM_WEIGHTS]).inv(8).chains(5).share().confuse()));
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
}

impl ConfigExt for GenConfig {
    fn inv(mut self, target: u32) -> Self { self.inventory_target = target; self }
    fn chains(mut self, n: u32) -> Self { self.door_chains = n; self }
    fn share(mut self) -> Self { self.path_sharing = true; self }
    fn confuse(mut self) -> Self { self.confusion_tiles = true; self }
    fn unique(mut self) -> Self { self.unique_solution = true; self }
}

impl ConfigExt for CampaignLevel {
    fn inv(mut self, target: u32) -> Self { self.config.inventory_target = target; self }
    fn chains(mut self, n: u32) -> Self { self.config.door_chains = n; self }
    fn share(mut self) -> Self { self.config.path_sharing = true; self }
    fn confuse(mut self) -> Self { self.config.confusion_tiles = true; self }
    fn unique(mut self) -> Self { self.config.unique_solution = true; self }
}
