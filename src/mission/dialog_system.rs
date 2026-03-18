// SPDX-License-Identifier: GPL-3.0-or-later
//! Dialog engine systems — triggering, advancing, typewriter, choice handling.
use bevy::prelude::*;
use super::dialog_types::*;
use super::dialog_ui;
use super::dialog_scenes;
use super::types::*;
use crate::save_state::{save_game_state, GameState};

/// System: check dialog triggers after returning from a game.
/// Queues applicable scenes and marks triggers as checked.
pub fn check_dialog_triggers(
    time: Res<Time>,
    mut state: ResMut<DialogState>,
    gs: Res<GameState>,
    ship: Res<ShipStatus>,
    running: Res<RunningGame>,
) {
    if running.0.is_some() { return; }
    if state.active_scene.is_some() { return; }
    if state.checked_triggers { return; }

    state.delay_timer -= time.delta_secs();
    if state.delay_timer > 0.0 { return; }

    state.checked_triggers = true;

    let all_scenes = dialog_scenes::all_scenes();
    for scene in all_scenes {
        let seen_key = format!("dialog_seen_{}", scene.id);
        if gs.decisions.iter().any(|d| d == &seen_key) {
            continue;
        }
        if should_trigger(scene, &gs, &ship) {
            state.queue.push(scene);
        }
    }
}

/// Check whether a scene's trigger condition is met.
fn should_trigger(scene: &DialogScene, gs: &GameState, ship: &ShipStatus) -> bool {
    match &scene.trigger {
        DialogTrigger::BotLevel(level) => gs.bot_level >= *level,
        DialogTrigger::GatheringReturn => gs.gathering_runs > 0,
        DialogTrigger::ResourceCritical(idx) => {
            [ship.power, ship.life_support, ship.cryo, ship.shields, ship.repair]
                .get(*idx).map(|v| *v < 20.0).unwrap_or(false)
        }
        DialogTrigger::FirstTime(event) => !gs.story_flags.contains(&event.to_string()),
        DialogTrigger::PlaythroughN(n) => gs.playthrough_count == *n,
        DialogTrigger::Decision(key) => gs.decisions.iter().any(|d| d == *key),
        DialogTrigger::DecisionAndLevel(key, level) =>
            gs.bot_level >= *level && gs.decisions.iter().any(|d| d == *key),
        DialogTrigger::PlaythroughAndLevel(n, level) =>
            gs.playthrough_count == *n && gs.bot_level >= *level,
        DialogTrigger::CrewLoss(threshold) => ship.crew_count < *threshold,
    }
}

/// System: start the next queued dialog scene if none is active.
pub fn start_next_dialog(
    mut state: ResMut<DialogState>,
    mut commands: Commands,
    font: Res<MissionFont>,
    overlay_q: Query<Entity, With<DialogOverlay>>,
    qs: Res<super::questions::QuestionState>,
) {
    if state.active_scene.is_some() { return; }
    if !overlay_q.is_empty() { return; }
    if qs.showing { return; }

    if let Some(scene) = state.queue.pop() {
        let total = scene.nodes.first()
            .map(|n| n.text.len()).unwrap_or(0);
        state.active_scene = Some(ActiveDialog {
            scene,
            node_index: 0,
            chars_revealed: 0,
            total_chars: total,
            char_timer: 0.0,
            text_complete: false,
            choices_visible: false,
            choice_delay: CHOICE_APPEAR_DELAY,
            reaction_text: None,
            reaction_timer: 0.0,
        });
        dialog_ui::spawn_dialog_overlay(&mut commands, &font.0);
    }
}

/// System: advance typewriter effect each frame.
pub fn update_typewriter(
    time: Res<Time>,
    mut state: ResMut<DialogState>,
    mut body_q: Query<&mut Text, (With<DialogBodyText>,
        Without<DialogSpeakerText>, Without<DialogSkipHint>)>,
    mut speaker_color_q: Query<&mut TextColor,
        (With<DialogSpeakerText>, Without<DialogBodyText>, Without<DialogSkipHint>)>,
    mut speaker_text_q: Query<&mut Text,
        (With<DialogSpeakerText>, Without<DialogBodyText>, Without<DialogSkipHint>)>,
    mut circle_q: Query<(&mut BackgroundColor, &mut BoxShadow),
        With<DialogAnnaCircle>>,
    mut hint_q: Query<(&mut Text, &mut TextColor), (With<DialogSkipHint>, Without<DialogBodyText>, Without<DialogSpeakerText>)>,
) {
    let active = match state.active_scene.as_mut() {
        Some(a) => a,
        None => return,
    };

    // Handle reaction display
    if active.reaction_text.is_some() {
        active.reaction_timer -= time.delta_secs();
        if active.reaction_timer <= 0.0 {
            active.reaction_text = None;
        } else {
            return;
        }
    }

    let node = match active.scene.nodes.get(active.node_index) {
        Some(n) => n,
        None => return,
    };

    // Update speaker display
    dialog_ui::update_speaker_display(
        node.speaker,
        &mut speaker_color_q,
        &mut speaker_text_q,
        &mut circle_q,
    );

    // Typewriter effect
    if !active.text_complete {
        active.char_timer += time.delta_secs();
        let chars_needed = (active.char_timer * TYPEWRITER_SPEED) as usize;
        if chars_needed > active.chars_revealed {
            active.chars_revealed = chars_needed.min(active.total_chars);
        }
        if active.chars_revealed >= active.total_chars {
            active.text_complete = true;
            active.chars_revealed = active.total_chars;
        }
        let revealed: String = node.text
            .chars().take(active.chars_revealed).collect();
        for mut text in body_q.iter_mut() {
            *text = Text::new(revealed.clone());
        }
    }

    // Handle choice delay
    if active.text_complete && !active.choices_visible {
        if let DialogNext::Choice(_) = &node.next {
            active.choice_delay -= time.delta_secs();
            if active.choice_delay <= 0.0 {
                active.choices_visible = true;
            }
        }
    }

    // Update hint
    let state_ref = &*state;
    dialog_ui::update_skip_hint(state_ref, &mut hint_q);
}

/// System: handle click on the dialog overlay (skip / advance).
pub fn dialog_click_advance(
    overlay_q: Query<&Interaction,
        (Changed<Interaction>, With<DialogOverlay>)>,
    mut state: ResMut<DialogState>,
    mut commands: Commands,
    mut gs: ResMut<GameState>,
    despawn_q: Query<Entity, With<DialogOverlay>>,
    container_q: Query<Entity, With<DialogChoiceContainer>>,
    font: Res<MissionFont>,
    mut body_q: Query<&mut Text, (With<DialogBodyText>,
        Without<DialogSpeakerText>, Without<DialogSkipHint>)>,
    mut anna_state: ResMut<AnnaState>,
    btn_q: Query<Entity, With<DialogChoiceBtn>>,
) {
    for interaction in overlay_q.iter() {
        if *interaction != Interaction::Pressed { continue; }

        let active = match state.active_scene.as_mut() {
            Some(a) => a,
            None => continue,
        };

        // If reaction is showing, skip it
        if active.reaction_text.is_some() {
            active.reaction_text = None;
            active.reaction_timer = 0.0;
            continue;
        }

        // If typewriter not done, skip to full text
        if !active.text_complete {
            active.chars_revealed = active.total_chars;
            active.text_complete = true;
            let node = &active.scene.nodes[active.node_index];
            for mut text in body_q.iter_mut() {
                *text = Text::new(node.text);
            }
            continue;
        }

        // If choices are pending, don't advance on click
        let node = &active.scene.nodes[active.node_index];
        if let DialogNext::Choice(_) = &node.next {
            if !active.choices_visible {
                active.choices_visible = true;
                active.choice_delay = 0.0;
            }
            continue;
        }

        // Advance to next node
        advance_dialog(
            &mut state, &mut commands, &mut gs, &despawn_q,
            &container_q, &font, &mut anna_state, &btn_q,
        );
    }
}

/// System: handle clicking a dialog choice button.
pub fn dialog_choice_click(
    query: Query<(&Interaction, &DialogChoiceBtn),
        Changed<Interaction>>,
    mut state: ResMut<DialogState>,
    mut commands: Commands,
    mut gs: ResMut<GameState>,
    mut body_q: Query<&mut Text, (With<DialogBodyText>,
        Without<DialogSpeakerText>, Without<DialogSkipHint>)>,
    btn_q: Query<Entity, With<DialogChoiceBtn>>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }

        let active = match state.active_scene.as_mut() {
            Some(a) => a,
            None => continue,
        };

        let node = &active.scene.nodes[active.node_index];
        let choices = match &node.next {
            DialogNext::Choice(c) => *c,
            _ => continue,
        };

        let choice = match choices.get(btn.0) {
            Some(c) => c,
            None => continue,
        };

        // Save decision if present
        if let Some(key) = choice.decision_key {
            if !gs.decisions.contains(&key.to_string()) {
                gs.decisions.push(key.to_string());
                save_game_state(&gs);
            }
        }

        let next = choice.next_node;
        let reaction = choice.anna_reacts;

        // Clear choice buttons
        dialog_ui::clear_choice_buttons(&mut commands, &btn_q);

        // Show reaction or advance
        if let Some(reaction_text) = reaction {
            active.reaction_text = Some(reaction_text);
            active.reaction_timer = REACTION_DURATION;
            for mut text in body_q.iter_mut() {
                *text = Text::new(reaction_text);
            }
            // After reaction, go to next node
            active.node_index = next;
            active.chars_revealed = 0;
            active.text_complete = false;
            active.choices_visible = false;
            active.choice_delay = CHOICE_APPEAR_DELAY;
            let next_node = &active.scene.nodes[next];
            active.total_chars = next_node.text.len();
            active.char_timer = 0.0;
        } else {
            active.node_index = next;
            setup_current_node(active);
        }
    }
}

/// Advance to the next node based on DialogNext.
fn advance_dialog(
    state: &mut ResMut<DialogState>,
    commands: &mut Commands,
    gs: &mut ResMut<GameState>,
    despawn_q: &Query<Entity, With<DialogOverlay>>,
    container_q: &Query<Entity, With<DialogChoiceContainer>>,
    font: &Res<MissionFont>,
    anna_state: &mut ResMut<AnnaState>,
    _btn_q: &Query<Entity, With<DialogChoiceBtn>>,
) {
    let active = match state.active_scene.as_mut() {
        Some(a) => a,
        None => return,
    };

    let node = &active.scene.nodes[active.node_index];
    match &node.next {
        DialogNext::Continue(next) => {
            active.node_index = *next;
            setup_current_node(active);
        }
        DialogNext::End => {
            end_scene(state, commands, gs, despawn_q, anna_state);
        }
        DialogNext::EndWithDecision(key) => {
            if !gs.decisions.contains(&key.to_string()) {
                gs.decisions.push(key.to_string());
                save_game_state(gs);
            }
            end_scene(state, commands, gs, despawn_q, anna_state);
        }
        DialogNext::Choice(choices) => {
            if active.choices_visible {
                dialog_ui::spawn_choice_buttons(
                    commands, container_q, choices, &font.0,
                );
            }
        }
    }
}

/// Set up the active dialog for a new current node.
fn setup_current_node(active: &mut ActiveDialog) {
    let node = &active.scene.nodes[active.node_index];
    active.chars_revealed = 0;
    active.total_chars = node.text.len();
    active.char_timer = 0.0;
    active.text_complete = false;
    active.choices_visible = false;
    active.choice_delay = CHOICE_APPEAR_DELAY;
    active.reaction_text = None;
    active.reaction_timer = 0.0;
}

/// End the current scene and clean up.
fn end_scene(
    state: &mut ResMut<DialogState>,
    commands: &mut Commands,
    gs: &mut ResMut<GameState>,
    despawn_q: &Query<Entity, With<DialogOverlay>>,
    anna_state: &mut ResMut<AnnaState>,
) {
    if let Some(active) = &state.active_scene {
        let seen_key = format!("dialog_seen_{}", active.scene.id);
        if !gs.decisions.contains(&seen_key) {
            gs.decisions.push(seen_key);
            save_game_state(gs);
        }
        anna_state.mood = AnnaMood::Story;
    }

    state.active_scene = None;
    dialog_ui::despawn_dialog_overlay(commands, despawn_q);
}

/// System: spawn choice buttons when choices become visible.
pub fn spawn_choices_when_ready(
    state: Res<DialogState>,
    mut commands: Commands,
    container_q: Query<Entity, With<DialogChoiceContainer>>,
    btn_q: Query<&DialogChoiceBtn>,
    font: Res<MissionFont>,
) {
    let active = match &state.active_scene {
        Some(a) => a,
        None => return,
    };
    if !active.choices_visible { return; }
    if active.reaction_text.is_some() { return; }
    if !btn_q.is_empty() { return; }

    let node = match active.scene.nodes.get(active.node_index) {
        Some(n) => n,
        None => return,
    };
    if let DialogNext::Choice(choices) = &node.next {
        dialog_ui::spawn_choice_buttons(
            &mut commands, &container_q, choices, &font.0,
        );
    }
}

/// Reset dialog state when returning from a game.
pub fn reset_dialog_check(state: &mut DialogState) {
    state.checked_triggers = false;
    state.delay_timer = DIALOG_START_DELAY;
}
