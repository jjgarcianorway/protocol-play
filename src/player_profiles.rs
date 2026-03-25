// SPDX-License-Identifier: GPL-3.0-or-later
//! Profile system — 3 save slots with subdirectory-based progress isolation.
//! Each profile stores its .progress.json, stats.json, and stats.jsonl files
//! in a dedicated `profile_{slot}/` subdirectory next to the executable.

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use crate::save_state::exe_dir;

// ─── Constants ──────────────────────────────────────────────────────────────

pub const NUM_PROFILES: u8 = 3;
const ACTIVE_PROFILE_FILE: &str = "active_profile.json";

/// File extensions/suffixes that belong to a profile's progress.
const PROGRESS_SUFFIXES: &[&str] = &[".progress.json"];
const STATS_FILES: &[&str] = &["stats.json", "stats.jsonl"];

// ─── Resource ───────────────────────────────────────────────────────────────

/// Tracks which profile slot (1–3) is currently active.
#[derive(Resource, Debug, Clone)]
pub struct ProfileState {
    pub active_slot: u8,
}

impl Default for ProfileState {
    fn default() -> Self { Self { active_slot: 1 } }
}

/// Summary info for displaying a profile slot in the UI.
pub struct ProfileInfo {
    pub slot: u8,
    pub exists: bool,
    pub levels_completed: u32,
    pub total_levels: u32,
}

// ─── Active profile persistence ─────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
struct ActiveProfileData {
    slot: u8,
}

/// Returns the directory for a given profile slot: `exe_dir/profile_{slot}/`.
pub fn profile_dir(slot: u8) -> PathBuf {
    exe_dir().join(format!("profile_{slot}"))
}

/// Read which slot is active from `active_profile.json`. Defaults to 1.
pub fn load_active_slot() -> u8 {
    let path = exe_dir().join(ACTIVE_PROFILE_FILE);
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str::<ActiveProfileData>(&s).ok())
        .map(|d| d.slot.clamp(1, NUM_PROFILES))
        .unwrap_or(1)
}

/// Persist the active slot to `active_profile.json`.
pub fn save_active_slot(slot: u8) {
    let path = exe_dir().join(ACTIVE_PROFILE_FILE);
    let data = ActiveProfileData { slot };
    if let Ok(json) = serde_json::to_string_pretty(&data) {
        let _ = fs::write(path, json);
    }
}

// ─── Profile info ───────────────────────────────────────────────────────────

/// Count campaign levels (NN_NN_*.json files) in the exe directory.
pub fn count_total_levels() -> u32 {
    let dir = exe_dir();
    fs::read_dir(&dir)
        .ok()
        .map(|entries| {
            entries.filter_map(|e| e.ok()).filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                is_campaign_level(&name)
            }).count() as u32
        })
        .unwrap_or(0)
}

/// Gather display info for a profile slot.
pub fn get_profile_info(slot: u8) -> ProfileInfo {
    let dir = profile_dir(slot);
    let total = count_total_levels();
    if !dir.exists() {
        return ProfileInfo { slot, exists: false, levels_completed: 0, total_levels: total };
    }
    let completed = fs::read_dir(&dir)
        .ok()
        .map(|entries| {
            entries.filter_map(|e| e.ok()).filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                name.ends_with(".progress.json")
            }).filter(|e| {
                // Check if level is actually completed
                fs::read_to_string(e.path()).ok()
                    .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
                    .and_then(|v| v.get("completed")?.as_bool())
                    .unwrap_or(false)
            }).count() as u32
        })
        .unwrap_or(0);
    ProfileInfo { slot, exists: true, levels_completed: completed, total_levels: total }
}

// ─── Activate / deactivate ──────────────────────────────────────────────────

/// Copy progress files from a profile's subdirectory into the working (exe) dir.
/// This makes the profile's progress "live" for the game engine to read.
pub fn activate_profile(slot: u8) {
    let src = profile_dir(slot);
    let dst = exe_dir();
    if !src.exists() {
        let _ = fs::create_dir_all(&src);
        return; // Empty profile — nothing to copy
    }
    copy_progress_files(&src, &dst);
}

/// Copy progress files from the working (exe) dir back into the profile's subdirectory.
/// This saves the current in-memory/on-disk state into the profile.
pub fn deactivate_profile(slot: u8) {
    let src = exe_dir();
    let dst = profile_dir(slot);
    let _ = fs::create_dir_all(&dst);
    copy_progress_files(&src, &dst);
}

/// Switch from one profile to another: save old, load new.
pub fn switch_profile(from: u8, to: u8) {
    if from == to { return; }
    deactivate_profile(from);
    // Clear working dir progress files before activating new profile
    clear_working_progress();
    activate_profile(to);
    save_active_slot(to);
}

/// Delete a profile's subdirectory and all its contents.
pub fn delete_profile(slot: u8) {
    let dir = profile_dir(slot);
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
}

// ─── Startup ────────────────────────────────────────────────────────────────

/// Called once at startup to ensure the active profile's data is in the working dir.
/// Also handles first-launch migration: if no profile dirs exist but progress files
/// are in the exe dir, migrate them to profile 1.
pub fn init_profiles() -> ProfileState {
    let slot = load_active_slot();
    let dir = profile_dir(slot);

    // First launch migration: progress files in exe dir but no profile dirs
    if !profile_dir(1).exists() && !profile_dir(2).exists() && !profile_dir(3).exists() {
        let exe = exe_dir();
        if has_progress_files(&exe) {
            let _ = fs::create_dir_all(profile_dir(1));
            copy_progress_files(&exe, &profile_dir(1));
            save_active_slot(1);
            return ProfileState { active_slot: 1 };
        }
    }

    // Ensure profile dir exists
    let _ = fs::create_dir_all(&dir);
    // Copy profile's progress into working dir
    activate_profile(slot);

    ProfileState { active_slot: slot }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Check if a filename is a campaign level file (NN_NN_name.json).
fn is_campaign_level(name: &str) -> bool {
    name.ends_with(".json")
        && !name.contains(".progress.")
        && name.len() > 6
        && name.as_bytes().get(2) == Some(&b'_')
        && name.as_bytes().first().is_some_and(|b| b.is_ascii_digit())
}

/// Copy .progress.json + stats files between directories.
fn copy_progress_files(src: &PathBuf, dst: &PathBuf) {
    if let Ok(entries) = fs::read_dir(src) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            let dominated = PROGRESS_SUFFIXES.iter().any(|s| name.ends_with(s))
                || STATS_FILES.iter().any(|s| name == *s)
                || name == "player_settings.json"
                || name == "game_state.json";
            if dominated {
                let _ = fs::copy(entry.path(), dst.join(&name));
            }
        }
    }
}

/// Check if the working dir has any .progress.json files.
fn has_progress_files(dir: &PathBuf) -> bool {
    fs::read_dir(dir)
        .ok()
        .map(|entries| {
            entries.filter_map(|e| e.ok())
                .any(|e| e.file_name().to_string_lossy().ends_with(".progress.json"))
        })
        .unwrap_or(false)
}

/// Remove all .progress.json and stats files from the working dir.
fn clear_working_progress() {
    let dir = exe_dir();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            let dominated = PROGRESS_SUFFIXES.iter().any(|s| name.ends_with(s))
                || STATS_FILES.iter().any(|s| name == *s);
            if dominated {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}

// ─── Menu UI components ─────────────────────────────────────────────────────

/// Marker for the profile selector row in the main menu.
#[derive(Component)]
pub struct ProfileSelectorRow;

/// Marker for an individual profile slot button. Stores the slot number (1–3).
#[derive(Component)]
pub struct ProfileSlotBtn(pub u8);

/// Marker for the profile slot label text (so we can update it).
#[derive(Component)]
pub struct ProfileSlotLabel(pub u8);

/// Spawn the profile selector row: three small slot buttons.
pub fn spawn_profile_selector(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    active_slot: u8,
    t: &crate::i18n::Translations,
) {
    use crate::ui_theme::{palette, typo, spacing};

    parent.spawn((
        ProfileSelectorRow,
        Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
    )).with_children(|row| {
        // Label
        row.spawn((
            Text::new(t.ui_or("profile", "Profile")),
            TextFont { font: font.clone(), font_size: typo::SMALL, ..default() },
            TextColor(palette::TEXT_MUTED),
        ));
        // 3 slot buttons
        for slot in 1..=NUM_PROFILES {
            let active = slot == active_slot;
            let info = get_profile_info(slot);
            let label = if active {
                format!("{slot}")
            } else if info.exists {
                format!("{slot} ({})", info.levels_completed)
            } else {
                format!("{slot}")
            };
            let bg = if active { palette::PRIMARY } else { palette::INACTIVE };
            let hover = if active { palette::PRIMARY_HOVER } else { palette::OUTLINE_HOVER };
            row.spawn((
                Button,
                ProfileSlotBtn(slot),
                crate::ui_theme::Hoverable::simple(bg, hover),
                Node {
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                    border_radius: BorderRadius::all(Val::Px(spacing::RADIUS_SM)),
                    min_width: Val::Px(36.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(bg),
            )).with_child((
                Text::new(label),
                TextFont { font: font.clone(), font_size: 13.0, ..default() },
                TextColor(if active { palette::PRIMARY_TEXT } else { palette::TEXT_MUTED }),
                ProfileSlotLabel(slot),
            ));
        }
    });
}
