// SPDX-License-Identifier: GPL-3.0-or-later

//! Profile management — data types and save/load logic for up to 5 player profiles.

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::save_state::{
    load_active_profile, load_profile_game_state, profile_exists, profile_path,
    save_active_profile, save_game_state_to, save_profile_game_state,
    delete_profile as fs_delete_profile, migrate_legacy_save, GameState,
};
use super::world_seed;

/// Maximum number of profiles (family members).
pub const MAX_PROFILES: usize = 5;

/// Default profile names.
const DEFAULT_NAMES: [&str; MAX_PROFILES] = [
    "Player 1", "Player 2", "Player 3", "Player 4", "Player 5",
];

/// Tracks the active profile index and manages profile switching.
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct ProfileManager {
    pub active_profile: usize, // 1-5
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self { active_profile: 1 }
    }
}

/// Summary information about a profile slot, used for the selection screen.
#[derive(Clone, Debug)]
pub struct ProfileInfo {
    pub name: String,
    pub exists: bool,
    pub bot_level: u32,
    pub collapse_type: String,
    pub crew_discovered: usize,
    pub day: u32,
    #[allow(dead_code)]
    pub world_seed: u64,
}

impl Default for ProfileInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            exists: false,
            bot_level: 0,
            collapse_type: String::new(),
            crew_discovered: 0,
            day: 0,
            world_seed: 0,
        }
    }
}

/// Load summary info for a single profile slot.
pub fn load_profile_info(index: usize) -> ProfileInfo {
    if !profile_exists(index) {
        return ProfileInfo {
            name: DEFAULT_NAMES[index - 1].to_string(),
            ..Default::default()
        };
    }

    let gs = load_profile_game_state(index);
    let has_progress = gs.day > 1 || gs.bot_level > 0 || gs.total_crystals_gathered > 0;

    let collapse_type = if gs.world_seed != 0 {
        let world = world_seed::generate_world(gs.world_seed);
        world.earth_collapse.primary_cause.name().to_string()
    } else {
        String::new()
    };

    // Try to load the profile name from the profile metadata
    let name = load_profile_name(index)
        .unwrap_or_else(|| DEFAULT_NAMES[index - 1].to_string());

    ProfileInfo {
        name,
        exists: has_progress,
        bot_level: gs.bot_level,
        collapse_type,
        crew_discovered: gs.discovered_crew.len(),
        day: gs.day,
        world_seed: gs.world_seed,
    }
}

/// Load all profile summaries (indices 1 through MAX_PROFILES).
pub fn load_all_profiles() -> Vec<ProfileInfo> {
    (1..=MAX_PROFILES).map(load_profile_info).collect()
}

/// Activate a profile: copy its save to game_state.json and set it as active.
pub fn activate_profile(index: usize) {
    save_active_profile(index);

    // Copy profile save to game_state.json so the rest of the game works unchanged
    let gs = load_profile_game_state(index);
    let legacy_path = crate::save_state::game_state_path();
    save_game_state_to(&gs, &legacy_path);
}

/// Sync the current game_state.json back to the active profile.
#[allow(dead_code)]
pub fn sync_to_active_profile() {
    let index = load_active_profile();
    let gs = crate::save_state::load_game_state();
    save_profile_game_state(index, &gs);
}

/// Delete a profile's save data.
pub fn delete_profile(index: usize) {
    fs_delete_profile(index);
    delete_profile_name(index);
}

/// Initialize the profile system — migrate legacy saves if needed.
pub fn init_profiles() -> ProfileManager {
    migrate_legacy_save();
    let active = load_active_profile();
    ProfileManager { active_profile: active }
}

// === Profile name persistence ===
// Names are stored in a small JSON sidecar: profile_{n}_meta.json

fn profile_meta_path(index: usize) -> std::path::PathBuf {
    crate::save_state::exe_dir().join(format!("profile_{index}_meta.json"))
}

/// Load the custom name for a profile, if set.
pub fn load_profile_name(index: usize) -> Option<String> {
    let path = profile_meta_path(index);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .and_then(|v| v.get("name")?.as_str().map(|s| s.to_string()))
}

/// Save a custom name for a profile.
pub fn save_profile_name(index: usize, name: &str) {
    let path = profile_meta_path(index);
    let json = format!("{{\"name\": {}}}", serde_json::json!(name));
    let _ = std::fs::write(&path, json);
}

/// Delete profile metadata (name).
fn delete_profile_name(index: usize) {
    let path = profile_meta_path(index);
    let _ = std::fs::remove_file(&path);
}
