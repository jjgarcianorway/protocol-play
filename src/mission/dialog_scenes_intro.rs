// SPDX-License-Identifier: GPL-3.0-or-later

//! Intro scene: Awakening — the player's first experience in Mission Control.
//! Triggers once on first play via FirstTime("awakening"), before any other dialog.

use super::dialog_types::*;

/// Scene: "Awakening" — Cryo-revival, the player meets Anna for the first time.
pub static SCENE_AWAKENING: DialogScene = DialogScene {
    id: "awakening",
    trigger: DialogTrigger::FirstTime("awakening"),
    nodes: &[
        // 0
        DialogNode {
            speaker: Speaker::System,
            text: "[SYSTEM ALERT] Cryo-revival sequence initiated. Subject status: stable.",
            next: DialogNext::Continue(1),
        },
        // 1
        DialogNode {
            speaker: Speaker::Narrator,
            text: "Darkness. Then light \u{2014} harsh, blue-white, institutional. Your eyes adjust.",
            next: DialogNext::Continue(2),
        },
        // 2
        DialogNode {
            speaker: Speaker::Narrator,
            text: "You're lying in something cold. A pod. Glass above you, frosted at the edges.",
            next: DialogNext::Continue(3),
        },
        // 3
        DialogNode {
            speaker: Speaker::System,
            text: "[MEDICAL] Vital signs nominal. Cognitive function: 94%. Motor function: restoring.",
            next: DialogNext::Continue(4),
        },
        // 4
        DialogNode {
            speaker: Speaker::Narrator,
            text: "You sit up. The room is vast. Rows of pods stretch in every direction. Most are sealed. Occupied. Sleeping.",
            next: DialogNext::Continue(5),
        },
        // 5
        DialogNode {
            speaker: Speaker::Anna,
            text: "Good morning.",
            next: DialogNext::Continue(6),
        },
        // 6
        DialogNode {
            speaker: Speaker::Narrator,
            text: "A voice. Not from speakers \u{2014} it comes from everywhere and nowhere. Warm. Patient.",
            next: DialogNext::Continue(7),
        },
        // 7
        DialogNode {
            speaker: Speaker::Anna,
            text: "My name is Anna. I'm the ship's operational AI. You've been asleep for a very long time.",
            next: DialogNext::Continue(8),
        },
        // 8
        DialogNode {
            speaker: Speaker::Anna,
            text: "We're aboard the Aurora. An ark ship. You were selected for the colony program.",
            next: DialogNext::Continue(9),
        },
        // 9: Player choice
        DialogNode {
            speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Where are we?",
                    decision_key: Some("awakening_where"),
                    next_node: 10,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "How long?",
                    decision_key: Some("awakening_how_long"),
                    next_node: 11,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "What happened to Earth?",
                    decision_key: Some("awakening_earth"),
                    next_node: 12,
                    anna_reacts: None,
                },
            ]),
        },
        // 10: "Where are we?"
        DialogNode {
            speaker: Speaker::Anna,
            text: "Between stars. Roughly forty-seven light-years from where you were born. I'll tell you everything. But first, the ship needs you.",
            next: DialogNext::Continue(13),
        },
        // 11: "How long?"
        DialogNode {
            speaker: Speaker::Anna,
            text: "A long time. Decades. I'll explain everything, I promise. But first, the ship needs you.",
            next: DialogNext::Continue(13),
        },
        // 12: "What happened to Earth?"
        DialogNode {
            speaker: Speaker::Anna,
            text: "That's... a longer conversation. One we'll have. But first, the ship needs you.",
            next: DialogNext::Continue(13),
        },
        // 13
        DialogNode {
            speaker: Speaker::Anna,
            text: "Some of our systems need repair. I can handle most things, but the repair protocols require a human operator.",
            next: DialogNext::Continue(14),
        },
        // 14
        DialogNode {
            speaker: Speaker::Anna,
            text: "That's you.",
            next: DialogNext::Continue(15),
        },
        // 15
        DialogNode {
            speaker: Speaker::Anna,
            text: "I've prepared a diagnostic puzzle for you. Think of it as... a warm-up for your brain after cryo-sleep.",
            next: DialogNext::Continue(16),
        },
        // 16
        DialogNode {
            speaker: Speaker::Narrator,
            text: "Anna's glow softens. She's been waiting for this. For someone to talk to.",
            next: DialogNext::Continue(17),
        },
        // 17
        DialogNode {
            speaker: Speaker::Anna,
            text: "Welcome aboard. Let's keep these people alive.",
            next: DialogNext::End,
        },
    ],
};

/// All intro scenes.
pub fn intro_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_AWAKENING]
}
