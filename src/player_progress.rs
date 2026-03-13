// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::types::{TileKind, LevelData};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LevelProgress {
    pub completed: bool,
    #[serde(default)] pub creative_solution: bool,
    pub stats: ProgressStats,
    pub board_state: Option<Vec<(u32, u32, TileKind)>>,
    pub inventory_state: Option<Vec<(TileKind, u8)>>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProgressStats {
    pub editing_time: f32,
    pub play_count: u32,
    pub reset_count: u32,
}

#[derive(Resource)]
pub struct PlayerProgress {
    pub data: Vec<LevelProgress>,
    pub filenames: Vec<String>,
    pub save_dir: PathBuf,
}

pub fn save_one(progress: &PlayerProgress, index: usize) {
    let path = progress.save_dir.join(format!("{}.progress.json", progress.filenames[index]));
    if let Ok(json) = serde_json::to_string_pretty(&progress.data[index]) {
        let _ = fs::write(&path, json);
    }
}

pub fn load_progress(dir: &Path, filename: &str) -> LevelProgress {
    let path = dir.join(format!("{filename}.progress.json"));
    fs::read_to_string(&path).ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

pub fn first_unsolved(data: &[LevelProgress]) -> Option<usize> {
    data.iter().position(|p| !p.completed)
}

pub fn next_level(data: &[LevelProgress], from: usize, dir: i32) -> usize {
    let count = data.len() as i32;
    ((from as i32 + dir).rem_euclid(count)) as usize
}

pub fn next_unsolved(data: &[LevelProgress], from: usize, dir: i32) -> Option<usize> {
    let count = data.len() as i32;
    for i in 1..=count {
        let idx = ((from as i32 + dir * i).rem_euclid(count)) as usize;
        if !data[idx].completed { return Some(idx); }
    }
    None
}

pub fn reset_all_progress(dir: &Path, filenames: &[String]) {
    for f in filenames {
        let _ = fs::remove_file(dir.join(format!("{f}.progress.json")));
    }
    let _ = fs::remove_file(dir.join("stats.jsonl"));
    let _ = fs::remove_file(dir.join("stats.json"));
}

pub fn ensure_stats_file(dir: &Path) {
    let path = dir.join("stats.jsonl");
    if !path.exists() { let _ = fs::File::create(&path); }
}

#[derive(Serialize)]
struct LevelStatsSummary {
    name: String, filename: String, completed: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")] creative_solution: bool,
    editing_time_secs: f32, play_count: u32, reset_count: u32,
}

pub fn save_stats_summary(dir: &Path, filenames: &[String], levels: &[LevelData],
    data: &[LevelProgress], current_idx: usize, live: &ProgressStats,
) {
    let entries: Vec<LevelStatsSummary> = data.iter().enumerate().map(|(i, p)| {
        let s = if i == current_idx && !p.completed { live } else { &p.stats };
        LevelStatsSummary { name: levels[i].name.clone(), filename: filenames[i].clone(),
            completed: p.completed, creative_solution: p.creative_solution,
            editing_time_secs: s.editing_time, play_count: s.play_count, reset_count: s.reset_count }
    }).collect();
    if let Ok(json) = serde_json::to_string_pretty(&entries) {
        let _ = fs::write(dir.join("stats.json"), json);
    }
}

pub fn is_creative_solution(solution: &[(u32, u32, TileKind)], placed: &[(u32, u32, TileKind)]) -> bool {
    if solution.is_empty() { return false; }
    let mut sol = solution.to_vec(); sol.sort_by_key(|(c, r, _)| (*c, *r));
    let mut pl = placed.to_vec(); pl.sort_by_key(|(c, r, _)| (*c, *r));
    sol != pl
}

#[derive(Serialize)]
struct StatsLogEntry {
    level_name: String,
    filename: String,
    editing_time_secs: f32,
    play_count: u32,
    reset_count: u32,
    creative_solution: bool,
    timestamp: u64,
}

pub fn append_stats_log(dir: &Path, filename: &str, level_name: &str,
    stats: &ProgressStats, creative: bool,
) {
    let entry = StatsLogEntry {
        level_name: level_name.into(), filename: filename.into(),
        editing_time_secs: stats.editing_time, play_count: stats.play_count,
        reset_count: stats.reset_count, creative_solution: creative,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0),
    };
    let path = dir.join("stats.jsonl");
    if let Ok(json) = serde_json::to_string(&entry) {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&path) {
            let _ = writeln!(f, "{json}");
        }
    }
}
