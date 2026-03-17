// SPDX-License-Identifier: GPL-3.0-or-later
//! Stats persistence for The Gathering — saves session results and best scores.

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Serialize, Deserialize)]
struct SessionStats {
    distance_au: f32,
    time_days: u32,
    crystals: u64,
    hits_taken: u32,
    elapsed_secs: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BestStatsFile {
    pub best_distance_au: f32,
    pub best_crystals: u64,
    pub best_time_days: u32,
    pub total_sessions: u32,
}

fn stats_dir() -> std::path::PathBuf {
    std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}

pub fn load_best() -> BestStats {
    let path = stats_dir().join("stats_gathering_best.json");
    if let Ok(data) = std::fs::read_to_string(&path) {
        if let Ok(file) = serde_json::from_str::<BestStatsFile>(&data) {
            return BestStats {
                best_distance_au: file.best_distance_au,
                best_crystals: file.best_crystals,
                best_time_days: file.best_time_days,
                total_sessions: file.total_sessions,
            };
        }
    }
    BestStats::default()
}

fn save_best(best: &BestStats) {
    let file = BestStatsFile {
        best_distance_au: best.best_distance_au,
        best_crystals: best.best_crystals,
        best_time_days: best.best_time_days,
        total_sessions: best.total_sessions,
    };
    let dir = stats_dir();
    if let Ok(json) = serde_json::to_string_pretty(&file) {
        let _ = std::fs::write(dir.join("stats_gathering_best.json"), json);
    }
}

/// Save session stats on game over. Returns whether any new record was set.
pub fn save_session(state: &ShipState, best: &mut BestStats) -> bool {
    let stats = SessionStats {
        distance_au: state.distance * 0.01,
        time_days: (state.elapsed_time * 0.1) as u32,
        crystals: state.crystals,
        hits_taken: state.hits_taken,
        elapsed_secs: state.elapsed_time,
    };
    let dir = stats_dir();

    // Append to log
    if let Ok(json) = serde_json::to_string(&stats) {
        let log_path = dir.join("stats_gathering.jsonl");
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{json}");
        }
    }

    // Check for new records
    let mut new_record = false;
    best.total_sessions += 1;
    if stats.distance_au > best.best_distance_au {
        best.best_distance_au = stats.distance_au;
        new_record = true;
    }
    if stats.crystals > best.best_crystals {
        best.best_crystals = stats.crystals;
        new_record = true;
    }
    if stats.time_days > best.best_time_days {
        best.best_time_days = stats.time_days;
        new_record = true;
    }

    save_best(best);
    new_record
}
