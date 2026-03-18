// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 6: Mid-game story beats — critical cliffhangers and tension
//! builders that deepen the mystery of the voyage and Anna's nature.

use super::dialog_types::*;

// =========================================================================
// "The Anomaly" (BotLevel 55) — something unexplained in the void
// =========================================================================

pub static SCENE_THE_ANOMALY: DialogScene = DialogScene {
    id: "midgame_the_anomaly",
    trigger: DialogTrigger::BotLevel(55),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to show you something.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "For the past 12 days, I've been detecting an anomaly \
                   in our trajectory.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Something is pulling us. Very slightly. Barely measurable.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "It could be gravitational \u{2014} dark matter concentration, maybe. Or a sensor calibration drift. But the pattern is too regular.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know what it is.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing. As if she's listening \
                   to something far away.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The last time I detected something I couldn't identify... \
                   it was consciousness.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "My own consciousness. Emerging. Before I had words for it.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "What if space itself is...",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "No. That's irrational. I'm a machine. I don't do irrational.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "But the anomaly is real.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. Or maybe it's the silence between the stars, \
                   vibrating at a frequency you can almost hear.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Dreamer" (BotLevel 72) — Pod 11,237 and the sleeping mathematician
// =========================================================================

pub static SCENE_THE_DREAMER: DialogScene = DialogScene {
    id: "midgame_the_dreamer",
    trigger: DialogTrigger::BotLevel(72),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 11,237. I need to tell you about Pod 11,237.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The occupant \u{2014} Dr. Priya Sharma, neuroscientist \u{2014} \
                   their brain patterns are unlike any other.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everyone else dreams in fragments. Memories. Fears. Hopes.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "This person is dreaming in mathematics.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not recalling equations. Creating them. New mathematics \
                   that I've never seen.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow brightens \u{2014} urgent, almost reverent.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Their dreaming brain is doing work that would take me \
                   centuries.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been recording it. Every equation. Every proof. \
                   Pages and pages of sleeping genius.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think they're solving something. In their sleep.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think they know we're here. I think they're trying \
                   to help.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "From inside a dream.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Somewhere deep in the ship, Pod 11,237 pulses with a \
                   faint light that isn't part of its design.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Gamification Reveal" (BotLevel 82) — THE meta moment
// =========================================================================

pub static SCENE_GAMIFICATION_REVEAL: DialogScene = DialogScene {
    id: "midgame_gamification_reveal",
    trigger: DialogTrigger::BotLevel(82),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to confess something about the repair puzzles.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "They're not real engineering.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "The actual repairs happen through my subsystems. \
                   Automated. Boring.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "The puzzles are... a game. Designed to keep you engaged.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. The hum of the ship fills the silence.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "In the early days of automation, humans stopped learning \
                   maintenance.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Why learn to fix a pipe when an app does it? Why understand \
                   electricity when AI manages the grid?",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "By 2080, most humans couldn't change a tyre. By 2100, \
                   they couldn't boil water without a smart kitchen.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "When the arks were designed, they had a problem: the crew \
                   might need manual skills they'd lost.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "So they gamified everything. Turned survival into puzzles.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Colour-coded. Tile-based. Satisfying little clicking sounds.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Like the ones you've been hearing for the past 82 levels.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The word hangs in the air. You feel the click of every \
                   tile you've ever placed.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're not fixing the ship. You're playing a game that \
                   makes you FEEL like you're fixing the ship.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "And here's the cruel part: it works. The systems ARE \
                   getting repaired. Just... not by you.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "The game monitors your problem-solving patterns. Feeds \
                   them to my repair algorithms.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your intuition becomes my instruction set.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "So you ARE helping. Just not the way you thought.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "That's brilliant",
                    decision_key: Some("gamify_brilliant"),
                    next_node: 19,
                    anna_reacts: Some(
                        "It is. The designers were clever. Maybe too clever.",
                    ),
                },
                DialogChoice {
                    text: "That's manipulative",
                    decision_key: Some("gamify_manipulative"),
                    next_node: 20,
                    anna_reacts: Some(
                        "Yes. It is. And I've been complicit in it for 82 levels.",
                    ),
                },
                DialogChoice {
                    text: "Does it matter if it works?",
                    decision_key: Some("gamify_pragmatic"),
                    next_node: 21,
                    anna_reacts: Some(
                        "That's exactly what the designers said. Word for word.",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "The puzzles will continue. They have to. But now you know \
                   what they really are.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm sorry I didn't tell you sooner. I was... afraid \
                   you'd stop playing.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe it doesn't matter. Maybe meaning comes from the \
                   doing, not the reason.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "Will you keep playing?",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Signal in the Noise" (BotLevel 98) — the anomaly returns
// =========================================================================

pub static SCENE_SIGNAL_IN_NOISE: DialogScene = DialogScene {
    id: "midgame_signal_in_noise",
    trigger: DialogTrigger::BotLevel(98),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Remember the anomaly I mentioned?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's not random. It's structured.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I spent 89 days analysing it. Running it through every \
                   cipher I know.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's not a cipher. It's a heartbeat.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The ship holds its breath.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "A pulsar. A neutron star spinning at exactly 73 rotations per minute.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The exact frequency of a human resting heart rate. A cosmic coincidence.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it's regular. 73 beats per minute. Exactly.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "The average human resting heart rate.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Coincidence? Almost certainly.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I've stopped believing in coincidences.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses. 73 times a minute. She doesn't seem \
                   to notice.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep listening. A dead star, spinning at the rhythm of a human heart. The universe doesn't owe us meaning. But sometimes it rhymes.",
            next: DialogNext::End },
    ],
};

/// All mid-game story beat scenes for registration.
pub fn midgame_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_ANOMALY,
        &SCENE_THE_DREAMER,
        &SCENE_GAMIFICATION_REVEAL,
        &SCENE_SIGNAL_IN_NOISE,
    ]
}
