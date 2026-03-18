// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use std::collections::HashMap;
use crate::constants::*;
use crate::types::*;
use crate::simulation::*;

fn formation_scale(n: usize) -> f32 { match n { 1 => 1.0, 2 => 0.62, 3 => 0.50, _ => 0.42 } }

fn formation_offsets(n: usize) -> &'static [f32] {
    match n { 2 => &[-0.25, 0.25], 3 => &[-0.28, 0.0, 0.28], _ => &[-0.28, -0.09, 0.09, 0.28] }
}

fn direction_axis(d: Direction) -> u8 { matches!(d, Direction::North | Direction::South) as u8 }

fn perp_offset(dir: Direction, slot: usize, n: usize) -> Vec2 {
    let t = formation_offsets(n).get(slot).copied().unwrap_or(0.0);
    match dir {
        Direction::North | Direction::South => Vec2::new(t, 0.0),
        Direction::East | Direction::West => Vec2::new(0.0, t),
    }
}

/// 2D grid offsets for bots with mixed travel directions on the same tile
fn grid_offset(slot: usize, n: usize) -> Vec2 {
    const G2: [Vec2; 2] = [Vec2::new(-0.20, -0.20), Vec2::new(0.20, 0.20)];
    const G3: [Vec2; 3] = [Vec2::new(-0.22, -0.15), Vec2::new(0.22, -0.15), Vec2::new(0.0, 0.22)];
    const G4: [Vec2; 4] = [Vec2::new(-0.20, -0.20), Vec2::new(0.20, -0.20),
                            Vec2::new(-0.20, 0.20), Vec2::new(0.20, 0.20)];
    match n {
        2 => G2.get(slot).copied().unwrap_or(Vec2::ZERO),
        3 => G3.get(slot).copied().unwrap_or(Vec2::ZERO),
        _ => G4.get(slot).copied().unwrap_or(Vec2::ZERO),
    }
}

fn is_special(phase: &BotPhase) -> bool {
    matches!(phase,
        BotPhase::TeleportShrink { .. } | BotPhase::TeleportGrow
        | BotPhase::Falling(_) | BotPhase::FallingDecel
        | BotPhase::FallingPause(_) | BotPhase::Stopped)
}

#[derive(Resource, Default)]
pub struct PrevTileCounts(pub HashMap<(i32, i32), usize>);

pub fn update_bot_formation(
    mut commands: Commands,
    play_mode: Res<PlayMode>,
    board_size: Res<BoardSize>,
    mut prev: ResMut<PrevTileCounts>,
    assets: Res<GameAssets>,
    mut bots: Query<(Entity, &BotMovement, &mut BotFormation), (With<Bot>, Without<DespawnAtZeroScale>)>,
) {
    if !matches!(*play_mode, PlayMode::Playing | PlayMode::TestPlaying) {
        for (_, _, mut f) in &mut bots { f.target_offset = Vec2::ZERO; f.target_scale = 1.0; }
        prev.0.clear();
        return;
    }

    // Collect immutable data first (release query borrow)
    let data: Vec<(Entity, i32, i32, usize, Direction, bool)> = bots.iter()
        .map(|(e, m, _)| (e, m.col, m.row, m.spawn_index, m.direction, is_special(&m.phase)))
        .collect();

    // Group bots by tile (only non-special bots)
    let mut groups: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (i, d) in data.iter().enumerate() {
        if !d.5 { groups.entry((d.1, d.2)).or_default().push(i); }
    }

    // Spawn merge flash when tile occupancy increases
    let half = (board_size.0 as f32 - 1.0) / 2.0;
    for (&(col, row), idxs) in &groups {
        let n = idxs.len();
        if n > 1 && n > prev.0.get(&(col, row)).copied().unwrap_or(0) {
            let pos = Vec3::new(col as f32 - half, FLOOR_TOP_Y + 0.006, row as f32 - half);
            commands.spawn((
                Mesh3d(assets.highlight_mesh.clone()),
                MeshMaterial3d(assets.flash_material.clone()),
                Transform::from_translation(pos).with_scale(Vec3::ZERO),
                TargetScale(Vec3::splat(1.3)), MergeFlash { progress: 0.0 },
            ));
        }
    }
    prev.0 = groups.iter().map(|(&k, v)| (k, v.len())).collect();

    // Compute formation targets
    let mut targets: HashMap<Entity, (Vec2, f32)> = HashMap::new();
    for idxs in groups.values() {
        let n = idxs.len();
        let scale = formation_scale(n);
        let dirs: Vec<Direction> = idxs.iter().map(|&i| data[i].4).collect();
        let same_axis = n > 1 && dirs.windows(2).all(|w| direction_axis(w[0]) == direction_axis(w[1]));
        let mut sorted = idxs.clone();
        sorted.sort_by_key(|&i| data[i].3); // stable slot by spawn_index
        for (slot, &idx) in sorted.iter().enumerate() {
            let off = if n == 1 { Vec2::ZERO }
                else if same_axis { perp_offset(data[idx].4, slot, n) }
                else { grid_offset(slot, n) };
            targets.insert(data[idx].0, (off, scale));
        }
    }
    // Bots not in any group (special phase) get default
    for d in &data { targets.entry(d.0).or_insert((Vec2::ZERO, 1.0)); }

    // Apply targets
    for (e, (off, sc)) in targets {
        if let Ok((_, _, mut f)) = bots.get_mut(e) { f.target_offset = off; f.target_scale = sc; }
    }
}

pub fn apply_bot_formation(
    time: Res<Time>,
    mut bots: Query<(&mut Transform, &mut TargetScale, &mut BotFormation, &BotMovement),
        (With<Bot>, Without<DespawnAtZeroScale>)>,
) {
    let dt = time.delta_secs();
    let pt = (10.0 * dt).min(1.0);
    let st = (8.0 * dt).min(1.0);
    for (mut tf, mut ts, mut form, mov) in &mut bots {
        form.offset = form.offset.lerp(form.target_offset, pt);
        tf.translation.x += form.offset.x;
        tf.translation.z += form.offset.y;
        if !is_special(&mov.phase) {
            form.visual_scale += (form.target_scale - form.visual_scale) * st;
            ts.0 = Vec3::splat(form.visual_scale.clamp(0.05, 1.5));
        }
    }
}

pub fn animate_merge_flashes(
    mut commands: Commands,
    time: Res<Time>,
    mut flashes: Query<(Entity, &mut MergeFlash, &mut TargetScale)>,
) {
    for (entity, mut flash, mut ts) in &mut flashes {
        let prev = flash.progress;
        flash.progress += time.delta_secs() / 0.35;
        if prev < 0.5 && flash.progress >= 0.5 {
            ts.0 = Vec3::ZERO;
            commands.entity(entity).insert(DespawnAtZeroScale);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MIN_SEP: f32 = 0.15; // minimum distance between any two bots on same tile

    fn no_overlap(offsets: &[Vec2]) {
        for i in 0..offsets.len() {
            for j in (i + 1)..offsets.len() {
                let d = offsets[i].distance(offsets[j]);
                assert!(d >= MIN_SEP, "Overlap: slot {i} and {j} dist={d:.4} < {MIN_SEP}");
            }
        }
    }

    #[test]
    fn single_bot_centered() {
        assert_eq!(formation_scale(1), 1.0);
    }

    #[test]
    fn scales_decrease_with_count() {
        let s: Vec<f32> = (1..=4).map(formation_scale).collect();
        for w in s.windows(2) { assert!(w[0] > w[1], "Scale should decrease: {} > {}", w[0], w[1]); }
    }

    #[test]
    fn perp_offsets_no_overlap_north() {
        for n in 2..=4 {
            let offs: Vec<Vec2> = (0..n).map(|s| perp_offset(Direction::North, s, n)).collect();
            no_overlap(&offs);
        }
    }

    #[test]
    fn perp_offsets_no_overlap_east() {
        for n in 2..=4 {
            let offs: Vec<Vec2> = (0..n).map(|s| perp_offset(Direction::East, s, n)).collect();
            no_overlap(&offs);
        }
    }

    #[test]
    fn perp_offsets_symmetric() {
        for n in [2, 3, 4] {
            let offs: Vec<f32> = formation_offsets(n).to_vec();
            let sum: f32 = offs.iter().sum();
            assert!(sum.abs() < 0.01, "n={n}: offsets should be roughly symmetric, sum={sum}");
        }
    }

    #[test]
    fn grid_offsets_no_overlap() {
        for n in 2..=4 {
            let offs: Vec<Vec2> = (0..n).map(|s| grid_offset(s, n)).collect();
            no_overlap(&offs);
        }
    }

    #[test]
    fn north_south_same_axis() {
        assert_eq!(direction_axis(Direction::North), direction_axis(Direction::South));
    }

    #[test]
    fn east_west_same_axis() {
        assert_eq!(direction_axis(Direction::East), direction_axis(Direction::West));
    }

    #[test]
    fn north_east_different_axis() {
        assert_ne!(direction_axis(Direction::North), direction_axis(Direction::East));
    }

    #[test]
    fn perp_north_spreads_on_x() {
        let o = perp_offset(Direction::North, 0, 2);
        assert!(o.x.abs() > 0.0 && o.y == 0.0, "North should spread on X: {o:?}");
    }

    #[test]
    fn perp_east_spreads_on_z() {
        let o = perp_offset(Direction::East, 0, 2);
        assert!(o.x == 0.0 && o.y.abs() > 0.0, "East should spread on Z: {o:?}");
    }

    #[test]
    fn special_phases_excluded() {
        assert!(is_special(&BotPhase::Stopped));
        assert!(is_special(&BotPhase::Falling(0.5)));
        assert!(is_special(&BotPhase::FallingPause(0.1)));
        assert!(is_special(&BotPhase::FallingDecel));
        assert!(is_special(&BotPhase::TeleportShrink { target_col: 0, target_row: 0 }));
        assert!(is_special(&BotPhase::TeleportGrow));
        assert!(!is_special(&BotPhase::Cruising));
        assert!(!is_special(&BotPhase::Accelerating));
    }

    #[test]
    fn all_offsets_within_half_tile() {
        for n in 2..=4 {
            for s in 0..n {
                for dir in Direction::all() {
                    let o = perp_offset(dir, s, n);
                    assert!(o.x.abs() < 0.5 && o.y.abs() < 0.5,
                        "Offset out of tile: dir={dir:?} n={n} slot={s} off={o:?}");
                }
                let g = grid_offset(s, n);
                assert!(g.x.abs() < 0.5 && g.y.abs() < 0.5,
                    "Grid offset out of tile: n={n} slot={s} off={g:?}");
            }
        }
    }

    /// Simulate the full formation assignment logic for a group of bots
    fn simulate_formation(dirs: &[Direction]) -> Vec<(Vec2, f32)> {
        let n = dirs.len();
        let scale = formation_scale(n);
        let same_axis = n > 1 && dirs.windows(2).all(|w| direction_axis(w[0]) == direction_axis(w[1]));
        (0..n).map(|slot| {
            let off = if n == 1 { Vec2::ZERO }
                else if same_axis { perp_offset(dirs[slot], slot, n) }
                else { grid_offset(slot, n) };
            (off, scale)
        }).collect()
    }

    #[test]
    fn two_north_bots_parallel_lanes() {
        let r = simulate_formation(&[Direction::North, Direction::North]);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].1, 0.62);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        // Both should spread on X only
        assert_eq!(r[0].0.y, 0.0);
        assert_eq!(r[1].0.y, 0.0);
    }

    #[test]
    fn two_east_bots_parallel_lanes() {
        let r = simulate_formation(&[Direction::East, Direction::East]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        assert_eq!(r[0].0.x, 0.0);
        assert_eq!(r[1].0.x, 0.0);
    }

    #[test]
    fn north_vs_south_same_axis_lanes() {
        let r = simulate_formation(&[Direction::North, Direction::South]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        // N/S share axis → perpendicular lanes on X
        assert_eq!(r[0].0.y, 0.0);
    }

    #[test]
    fn north_vs_east_mixed_grid() {
        let r = simulate_formation(&[Direction::North, Direction::East]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        // Mixed → 2D grid, both components nonzero
        assert!(r[0].0.x.abs() > 0.0 && r[0].0.y.abs() > 0.0);
    }

    #[test]
    fn three_mixed_directions() {
        let r = simulate_formation(&[Direction::North, Direction::East, Direction::South]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        assert_eq!(r[0].1, 0.50);
    }

    #[test]
    fn four_mixed_directions() {
        let r = simulate_formation(&[Direction::North, Direction::East, Direction::South, Direction::West]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        assert_eq!(r[0].1, 0.42);
    }

    #[test]
    fn four_same_direction() {
        let r = simulate_formation(&[Direction::South; 4]);
        no_overlap(&r.iter().map(|r| r.0).collect::<Vec<_>>());
        assert_eq!(r[0].1, 0.42);
    }

    #[test]
    fn bots_fit_within_scaled_tile() {
        // Verify scaled bots + offsets don't exceed tile boundaries
        let bot = BOT_SIZE; // 0.35
        for n in 2..=4 {
            let s = formation_scale(n);
            let half_bot = bot * s / 2.0;
            for slot in 0..n {
                for dir in Direction::all() {
                    let o = perp_offset(dir, slot, n);
                    let max_extent = (o.x.abs() + half_bot).max(o.y.abs() + half_bot);
                    assert!(max_extent < 0.5, "Bot exceeds tile: n={n} dir={dir:?} slot={slot} ext={max_extent:.3}");
                }
                let g = grid_offset(slot, n);
                let max_extent = (g.x.abs() + half_bot).max(g.y.abs() + half_bot);
                assert!(max_extent < 0.5, "Grid bot exceeds tile: n={n} slot={slot} ext={max_extent:.3}");
            }
        }
    }
}
