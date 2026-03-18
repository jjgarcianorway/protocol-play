// SPDX-License-Identifier: GPL-3.0-or-later

use rand::seq::SliceRandom;
use super::constants::*;
use super::types::*;

/// Create a full deck of 40 orbs (values 1-10, 4 colors each).
pub fn create_deck() -> Vec<Orb> {
    let mut deck = Vec::with_capacity(DECK_SIZE);
    for value in 1..=ORB_VALUES {
        for color_idx in 0..ORB_SUITS {
            deck.push(Orb {
                value,
                color: OrbColor::from_index(color_idx),
            });
        }
    }
    deck
}

/// Shuffle the deck in place.
pub fn shuffle_deck(deck: &mut Vec<Orb>) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

/// Deal `count` orbs from the deck into a hand.
/// Returns the dealt orbs. Removes them from the deck.
pub fn deal(deck: &mut Vec<Orb>, count: usize) -> Vec<Orb> {
    let n = count.min(deck.len());
    deck.drain(..n).collect()
}

/// Deal initial hands and table for a new game.
pub fn deal_initial(state: &mut OrbGameState) {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);
    state.player_hand = deck.drain(..HAND_SIZE).collect();
    state.npc_hand = deck.drain(..HAND_SIZE).collect();
    state.table = deck.drain(..TABLE_INITIAL).collect();
    state.deck = deck;
}

/// Deal new hands from remaining deck (called when both hands empty).
pub fn deal_new_hands(state: &mut OrbGameState) {
    let player_count = HAND_SIZE.min(state.deck.len());
    state.player_hand = state.deck.drain(..player_count).collect();
    let npc_count = HAND_SIZE.min(state.deck.len());
    state.npc_hand = state.deck.drain(..npc_count).collect();
    state.round_number += 1;
}
