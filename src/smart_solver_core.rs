// SPDX-License-Identifier: GPL-3.0-or-later
// Smart solver: demand-driven placement + path-constrained backtracking.
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use crate::types::*;
use crate::constants::*;
use crate::smart_solver_sim::FastGrid;
pub struct SolveResult {
    pub solved: bool,
    pub attempts: u64,
    pub strategy: String,
    pub difficulty: &'static str,
}

pub fn solve_level(data: &LevelData) -> SolveResult {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs_f64(0.5);
    let size = data.board_size;
    let mut board: Vec<(u32, u32, TileKind)> = Vec::new();
    let mut inventory: Vec<TileKind> = Vec::new();
    for &(c, r, kind, marked) in &data.tiles {
        if marked {
            inventory.push(kind);
            board.push((c, r, TileKind::Floor));
        } else {
            board.push((c, r, kind));
        }
    }
    if inventory.is_empty() {
        let fg = FastGrid::new(size, &board);
        let ok = fg.simulate();
        return SolveResult {
            solved: ok, attempts: if ok { 1 } else { 0 },
            strategy: "no-inventory".into(), difficulty: "trivial",
        };
    }
    let floor_set: HashSet<(u32, u32)> = board.iter()
        .filter(|(_, _, k)| matches!(k, TileKind::Floor))
        .map(|(c, r, _)| (*c, *r)).collect();
    let pos_idx: HashMap<(u32, u32), usize> = board.iter()
        .enumerate().map(|(i, (c, r, _))| ((*c, *r), i)).collect();
    // Sort to group identical tiles together (for symmetry breaking)
    inventory.sort_by(|a, b| tile_sort_key(a).cmp(&tile_sort_key(b)));
    let mut total_att = 0u64;
    let mut fg = FastGrid::new(size, &board);

    // Phase 1: demand-driven (smart, constraint-based)
    // Best for levels where floors >> inventory (high ratio = many wrong positions to skip)
    let demand_limit = 5_000_000u64;
    let mut att = 0u64;
    let mut used_d = vec![false; inventory.len()];
    if demand_solve(&inventory, &mut used_d, &mut fg, &board, &pos_idx, &floor_set, size, &mut att, demand_limit, &deadline) {
        return mk_result(true, att, "demand-driven", inventory.len());
    }
    total_att += att;

    // Phase 2: constraint propagation + viable backtracking
    let color_reach = find_color_reachable(size, &board, &floor_set);
    att = 0;
    let mut fg2 = FastGrid::new(size, &board);
    let mut remaining_inv = inventory.clone();
    let mut placed_set = HashSet::new();
    // Propagate: place tiles with only 1 viable position
    loop {
        let viable = compute_viable_positions(&remaining_inv, &floor_set, &fg2, &color_reach);
        let mut progress = false;
        let mut to_remove = Vec::new();
        for (i, positions) in viable.iter().enumerate() {
            let free: Vec<_> = positions.iter().filter(|&&p| !placed_set.contains(&p)).copied().collect();
            if free.len() == 1 {
                let (c, r) = free[0];
                fg2.set(c, r, remaining_inv[i]);
                placed_set.insert((c, r));
                to_remove.push(i);
                progress = true;
            }
        }
        for &i in to_remove.iter().rev() { remaining_inv.remove(i); }
        if !progress { break; }
    }
    // Now backtrack on remaining tiles
    if remaining_inv.is_empty() {
        att += 1;
        if fg2.simulate() {
            return mk_result(true, total_att + att, "propagation", inventory.len());
        }
    } else {
        let viable = compute_viable_positions(&remaining_inv, &floor_set, &fg2, &color_reach);
        // Fill empty viable sets with all floors (fallback)
        let all_fl: Vec<(u32,u32)> = {let mut v: Vec<_> = floor_set.iter().copied().collect(); v.sort(); v};
        let viable: Vec<Vec<(u32,u32)>> = viable.into_iter().map(|v| if v.is_empty() { all_fl.clone() } else { v }).collect();
        if viable.iter().all(|v| !v.is_empty()) {
            // Limit based on inventory size to avoid spending too long on unsolvable configs
            let bt_limit = 500_000_000u64;
            if backtrack_viable(&remaining_inv, 0, &viable, &mut placed_set,
                &mut fg2, size, &mut att, bt_limit, &deadline) {
                return mk_result(true, total_att + att, "viable-constrained", inventory.len());
            }
        }
    }
    total_att += att;

    mk_result(false, total_att, "exhausted", inventory.len())
}

/// Demand-driven: simulate, find first stuck bot, try tiles there.
fn demand_solve(
    inv: &[TileKind], used: &mut Vec<bool>, fg: &mut FastGrid,
    board: &[(u32, u32, TileKind)], pos_idx: &HashMap<(u32, u32), usize>,
    floor_set: &HashSet<(u32, u32)>, size: u32, attempts: &mut u64, limit: u64,
    deadline: &std::time::Instant,
) -> bool {
    if *attempts >= limit { return false; }
    if *attempts % 10_000 == 0 && std::time::Instant::now() > *deadline { return false; }
    *attempts += 1;
    if fg.simulate() { return true; }
    let needs = find_needs(size, board, fg, floor_set);
    if needs.is_empty() { return false; }
    // Score and sort by constraint count (ascending = most constrained first)
    let mut scored: Vec<(usize, u32, u32, Direction, usize)> = Vec::new();
    let mut seen_pos = HashSet::new();
    for &(nc, nr, dir, color) in &needs {
        if !seen_pos.insert((nc, nr)) { continue; } // deduplicate positions
        if pos_idx.get(&(nc, nr)).is_none() { continue; }
        if !matches!(fg.get_tile(nc, nr), TileKind::Floor) { continue; }
        let mut count = 0usize;
        for i in 0..inv.len() {
            if used[i] { continue; }
            if i > 0 && inv[i] == inv[i-1] && !used[i-1] { continue; }
            if tile_helps_fast(inv[i], dir, color, nc, nr, size, fg) { count += 1; }
        }
        if count > 0 { scored.push((count, nc, nr, dir, color)); }
    }
    if scored.is_empty() { return false; }
    scored.sort_by_key(|s| s.0);
    // Limit branching: only try top 3 need points
    let max_needs = 3.min(scored.len());
    for idx in 0..max_needs {
        let (_, nc, nr, dir, color) = scored[idx];
        for i in 0..inv.len() {
            if used[i] { continue; }
            if i > 0 && inv[i] == inv[i-1] && !used[i-1] { continue; }
            if !tile_helps_fast(inv[i], dir, color, nc, nr, size, fg) { continue; }
            fg.set(nc, nr, inv[i]);
            used[i] = true;
            if demand_solve(inv, used, fg, board, pos_idx, floor_set, size, attempts, limit, deadline) {
                return true;
            }
            used[i] = false;
            fg.set(nc, nr, TileKind::Floor);
        }
    }
    false
}

/// Check if placing `kind` could help a bot going `dir` with `color`.
fn tile_helps_fast(kind: TileKind, dir: Direction, color: usize, c: u32, r: u32, size: u32, _fg: &FastGrid) -> bool {
    match kind {
        TileKind::Floor => false,
        TileKind::Turn(ci, td) if ci == color => {
            dir.turn_exit(td).is_some_and(|exit| {
                let (dc,dr) = exit.grid_delta();
                let (nc,nr) = (c as i32+dc, r as i32+dr);
                nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
            })
        }
        TileKind::TurnBut(ci, td) if ci != color => {
            dir.turn_exit(td).is_some_and(|exit| {
                let (dc,dr) = exit.grid_delta();
                let (nc,nr) = (c as i32+dc, r as i32+dr);
                nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
            })
        }
        TileKind::Arrow(ci, d) if ci == color => {
            let (dc,dr) = d.grid_delta();
            let (nc,nr) = (c as i32+dc, r as i32+dr);
            nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
        }
        TileKind::ArrowBut(ci, d) if ci != color => {
            let (dc,dr) = d.grid_delta();
            let (nc,nr) = (c as i32+dc, r as i32+dr);
            nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
        }
        TileKind::Bounce(ci) if ci == color => {
            let opp = dir.opposite();
            let (dc,dr) = opp.grid_delta();
            let (nc,nr) = (c as i32+dc, r as i32+dr);
            nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
        }
        TileKind::BounceBut(ci) if ci != color => {
            let opp = dir.opposite();
            let (dc,dr) = opp.grid_delta();
            let (nc,nr) = (c as i32+dc, r as i32+dr);
            nc >= 0 && nr >= 0 && nc < size as i32 && nr < size as i32
        }
        _ => true, // teleport, switch, painter, goal, pass-through turns/arrows
    }
}

struct NeedBot { col: i32, row: i32, dir: Direction, color: usize, alive: bool, at_goal: bool }

fn find_needs(size: u32, _board: &[(u32,u32,TileKind)], fg: &FastGrid, floor_set: &HashSet<(u32,u32)>) -> Vec<(u32,u32,Direction,usize)> {
    // Track ALL floor cells bots visit (not just stuck points).
    // Each is a candidate for tile placement. Ordered: first cells that MUST
    // have a tile (stuck points) then cells the bot passes through.
    let mut stuck = Vec::new();
    let mut visited = Vec::new();
    let mut seen = HashSet::new();
    let mut bots: Vec<NeedBot> = Vec::new();
    for r in 0..size { for c in 0..size {
        if let TileKind::Source(ci, dir) = fg.get_tile(c, r) {
            bots.push(NeedBot { col: c as i32, row: r as i32, dir, color: ci, alive: true, at_goal: false });
        }
    }}
    for _ in 0..GEN_MAX_SIM_STEPS {
        if bots.iter().all(|b| b.at_goal || !b.alive) { break; }
        for bot in bots.iter_mut() {
            if !bot.alive || bot.at_goal { continue; }
            let (dc, dr) = bot.dir.grid_delta();
            let (nc, nr) = (bot.col + dc, bot.row + dr);
            if nc < 0 || nr < 0 || nc >= size as i32 || nr >= size as i32 {
                let p = (bot.col as u32, bot.row as u32);
                if floor_set.contains(&p) && seen.insert(p) { stuck.push((p.0,p.1,bot.dir,bot.color)); }
                bot.alive = false; continue;
            }
            let tile = fg.get_tile(nc as u32, nr as u32);
            match tile {
                TileKind::Empty => {
                    let p = (bot.col as u32, bot.row as u32);
                    if floor_set.contains(&p) && seen.insert(p) { stuck.push((p.0,p.1,bot.dir,bot.color)); }
                    bot.alive = false; continue;
                }
                TileKind::Door(false) => { bot.dir = bot.dir.opposite(); continue; }
                _ => {
                    bot.col = nc; bot.row = nr;
                    // Track floor cells bot passes through
                    let np = (nc as u32, nr as u32);
                    if floor_set.contains(&np) && seen.insert(np) {
                        visited.push((np.0, np.1, bot.dir, bot.color));
                    }
                }
            }
            match tile {
                TileKind::Goal(ci) if ci == bot.color => { bot.at_goal = true; }
                TileKind::Painter(ci) => { bot.color = ci; }
                TileKind::Turn(ci, td) if ci == bot.color => { if let Some(e) = bot.dir.turn_exit(td) { bot.dir = e; } }
                TileKind::TurnBut(ci, td) if ci != bot.color => { if let Some(e) = bot.dir.turn_exit(td) { bot.dir = e; } }
                TileKind::Bounce(ci) if ci == bot.color => { bot.dir = bot.dir.opposite(); }
                TileKind::BounceBut(ci) if ci != bot.color => { bot.dir = bot.dir.opposite(); }
                TileKind::Arrow(ci, d) if ci == bot.color => { bot.dir = d; }
                TileKind::ArrowBut(ci, d) if ci != bot.color => { bot.dir = d; }
                _ => {}
            }
        }
    }
    // Stuck points first (highest priority), then visited floors
    stuck.extend(visited);
    stuck
}

fn mk_result(ok: bool, att: u64, s: &str, n: usize) -> SolveResult {
    let d = if n<=2||att<=10{"easy"} else if att<=200{"medium"} else if att<=10_000{"hard"} else {"expert"};
    SolveResult{solved:ok,attempts:att,strategy:s.to_string(),difficulty:d}
}

fn compute_viable_positions(inv: &[TileKind], floors: &HashSet<(u32,u32)>, fg: &FastGrid,
    cr: &HashMap<usize,HashSet<(u32,u32)>>) -> Vec<Vec<(u32,u32)>> {
    inv.iter().map(|kind| {
        let tc = tile_active_color(kind);
        let mut v: Vec<_> = floors.iter().filter(|&&(c,r)| {
            if !fg.tile_viable_at(*kind,c,r) { return false; }
            tc.map_or(true, |col| cr.get(&col).is_some_and(|reach| reach.contains(&(c,r))))
        }).copied().collect();
        v.sort(); v
    }).collect()
}

fn tile_active_color(k: &TileKind) -> Option<usize> {
    match k {
        TileKind::Turn(c,_)|TileKind::Arrow(c,_)|TileKind::Bounce(c)|TileKind::Goal(c) => Some(*c),
        _ => None, // But variants, painters, etc. have no single-color constraint
    }
}

/// Backtrack with per-tile viable positions.
fn backtrack_viable(
    inv: &[TileKind], depth: usize, viable: &[Vec<(u32, u32)>],
    placed: &mut HashSet<(u32, u32)>, fg: &mut FastGrid, _size: u32,
    att: &mut u64, lim: u64, deadline: &std::time::Instant,
) -> bool {
    if *att >= lim { return false; }
    if *att % 10_000 == 0 && std::time::Instant::now() > *deadline { return false; }
    if depth == inv.len() { *att += 1; return fg.simulate(); }
    let remaining = inv.len() - depth;
    if remaining <= 2 && remaining < inv.len() {
        *att += 1;
        if !quick_viable(fg) { return false; }
    }
    let kind = inv[depth];
    let skip_before = if depth > 0 && inv[depth] == inv[depth - 1] {
        // Symmetry: for identical tiles, only try positions after the previous one
        viable[depth - 1].iter().position(|p| placed.contains(p))
            .and_then(|idx| viable[depth].iter().position(|p| *p > viable[depth-1][idx]))
    } else { None };
    let start = skip_before.unwrap_or(0);
    for i in start..viable[depth].len() {
        let (c, r) = viable[depth][i];
        if placed.contains(&(c, r)) { continue; }
        placed.insert((c, r));
        fg.set(c, r, kind);
        if backtrack_viable(inv, depth + 1, viable, placed, fg, _size, att, lim, deadline) {
            return true;
        }
        fg.set(c, r, TileKind::Floor);
        placed.remove(&(c, r));
    }
    false
}

fn quick_viable(fg: &FastGrid) -> bool { fg.simulate_partial() }

fn tile_sort_key(k: &TileKind) -> (u8, usize, u8) {
    let pri = match k {
        TileKind::Teleport(..)|TileKind::TeleportBut(..) => 0, TileKind::Source(..) => 1,
        TileKind::Turn(..)|TileKind::TurnBut(..)|TileKind::Arrow(..)|TileKind::ArrowBut(..) => 2,
        TileKind::Bounce(..)|TileKind::BounceBut(..)|TileKind::ColorSwitch(..)|TileKind::ColorSwitchBut(..) => 3,
        TileKind::Switch|TileKind::Door(..)|TileKind::Painter(..) => 4, TileKind::Goal(..) => 5, _ => 6,
    };
    let (co,di) = match k {
        TileKind::Turn(c,d)|TileKind::TurnBut(c,d)|TileKind::Arrow(c,d)|TileKind::ArrowBut(c,d) => (*c, d.index() as u8),
        TileKind::Teleport(c,n)|TileKind::TeleportBut(c,n) => (*c+n*100, 0),
        TileKind::Bounce(c)|TileKind::BounceBut(c)|TileKind::Goal(c)|TileKind::Painter(c)
        |TileKind::ColorSwitch(c)|TileKind::ColorSwitchBut(c) => (*c, 0),
        TileKind::Source(c,d) => (*c, d.index() as u8), _ => (0, 0),
    };
    (pri, co, di)
}

fn find_color_reachable(sz: u32, tiles: &[(u32,u32,TileKind)], fs: &HashSet<(u32,u32)>) -> HashMap<usize, HashSet<(u32,u32)>> {
    let fg = FastGrid::new(sz, tiles);
    let mut res: HashMap<usize, HashSet<(u32,u32)>> = HashMap::new();
    let mut bots: Vec<(NeedBot, usize)> = Vec::new();
    for r in 0..sz { for c in 0..sz {
        if let TileKind::Source(ci, dir) = fg.get_tile(c, r) {
            bots.push((NeedBot{col:c as i32,row:r as i32,dir,color:ci,alive:true,at_goal:false}, ci));
        }
    }}
    for _ in 0..GEN_MAX_SIM_STEPS {
        if bots.iter().all(|(b,_)| b.at_goal||!b.alive) { break; }
        for (bot, oc) in bots.iter_mut() {
            if !bot.alive||bot.at_goal { continue; }
            if fs.contains(&(bot.col as u32,bot.row as u32)) { res.entry(*oc).or_default().insert((bot.col as u32,bot.row as u32)); }
            let (dc,dr) = bot.dir.grid_delta();
            let (nc,nr) = (bot.col+dc, bot.row+dr);
            if nc<0||nr<0||nc>=sz as i32||nr>=sz as i32 {
                bot.alive=false;
                for d in Direction::all() { let(dc2,dr2)=d.grid_delta();
                    let(nc2,nr2)=(bot.col as i32+dc2,bot.row as i32+dr2);
                    if nc2>=0&&nr2>=0&&nc2<sz as i32&&nr2<sz as i32 {
                        if fs.contains(&(nc2 as u32,nr2 as u32)) { res.entry(*oc).or_default().insert((nc2 as u32,nr2 as u32)); }
                    }
                }
                continue;
            }
            let t = fg.get_tile(nc as u32, nr as u32);
            match t {
                TileKind::Empty => { bot.alive=false; if fs.contains(&(nc as u32,nr as u32)) { res.entry(*oc).or_default().insert((nc as u32,nr as u32)); } continue; }
                TileKind::Door(false) => { bot.dir=bot.dir.opposite(); continue; }
                _ => { bot.col=nc; bot.row=nr; if fs.contains(&(nc as u32,nr as u32)) { res.entry(*oc).or_default().insert((nc as u32,nr as u32)); }
                    match t {
                        TileKind::Goal(ci) if ci==bot.color => {bot.at_goal=true;}
                        TileKind::Painter(ci) => {bot.color=ci;}
                        TileKind::Turn(ci,td) if ci==bot.color => {if let Some(e)=bot.dir.turn_exit(td){bot.dir=e;}}
                        TileKind::TurnBut(ci,td) if ci!=bot.color => {if let Some(e)=bot.dir.turn_exit(td){bot.dir=e;}}
                        TileKind::Bounce(ci) if ci==bot.color => {bot.dir=bot.dir.opposite();}
                        TileKind::BounceBut(ci) if ci!=bot.color => {bot.dir=bot.dir.opposite();}
                        TileKind::Arrow(ci,d) if ci==bot.color => {bot.dir=d;}
                        TileKind::ArrowBut(ci,d) if ci!=bot.color => {bot.dir=d;}
                        _ => {}
                    }
                }
            }
        }
    }
    // Flood-fill through all non-empty cells, then filter to floors
    for reach in res.values_mut() {
        let mut exp = reach.clone();
        for _ in 0..8 {
            let snap: Vec<_> = exp.iter().copied().collect();
            let n0 = exp.len();
            for (c,r) in snap { for d in Direction::all() { let(dc,dr)=d.grid_delta();
                let(nc,nr)=(c as i32+dc,r as i32+dr);
                if nc>=0&&nr>=0&&nc<sz as i32&&nr<sz as i32 && !matches!(fg.get_tile(nc as u32,nr as u32),TileKind::Empty) { exp.insert((nc as u32,nr as u32)); }
            }}
            if exp.len()==n0 { break; }
        }
        *reach = exp.into_iter().filter(|p| fs.contains(p)).collect();
    }
    res
}
