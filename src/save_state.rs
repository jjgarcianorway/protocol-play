// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

/// Cross-game persistent state, shared by all game modes.
/// Stored as `game_state.json` next to the executable.
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    // Resources (0.0 to 100.0 percentage)
    pub power: f32,
    pub life_support: f32,
    pub cryo: f32,
    pub shields: f32,
    pub repair: f32,

    // Crystal inventory (from Gathering, consumed by Converter)
    pub crystals_red: u64,
    pub crystals_green: u64,
    pub crystals_blue: u64,
    pub crystals_yellow: u64,
    pub crystals_purple: u64,

    // Progress
    pub bot_level: u32,
    pub crew_count: u32,
    pub day: u32,
    pub distance_au: f32,
    pub gathering_runs: u32,
    pub total_crystals_gathered: u64,

    // Story progression
    #[serde(default)]
    pub story_chapter: u32,
    #[serde(default)]
    pub story_seen: Vec<u32>,
    #[serde(default)]
    pub decisions: Vec<String>,

    // Story flags (for Anna's questions later)
    pub story_flags: Vec<String>,

    // New Game+ tracking
    #[serde(default)]
    pub playthrough_count: u32,

    // Ending flag — player reached New Earth
    #[serde(default)]
    pub reached_new_earth: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            power: 50.0,
            life_support: 50.0,
            cryo: 80.0,
            shields: 30.0,
            repair: 20.0,
            crystals_red: 0,
            crystals_green: 0,
            crystals_blue: 0,
            crystals_yellow: 0,
            crystals_purple: 0,
            bot_level: 0,
            crew_count: 14_892,
            day: 1,
            distance_au: 0.0,
            gathering_runs: 0,
            total_crystals_gathered: 0,
            story_chapter: 0,
            story_seen: Vec::new(),
            decisions: Vec::new(),
            story_flags: Vec::new(),
            playthrough_count: 0,
            reached_new_earth: false,
        }
    }
}

/// Reset state for New Game+ — keeps playthrough_count (incremented) and resets everything else.
pub fn reset_for_new_game(state: &mut GameState) {
    let next_playthrough = state.playthrough_count + 1;
    *state = GameState::default();
    state.playthrough_count = next_playthrough;
}

impl GameState {
    /// Total crystals across all colors.
    pub fn total_crystals(&self) -> u64 {
        self.crystals_red + self.crystals_green + self.crystals_blue
            + self.crystals_yellow + self.crystals_purple
    }
}

/// Directory next to the executable.
pub fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Path to the game state file (next to executable).
pub fn game_state_path() -> PathBuf {
    exe_dir().join("game_state.json")
}

/// Load game state from disk, or return defaults for a new game.
pub fn load_game_state() -> GameState {
    let path = game_state_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

/// Save game state to disk (pretty-printed JSON).
pub fn save_game_state(state: &GameState) {
    let path = game_state_path();
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = fs::write(&path, json);
    }
}
