// SPDX-License-Identifier: GPL-3.0-or-later

use rand::Rng;
use super::constants::*;
use super::types::*;
use super::rules;

/// NPC decides which orb to play from its hand.
/// Strategy: prefer captures (especially mesa limpia), else play highest value.
pub fn choose_play(hand: &[Orb], table: &[Orb]) -> usize {
    if hand.is_empty() {
        return 0;
    }

    let mut best_index = 0;
    let mut best_score: i32 = -1;

    for (i, orb) in hand.iter().enumerate() {
        let captures = rules::find_captures(table, orb.value);
        let capture_count = captures.len() as i32;

        // Score: captures are valuable, mesa limpia is a big bonus
        let mut score = capture_count * 10;
        if rules::is_mesa_limpia(table.len(), captures.len()) {
            score += 50; // Strongly prefer mesa limpia
        }

        // If no captures, prefer playing high values (they stay on table)
        if capture_count == 0 {
            score = -(orb.value as i32); // Negative = no capture, but higher is less bad
        }

        if score > best_score {
            best_score = score;
            best_index = i;
        }
    }

    best_index
}

/// Decide if NPC will react to "se cayo" and with what delay.
/// Returns (will_react, delay_seconds).
pub fn decide_se_cayo_reaction() -> (bool, f32) {
    let mut rng = rand::thread_rng();
    let will_react = rng.r#gen::<f32>() < NPC_REACT_CHANCE;
    let delay = NPC_REACT_DELAY + rng.gen_range(-0.3..0.3);
    (will_react, delay.max(0.3))
}

/// Check if NPC has a matching orb for se cayo after player plays.
pub fn npc_has_se_cayo_match(npc_hand: &[Orb], played_value: u8) -> bool {
    npc_hand.iter().any(|orb| orb.value == played_value)
}
