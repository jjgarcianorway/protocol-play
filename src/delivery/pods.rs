// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use rand::Rng;
use super::constants::*;
use super::types::*;

/// Spawn pods on a timer, increasing difficulty over time.
pub fn spawn_pods(
    time: Res<Time>,
    mut state: ResMut<DeliveryState>,
    mut commands: Commands,
    font: Res<DeliveryFont>,
) {
    if !state.game_started { return; }
    if state.pods_spawned >= state.total_pods { return; }

    state.spawn_timer -= time.delta_secs();
    if state.spawn_timer > 0.0 { return; }

    state.spawn_timer = state.spawn_interval();

    let mut rng = rand::thread_rng();
    let color = PodColor::from_index(rng.gen_range(0..5));
    let fall_duration = state.fall_duration();

    state.pods_spawned += 1;
    state.difficulty = state.pods_spawned as f32 / state.total_pods as f32;

    let (r, g, b) = color.rgb();
    commands.spawn((
        Pod {
            color,
            progress: 0.0,
            fall_duration,
            routed: None,
            route_progress: 0.0,
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(POD_SIZE),
            height: Val::Px(POD_SIZE),
            left: Val::Percent(50.0),
            top: Val::Px(FALL_ZONE_TOP),
            border_radius: BorderRadius::all(Val::Px(POD_CORNER)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(r, g, b)),
        BoxShadow::new(
            Color::srgba(r, g, b, 0.6),
            Val::ZERO, Val::ZERO,
            Val::Px(POD_GLOW_SPREAD), Val::Px(POD_GLOW_BLUR),
        ),
        PodVisual,
    )).with_child((
        Text::new(color.icon()),
        TextFont { font: font.0.clone(), font_size: 18.0, ..default() },
        TextColor(Color::WHITE),
    ));
}

/// Move pods downward and animate horizontal routing.
pub fn move_pods(
    time: Res<Time>,
    mut pod_q: Query<(&mut Pod, &mut Node)>,
    window_q: Query<&Window>,
) {
    let dt = time.delta_secs();
    let win_width = window_q.iter().next()
        .map(|w| w.resolution.width())
        .unwrap_or(1280.0);

    for (mut pod, mut node) in pod_q.iter_mut() {
        let speed = 1.0 / pod.fall_duration;
        pod.progress += speed * dt;

        let y = FALL_ZONE_TOP + pod.progress * FALL_ZONE_HEIGHT;
        node.top = Val::Px(y);

        if let Some(slot_idx) = pod.routed {
            pod.route_progress = (pod.route_progress + dt / ROUTE_ANIM_DURATION).min(1.0);
            let total_slots_width = SLOT_COUNT as f32 * SLOT_WIDTH
                + (SLOT_COUNT as f32 - 1.0) * SLOT_GAP;
            let slots_left = (win_width - total_slots_width) / 2.0;
            let target_x = slots_left
                + slot_idx as f32 * (SLOT_WIDTH + SLOT_GAP)
                + SLOT_WIDTH / 2.0
                - POD_SIZE / 2.0;
            let start_x = win_width / 2.0 - POD_SIZE / 2.0;
            let t = smoothstep(pod.route_progress);
            let x = start_x + (target_x - start_x) * t;
            node.left = Val::Px(x);
        } else {
            node.left = Val::Px(win_width / 2.0 - POD_SIZE / 2.0);
        }
    }
}

/// Resolve pods that reached the bottom.
pub fn resolve_pods(
    mut state: ResMut<DeliveryState>,
    pod_q: Query<(Entity, &Pod)>,
    mut commands: Commands,
    slot_q: Query<(Entity, &DepositSlot)>,
) {
    for (entity, pod) in pod_q.iter() {
        if pod.progress < 1.0 { continue; }

        state.pods_resolved += 1;

        match pod.routed {
            Some(slot_idx) => {
                if pod.color.index() == slot_idx {
                    state.score[slot_idx] += 1;
                    state.streak += 1;
                    if state.streak > state.best_streak {
                        state.best_streak = state.streak;
                    }
                    spawn_flash(&mut commands, &slot_q, slot_idx, true);
                } else {
                    state.wasted += 1;
                    state.streak = 0;
                    spawn_flash(&mut commands, &slot_q, slot_idx, false);
                }
            }
            None => {
                state.missed += 1;
                state.streak = 0;
            }
        }

        commands.entity(entity).despawn();
    }
}

/// Spawn a flash overlay on a deposit slot.
fn spawn_flash(
    commands: &mut Commands,
    slot_q: &Query<(Entity, &DepositSlot)>,
    slot_idx: usize,
    correct: bool,
) {
    for (entity, slot) in slot_q.iter() {
        if slot.0 != slot_idx { continue; }
        let dur = if correct { CORRECT_FLASH_DURATION } else { WRONG_FLASH_DURATION };
        let c = if correct { CORRECT_FLASH_COLOR } else { WRONG_FLASH_COLOR };
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SlotFlash { timer: dur, correct },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(SLOT_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgba(c.0, c.1, c.2, c.3)),
            ));
        });
    }
}

/// Route the lowest unrouted pod to the clicked slot.
pub fn route_pod_to_slot(
    mut state: ResMut<DeliveryState>,
    interaction_q: Query<(&Interaction, &DepositSlot), Changed<Interaction>>,
    mut pod_q: Query<(Entity, &mut Pod)>,
) {
    for (interaction, slot) in interaction_q.iter() {
        if *interaction != Interaction::Pressed { continue; }

        state.selected_slot = Some(slot.0);

        // Find the lowest-progress (most recently spawned / highest on screen) unrouted pod
        let mut best_entity: Option<Entity> = None;
        let mut best_progress = -1.0_f32;

        for (entity, pod) in pod_q.iter() {
            if pod.routed.is_some() { continue; }
            if pod.progress > best_progress {
                best_progress = pod.progress;
                best_entity = Some(entity);
            }
        }

        if let Some(entity) = best_entity {
            if let Ok((_, mut pod)) = pod_q.get_mut(entity) {
                pod.routed = Some(slot.0);
                pod.route_progress = 0.0;
            }
        }
    }
}

/// Update flash effect overlays.
pub fn update_slot_flashes(
    time: Res<Time>,
    mut flash_q: Query<(Entity, &mut SlotFlash, &mut BackgroundColor)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut flash, mut bg) in flash_q.iter_mut() {
        flash.timer -= dt;
        if flash.timer <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            let max_dur = if flash.correct {
                CORRECT_FLASH_DURATION
            } else {
                WRONG_FLASH_DURATION
            };
            let alpha = (flash.timer / max_dur).clamp(0.0, 1.0);
            let c = if flash.correct { CORRECT_FLASH_COLOR } else { WRONG_FLASH_COLOR };
            *bg = BackgroundColor(Color::srgba(c.0, c.1, c.2, c.3 * alpha));
        }
    }
}

/// Check if all pods are resolved and transition to results.
pub fn check_game_complete(
    state: Res<DeliveryState>,
    pod_q: Query<&Pod>,
    mut next_state: ResMut<NextState<DeliveryPhase>>,
) {
    if !state.game_started { return; }
    if state.pods_spawned < state.total_pods { return; }
    if pod_q.iter().count() > 0 { return; }
    next_state.set(DeliveryPhase::Results);
}

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
