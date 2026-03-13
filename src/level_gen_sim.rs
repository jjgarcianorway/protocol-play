// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use crate::constants::*;
use crate::types::*;

struct SimBot {
    col: i32, row: i32, dir: Direction, color: usize,
    alive: bool, at_goal: bool, switch_pending: bool,
}

fn in_bounds(c: i32, r: i32, size: u32) -> bool {
    c >= 0 && r >= 0 && c < size as i32 && r < size as i32
}

pub fn simulate_headless(size: u32, tiles: &[(u32, u32, TileKind)]) -> bool {
    let mut grid: HashMap<(u32, u32), TileKind> = tiles.iter()
        .map(|(c, r, k)| ((*c, *r), *k)).collect();
    let mut bots: Vec<SimBot> = Vec::new();
    for (&(c, r), kind) in &grid {
        if let TileKind::Source(ci, dir) = kind {
            bots.push(SimBot {
                col: c as i32, row: r as i32, dir: *dir,
                color: *ci, alive: true, at_goal: false, switch_pending: false,
            });
        }
    }
    if bots.is_empty() { return false; }

    for _ in 0..GEN_MAX_SIM_STEPS {
        if bots.iter().all(|b| b.at_goal || !b.alive) { break; }

        // Toggle doors if any bot has pending switch
        if bots.iter().any(|b| b.switch_pending) {
            for kind in grid.values_mut() {
                if let TileKind::Door(open) = kind { *open = !*open; }
            }
            for bot in &mut bots { bot.switch_pending = false; }
        }

        for bot in &mut bots {
            if !bot.alive || bot.at_goal { continue; }
            let (dc, dr) = bot.dir.grid_delta();
            let (nc, nr) = (bot.col + dc, bot.row + dr);
            if !in_bounds(nc, nr, size) { bot.alive = false; continue; }
            let tile = grid.get(&(nc as u32, nr as u32)).copied();
            match tile {
                None | Some(TileKind::Empty) => { bot.alive = false; continue; }
                Some(TileKind::Door(false)) => {
                    bot.dir = bot.dir.opposite(); continue;
                }
                _ => { bot.col = nc; bot.row = nr; }
            }
            match tile {
                Some(TileKind::Goal(ci)) if ci == bot.color => { bot.at_goal = true; }
                Some(TileKind::Painter(ci)) => { bot.color = ci; }
                Some(TileKind::Turn(_, td)) => {
                    if let Some(e) = bot.dir.turn_exit(td) { bot.dir = e; }
                }
                Some(TileKind::TurnBut(ci, td)) if ci != bot.color => {
                    if let Some(e) = bot.dir.turn_exit(td) { bot.dir = e; }
                }
                Some(TileKind::Bounce(_)) => { bot.dir = bot.dir.opposite(); }
                Some(TileKind::BounceBut(ci)) if ci != bot.color => { bot.dir = bot.dir.opposite(); }
                Some(TileKind::Arrow(_, d)) => { bot.dir = d; }
                Some(TileKind::ArrowBut(ci, d)) if ci != bot.color => { bot.dir = d; }
                Some(TileKind::Switch) => { bot.switch_pending = true; }
                Some(TileKind::ColorSwitch(ci)) if ci == bot.color => { bot.switch_pending = true; }
                Some(TileKind::ColorSwitchBut(ci)) if ci != bot.color => { bot.switch_pending = true; }
                Some(TileKind::Teleport(co, num)) if co == NUM_COLORS || co == bot.color => {
                    if let Some((&p, _)) = grid.iter().find(|(pos, k)|
                        matches!(k, TileKind::Teleport(c, n) if *c == co && *n == num)
                        && **pos != (nc as u32, nr as u32))
                    { bot.col = p.0 as i32; bot.row = p.1 as i32; }
                }
                Some(TileKind::TeleportBut(co, num)) if co != bot.color => {
                    if let Some((&p, _)) = grid.iter().find(|(pos, k)|
                        matches!(k, TileKind::TeleportBut(c, n) if *c == co && *n == num)
                        && **pos != (nc as u32, nr as u32))
                    { bot.col = p.0 as i32; bot.row = p.1 as i32; }
                }
                _ => {}
            }
        }
    }
    bots.iter().all(|b| b.at_goal)
}
