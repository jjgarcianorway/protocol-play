// SPDX-License-Identifier: GPL-3.0-or-later
// Randomized in-game messages for simulation feedback and player stats.

use rand::Rng;

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
pub fn pick_creative_msg() -> &'static str {
    pick(&["Creative solution!", "You found a path the designer didn't plan!",
        "Unexpected approach — not the intended route!", "Your own way — and it works!",
        "Original solution discovered!"])
}

#[cfg(feature = "player")]
pub fn pick_congrats() -> (&'static str, &'static str) {
    let pairs = [
        ("Congratulations!", "All levels completed!"),
        ("You did it!", "Every single level — conquered!"),
        ("Mission Complete!", "The whole campaign — done!"),
        ("Outstanding!", "From First Steps to Protocol Complete!"),
    ];
    pairs[rand::thread_rng().gen_range(0..pairs.len())]
}

#[cfg(feature = "player")]
pub fn format_time(secs: u64) -> String {
    if secs >= 60 { format!("{}:{:02} of puzzle thinking", secs / 60, secs % 60) }
    else { format!("{} seconds — that was quick!", secs) }
}

#[cfg(feature = "player")]
pub fn format_attempts(play_count: u32) -> String {
    match play_count {
        1 => "First try!".into(), 2 => "Solved on the second attempt.".into(),
        3..=5 => format!("Cracked it on attempt {}.", play_count),
        _ => format!("Persistence pays off — attempt {}!", play_count),
    }
}

#[cfg(feature = "player")]
pub fn format_resets(reset_count: u32) -> String {
    match reset_count {
        1 => "1 fresh start along the way.".into(),
        _ => format!("{} resets — sometimes you need a clean slate.", reset_count),
    }
}
