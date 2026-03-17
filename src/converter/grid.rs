// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Fill the entire grid with random crystals, drawing from the pile.
pub fn fill_grid(grid: &mut GridState, pile: &mut CrystalPile) {
    let mut rng = rand::thread_rng();
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.cells[row][col].is_none() && pile.remaining > 0 {
                grid.cells[row][col] = Some(CrystalColor::from_index(rng.gen_range(0..5)));
                pile.remaining -= 1;
            }
        }
    }
}

/// Flood-fill to find all connected same-color crystals from (row, col).
pub fn flood_fill(grid: &GridState, row: usize, col: usize) -> Vec<(usize, usize)> {
    let color = match grid.cells[row][col] {
        Some(c) => c,
        None => return vec![],
    };
    let mut visited = vec![vec![false; grid.width]; grid.height];
    let mut stack = vec![(row, col)];
    let mut group = Vec::new();
    while let Some((r, c)) = stack.pop() {
        if r >= grid.height || c >= grid.width { continue; }
        if visited[r][c] { continue; }
        if grid.cells[r][c] != Some(color) { continue; }
        visited[r][c] = true;
        group.push((r, c));
        if r > 0 { stack.push((r - 1, c)); }
        if r + 1 < grid.height { stack.push((r + 1, c)); }
        if c > 0 { stack.push((r, c - 1)); }
        if c + 1 < grid.width { stack.push((r, c + 1)); }
    }
    group
}

/// Remove crystals at given positions, returns count removed.
pub fn remove_cells(grid: &mut GridState, cells: &[(usize, usize)]) -> u32 {
    let mut count = 0u32;
    for &(r, c) in cells {
        if grid.cells[r][c].is_some() {
            grid.cells[r][c] = None;
            count += 1;
        }
    }
    count
}

/// Apply gravity: crystals fall down to fill empty spaces.
/// Returns true if anything moved.
pub fn apply_gravity(grid: &mut GridState) -> bool {
    let mut moved = false;
    for col in 0..grid.width {
        // Compact column downward
        let mut write = grid.height;
        for row in (0..grid.height).rev() {
            if grid.cells[row][col].is_some() {
                write -= 1;
                if write != row {
                    grid.cells[write][col] = grid.cells[row][col];
                    grid.cells[row][col] = None;
                    moved = true;
                }
            }
        }
        // Remaining cells above are already None
        while write > 0 {
            write -= 1;
            grid.cells[write][col] = None;
        }
    }
    moved
}

/// Find all connected groups of 2+ same-color crystals.
/// Returns groups sorted by size (largest first).
pub fn find_all_groups(grid: &GridState) -> Vec<(CrystalColor, Vec<(usize, usize)>)> {
    let mut visited = vec![vec![false; grid.width]; grid.height];
    let mut groups = Vec::new();
    for row in 0..grid.height {
        for col in 0..grid.width {
            if visited[row][col] { continue; }
            let color = match grid.cells[row][col] {
                Some(c) => c,
                None => continue,
            };
            let group = flood_fill(grid, row, col);
            for &(r, c) in &group {
                visited[r][c] = true;
            }
            if group.len() >= 2 {
                groups.push((color, group));
            }
        }
    }
    groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    groups
}

/// Refill empty cells from the pile (top rows first).
/// Returns number of cells filled.
pub fn refill_from_pile(grid: &mut GridState, pile: &mut CrystalPile) -> u32 {
    let mut rng = rand::thread_rng();
    let mut filled = 0u32;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.cells[row][col].is_none() && pile.remaining > 0 {
                grid.cells[row][col] = Some(CrystalColor::from_index(rng.gen_range(0..5)));
                pile.remaining -= 1;
                filled += 1;
            }
        }
    }
    filled
}

/// Check if the grid is completely empty.
pub fn grid_is_empty(grid: &GridState) -> bool {
    grid.cells.iter().all(|row| row.iter().all(|c| c.is_none()))
}

/// Check if there are any crystals left on the grid.
pub fn grid_has_crystals(grid: &GridState) -> bool {
    grid.cells.iter().any(|row| row.iter().any(|c| c.is_some()))
}
