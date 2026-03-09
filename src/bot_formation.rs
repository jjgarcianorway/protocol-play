// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use std::collections::HashMap;
use crate::constants::*;
use crate::types::*;
use crate::simulation::*;

fn formation_scale(n: usize) -> f32 { match n { 1 => 1.0, 2 => 0.70, 3 => 0.56, _ => 0.48 } }

fn formation_offsets(n: usize) -> &'static [f32] {
    match n { 2 => &[-0.17, 0.17], 3 => &[-0.21, 0.0, 0.21], _ => &[-0.22, -0.073, 0.073, 0.22] }
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
    const G2: [Vec2; 2] = [Vec2::new(-0.13, -0.13), Vec2::new(0.13, 0.13)];
    const G3: [Vec2; 3] = [Vec2::new(-0.15, -0.10), Vec2::new(0.15, -0.10), Vec2::new(0.0, 0.15)];
    const G4: [Vec2; 4] = [Vec2::new(-0.13, -0.13), Vec2::new(0.13, -0.13),
                            Vec2::new(-0.13, 0.13), Vec2::new(0.13, 0.13)];
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
