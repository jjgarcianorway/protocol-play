// SPDX-License-Identifier: GPL-3.0-or-later

//! Dr. Elena Vasquez — "The Doctor Who Chose" (Part 2)
//! "Elena and Amira" connection scene. Requires elena_list_seen + amiras_water.

use super::dialog_types::*;

// =========================================================================
// "Elena and Amira" — BotLevel 105
// Requires elena_list_seen AND one of the Amira endings.
// Elena passed Leyla (Amira's daughter) despite the age minimum.
// Neither woman knows about the other.
// =========================================================================

static ELENA_AMIRA_REQS_A: &[&str] = &["elena_list_seen", "amira_hopeful"];
static ELENA_AMIRA_REQS_B: &[&str] = &["elena_list_seen", "amira_realist"];
static ELENA_AMIRA_REQS_C: &[&str] = &["elena_list_seen", "amira_next_gen"];

/// Shared node content for all three trigger variants.
macro_rules! elena_amira_nodes {
    () => { &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I found a connection. Between two people who have \
                   never met.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Elena Vasquez. Pod 2,115. And Dr. Amira Hassan. \
                   Pod 4,231.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Remember Elena's list? Entry 603. The seven-year-old \
                   who draws rivers she's never seen.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "That's Leyla. Amira's daughter.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to deep blue \u{2014} river blue, \
                   crayon blue, the colour of imagined water.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla was seven. The age minimum was eighteen. The \
                   algorithm rejected her instantly.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Elena saw the drawings in Leyla's bag. Blue crayon \
                   rivers with fish and boats. Yellow suns above green \
                   banks. Rivers that didn't exist anymore.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She stamped PASS. And wrote: \"Seven years old. \
                   Draws rivers she's never seen. I couldn't be the \
                   person who stopped her from seeing one.\"",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "But here's what I didn't tell you before.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Amira hacked the boarding queue. Moved herself and \
                   Leyla up 12,000 positions. I found the trace in the \
                   system logs.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "The hack got them to Elena's station. But the hack \
                   alone wouldn't have been enough. The algorithm still \
                   would have stopped Leyla at the gate.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "It took both of them. Amira's desperation from one \
                   side. Elena's conscience from the other.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Neither knows about the other.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. Somewhere in the cryo deck, Pod 2,115 \
                   and Pod 4,231 are separated by two thousand pods and \
                   a secret neither occupant carries.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Amira thinks her hack saved Leyla. Elena thinks her \
                   stamp saved Leyla. The truth is messier. The truth \
                   is always messier.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Two strangers, acting alone, saved the same child for \
                   different reasons. One broke the rules from outside. \
                   One broke them from inside.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And Leyla sleeps in Pod 4,232. Dreaming in blue crayon. \
                   Not knowing that two women she's never met bent the \
                   entire system around her.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "When she wakes up \u{2014} if we get there \u{2014} \
                   should I tell her?",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Tell her. She deserves to know who fought \
                           for her.",
                    decision_key: Some("elena_amira_tell"),
                    next_node: 19,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Don't. Let her think she just got lucky.",
                    decision_key: Some("elena_amira_quiet"),
                    next_node: 22,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Let Elena and Amira decide together.",
                    decision_key: Some("elena_amira_together"),
                    next_node: 25,
                    anna_reacts: None,
                },
            ]) },
        // 19 — Tell her
        DialogNode { speaker: Speaker::Anna,
            text: "She deserves to know. That's what you think.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe you're right. Maybe knowing that strangers fought \
                   for you is the kind of thing that makes a person fight \
                   for strangers.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Blue crayon rivers. Maybe she'll build real ones.",
            next: DialogNext::EndWithDecision("elena_amira_resolved") },
        // 22 — Don't tell
        DialogNode { speaker: Speaker::Anna,
            text: "Lucky. There's a word.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Sometimes the kindest thing is letting someone believe \
                   the universe was gentle to them. Even when it was two \
                   exhausted women choosing to break the rules.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla will draw real rivers someday. That's enough.",
            next: DialogNext::EndWithDecision("elena_amira_resolved") },
        // 25 — Let them decide together
        DialogNode { speaker: Speaker::Anna,
            text: "Together. That word again. The thing nobody managed \
                   on Earth.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe that's what waking up on a new world is for. \
                   Learning to do the thing that was impossible before.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Two women who saved a child without knowing each other \
                   exist. If anyone can figure out what to tell her, \
                   it's them.",
            next: DialogNext::EndWithDecision("elena_amira_resolved") },
    ] };
}

pub static SCENE_ELENA_AMIRA_A: DialogScene = DialogScene {
    id: "elena_and_amira_a",
    trigger: DialogTrigger::AllDecisionsAndLevel(ELENA_AMIRA_REQS_A, 105),
    nodes: elena_amira_nodes!(),
};

pub static SCENE_ELENA_AMIRA_B: DialogScene = DialogScene {
    id: "elena_and_amira_b",
    trigger: DialogTrigger::AllDecisionsAndLevel(ELENA_AMIRA_REQS_B, 105),
    nodes: elena_amira_nodes!(),
};

pub static SCENE_ELENA_AMIRA_C: DialogScene = DialogScene {
    id: "elena_and_amira_c",
    trigger: DialogTrigger::AllDecisionsAndLevel(ELENA_AMIRA_REQS_C, 105),
    nodes: elena_amira_nodes!(),
};

/// Elena-Amira connection scenes (three trigger variants).
pub fn elena_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_ELENA_AMIRA_A,
        &SCENE_ELENA_AMIRA_B,
        &SCENE_ELENA_AMIRA_C,
    ]
}
