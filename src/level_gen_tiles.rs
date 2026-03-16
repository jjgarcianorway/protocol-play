// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(dead_code)]
use rand::Rng;
use std::collections::{HashMap, HashSet};
use crate::constants::*;
use crate::types::*;
use crate::level_gen_sim::simulate_headless;
// === Helpers ===
pub fn is_floor(grid: &HashMap<(u32, u32), TileKind>, pos: (u32, u32)) -> bool {
    matches!(grid.get(&pos), Some(TileKind::Floor))
}
pub fn in_bounds(c: i32, r: i32, size: u32) -> bool {
    c >= 0 && r >= 0 && c < size as i32 && r < size as i32
}
pub fn try_exit(grid: &HashMap<(u32, u32), TileKind>, col: i32, row: i32, dir: Direction, size: u32) -> bool {
    let (dc, dr) = dir.grid_delta();
    in_bounds(col + dc, row + dr, size) && !grid.contains_key(&((col + dc) as u32, (row + dr) as u32))
}
pub fn shuffle<T>(v: &mut Vec<T>, rng: &mut impl Rng) {
    for i in (1..v.len()).rev() { let j = rng.gen_range(0..=i); v.swap(i, j); }
}
fn but_color(ci: usize) -> usize { (ci + 3) % NUM_COLORS } // offset by 3 to pick a plausible other-bot color
pub fn possible_turns(bot_dir: Direction) -> Vec<(Direction, Direction)> {
    Direction::all().iter().filter_map(|&td| bot_dir.turn_exit(td).map(|exit| (td, exit))).collect()
}

// Source placement: pick empty cell with valid direction, preferring cells far from existing sources.
pub fn pick_start(
    size: u32, rng: &mut impl Rng, grid: &HashMap<(u32, u32), TileKind>,
) -> Option<(u32, u32, Direction)> {
    let empty: Vec<_> = (0..size).flat_map(|r| (0..size).map(move |c| (c, r)))
        .filter(|p| !grid.contains_key(p)).collect();
    if empty.is_empty() { return None; }
    let sources: Vec<(u32, u32)> = grid.iter()
        .filter(|(_, k)| matches!(k, TileKind::Source(..)))
        .map(|(&p, _)| p).collect();
    let max_tries = if sources.is_empty() { 60 } else { 300 };
    let mut best: Option<(u32, u32, Direction, u32)> = None;
    for _ in 0..max_tries {
        let (c, r) = empty[rng.gen_range(0..empty.len())];
        let mut dirs = Direction::all().to_vec();
        shuffle(&mut dirs, rng);
        // Prefer directions pointing toward the board center
        let (cx, cy) = (size as f32 / 2.0 - c as f32, size as f32 / 2.0 - r as f32);
        dirs.sort_by(|a, b| {
            let (da, db) = (a.grid_delta(), b.grid_delta());
            let dot_a = da.0 as f32 * cx + da.1 as f32 * cy;
            let dot_b = db.0 as f32 * cx + db.1 as f32 * cy;
            dot_b.partial_cmp(&dot_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        // Add some randomness: 40% chance to swap top two choices
        if dirs.len() >= 2 && rng.gen_bool(0.4) { dirs.swap(0, 1); }
        for d in &dirs {
            let (dc, dr) = d.grid_delta();
            let (nc, nr) = (c as i32 + dc, r as i32 + dr);
            if in_bounds(nc, nr, size) && !grid.contains_key(&(nc as u32, nr as u32)) {
                if sources.is_empty() { return Some((c, r, *d)); }
                let min_dist = sources.iter()
                    .map(|&(sc, sr)| (c as i32 - sc as i32).unsigned_abs()
                        + (r as i32 - sr as i32).unsigned_abs())
                    .min().unwrap_or(0);
                if best.as_ref().map_or(true, |b| min_dist > b.3) {
                    best = Some((c, r, *d, min_dist));
                }
                break;
            }
        }
    }
    best.map(|(c, r, d, _)| (c, r, d))
}
// Goal placement with look-back: try current pos, then scan path history
pub fn try_place_goal_lookback(
    grid: &mut HashMap<(u32, u32), TileKind>, col: i32, row: i32, ci: usize,
    path_history: &[(i32, i32, Direction)],
) -> bool {
    let pos = (col as u32, row as u32);
    if is_floor(grid, pos) { grid.insert(pos, TileKind::Goal(ci)); return true; }
    for i in (0..path_history.len().saturating_sub(1)).rev().take(3) {
        let (hc, hr, _) = path_history[i];
        let hp = (hc as u32, hr as u32);
        if is_floor(grid, hp) { grid.insert(hp, TileKind::Goal(ci)); return true; }
    }
    false
}
// Backtrack-and-redirect: scan back for a Floor tile to place a Turn/Arrow
pub fn try_backtrack_redirect(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: &mut i32, row: &mut i32, dir: &mut Direction, size: u32, rng: &mut impl Rng,
    ci: usize, path_history: &[(i32, i32, Direction)],
    weights: &[u32; GEN_NUM_WEIGHTS],
) -> bool {
    for i in (1..path_history.len().saturating_sub(1)).rev().take(4) {
        let (hc, hr, hd) = path_history[i];
        if !is_floor(grid, (hc as u32, hr as u32)) { continue; }
        if weights[0] > 0 || weights[1] > 0 {
            let but = weights[1] > 0 && (weights[0] == 0 || rng.gen_bool(weights[1] as f64 / (weights[0] + weights[1]) as f64));
            if try_turn_at(grid, solution, hc, hr, dir, size, rng, ci, but) {
                *col = hc; *row = hr;
                return true;
            }
        }
        if weights[2] > 0 || weights[3] > 0 {
            let but = weights[3] > 0 && (weights[2] == 0 || rng.gen_bool(weights[3] as f64 / (weights[2] + weights[3]) as f64));
            let mut test_dir = hd;
            if try_arrow_at(grid, solution, hc, hr, &mut test_dir, size, rng, ci, but) {
                *col = hc; *row = hr; *dir = test_dir;
                return true;
            }
        }
    }
    false
}
// === Turn ===
fn try_turn_at(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: i32, row: i32, dir: &mut Direction, size: u32, rng: &mut impl Rng,
    ci: usize, but: bool,
) -> bool {
    let pos = (col as u32, row as u32);
    if !is_floor(grid, pos) { return false; }
    let mut opts = possible_turns(*dir);
    shuffle(&mut opts, rng);
    for (td, ed) in opts {
        if try_exit(grid, col, row, ed, size) {
            let c = if but { but_color(ci) } else { ci };
            grid.insert(pos, if but { TileKind::TurnBut(c, td) } else { TileKind::Turn(c, td) });
            solution.insert(pos);
            *dir = ed;
            return true;
        }
    }
    false
}
// === Arrow ===
fn try_arrow_at(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: i32, row: i32, dir: &mut Direction, size: u32, rng: &mut impl Rng,
    ci: usize, but: bool,
) -> bool {
    let pos = (col as u32, row as u32);
    if !is_floor(grid, pos) { return false; }
    let mut dirs: Vec<Direction> = Direction::all().iter().copied()
        .filter(|d| *d != *dir).collect();
    shuffle(&mut dirs, rng);
    for new_dir in dirs {
        if try_exit(grid, col, row, new_dir, size) {
            let c = if but { but_color(ci) } else { ci };
            grid.insert(pos, if but { TileKind::ArrowBut(c, new_dir) } else { TileKind::Arrow(c, new_dir) });
            solution.insert(pos);
            *dir = new_dir;
            return true;
        }
    }
    false
}

// === Teleport — min 2 Manhattan distance, prefer far destinations ===
fn try_teleport_at(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: &mut i32, row: &mut i32, dir: &Direction, size: u32, rng: &mut impl Rng,
    teleport_num: &mut usize, ci: usize, but: bool,
) -> bool {
    let pos = (*col as u32, *row as u32);
    if !is_floor(grid, pos) || *teleport_num >= NUM_TELEPORTS { return false; }
    let (dc, dr) = dir.grid_delta();
    let mut candidates: Vec<((u32, u32), u32)> = (0..size).flat_map(|r| (0..size).map(move |c| (c, r)))
        .filter(|&(c, r)| !grid.contains_key(&(c, r)) && (c, r) != pos
            && in_bounds(c as i32 + dc, r as i32 + dr, size)
            && !grid.contains_key(&((c as i32 + dc) as u32, (r as i32 + dr) as u32)))
        .map(|(c, r)| {
            let dist = (c as i32 - *col).unsigned_abs() + (r as i32 - *row).unsigned_abs();
            ((c, r), dist)
        })
        .filter(|(_, dist)| *dist >= 2) // min 2 Manhattan distance
        .collect();
    if candidates.is_empty() { return false; }
    // Sort by distance descending, prefer far destinations
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    let top = (candidates.len() * 2 / 5).max(1);
    let (dest, _) = candidates[rng.gen_range(0..top)];
    let num = *teleport_num; *teleport_num += 1;
    let c = if but { but_color(ci) } else { ci };
    let mk = |n| if but { TileKind::TeleportBut(c, n) } else { TileKind::Teleport(c, n) };
    grid.insert(pos, mk(num));
    grid.insert(dest, mk(num));
    solution.insert(pos);
    solution.insert(dest);
    *col = dest.0 as i32; *row = dest.1 as i32;
    true
}

// === Bounce ===
fn try_bounce_full(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: &mut i32, row: &mut i32, dir: &mut Direction, size: u32, rng: &mut impl Rng,
    ci: usize, but: bool, path_history: &[(i32, i32, Direction)],
) -> bool {
    let pos = (*col as u32, *row as u32);
    if !is_floor(grid, pos) || path_history.len() < 3 { return false; }
    let ret_dir = dir.opposite();
    let mut candidates: Vec<(usize, Direction, Direction)> = Vec::new();
    for i in (1..path_history.len().saturating_sub(1)).rev() {
        let (hc, hr, _) = path_history[i];
        if !is_floor(grid, (hc as u32, hr as u32)) { continue; }
        for &(td, ed) in &possible_turns(ret_dir) {
            if try_exit(grid, hc, hr, ed, size) {
                candidates.push((i, td, ed));
            }
        }
        if candidates.len() >= 4 { break; }
    }
    if candidates.is_empty() { return false; }
    let &(idx, td, ed) = &candidates[rng.gen_range(0..candidates.len())];
    let (hc, hr, _) = path_history[idx];
    let bc = if but { but_color(ci) } else { ci };
    grid.insert(pos, if but { TileKind::BounceBut(bc) } else { TileKind::Bounce(bc) });
    solution.insert(pos);
    grid.insert((hc as u32, hr as u32), TileKind::Turn(ci, td));
    solution.insert((hc as u32, hr as u32));
    *col = hc; *row = hr; *dir = ed;
    true
}

// === Switch + Door ===
fn try_switch_door_at(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: &mut i32, row: &mut i32, dir: &Direction, size: u32, rng: &mut impl Rng,
    _ci: usize, is_color: bool, ci_for_color: usize, but: bool,
) -> bool {
    let pos = (*col as u32, *row as u32);
    if !is_floor(grid, pos) { return false; }
    let (dc, dr) = dir.grid_delta();
    let mut max_dist = 0u32;
    for d in 1..=4 {
        let (nc, nr) = (*col + dc * d as i32, *row + dr * d as i32);
        if !in_bounds(nc, nr, size) || grid.contains_key(&(nc as u32, nr as u32)) { break; }
        max_dist = d;
    }
    if max_dist < 2 { return false; }
    let door_dist = rng.gen_range(2..=max_dist);
    let switch = if is_color {
        let c = if but { but_color(ci_for_color) } else { ci_for_color };
        if but { TileKind::ColorSwitchBut(c) } else { TileKind::ColorSwitch(c) }
    } else { TileKind::Switch };
    grid.insert(pos, switch);
    solution.insert(pos);
    for d in 1..door_dist {
        let (nc, nr) = (*col + dc * d as i32, *row + dr * d as i32);
        grid.insert((nc as u32, nr as u32), TileKind::Floor);
    }
    let (dc_pos, dr_pos) = (*col + dc * door_dist as i32, *row + dr * door_dist as i32);
    grid.insert((dc_pos as u32, dr_pos as u32), TileKind::Door(false));
    *col = dc_pos; *row = dr_pos;
    true
}

// === Painter ===
fn try_painter_at(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: i32, row: i32, dir: &Direction, size: u32,
    current_color: &mut usize, rng: &mut impl Rng,
) -> bool {
    let pos = (col as u32, row as u32);
    if !is_floor(grid, pos) { return false; }
    if !try_exit(grid, col, row, *dir, size) { return false; }
    let new_color = loop {
        let c = rng.gen_range(0..NUM_COLORS);
        if c != *current_color { break c; }
    };
    grid.insert(pos, TileKind::Painter(new_color));
    solution.insert(pos);
    *current_color = new_color;
    true
}

// === Confusion tiles: red herrings for inventory ===
pub fn add_confusion_tiles(
    tiles: &mut Vec<(u32, u32, TileKind, bool)>, size: u32, rng: &mut impl Rng,
) {
    let sol_kinds: Vec<TileKind> = tiles.iter().filter(|t| t.3).map(|t| t.2).collect();
    if sol_kinds.is_empty() { return; }
    // Prefer floor tiles adjacent to non-floor (path) tiles — creates plausible false paths
    let non_floor: HashSet<(u32, u32)> = tiles.iter()
        .filter(|t| !matches!(t.2, TileKind::Floor | TileKind::Empty)).map(|t| (t.0, t.1)).collect();
    let floor_idxs: Vec<usize> = tiles.iter().enumerate()
        .filter(|(_, t)| !t.3 && matches!(t.2, TileKind::Floor)).map(|(i, _)| i).collect();
    if floor_idxs.is_empty() { return; }
    // Score floors by adjacency to path: adjacent tiles first, then random
    let adj_idxs: Vec<usize> = floor_idxs.iter().copied().filter(|&i| {
        let (c, r) = (tiles[i].0 as i32, tiles[i].1 as i32);
        [(0,1),(0,-1),(1,0),(-1,0)].iter()
            .any(|&(dc, dr)| non_floor.contains(&((c+dc) as u32, (r+dr) as u32)))
    }).collect();
    let all: Vec<_> = tiles.iter().map(|t| (t.0, t.1, t.2)).collect();
    let max_conf = (sol_kinds.len() as u32 / 2).max(2).min(6);
    let count = rng.gen_range(2..=max_conf).min(floor_idxs.len() as u32);
    let mut used: HashSet<usize> = HashSet::new();
    for _ in 0..count * 4 {
        if used.len() >= count as usize { break; }
        let template = sol_kinds[rng.gen_range(0..sol_kinds.len())];
        let Some(ct) = confusion_variant(template, rng) else { continue };
        // Pick from adjacent tiles 80% of the time, fall back to any floor
        let pool: &[usize] = if !adj_idxs.is_empty() && rng.gen_bool(0.8) { &adj_idxs } else { &floor_idxs };
        let avail: Vec<_> = pool.iter().filter(|i| !used.contains(i)).copied().collect();
        if avail.is_empty() { break; }
        let idx = avail[rng.gen_range(0..avail.len())];
        let mut test_full = all.clone();
        test_full[idx].2 = ct;
        if !simulate_headless(size, &test_full) { continue; }
        let mut test_strip: Vec<_> = tiles.iter()
            .map(|t| (t.0, t.1, if t.3 { TileKind::Floor } else { t.2 })).collect();
        test_strip[idx].2 = ct;
        if simulate_headless(size, &test_strip) { continue; }
        tiles[idx].2 = ct;
        tiles[idx].3 = true;
        used.insert(idx);
    }
}

fn rot_dir(d: Direction, rng: &mut impl Rng) -> Direction {
    let a = Direction::all();
    loop { let nd = a[rng.gen_range(0..4)]; if nd != d { return nd; } }
}

fn confusion_variant(k: TileKind, rng: &mut impl Rng) -> Option<TileKind> {
    let mut sc = |c: usize| (c + 1 + rng.gen_range(0..NUM_COLORS - 1)) % NUM_COLORS;
    match k {
        TileKind::Turn(c, d) => Some(TileKind::Turn(c, rot_dir(d, rng))),
        TileKind::TurnBut(c, d) => Some(TileKind::TurnBut(c, rot_dir(d, rng))),
        TileKind::Arrow(c, d) => Some(TileKind::Arrow(c, rot_dir(d, rng))),
        TileKind::ArrowBut(c, d) => Some(TileKind::ArrowBut(c, rot_dir(d, rng))),
        TileKind::Bounce(c) => Some(TileKind::Bounce(sc(c))),
        TileKind::BounceBut(c) => Some(TileKind::BounceBut(sc(c))),
        TileKind::ColorSwitch(c) => Some(TileKind::ColorSwitch(sc(c))),
        TileKind::ColorSwitchBut(c) => Some(TileKind::ColorSwitchBut(sc(c))),
        TileKind::Painter(c) => Some(TileKind::Painter(sc(c))),
        _ => None,
    }
}

// === Weighted random selection ===
fn pick_weighted(weights: &[u32; GEN_NUM_WEIGHTS], indices: &[u8], rng: &mut impl Rng) -> Option<u8> {
    let total: u32 = indices.iter().map(|&i| weights[i as usize]).sum();
    if total == 0 { return None; }
    let r = rng.gen_range(0..total);
    let mut acc = 0u32;
    for &i in indices {
        acc += weights[i as usize];
        if r < acc { return Some(i); }
    }
    indices.last().copied()
}

// === Master dispatcher ===
pub fn try_mechanic(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    col: &mut i32, row: &mut i32, dir: &mut Direction, size: u32, rng: &mut impl Rng,
    weights: &[u32; GEN_NUM_WEIGHTS], teleport_num: &mut usize, ci: usize,
    current_color: &mut usize, path_history: &[(i32, i32, Direction)],
    need_dir_change: bool,
) -> bool {
    let dir_indices: Vec<u8> = (0..8u8).filter(|&i| weights[i as usize] > 0).collect();
    let pass_indices: Vec<u8> = (8..12u8).filter(|&i| weights[i as usize] > 0).collect();

    let pool: Vec<u8> = if need_dir_change {
        dir_indices
    } else {
        let mut all = dir_indices;
        all.extend_from_slice(&pass_indices);
        all
    };
    if pool.is_empty() { return false; }

    let mut tried: Vec<u8> = Vec::new();
    for _ in 0..GEN_MAX_MECHANIC_PICKS {
        let remaining: Vec<u8> = pool.iter().filter(|i| !tried.contains(i)).copied().collect();
        if remaining.is_empty() { break; }
        let pick = match pick_weighted(weights, &remaining, rng) {
            Some(p) => p,
            None => break,
        };
        tried.push(pick);
        let ok = match pick {
            0 => try_turn_at(grid, solution, *col, *row, dir, size, rng, ci, false),
            1 => try_turn_at(grid, solution, *col, *row, dir, size, rng, ci, true),
            2 => try_arrow_at(grid, solution, *col, *row, dir, size, rng, ci, false),
            3 => try_arrow_at(grid, solution, *col, *row, dir, size, rng, ci, true),
            4 => try_teleport_at(grid, solution, col, row, dir, size, rng, teleport_num, ci, false),
            5 => try_teleport_at(grid, solution, col, row, dir, size, rng, teleport_num, ci, true),
            6 => try_bounce_full(grid, solution, col, row, dir, size, rng, ci, false, path_history),
            7 => try_bounce_full(grid, solution, col, row, dir, size, rng, ci, true, path_history),
            8 => try_switch_door_at(grid, solution, col, row, dir, size, rng, ci, false, 0, false),
            9 => try_switch_door_at(grid, solution, col, row, dir, size, rng, ci, true, *current_color, false),
            10 => try_switch_door_at(grid, solution, col, row, dir, size, rng, ci, true, *current_color, true),
            11 => try_painter_at(grid, solution, *col, *row, dir, size, current_color, rng),
            _ => false,
        };
        if ok { return true; }
    }
    false
}
