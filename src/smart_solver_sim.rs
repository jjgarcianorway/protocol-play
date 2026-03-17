// SPDX-License-Identifier: GPL-3.0-or-later
// Fast array-based simulation for the solver (no HashMap allocation per call).
#![allow(dead_code)]

use crate::types::*;
use crate::constants::*;

const MAX_SIZE: usize = 13;
const MAX_BOTS: usize = 16;
const MAX_TP: usize = 32;

#[derive(Clone, Copy)]
struct FastBot {
    col: i32, row: i32, dir: Direction, color: usize,
    alive: bool, at_goal: bool, switch_pending: bool,
}

/// Pre-built level grid for fast repeated simulation.
pub struct FastGrid {
    grid: [[TileKind; MAX_SIZE]; MAX_SIZE],
    size: u32,
    bots: [FastBot; MAX_BOTS],
    num_bots: usize,
    tp_pairs: [(u32, u32, u32, u32); MAX_TP], // (c1,r1,c2,r2)
    num_tp: usize,
}

impl FastGrid {
    pub fn new(size: u32, tiles: &[(u32, u32, TileKind)]) -> Self {
        let mut grid = [[TileKind::Empty; MAX_SIZE]; MAX_SIZE];
        let mut bots = [FastBot {
            col: 0, row: 0, dir: Direction::North, color: 0,
            alive: false, at_goal: false, switch_pending: false,
        }; MAX_BOTS];
        let mut num_bots = 0;
        for &(c, r, kind) in tiles {
            grid[r as usize][c as usize] = kind;
            if let TileKind::Source(ci, dir) = kind {
                if num_bots < MAX_BOTS {
                    bots[num_bots] = FastBot {
                        col: c as i32, row: r as i32, dir, color: ci,
                        alive: true, at_goal: false, switch_pending: false,
                    };
                    num_bots += 1;
                }
            }
        }
        // Build teleport pairs
        let mut tp_pairs = [(0u32, 0u32, 0u32, 0u32); MAX_TP];
        let mut num_tp = 0;
        for (i, &(c, r, k)) in tiles.iter().enumerate() {
            let (co, num, is_but) = match k {
                TileKind::Teleport(co, n) => (co, n, false),
                TileKind::TeleportBut(co, n) => (co, n, true),
                _ => continue,
            };
            for &(c2, r2, k2) in &tiles[i+1..] {
                let partner = if is_but {
                    matches!(k2, TileKind::TeleportBut(co2, n2) if co2 == co && n2 == num)
                } else {
                    matches!(k2, TileKind::Teleport(co2, n2) if co2 == co && n2 == num)
                };
                if partner && num_tp < MAX_TP {
                    tp_pairs[num_tp] = (c, r, c2, r2);
                    num_tp += 1;
                    break;
                }
            }
        }
        FastGrid { grid, size, bots, num_bots, tp_pairs, num_tp }
    }

    #[inline]
    pub fn set(&mut self, c: u32, r: u32, kind: TileKind) {
        self.grid[r as usize][c as usize] = kind;
    }

    #[inline]
    pub fn get_tile(&self, c: u32, r: u32) -> TileKind {
        self.grid[r as usize][c as usize]
    }


    /// Rebuild teleport pairs from current grid state.
    fn rebuild_tp_pairs(&self) -> ([(u32,u32,u32,u32); MAX_TP], usize) {
        let mut pairs = [(0u32,0u32,0u32,0u32); MAX_TP];
        let mut num = 0;
        let mut tps: Vec<(u32,u32,TileKind)> = Vec::new();
        for r in 0..self.size { for c in 0..self.size {
            let k = self.grid[r as usize][c as usize];
            if matches!(k, TileKind::Teleport(..) | TileKind::TeleportBut(..)) {
                tps.push((c, r, k));
            }
        }}
        for (i, &(c,r,k)) in tps.iter().enumerate() {
            let (co,n,ib) = match k { TileKind::Teleport(co,n)=>(co,n,false), TileKind::TeleportBut(co,n)=>(co,n,true), _=>continue };
            for &(c2,r2,k2) in &tps[i+1..] {
                let ok = if ib { matches!(k2,TileKind::TeleportBut(co2,n2) if co2==co&&n2==n) }
                    else { matches!(k2,TileKind::Teleport(co2,n2) if co2==co&&n2==n) };
                if ok && num < MAX_TP { pairs[num]=(c,r,c2,r2); num+=1; break; }
            }
        }
        (pairs, num)
    }

    /// Fast simulation without heap allocation.
    pub fn simulate(&self) -> bool {
        let mut bots = self.bots;
        let num = self.num_bots;
        if num == 0 { return false; }
        let mut grid_copy = self.grid;
        let has_doors = self.has_doors();
        // Rebuild teleport pairs from current grid state
        let (tp_pairs, num_tp) = self.rebuild_tp_pairs();

        for _ in 0..GEN_MAX_SIM_STEPS {
            if all_done(&bots, num) { break; }
            if has_doors && any_switch(&bots, num) {
                toggle_doors(&mut grid_copy, self.size);
                clear_switches(&mut bots, num);
            }
            for bi in 0..num {
                let bot = &mut bots[bi];
                if !bot.alive || bot.at_goal { continue; }
                let (dc, dr) = bot.dir.grid_delta();
                let (nc, nr) = (bot.col + dc, bot.row + dr);
                if nc < 0 || nr < 0 || nc >= self.size as i32 || nr >= self.size as i32 {
                    bot.alive = false; continue;
                }
                let tile = grid_copy[nr as usize][nc as usize];
                match tile {
                    TileKind::Empty => { bot.alive = false; continue; }
                    TileKind::Door(false) => { bot.dir = bot.dir.opposite(); continue; }
                    _ => { bot.col = nc; bot.row = nr; }
                }
                Self::apply_tile(bot, tile, nc as u32, nr as u32, &tp_pairs, num_tp);
            }
        }
        (0..num).all(|i| bots[i].at_goal)
    }

    pub fn simulate_partial(&self) -> bool {
        let mut bots = self.bots;
        let num = self.num_bots;
        if num == 0 { return false; }
        let grid_copy = self.grid;
        let (tp_pairs, num_tp) = self.rebuild_tp_pairs();
        for _ in 0..GEN_MAX_SIM_STEPS {
            if (0..num).all(|i| bots[i].at_goal || !bots[i].alive) { break; }
            for bi in 0..num {
                let bot = &mut bots[bi];
                if !bot.alive || bot.at_goal { continue; }
                let (dc, dr) = bot.dir.grid_delta();
                let (nc, nr) = (bot.col + dc, bot.row + dr);
                if nc < 0 || nr < 0 || nc >= self.size as i32 || nr >= self.size as i32 {
                    bot.alive = false; continue;
                }
                let tile = grid_copy[nr as usize][nc as usize];
                match tile {
                    TileKind::Empty => { bot.alive = false; continue; }
                    TileKind::Door(false) => { bot.dir = bot.dir.opposite(); continue; }
                    _ => { bot.col = nc; bot.row = nr; }
                }
                Self::apply_tile(bot, tile, nc as u32, nr as u32, &tp_pairs, num_tp);
            }
        }
        (0..num).any(|i| bots[i].at_goal)
    }

    /// Check if a tile is viable at position (c,r) based on board structure.
    pub fn tile_viable_at(&self, kind: TileKind, c: u32, r: u32) -> bool {
        match kind {
            TileKind::Turn(_, td) | TileKind::TurnBut(_, td) => {
                let (arm1, arm2) = match td {
                    Direction::North => (Direction::East, Direction::North),
                    Direction::East => (Direction::South, Direction::East),
                    Direction::South => (Direction::West, Direction::South),
                    Direction::West => (Direction::North, Direction::West),
                };
                self.has_nbr(c,r,arm1) && self.has_nbr(c,r,arm2)
            }
            TileKind::Arrow(_, d) | TileKind::ArrowBut(_, d) => {
                self.has_nbr(c,r,d) && self.has_any_nbr(c,r)
            }
            TileKind::Bounce(_) | TileKind::BounceBut(_) => self.has_any_nbr(c,r),
            _ => true,
        }
    }

    fn has_nbr(&self, c: u32, r: u32, dir: Direction) -> bool {
        let (dc,dr) = dir.grid_delta();
        let (nc,nr) = (c as i32+dc, r as i32+dr);
        if nc < 0 || nr < 0 || nc >= self.size as i32 || nr >= self.size as i32 { return false; }
        !matches!(self.grid[nr as usize][nc as usize], TileKind::Empty)
    }

    fn has_any_nbr(&self, c: u32, r: u32) -> bool {
        Direction::all().iter().any(|&d| self.has_nbr(c, r, d.opposite()))
    }

    fn has_doors(&self) -> bool {
        for r in 0..self.size as usize {
            for c in 0..self.size as usize {
                if matches!(self.grid[r][c], TileKind::Door(_)) { return true; }
            }
        }
        false
    }

    fn apply_tile(bot: &mut FastBot, tile: TileKind, nc: u32, nr: u32,
        tp_pairs: &[(u32,u32,u32,u32); MAX_TP], num_tp: usize) {
        match tile {
            TileKind::Goal(ci) if ci == bot.color => { bot.at_goal = true; }
            TileKind::Painter(ci) => { bot.color = ci; }
            TileKind::Turn(ci,td) if ci == bot.color => { if let Some(e)=bot.dir.turn_exit(td){bot.dir=e;} }
            TileKind::TurnBut(ci,td) if ci != bot.color => { if let Some(e)=bot.dir.turn_exit(td){bot.dir=e;} }
            TileKind::Bounce(ci) if ci == bot.color => { bot.dir = bot.dir.opposite(); }
            TileKind::BounceBut(ci) if ci != bot.color => { bot.dir = bot.dir.opposite(); }
            TileKind::Arrow(ci,d) if ci == bot.color => { bot.dir = d; }
            TileKind::ArrowBut(ci,d) if ci != bot.color => { bot.dir = d; }
            TileKind::Switch => { bot.switch_pending = true; }
            TileKind::ColorSwitch(ci) if ci == bot.color => { bot.switch_pending = true; }
            TileKind::ColorSwitchBut(ci) if ci != bot.color => { bot.switch_pending = true; }
            TileKind::Teleport(co,_) if co == NUM_COLORS || co == bot.color => {
                for i in 0..num_tp { let(c1,r1,c2,r2)=tp_pairs[i];
                    if c1==nc&&r1==nr { bot.col=c2 as i32; bot.row=r2 as i32; break; }
                    if c2==nc&&r2==nr { bot.col=c1 as i32; bot.row=r1 as i32; break; }
                }
            }
            TileKind::TeleportBut(co,_) if co != bot.color => {
                for i in 0..num_tp { let(c1,r1,c2,r2)=tp_pairs[i];
                    if c1==nc&&r1==nr { bot.col=c2 as i32; bot.row=r2 as i32; break; }
                    if c2==nc&&r2==nr { bot.col=c1 as i32; bot.row=r1 as i32; break; }
                }
            }
            _ => {}
        }
    }
}

#[inline]
fn all_done(bots: &[FastBot; MAX_BOTS], n: usize) -> bool {
    (0..n).all(|i| bots[i].at_goal || !bots[i].alive)
}

#[inline]
fn any_switch(bots: &[FastBot; MAX_BOTS], n: usize) -> bool {
    (0..n).any(|i| bots[i].switch_pending)
}

fn toggle_doors(grid: &mut [[TileKind; MAX_SIZE]; MAX_SIZE], size: u32) {
    for r in 0..size as usize {
        for c in 0..size as usize {
            if let TileKind::Door(open) = &mut grid[r][c] { *open = !*open; }
        }
    }
}

fn clear_switches(bots: &mut [FastBot; MAX_BOTS], n: usize) {
    for i in 0..n { bots[i].switch_pending = false; }
}
