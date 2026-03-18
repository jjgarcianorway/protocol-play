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

    // World seed — generated on first game, preserved across sessions
    #[serde(default)]
    pub world_seed: u64,

    // Crew members the player has learned about (pod numbers)
    #[serde(default)]
    pub discovered_crew: Vec<u32>,

    // Ark names the player has learned about
    #[serde(default)]
    pub discovered_arks: Vec<String>,

    // Orben card games completed
    #[serde(default)]
    pub orben_games_played: u32,

    // Language code ("en", "es", etc.)
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
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
            world_seed: 0,
            discovered_crew: Vec::new(),
            discovered_arks: Vec::new(),
            orben_games_played: 0,
            language: "en".to_string(),
        }
    }
}

/// Reset state for New Game+ — keeps playthrough_count (incremented) and world seed.
/// Pass `keep_seed = true` for "Same World" or `false` for "New World".
pub fn reset_for_new_game(state: &mut GameState) {
    let next_playthrough = state.playthrough_count + 1;
    let seed = state.world_seed;
    *state = GameState::default();
    state.playthrough_count = next_playthrough;
    state.world_seed = seed;
}

/// Reset for New Game+ with a fresh world seed.
pub fn reset_for_new_world(state: &mut GameState) {
    let next_playthrough = state.playthrough_count + 1;
    *state = GameState::default();
    state.playthrough_count = next_playthrough;
    state.world_seed = rand::random::<u64>();
}

/// Reset for New Game+ with a specific custom seed (for seed sharing).
pub fn reset_for_custom_seed(state: &mut GameState, seed: u64) {
    let next_playthrough = state.playthrough_count + 1;
    *state = GameState::default();
    state.playthrough_count = next_playthrough;
    state.world_seed = seed;
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

/// Path to a specific profile save file (1-5).
pub fn profile_path(index: usize) -> PathBuf {
    exe_dir().join(format!("profile_{index}.json"))
}

/// Path to the current profile tracking file.
pub fn current_profile_path() -> PathBuf {
    exe_dir().join("current_profile.json")
}

/// Load the active profile index (1-5). Returns 1 if no profile file exists.
pub fn load_active_profile() -> usize {
    let path = current_profile_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .and_then(|v| v.get("active")?.as_u64())
        .map(|n| (n as usize).clamp(1, 5))
        .unwrap_or(1)
}

/// Save the active profile index (1-5).
pub fn save_active_profile(index: usize) {
    let path = current_profile_path();
    let json = format!("{{\"active\": {index}}}");
    let _ = fs::write(&path, json);
}

/// Load game state from disk, or return defaults for a new game.
/// On first load, generates a world seed if none exists.
pub fn load_game_state() -> GameState {
    let path = game_state_path();
    load_game_state_from(&path)
}

/// Load game state from a specific path.
pub fn load_game_state_from(path: &PathBuf) -> GameState {
    let mut state: GameState = fs::read_to_string(path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();
    // Generate world seed on first ever game
    if state.world_seed == 0 {
        state.world_seed = rand::random::<u64>();
        save_game_state_to(&state, path);
    }
    state
}

/// Save game state to disk (pretty-printed JSON).
/// Also syncs to the active profile file.
pub fn save_game_state(state: &GameState) {
    let path = game_state_path();
    save_game_state_to(state, &path);
    // Also sync to the active profile
    let active = load_active_profile();
    let prof = profile_path(active);
    if prof != path {
        save_game_state_to(state, &prof);
    }
}

/// Save game state to a specific path.
pub fn save_game_state_to(state: &GameState, path: &PathBuf) {
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = fs::write(path, json);
    }
}

/// Load game state for a specific profile (1-5).
pub fn load_profile_game_state(index: usize) -> GameState {
    let path = profile_path(index);
    if path.exists() {
        load_game_state_from(&path)
    } else {
        GameState::default()
    }
}

/// Save game state for a specific profile (1-5).
pub fn save_profile_game_state(index: usize, state: &GameState) {
    let path = profile_path(index);
    save_game_state_to(state, &path);
}

/// Delete a profile's save file.
pub fn delete_profile(index: usize) {
    let path = profile_path(index);
    let _ = fs::remove_file(&path);
}

/// Check if a profile save file exists and has meaningful progress.
pub fn profile_exists(index: usize) -> bool {
    let path = profile_path(index);
    path.exists()
}

/// Migrate legacy game_state.json to profile_1.json if no profiles exist yet.
pub fn migrate_legacy_save() {
    let legacy = game_state_path();
    let profile1 = profile_path(1);
    if legacy.exists() && !profile1.exists() {
        let _ = fs::copy(&legacy, &profile1);
        save_active_profile(1);
    }
}
