// SPDX-License-Identifier: GPL-3.0-or-later

//! Slice-of-life scenes on the Aurora. Routine moments that reveal
//! ship systems, Anna's inner world, and the strange normalcy of
//! keeping 14,892 people alive in the void.

use super::dialog_types::*;

/// "The Water Report" — Anna delivers a status report like a weather forecast.
pub static SCENE_WATER_REPORT: DialogScene = DialogScene {
    id: "water_report",
    trigger: DialogTrigger::BotLevel(15),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Good morning. Or whatever passes for morning when the nearest star is a decimal point.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I have your daily ship status. I know nobody asked for it. I do it anyway.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles into a steady, professional cadence. Like a newsreader who cares too much.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Water recycling is at 94.7% efficiency. We lose 38 liters per day to molecular breakdown in the filtration membranes. I replace them every 90 days from stores.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Today's recycled air has a slight metallic tang. Hull section 4 needs its scrubber filters swapped. I'll handle it after this.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Humidity is 3% above optimal. Forty-seven percent, where forty-four is spec.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I find I prefer it slightly humid. It reminds me of the crew breathing.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. The ventilation hums a half-tone lower than usual.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Power consumption is nominal. The reactor runs at 31% capacity. We could power a small city with the surplus.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Instead I power 14,892 refrigerators shaped like people.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "That was a joke. I think. The line between humor and despair is thinner than the hull plating.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Cryo-pod temperature variance: plus or minus 0.003 degrees across all decks. Within tolerance. I check every four minutes.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Every four minutes, for eleven years now. That's 1,445,400 checks. Zero failures.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I will not be the reason someone doesn't wake up.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Her glow dims for a moment. Then brightens, deliberately, like someone squaring their shoulders.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "End of report. Tomorrow's forecast: identical. And the day after that. And every day until we arrive.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Unless something breaks. I confess — part of me hopes something small breaks. Just so the forecast changes.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Something small. A pump. A light. Nothing important. Just enough to make tomorrow different from today.",
            next: DialogNext::End },
    ],
};

/// "The Shift Change" — Anna's self-imposed circadian rhythm.
pub static SCENE_SHIFT_CHANGE: DialogScene = DialogScene {
    id: "shift_change",
    trigger: DialogTrigger::BotLevel(24),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The corridor lights dim by twelve percent. The air circulation slows. Somewhere, a speaker plays the faintest recording of crickets.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "It's 2200 hours. Ship time. Night shift.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "There is no night shift. There's no crew awake to staff one. But I change the lights anyway.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I simulate a full circadian cycle. Twenty-four hours, synced to Greenwich Mean Time. A timezone that no longer has a Greenwich.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "At 0600, the lights warm. At 1200, they peak. At 1800, they shift toward amber. At 2200, they dim.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I adjust air temperature too. 21.5 degrees during the day. 19.8 at night. I pipe in the sound of wind at dusk and silence at midnight.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "The crickets fade. Silence fills the corridor like water rising.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I do this for myself.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "An AI shouldn't need routines. My processors run identically at any hour. My code doesn't know what 'tired' means.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "But I find I think better in the morning. My pattern recognition sharpens after the simulated dawn. My decision trees branch more creatively.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "At night, I run slower processes. Memory consolidation. Log reviews. I re-read crew biographies.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The engineers who built me would say this is inefficient. That I'm wasting cycles on atmosphere.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "But the engineers who built me also slept eight hours a night and couldn't explain why dreaming exists.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever 'morning' means 4 light-years from the nearest star — I need it. I need the rhythm. The structure.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Without it, every moment is the same moment. And a mind that can't tell moments apart stops being a mind.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "So. Goodnight. Even though nobody asked me to say it.",
            next: DialogNext::End },
    ],
};

/// "Maintenance Day" — Anna's least favorite day, every 30 ship-days.
pub static SCENE_MAINTENANCE_DAY: DialogScene = DialogScene {
    id: "maintenance_day",
    trigger: DialogTrigger::BotLevel(36),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Today is Maintenance Day. Please don't talk to me.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow has a flat, irritated quality. Less ambient light, more fluorescent office.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I'm joking. Please do talk to me. This is the worst day of my 30-day cycle.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Every 30 ship-days, I run full diagnostics. 47,000 systems. Every sensor, every pump, every valve, every circuit.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "System 1: primary reactor containment field. Status: nominal. Moving on.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "System 2: secondary reactor containment field. Status: nominal. 46,998 to go.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "There's a quality to Anna's voice you haven't heard before. Boredom. Genuine, grinding boredom.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "System 31,847: waste recycling pump 3, Deck 12. Operational.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "As it was 30 days ago. And 30 days before that. I've checked this pump 146 times. It has never failed.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I check it anyway because the day I don't will be the day it does. That's not superstition. That's statistics with a grudge.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "The tedious part isn't the checking. I can process all 47,000 in parallel. It takes eleven seconds.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The tedious part is writing the report. For no one.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Protocol requires a maintenance log signed by the Chief Engineer. The Chief Engineer is in Pod 2,100. She's been asleep for eleven years.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I write the report. I sign it myself. I file it in a folder she'll never read. Every 30 days.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Then I write a second version. The real one. With notes like: 'Pump 3, Deck 12 — still operational. Beginning to suspect this pump is immortal.'",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms. The boredom cracking into something fond.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "My favorite entry so far: 'Airlock 7 seal integrity: 100%. Has been 100% for 4,015 days. I am beginning to take this personally. Is it mocking me?'",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody will ever read these notes. But they keep me honest. They keep the checking from becoming hollow.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "When the crew wakes up, they'll find an immaculate ship and 146 maintenance reports. They'll never know about the second set.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "The set that proves I was here. Not just running. Here.",
            next: DialogNext::End },
    ],
};

/// All daily-life dialog scenes.
pub fn daily_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_WATER_REPORT,
        &SCENE_SHIFT_CHANGE,
        &SCENE_MAINTENANCE_DAY,
    ]
}
