// SPDX-License-Identifier: GPL-3.0-or-later
// Randomized in-game messages for simulation feedback and player stats.

use rand::Rng;
#[cfg(feature = "player")]
use crate::i18n::Translations;

fn pick<'a>(msgs: &'a [&'a str]) -> &'a str { msgs[rand::thread_rng().gen_range(0..msgs.len())] }

pub fn pick_error_msg(crushed: bool) -> &'static str {
    if crushed { pick(&["Bot was crushed by a door!", "Squished! Watch the door timing.",
        "That door had other plans.", "Crunch! Wrong place, wrong time."])
    } else { pick(&["Bot fell off the board!", "Lost a bot to the void!",
        "One bot took a wrong turn... off the edge.", "Bot vanished into the abyss!"]) }
}

pub fn pick_success_msg(bot_count: usize, pieces_left: usize, in_test: bool) -> String {
    if in_test && pieces_left > 0 {
        return pick(&["Solved with pieces to spare!", "Done — and you didn't even need everything!",
            "Clean solve with leftovers!", "Efficient! You had pieces to spare.",
            "That's one way to do it — extra tiles and all!"]).into();
    }
    if bot_count == 1 {
        return pick(&["Bot reached its goal!", "Nailed it!", "Safe and sound.",
            "Home at last!", "Delivered!"]).into();
    }
    if rand::thread_rng().gen_range(0..6) == 5 { return format!("All {} bots found their way!", bot_count); }
    pick(&["All bots reached their goals!", "Full house — every bot delivered!", "Clean sweep!",
        "All bots home safe.", "Perfectly routed!"]).into()
}

#[cfg(feature = "player")]
pub fn pick_creative_msg(t: &Translations) -> String {
    const MSGS: &[&str] = &[
        "Creative solution!",
        "You found a path the designer didn't plan!",
        "Unexpected approach — not the intended route!",
        "Your own way — and it works!",
        "Original solution discovered!",
    ];
    let idx = rand::thread_rng().gen_range(0..MSGS.len());
    t.ui(&format!("creative.{idx}"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| MSGS[idx].to_string())
}

#[cfg(feature = "player")]
pub fn pick_congrats(t: &Translations) -> (String, String) {
    const PAIRS: &[(&str, &str)] = &[
        ("Congratulations!", "All levels completed!"),
        ("You did it!", "Every single level — conquered!"),
        ("Mission Complete!", "The whole campaign — done!"),
        ("Outstanding!", "From First Steps to Protocol Complete!"),
    ];
    let idx = rand::thread_rng().gen_range(0..PAIRS.len());
    let title = t.ui(&format!("congrats.{idx}.title"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| PAIRS[idx].0.to_string());
    let body = t.ui(&format!("congrats.{idx}.body"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| PAIRS[idx].1.to_string());
    (title, body)
}

#[cfg(feature = "player")]
pub fn format_time(secs: u64, t: &Translations) -> String {
    if secs >= 60 {
        let tmpl = t.ui_or("minutes_thinking", "{}:{:02} of puzzle thinking");
        tmpl.replacen("{}", &(secs / 60).to_string(), 1)
            .replacen("{:02}", &format!("{:02}", secs % 60), 1)
    } else {
        let tmpl = t.ui_or("seconds_quick", "{} seconds — that was quick!");
        tmpl.replacen("{}", &secs.to_string(), 1)
    }
}

#[cfg(feature = "player")]
pub fn format_attempts(play_count: u32, t: &Translations) -> String {
    match play_count {
        1 => t.ui_or("first_try", "First try!").to_string(),
        2 => t.ui_or("second_attempt", "Solved on the second attempt.").to_string(),
        3..=5 => {
            let tmpl = t.ui_or("attempt_n", "Cracked it on attempt {}.");
            tmpl.replacen("{}", &play_count.to_string(), 1)
        }
        _ => {
            let tmpl = t.ui_or("persistence", "Persistence pays off — attempt {}!");
            tmpl.replacen("{}", &play_count.to_string(), 1)
        }
    }
}

#[cfg(feature = "player")]
pub fn format_resets(reset_count: u32, t: &Translations) -> String {
    match reset_count {
        1 => t.ui_or("fresh_start", "1 fresh start along the way.").to_string(),
        _ => {
            let tmpl = t.ui_or("resets_n", "{} resets — sometimes you need a clean slate.");
            tmpl.replacen("{}", &reset_count.to_string(), 1)
        }
    }
}

#[cfg(feature = "player")]
pub fn format_stars(stars: u8, t: &Translations) -> String {
    match stars {
        3 => t.ui_or("perfect_solve", "★★★ — Perfect solve!").to_string(),
        2 => "★★☆".into(),
        _ => "★☆☆".into(),
    }
}

#[cfg(feature = "player")]
pub fn format_solution_count(count: u32, t: &Translations) -> String {
    match count {
        1 => t.ui_or("unique_solution", "One unique solution — you found it.").to_string(),
        2 => t.ui_or("two_paths", "Two paths through this puzzle.").to_string(),
        n => {
            let tmpl = t.ui_or("n_solutions", "{} possible solutions exist.");
            tmpl.replacen("{}", &n.to_string(), 1)
        }
    }
}

#[cfg(feature = "player")]
pub fn star_label(stars: u8) -> &'static str {
    match stars { 3 => "★★★", 2 => "★★☆", 1 => "★☆☆", _ => "" }
}
