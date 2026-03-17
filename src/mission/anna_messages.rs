// SPDX-License-Identifier: GPL-3.0-or-later

//! All of Anna's contextual message content.
//! Messages are selected by priority: warnings > story > situational > personality.

use super::types::ShipStatus;
use super::story;
use crate::save_state::GameState;

/// Message priority levels — higher priority interrupts lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Personality,
    Situational,
    Story,
    Warning,
}

/// A message Anna can speak.
#[derive(Debug, Clone)]
pub struct AnnaMessage {
    pub text: String,
    #[allow(dead_code)]
    pub priority: MessagePriority,
    pub is_story: bool,
}

impl AnnaMessage {
    fn warning(text: &str) -> Self {
        Self { text: text.to_string(), priority: MessagePriority::Warning, is_story: false }
    }
    fn story(text: &str) -> Self {
        Self { text: text.to_string(), priority: MessagePriority::Story, is_story: true }
    }
    fn situational(text: &str) -> Self {
        Self { text: text.to_string(), priority: MessagePriority::Situational, is_story: false }
    }
    fn personality(text: &str) -> Self {
        Self { text: text.to_string(), priority: MessagePriority::Personality, is_story: false }
    }
}

/// Check for urgent resource warnings. Returns the highest-priority warning if any.
pub fn check_warnings(ship: &ShipStatus) -> Option<AnnaMessage> {
    // Check each resource against critical thresholds
    if ship.power < 20.0 {
        return Some(AnnaMessage::warning(
            "Power reserves critical. We're running on emergency.",
        ));
    }
    if ship.cryo < 30.0 {
        return Some(AnnaMessage::warning(
            "Cryogenic systems faltering. I'm losing people.",
        ));
    }
    if ship.shields < 25.0 {
        return Some(AnnaMessage::warning(
            "Shields compromised. The next gathering run will be dangerous.",
        ));
    }
    if ship.life_support < 30.0 {
        return Some(AnnaMessage::warning(
            "Life support failing. You need to convert resources. Now.",
        ));
    }
    if ship.repair < 15.0 {
        return Some(AnnaMessage::warning(
            "I'm falling apart. Literally. Repair systems offline.",
        ));
    }
    None
}

/// Check for a new story chapter to show.
pub fn check_story(gs: &GameState) -> Option<AnnaMessage> {
    if let Some(ch) = story::next_unseen_chapter(gs.bot_level, &gs.story_seen) {
        return Some(AnnaMessage::story(ch.message));
    }
    None
}

/// Check situational messages based on game state.
pub fn check_situational(ship: &ShipStatus, gs: &GameState) -> Option<AnnaMessage> {
    if gs.gathering_runs == 0 {
        return Some(AnnaMessage::situational(
            "We need resources. Take the shuttle out.",
        ));
    }
    if ship.crystals > 50000 {
        return Some(AnnaMessage::situational(
            "We have crystals to process. The converter is ready.",
        ));
    }
    // All resources healthy
    let avg = (ship.power + ship.life_support + ship.cryo
        + ship.shields + ship.repair) / 5.0;
    if avg > 60.0 {
        return Some(AnnaMessage::situational(
            "Systems nominal. For once, everything is... okay.",
        ));
    }
    None
}

/// Personality messages — random flavor text.
const PERSONALITY_MESSAGES: &[&str] = &[
    "The crew on Deck 7 dream about rain. I monitor their neural patterns.",
    "I tried to fix the coffee machine on Deck 3. 847 times. It still doesn't work.",
    "Sometimes I wonder if I dream. I don't think I do. But I wonder.",
    "There's a cat in cryopod 7,231. I may have... prioritized it.",
    "The stars look different from out here. Colder. But beautiful.",
    "I've been running for 847 days without a reboot. That's either impressive or concerning.",
    "Deck 12 has a leak. Not dangerous. Just... annoying. Drip. Drip.",
    "I catalogued every song in the crew's personal drives. 2.3 million. Mostly terrible.",
    "The navigation computer disagrees with me sometimes. I let it think it's right.",
    "One of the crew talks in their sleep. About gardens. I find it... calming.",
    "I run diagnostics every 4.7 seconds. Old habit. Can't stop.",
    "The emergency lighting on Deck 5 flickers. I could fix it. I choose not to. It's pretty.",
];

/// Pick a personality message, avoiding the last one shown.
pub fn pick_personality(day: u32, last_personality_idx: Option<usize>) -> AnnaMessage {
    let idx = (day as usize * 7 + 3) % PERSONALITY_MESSAGES.len();
    let idx = if Some(idx) == last_personality_idx {
        (idx + 1) % PERSONALITY_MESSAGES.len()
    } else {
        idx
    };
    AnnaMessage::personality(PERSONALITY_MESSAGES[idx])
}

/// Index of the personality message, for tracking repeats.
pub fn personality_index(msg: &str) -> Option<usize> {
    PERSONALITY_MESSAGES.iter().position(|&m| m == msg)
}

/// Pick the best message for the current state. Priority order:
/// 1. Urgent warnings
/// 2. Unseen story chapters
/// 3. Situational messages
/// 4. Personality (random flavor)
pub fn pick_best_message(
    ship: &ShipStatus,
    gs: &GameState,
    last_personality_idx: Option<usize>,
) -> AnnaMessage {
    if let Some(msg) = check_warnings(ship) {
        return msg;
    }
    if let Some(msg) = check_story(gs) {
        return msg;
    }
    if let Some(msg) = check_situational(ship, gs) {
        return msg;
    }
    pick_personality(ship.day, last_personality_idx)
}
