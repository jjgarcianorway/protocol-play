// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::rules;
use super::npc;
use super::deck;

/// Handle clicks on orb nodes (player hand and table).
pub fn handle_orb_click(
    mut state: ResMut<OrbGameState>,
    interaction_q: Query<(&Interaction, &OrbNode), Changed<Interaction>>,
) {
    if state.turn_phase != TurnPhase::PlayerTurn
        && state.turn_phase != TurnPhase::SeCayoWindow
    {
        return;
    }

    for (interaction, orb_node) in interaction_q.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match state.turn_phase {
            TurnPhase::PlayerTurn => handle_player_turn(&mut state, orb_node),
            TurnPhase::SeCayoWindow => handle_se_cayo_slam(&mut state, orb_node),
            _ => {}
        }
    }
}

fn handle_player_turn(state: &mut OrbGameState, orb_node: &OrbNode) {
    match orb_node.hand {
        OrbHand::Player => {
            // Select/deselect orb in hand
            if state.selected_orb == Some(orb_node.index) {
                state.selected_orb = None;
            } else {
                state.selected_orb = Some(orb_node.index);
            }
        }
        OrbHand::Table => {
            // If an orb is selected, try to play it
            if state.selected_orb.is_some() {
                play_player_orb(state);
            }
        }
        _ => {}
    }
}

fn handle_se_cayo_slam(state: &mut OrbGameState, orb_node: &OrbNode) {
    if orb_node.hand != OrbHand::Player {
        return;
    }
    if let Some(target_val) = state.se_cayo_orb {
        let idx = orb_node.index;
        if idx < state.player_hand.len() && state.player_hand[idx].value == target_val {
            // Slam! Remove the orb from hand, penalty to NPC
            state.player_hand.remove(idx);
            state.se_cayo_slams += 1;
            state.npc_treasure -= 1;
            state.player_captured_orbs += 1;
            state.total_orbs_played += 1;
            state.status_message = format!(
                "Se cayo! Slam {}! NPC pays 1", state.se_cayo_slams
            );
            // Check if player has another matching orb
            let more = rules::find_se_cayo_matches(&state.player_hand, target_val);
            if more.is_empty() || state.se_cayo_slams >= 3 {
                // End se cayo window
                end_se_cayo(state);
            }
            // else: timer continues, player can slam again
        }
    }
}

/// Play the selected orb from player's hand onto the table.
fn play_player_orb(state: &mut OrbGameState) {
    let idx = match state.selected_orb {
        Some(i) if i < state.player_hand.len() => i,
        _ => return,
    };

    let played = state.player_hand.remove(idx);
    state.selected_orb = None;
    state.total_orbs_played += 1;

    // Check captures
    let captures = rules::find_captures(&state.table, played.value);

    if captures.is_empty() {
        // No capture: orb stays on table
        state.table.push(played);
        state.status_message = format!("Played {} - no capture", played.value);
    } else {
        // Capture!
        let captured_count = captures.len() as i32;
        let mesa_limpia = rules::is_mesa_limpia(state.table.len(), captures.len());

        // Remove captured orbs (reverse order to maintain indices)
        let mut to_remove = captures.clone();
        to_remove.sort();
        to_remove.reverse();
        for idx in to_remove {
            state.table.remove(idx);
        }

        // Add to treasure: captured orbs + the played orb
        state.player_captured_orbs += captured_count + 1;
        state.last_capturer = Some(true);

        if mesa_limpia {
            state.player_treasure += 1;
            state.mesa_limpia_flash = MESA_LIMPIA_DURATION;
            state.status_message = format!(
                "Captured {} + Mesa Limpia! +1 bonus", captured_count
            );
        } else {
            state.status_message = format!("Captured {} orbs", captured_count);
        }
    }

    // Check if NPC can "se cayo"
    if npc::npc_has_se_cayo_match(&state.npc_hand, played.value) {
        let (will_react, delay) = npc::decide_se_cayo_reaction();
        if will_react {
            state.npc_will_react = true;
            state.npc_react_timer = delay;
            state.turn_phase = TurnPhase::NpcSeCayo;
            state.phase_timer = delay;
            return;
        }
    }

    // Move to NPC turn
    start_npc_turn(state);
}

fn start_npc_turn(state: &mut OrbGameState) {
    if state.npc_hand.is_empty() && state.player_hand.is_empty() {
        check_round_end(state);
    } else if state.npc_hand.is_empty() {
        state.turn_phase = TurnPhase::PlayerTurn;
        state.player_turn = true;
    } else {
        state.turn_phase = TurnPhase::NpcThinking;
        state.phase_timer = NPC_TURN_DELAY;
        state.player_turn = false;
    }
}

fn end_se_cayo(state: &mut OrbGameState) {
    state.se_cayo_timer = None;
    state.se_cayo_orb = None;
    state.se_cayo_slams = 0;

    // After se cayo, check if hands empty
    if state.player_hand.is_empty() && state.npc_hand.is_empty() {
        check_round_end(state);
    } else if state.player_hand.is_empty() {
        start_npc_turn(state);
    } else {
        state.turn_phase = TurnPhase::PlayerTurn;
        state.player_turn = true;
    }
}

/// Process game phases with timers.
pub fn process_turn_phases(
    time: Res<Time>,
    mut state: ResMut<OrbGameState>,
) {
    let dt = time.delta_secs();

    // Mesa limpia flash countdown
    if state.mesa_limpia_flash > 0.0 {
        state.mesa_limpia_flash -= dt;
    }

    match state.turn_phase {
        TurnPhase::Dealing => {
            state.phase_timer -= dt;
            if state.phase_timer <= 0.0 {
                if state.deck.is_empty()
                    && state.player_hand.is_empty()
                    && state.npc_hand.is_empty()
                {
                    state.turn_phase = TurnPhase::GameOver;
                } else {
                    deck::deal_new_hands(&mut state);
                    state.turn_phase = TurnPhase::RondaCheck;
                    state.phase_timer = 0.1;
                }
            }
        }
        TurnPhase::RondaCheck => {
            state.phase_timer -= dt;
            if state.phase_timer <= 0.0 {
                check_rondas(&mut state);
            }
        }
        TurnPhase::RondaDisplay => {
            state.phase_timer -= dt;
            if state.phase_timer <= 0.0 {
                state.ronda_message = None;
                if state.player_turn {
                    state.turn_phase = TurnPhase::PlayerTurn;
                    state.status_message = "Your turn - select an orb".into();
                } else {
                    start_npc_turn(&mut state);
                }
            }
        }
        TurnPhase::NpcThinking => {
            state.phase_timer -= dt;
            if state.phase_timer <= 0.0 {
                execute_npc_play(&mut state);
            }
        }
        TurnPhase::NpcSeCayo => {
            state.phase_timer -= dt;
            if state.phase_timer <= 0.0 {
                execute_npc_se_cayo(&mut state);
            }
        }
        TurnPhase::SeCayoWindow => {
            if let Some(ref mut timer) = state.se_cayo_timer {
                *timer -= dt;
                if *timer <= 0.0 {
                    state.status_message = "Too slow!".into();
                    end_se_cayo(&mut state);
                }
            }
        }
        TurnPhase::GameOver => {
            // Handled by mod.rs transition
        }
        TurnPhase::PlayerTurn | TurnPhase::PlayerCapture | TurnPhase::NpcPlay => {}
    }
}

fn check_rondas(state: &mut OrbGameState) {
    let (p_ronda, p_rondin, _p_val) = rules::check_ronda(&state.player_hand);
    let (n_ronda, n_rondin, _n_val) = rules::check_ronda(&state.npc_hand);

    let mut msg = String::new();

    if p_rondin {
        state.npc_treasure -= 2;
        msg.push_str("You have Rondin! NPC pays 2. ");
    } else if p_ronda {
        state.npc_treasure -= 1;
        msg.push_str("You have Ronda! NPC pays 1. ");
    }

    if n_rondin {
        state.player_treasure -= 2;
        msg.push_str("NPC has Rondin! You pay 2.");
    } else if n_ronda {
        state.player_treasure -= 1;
        msg.push_str("NPC has Ronda! You pay 1.");
    }

    if msg.is_empty() {
        // No ronda, go straight to play
        if state.player_turn {
            state.turn_phase = TurnPhase::PlayerTurn;
            state.status_message = "Your turn - select an orb".into();
        } else {
            start_npc_turn(state);
        }
    } else {
        state.ronda_message = Some(msg.clone());
        state.status_message = msg;
        state.turn_phase = TurnPhase::RondaDisplay;
        state.phase_timer = 2.0;
    }
}

fn execute_npc_play(state: &mut OrbGameState) {
    if state.npc_hand.is_empty() {
        if state.player_hand.is_empty() {
            check_round_end(state);
        } else {
            state.turn_phase = TurnPhase::PlayerTurn;
            state.player_turn = true;
        }
        return;
    }

    let choice = npc::choose_play(&state.npc_hand, &state.table);
    let played = state.npc_hand.remove(choice);
    state.total_orbs_played += 1;
    state.npc_played_orb = Some(played);

    // Check captures
    let captures = rules::find_captures(&state.table, played.value);

    if captures.is_empty() {
        state.table.push(played);
        state.status_message = format!("NPC played {} - no capture", played.value);
    } else {
        let captured_count = captures.len() as i32;
        let mesa_limpia = rules::is_mesa_limpia(state.table.len(), captures.len());

        let mut to_remove = captures;
        to_remove.sort();
        to_remove.reverse();
        for idx in to_remove {
            state.table.remove(idx);
        }

        state.npc_captured_orbs += captured_count + 1;
        state.last_capturer = Some(false);

        if mesa_limpia {
            state.npc_treasure += 1;
            state.mesa_limpia_flash = MESA_LIMPIA_DURATION;
            state.status_message = format!(
                "NPC captured {} + Mesa Limpia!", captured_count
            );
        } else {
            state.status_message = format!("NPC captured {} orbs", captured_count);
        }
    }

    // Check if player can "se cayo"
    let matches = rules::find_se_cayo_matches(&state.player_hand, played.value);
    if !matches.is_empty() {
        state.se_cayo_timer = Some(SE_CAYO_DURATION);
        state.se_cayo_orb = Some(played.value);
        state.se_cayo_slams = 0;
        state.turn_phase = TurnPhase::SeCayoWindow;
        state.status_message = format!(
            "Se cayo! Click your {} to slam!", played.value
        );
        return;
    }

    // Move to player turn
    if state.player_hand.is_empty() && state.npc_hand.is_empty() {
        check_round_end(state);
    } else if state.player_hand.is_empty() {
        start_npc_turn(state);
    } else {
        state.turn_phase = TurnPhase::PlayerTurn;
        state.player_turn = true;
        state.status_message = "Your turn - select an orb".into();
    }
}

fn execute_npc_se_cayo(state: &mut OrbGameState) {
    // NPC slams against the player
    if let Some(played) = state.npc_played_orb {
        let val = played.value;
        // Find and remove matching orb from NPC hand
        if let Some(idx) = state.npc_hand.iter().position(|o| o.value == val) {
            state.npc_hand.remove(idx);
            state.player_treasure -= 1;
            state.npc_captured_orbs += 1;
            state.total_orbs_played += 1;
            state.status_message = format!("NPC: Se cayo! You pay 1");
        }
    }
    state.npc_played_orb = None;
    start_npc_turn(state);
}

fn check_round_end(state: &mut OrbGameState) {
    if state.deck.is_empty() {
        // Game over: last capturer takes remaining table orbs
        let remaining = state.table.len() as i32;
        if remaining > 0 {
            match state.last_capturer {
                Some(true) => state.player_captured_orbs += remaining,
                Some(false) => state.npc_captured_orbs += remaining,
                None => state.player_captured_orbs += remaining,
            }
            state.table.clear();
        }
        state.turn_phase = TurnPhase::GameOver;
    } else {
        // Deal new hands
        state.turn_phase = TurnPhase::Dealing;
        state.phase_timer = 0.8;
        state.status_message = "Dealing new hands...".into();
        // Determine who starts: last capturer, or keep current
        if let Some(was_player) = state.last_capturer {
            state.player_turn = was_player;
        }
    }
}
