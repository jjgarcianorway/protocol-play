// SPDX-License-Identifier: GPL-3.0-or-later
#![allow(dead_code)]
use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use crate::constants::*;
use crate::types::*;
use crate::level_gen_sim::simulate_headless;
use crate::level_gen_tiles::*;
// === Configuration ===
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HolePlacement { Edges, Middle, Both }

impl HolePlacement {
    pub fn label(self) -> &'static str {
        match self { Self::Edges => "Edges", Self::Middle => "Middle", Self::Both => "Both" }
    }
    pub fn next(self) -> Self {
        match self { Self::Edges => Self::Middle, Self::Middle => Self::Both, Self::Both => Self::Edges }
    }
    pub fn prev(self) -> Self {
        match self { Self::Edges => Self::Both, Self::Middle => Self::Edges, Self::Both => Self::Middle }
    }
    pub fn is_eligible(self, col: u32, row: u32, size: u32) -> bool {
        let on_edge = col == 0 || row == 0 || col == size - 1 || row == size - 1;
        match self { Self::Edges => on_edge, Self::Middle => !on_edge, Self::Both => true }
    }
}

#[derive(Clone)]
pub struct GenConfig {
    pub board_size: u32,
    pub num_bots: u32,
    pub hole_percent: u32,
    pub hole_placement: HolePlacement,
    pub difficulty: u32,
    pub weights: [u32; GEN_NUM_WEIGHTS],
    pub unique_solution: bool,
    pub inventory_target: u32,
    pub door_chains: u32,
    pub path_sharing: bool,
    pub confusion_tiles: bool,
    #[allow(dead_code)] pub required_tile: Option<fn(&TileKind) -> bool>,
}
// === Generator state ===
pub enum GenPhase {
    Idle,
    Running { attempt: usize, config: GenConfig, seed: u64,
        best: Option<(Vec<(u32, u32, TileKind, bool)>, u32)> },
    Done(Vec<(u32, u32, TileKind, bool)>, u32, u64), // tiles, rating, seed
    Failed,
}

#[derive(Resource)]
pub struct GeneratorState { pub phase: GenPhase }

impl Default for GeneratorState {
    fn default() -> Self { Self { phase: GenPhase::Idle } }
}
// === Core generation ===
pub fn generate_attempt(config: &GenConfig, rng: &mut impl Rng) -> Option<(Vec<(u32, u32, TileKind, bool)>, u32)> {
    let size = config.board_size;
    let diff = config.difficulty as f32 / 100.0;
    let mut grid: HashMap<(u32, u32), TileKind> = HashMap::new();
    let mut solution_positions: HashSet<(u32, u32)> = HashSet::new();
    let mut bot_floor_paths: Vec<Vec<(u32, u32)>> = Vec::new();
    let total_cells = (size * size) as usize;

    // For 3+ bots, auto-enable path sharing; scale paths to balance density vs generation
    let sharing = config.path_sharing || config.num_bots >= 3;
    let nb = config.num_bots as f32;
    let bot_scale = if config.num_bots <= 2 { 1.0 } else { (2.5 / nb).max(0.4) };
    let cell_budget = total_cells * 2 / (config.num_bots.max(2) as usize + 1);

    // Spread colors apart — skip enough to avoid similar pairs (e.g. orange/gold)
    let ideal_step = NUM_COLORS / config.num_bots.max(1) as usize;
    let color_step = if config.num_bots <= 3 { ideal_step.max(3) } else { ideal_step.max(2) };
    let color_offset: usize = rng.gen_range(0..NUM_COLORS);
    for bot_idx in 0..config.num_bots {
        let ci = (bot_idx as usize * color_step + color_offset) % NUM_COLORS;
        let (sc, sr, sd) = pick_start(size, rng, &grid)?;
        grid.insert((sc, sr), TileKind::Source(ci, sd));

        let mut col = sc as i32;
        let mut row = sr as i32;
        let mut dir = sd;
        let mut current_color = ci;
        let base_min = GEN_MIN_PATH_LENGTH.max((size as f32 * (0.6 + diff * 1.0)) as usize);
        let min_len = ((base_min as f32 * bot_scale) as usize).max(GEN_MIN_PATH_LENGTH);
        let max_len = cell_budget.max(min_len + 2);
        let target = rng.gen_range(min_len..=max_len.max(min_len));
        let (mut steps, mut turns) = (0, 0);
        let mut straight_run = 0u32;
        let mut goal_placed = false;
        let mut teleport_num = 0usize;
        let mut path_history: Vec<(i32, i32, Direction)> = vec![(sc as i32, sr as i32, sd)];
        let mut floor_path: Vec<(u32, u32)> = Vec::new();
        let base_chance = 0.15 + diff * 0.35;

        for _ in 0..(target + 15) {
            let (dc, dr) = dir.grid_delta();
            let (nc, nr) = (col + dc, row + dr);
            // Allow walking on existing Floor tiles (shared paths between bots)
            let next_tile = if in_bounds(nc, nr, size) { grid.get(&(nc as u32, nr as u32)).copied() } else { None };
            let can_advance = in_bounds(nc, nr, size)
                && (next_tile.is_none() || (sharing && next_tile == Some(TileKind::Floor)));

            if !can_advance || steps >= target {
                if steps >= min_len && turns > 0 {
                    if try_place_goal_lookback(&mut grid, col, row, current_color, &path_history) {
                        goal_placed = true; break;
                    }
                }
                if try_mechanic(&mut grid, &mut solution_positions,
                    &mut col, &mut row, &mut dir, size, rng, &config.weights, &mut teleport_num,
                    ci, &mut current_color, &path_history, true)
                { turns += 1; straight_run = 0; path_history.push((col, row, dir)); continue; }
                if try_backtrack_redirect(&mut grid, &mut solution_positions,
                    &mut col, &mut row, &mut dir, size, rng, ci, &path_history, &config.weights)
                { turns += 1; straight_run = 0; path_history.push((col, row, dir)); continue; }
                if steps >= min_len && turns > 0 {
                    if try_place_goal_lookback(&mut grid, col, row, current_color, &path_history) {
                        goal_placed = true; break;
                    }
                }
                return None;
            }

            col = nc; row = nr;
            if next_tile.is_none() {
                grid.insert((nc as u32, nr as u32), TileKind::Floor);
            }
            floor_path.push((nc as u32, nr as u32));
            steps += 1;
            straight_run += 1;
            path_history.push((col, row, dir));

            // Straight run range before placing a mechanic — based on board size + randomization
            let min_gap = 1 + (size / 3) as u32; // 2 (3x3) to 4 (12x12)
            let max_gap = min_gap + 1 + (diff * 2.0) as u32; // adds 1-3 based on difficulty
            let gap_target = rng.gen_range(min_gap..=max_gap);
            if steps > 1 && steps < target - 1 && straight_run >= gap_target {
                let straight_bonus = ((straight_run - gap_target) as f32 * 0.12).min(0.35);
                let near_edge = (col == 0 || col == size as i32 - 1 || row == 0 || row == size as i32 - 1) as u8 as f32 * 0.15;
                let near_center = {
                    let cx = (col as f32 - (size as f32 - 1.0) / 2.0).abs() / (size as f32 / 2.0);
                    let cy = (row as f32 - (size as f32 - 1.0) / 2.0).abs() / (size as f32 / 2.0);
                    if cx < 0.3 && cy < 0.3 { 0.1 } else { 0.0 }
                };
                let chance = (base_chance + straight_bonus + near_edge + near_center).min(0.85);
                if rng.gen_bool(chance as f64) {
                    if try_mechanic(&mut grid, &mut solution_positions,
                        &mut col, &mut row, &mut dir, size, rng, &config.weights, &mut teleport_num,
                        ci, &mut current_color, &path_history, false)
                    { turns += 1; straight_run = 0; path_history.push((col, row, dir)); }
                }
            }
        }

        if !goal_placed
            && !try_place_goal_lookback(&mut grid, col, row, current_color, &path_history)
        { return None; }
        if turns == 0 { return None; }
        bot_floor_paths.push(floor_path);
    }

    // Reject if paths cover less than 15% of the board (too concentrated)
    let total_floor: usize = bot_floor_paths.iter().map(|p| p.len()).sum();
    let coverage = total_floor as f32 / (size * size) as f32;
    if coverage < 0.25 && size >= 5 { return None; }

    // === Door chain interactions ===
    if config.door_chains > 0 && config.weights[8] > 0 {
        place_door_chains(&mut grid, &mut solution_positions, &bot_floor_paths, rng, config.door_chains as usize);
    }

    // Purposeful hole placement — score cells by edge proximity and path distance
    let empties: Vec<(u32, u32)> = (0..size).flat_map(|r| (0..size).map(move |c| (c, r)))
        .filter(|p| !grid.contains_key(p)).collect();
    if config.hole_percent > 0 && !empties.is_empty() {
        let path_cells: HashSet<(u32, u32)> = grid.keys().copied().collect();
        let mut scored: Vec<(u32, u32, f32)> = empties.iter().map(|&(c, r)| {
            if !config.hole_placement.is_eligible(c, r, size) { return (c, r, -1.0); }
            let edge_dist = (c.min(size - 1 - c).min(r).min(size - 1 - r)) as f32;
            let min_path_dist = path_cells.iter()
                .map(|&(pc, pr)| (c as i32 - pc as i32).unsigned_abs() + (r as i32 - pr as i32).unsigned_abs())
                .min().unwrap_or(size) as f32;
            (c, r, (1.0 / (edge_dist + 1.0)) * 0.3 + (min_path_dist / size as f32) * 0.7)
        }).collect();
        scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        let target_holes = (empties.len() as u32 * config.hole_percent / 100) as usize;
        let mut holes_placed = 0;
        for &(c, r, score) in &scored {
            if holes_placed >= target_holes { break; }
            if score < 0.0 { continue; }
            let threshold = 0.3 + score * 0.5;
            if rng.gen_bool((threshold as f64).clamp(0.1, 0.9)) {
                grid.insert((c, r), TileKind::Empty); holes_placed += 1;
            } else { grid.insert((c, r), TileKind::Floor); }
        }
    }
    for r in 0..size { for c in 0..size {
        if !grid.contains_key(&(c, r)) { grid.insert((c, r), TileKind::Floor); }
    }}

    let mut tiles: Vec<(u32, u32, TileKind, bool)> = grid.iter()
        .map(|(&(c, r), &k)| (c, r, k, solution_positions.contains(&(c, r))))
        .collect();

    let all: Vec<_> = tiles.iter().map(|(c, r, k, _)| (*c, *r, *k)).collect();
    if !simulate_headless(size, &all) { return None; } // must succeed with all tiles
    let mut without: Vec<_> = tiles.iter()
        .filter(|(_, _, _, sol)| !sol).map(|(c, r, k, _)| (*c, *r, *k)).collect();
    for &(c, r) in &solution_positions { without.push((c, r, TileKind::Floor)); }
    if simulate_headless(size, &without) { return None; } // must fail overall

    // Strip solution tiles down to inventory_target (not to absolute minimum).
    // This keeps enough tiles for the player to have a satisfying puzzle.
    {
        let tgt = if config.inventory_target > 0 { config.inventory_target as usize } else { 1 };
        let mut sol_vec: Vec<(u32, u32)> = solution_positions.iter().copied().collect();
        shuffle(&mut sol_vec, rng);
        let mut essential: HashSet<(u32, u32)> = solution_positions.clone();
        for &pos in &sol_vec {
            if essential.len() <= tgt { break; } // keep at least target tiles
            essential.remove(&pos);
            let test: Vec<_> = tiles.iter().map(|(c, r, k, _)| {
                if essential.contains(&(*c, *r)) { (*c, *r, TileKind::Floor) } else { (*c, *r, *k) }
            }).collect();
            if simulate_headless(size, &test) { essential.insert(pos); } // still needed
        }
        for t in &mut tiles { t.3 = essential.contains(&(t.0, t.1)); }
    }

    if config.unique_solution {
        let inv: Vec<(u32, u32)> = tiles.iter()
            .filter(|(_, _, _, s)| *s).map(|(c, r, _, _)| (*c, *r)).collect();
        if !inv.is_empty() {
            let floors: Vec<(u32, u32)> = tiles.iter()
                .filter(|(_, _, k, sol)| !sol && matches!(k, TileKind::Floor))
                .map(|(c, r, _, _)| (*c, *r)).collect();
            let checks = (50 * inv.len()).min(500);
            for _ in 0..checks {
                if floors.is_empty() { break; }
                let sp = inv[rng.gen_range(0..inv.len())];
                let ep = floors[rng.gen_range(0..floors.len())];
                let sk = *grid.get(&sp).unwrap();
                let mut modified = all.clone();
                for t in &mut modified {
                    if t.0 == sp.0 && t.1 == sp.1 { t.2 = TileKind::Floor; }
                    if t.0 == ep.0 && t.1 == ep.1 { t.2 = sk; }
                }
                if simulate_headless(size, &modified) { return None; }
            }
        }
    }

    // Reject if ANY bot is pre-solved after stripping
    {
        let stripped: Vec<_> = tiles.iter().map(|(c, r, k, sol)| {
            (*c, *r, if *sol { TileKind::Floor } else { *k })
        }).collect();
        // Quick check: test each bot individually only if there are 2+ bots
        let sources: Vec<(u32, u32, usize)> = stripped.iter()
            .filter_map(|(c, r, k)| if let TileKind::Source(ci, _) = k { Some((*c, *r, *ci)) } else { None })
            .collect();
        if sources.len() > 1 {
            for &(sc, sr, sci) in &sources {
                // Only check bots near their goal (within manhattan distance of board_size/2)
                let goal_near = stripped.iter().any(|(c, r, k)| {
                    if let TileKind::Goal(gi) = k { *gi == sci && ((*c as i32 - sc as i32).unsigned_abs() + (*r as i32 - sr as i32).unsigned_abs()) <= size / 2 }
                    else { false }
                });
                if !goal_near { continue; }
                let single: Vec<_> = stripped.iter().map(|(c, r, k)| {
                    if matches!(k, TileKind::Source(..)) && (*c != sc || *r != sr) {
                        (*c, *r, TileKind::Floor)
                    } else { (*c, *r, *k) }
                }).collect();
                if simulate_headless(size, &single) { return None; }
            }
        }
    }

    // Reject if required_tile is set but not in inventory (must be placeable, not baked in)
    if let Some(req) = config.required_tile {
        let has_required_in_inv = tiles.iter().any(|(_, _, k, sol)| *sol && req(k));
        if !has_required_in_inv { return None; }
    }

    if config.confusion_tiles { add_confusion_tiles(&mut tiles, size, rng); }
    let rating = rate_difficulty(&tiles, config.num_bots, size);
    Some((tiles, rating))
}

// === Door chain placement — prefer path crossing points ===
fn place_door_chains(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    bot_paths: &[Vec<(u32, u32)>], rng: &mut impl Rng, chain_count: usize,
) {
    let nb = bot_paths.len();
    let path_sets: Vec<HashSet<(u32, u32)>> = bot_paths.iter()
        .map(|p| p.iter().copied().collect()).collect();
    for _ in 0..chain_count {
        if nb >= 2 {
            let a = rng.gen_range(0..nb);
            let b = (a + 1 + rng.gen_range(0..nb - 1)) % nb;
            // Find positions where bot paths are close (within 3 cells)
            let mut cross_pairs: Vec<((u32, u32), (u32, u32))> = Vec::new();
            for &sp in &path_sets[a] { for &dp in &path_sets[b] {
                let dist = (sp.0 as i32 - dp.0 as i32).unsigned_abs()
                    + (sp.1 as i32 - dp.1 as i32).unsigned_abs();
                if dist <= 3 && is_floor(grid, sp) && is_floor(grid, dp) && sp != dp {
                    cross_pairs.push((sp, dp));
                }
            }}
            if !cross_pairs.is_empty() {
                let &(sp, dp) = &cross_pairs[rng.gen_range(0..cross_pairs.len())];
                grid.insert(sp, TileKind::Switch); grid.insert(dp, TileKind::Door(false));
                solution.insert(sp);
            } else if try_place_switch_door(grid, solution, &bot_paths[a], &bot_paths[b], rng, true)
                && rng.gen_bool(0.3) {
                let e = rng.gen_range(0..nb);
                let end = (bot_paths[e].len() * 3 / 5).max(1);
                let fl: Vec<_> = bot_paths[e][..end].iter()
                    .filter(|p| is_floor(grid, **p)).copied().collect();
                if !fl.is_empty() {
                    let sp = fl[rng.gen_range(0..fl.len())];
                    grid.insert(sp, TileKind::Switch); solution.insert(sp);
                }
            }
        } else {
            let a = rng.gen_range(0..nb);
            try_place_switch_door(grid, solution, &bot_paths[a], &bot_paths[a], rng, false);
        }
    }
}

fn try_place_switch_door(
    grid: &mut HashMap<(u32, u32), TileKind>, solution: &mut HashSet<(u32, u32)>,
    sw_path: &[(u32, u32)], dr_path: &[(u32, u32)], rng: &mut impl Rng, cross: bool,
) -> bool {
    if sw_path.len() < 4 || dr_path.len() < 4 { return false; }
    let sw_end = (sw_path.len() * 2 / 5).max(1);
    let sf: Vec<_> = sw_path[..sw_end].iter().filter(|p| is_floor(grid, **p)).copied().collect();
    let dr_start = if cross { dr_path.len() / 2 } else { dr_path.len() * 3 / 5 };
    let df: Vec<_> = dr_path[dr_start..].iter().filter(|p| is_floor(grid, **p)).copied().collect();
    if sf.is_empty() || df.is_empty() { return false; }
    let sp = sf[rng.gen_range(0..sf.len())];
    let dp = df[rng.gen_range(0..df.len())];
    if sp == dp { return false; }
    grid.insert(sp, TileKind::Switch); grid.insert(dp, TileKind::Door(false));
    solution.insert(sp);
    true
}

// === Difficulty rating ===
fn rate_difficulty(tiles: &[(u32, u32, TileKind, bool)], num_bots: u32, board_size: u32) -> u32 {
    let (mut mechanics, mut type_set, mut has_but) = (0u32, 0u32, false);
    let (mut door_count, mut switch_count, mut inv_count) = (0u32, 0u32, 0u32);
    let cells = (board_size * board_size) as f32;
    for (_, _, k, sol) in tiles {
        if *sol { inv_count += 1; }
        match k {
            TileKind::Turn(..) => { mechanics += 1; type_set |= 1; }
            TileKind::TurnBut(..) => { mechanics += 1; type_set |= 1; has_but = true; }
            TileKind::Arrow(..) => { mechanics += 1; type_set |= 2; }
            TileKind::ArrowBut(..) => { mechanics += 1; type_set |= 2; has_but = true; }
            TileKind::Teleport(..) => { mechanics += 2; type_set |= 4; }
            TileKind::TeleportBut(..) => { mechanics += 2; type_set |= 4; has_but = true; }
            TileKind::Bounce(..) => { mechanics += 2; type_set |= 8; }
            TileKind::BounceBut(..) => { mechanics += 2; type_set |= 8; has_but = true; }
            TileKind::Switch => { mechanics += 2; type_set |= 16; switch_count += 1; }
            TileKind::Door(..) => { type_set |= 16; door_count += 1; }
            TileKind::ColorSwitch(..) => { mechanics += 2; type_set |= 32; }
            TileKind::ColorSwitchBut(..) => { mechanics += 2; type_set |= 32; has_but = true; }
            TileKind::Painter(..) => { mechanics += 2; type_set |= 64; }
            _ => {}
        }
    }
    // Path efficiency: levels where each solution tile matters more
    let sol_count = tiles.iter().filter(|(_, _, _, s)| *s).count() as f32;
    let path_tiles = tiles.iter().filter(|(_, _, k, _)| !matches!(k, TileKind::Floor | TileKind::Empty)).count() as f32;
    let efficiency_bonus = if sol_count > 0.0 && path_tiles > 0.0 {
        (path_tiles / sol_count / 3.0).min(1.0) * 5.0 } else { 0.0 };
    let diversity = type_set.count_ones();
    let density = (mechanics as f32 / cells * 150.0).min(25.0);
    // Door chains: multiple switch/door pairs = exponential complexity
    let chain_bonus = (switch_count.min(door_count) as f32 * 6.0).min(18.0);
    let score = density
        + diversity as f32 * 5.0
        + if has_but { 8.0 } else { 0.0 }
        + chain_bonus
        + ((num_bots - 1) as f32 * 8.0).min(20.0)
        + (mechanics as f32 * 1.0).min(10.0)
        + (inv_count as f32 * 0.5).min(7.0)
        + efficiency_bonus;
    (score as u32).min(100)
}

// === Generator system (best-of-N: picks closest to target difficulty) ===
pub fn update_generator(mut state: ResMut<GeneratorState>) {
    let phase = std::mem::replace(&mut state.phase, GenPhase::Idle);
    let GenPhase::Running { attempt: start, config, seed, mut best } = phase else {
        state.phase = phase; return;
    };
    let mut rng = rand::thread_rng();
    let mut current = start;
    for _ in 0..GEN_ATTEMPTS_PER_FRAME {
        current += 1;
        if current > GEN_MAX_ATTEMPTS {
            state.phase = match best {
                Some((t, r)) => GenPhase::Done(t, r, seed),
                None => GenPhase::Failed,
            };
            return;
        }
        if let Some((tiles, rating)) = generate_attempt(&config, &mut rng) {
            let gap = (rating as i32 - config.difficulty as i32).unsigned_abs();
            let prev_gap = best.as_ref()
                .map(|(_, r)| (*r as i32 - config.difficulty as i32).unsigned_abs())
                .unwrap_or(u32::MAX);
            if gap < prev_gap { best = Some((tiles, rating)); }
            if gap <= GEN_BEST_TOLERANCE {
                let (t, r) = best.unwrap();
                state.phase = GenPhase::Done(t, r, seed); return;
            }
        }
    }
    state.phase = GenPhase::Running { attempt: current, config, seed, best };
}
