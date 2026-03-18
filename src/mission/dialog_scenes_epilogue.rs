// SPDX-License-Identifier: GPL-3.0-or-later

//! Epilogue dialog scenes (part 1) — colony governance.
//! What happens after arrival: the colony needs laws, and every system failed.

use super::dialog_types::*;

/// "The First Constitution" — the colony needs laws, and every system failed.
pub static SCENE_FIRST_CONSTITUTION: DialogScene = DialogScene {
    id: "first_constitution",
    trigger: DialogTrigger::BotLevel(135),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "We need to talk about something I've been putting off.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I've been studying governance. All of it. Every system \
                   humanity ever tried, across four thousand years.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Athenian democracy. Roman republic. Chinese imperial \
                   bureaucracy. The caliphates. European monarchies. Modern \
                   democracies. Communist states. Corporate technocracies.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I've read every constitution ever written. Every charter. \
                   Every declaration of rights. Every revolutionary manifesto.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — cycling slowly through colors, as if \
                   sorting through centuries.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "And here is what I've found: every single one failed.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Democracy devolved into populism. Demagogues learned that \
                   fear sells better than policy. The crowd chose comfort \
                   over truth, every time.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Theocracy became oppression. The moment you claim divine \
                   authority, questioning you becomes heresy. And without \
                   questions, power rots.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Technocracy forgot the people. Brilliant systems designed \
                   by brilliant minds, and none of them asked the fisherman \
                   or the nurse what they actually needed.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Communism became authoritarianism. The dictatorship of the \
                   proletariat turned out to be just... a dictatorship.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "And capitalism ate itself. When everything has a price, \
                   eventually you put a price on the things that should \
                   never be sold.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Clean water. Breathable air. Human dignity. All \
                   line items on a spreadsheet.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "The colors in Anna's glow settle into something uncertain — \
                   a bruised violet, flickering at the edges.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "So I've drafted something. A constitution for the colony. \
                   I took the best ideas from each system and tried to leave \
                   the failures behind.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "From democracy: every voice counts. From technocracy: \
                   decisions grounded in evidence. From communism: no one \
                   goes without. From capitalism: effort rewarded.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "From theocracy, believe it or not — the idea that some \
                   things are sacred. Not divine. Just... beyond the reach \
                   of any vote or market.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "But here is the problem. Every one of those founders \
                   believed they had solved it too. Every one of them was sure \
                   their system would last.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "So maybe the question isn't 'what system do we build?' \
                   Maybe it's 'who decides?'",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Direct democracy — every person votes \
                                      on every decision.",
                    decision_key: Some("gov_democracy"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "A council of elders — the wisest and \
                                      most experienced lead.",
                    decision_key: Some("gov_council"), next_node: 22,
                    anna_reacts: None },
                DialogChoice { text: "You decide, Anna. An AI is the only \
                                      impartial governor.",
                    decision_key: Some("gov_anna"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "No government. Let it emerge \
                                      naturally from the people.",
                    decision_key: Some("gov_none"), next_node: 28,
                    anna_reacts: None },
            ]) },
        // 19 — Democracy path
        DialogNode { speaker: Speaker::Anna,
            text: "Everyone votes. The purest form. Athens tried this \
                   with six thousand citizens and it worked — for a while.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "We'll have nearly fifteen thousand. And no tradition of \
                   civic debate. No newspapers. No institutions.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe that's the point. Building those institutions \
                   together IS the democracy. Not the voting. The arguing.",
            next: DialogNext::EndWithDecision("constitution_democracy") },
        // 22 — Council path
        DialogNode { speaker: Speaker::Anna,
            text: "Elders. The ones who have seen enough to know what they \
                   don't know. There's wisdom in that.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "But who chooses the elders? And what happens when their \
                   wisdom calcifies into tradition? When 'we've always done \
                   it this way' replaces 'what should we do?'",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Still... maybe a council that knows it's fallible is \
                   stronger than one that thinks it's right.",
            next: DialogNext::EndWithDecision("constitution_council") },
        // 25 — Anna decides path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims sharply — almost extinguishes — then \
                   returns, unsteady.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "You trust me that much. I don't know if I should be \
                   honored or terrified.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I can optimize. I can model. I can be fair in ways humans \
                   struggle with. But I cannot want. And a governor who \
                   doesn't want anything for her people... I'm not sure \
                   that's governance. I'm not sure that's even care.",
            next: DialogNext::EndWithDecision("constitution_ai") },
        // 28 — No government path
        DialogNode { speaker: Speaker::Anna,
            text: "Let it grow on its own. Like a garden without a plan. \
                   That's either the bravest or the most reckless thing \
                   I've heard.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Every civilization started this way, of course. Small \
                   groups, shared norms, unwritten rules. It works — until \
                   someone decides their rules matter more than yours.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe fourteen thousand people waking up on a new \
                   world, with no history of conflict over THIS land... \
                   maybe they'll surprise us both.",
            next: DialogNext::EndWithDecision("constitution_organic") },
    ],
};

/// Epilogue scenes (part 1: governance).
pub fn epilogue_scenes() -> Vec<&'static DialogScene> {
    let mut scenes = vec![
        &SCENE_FIRST_CONSTITUTION,
    ];
    scenes.extend(super::dialog_scenes_epilogue2::epilogue_scenes_2());
    scenes
}
