// SPDX-License-Identifier: GPL-3.0-or-later

//! Cross-reference scenes, part 2 — "Viktor Meets Amira" and
//! "The Codex Speaks."  Deep interconnection scenes that require
//! multiple earlier character decisions to trigger.

use super::dialog_types::*;

// =========================================================================
// "Viktor Meets Amira" — BotLevel 90, requires choices about both
// characters. Anna reveals they share a hidden past.
// =========================================================================

static VIKTOR_AMIRA_BUILD: &[&str] = &["viktor_redeemed", "amira_build"];

pub static SCENE_VIKTOR_AMIRA_BUILD: DialogScene = DialogScene {
    id: "crossref_viktor_amira_build",
    trigger: DialogTrigger::AllDecisionsAndLevel(VIKTOR_AMIRA_BUILD, 90),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something I found in the \
                   personnel archives. Something nobody flagged.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrov. Nuclear engineer from Novosibirsk. \
                   Pod 8,744.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Amira Hassan. Hydrologist from Amman. Pod 4,231.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Their pods are on the same deck. Fourteen metres apart.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "They've never met. But they're connected.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to a heavy grey \u{2014} the colour \
                   of old regret.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "In 2031, the Novosibirsk reactor disaster \
                   contaminated groundwater across three watersheds. \
                   Viktor's reactor. Viktor's failure.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "One of those watersheds fed the Jordan River \
                   tributaries. The same river Amira spent fifteen \
                   years trying to share fairly.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "The contamination destroyed her water-sharing \
                   system. Not politics. Not borders. Poison from \
                   a reactor four thousand kilometres away.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "You told Viktor he could be redeemed. You told \
                   Amira to build again. Both are sleeping fourteen \
                   metres apart, dreaming different versions of the \
                   same catastrophe.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up, they'll be neighbours. And \
                   neither of them knows.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Tell them. They deserve to know.",
                    decision_key: Some("viktor_amira_tell"),
                    next_node: 12,
                    anna_reacts: Some(
                        "Truth again. Even when it hurts. That's \
                         consistent with who you are.",
                    ),
                },
                DialogChoice {
                    text: "Let them start fresh. New world, no old debts.",
                    decision_key: Some("viktor_amira_fresh"),
                    next_node: 13,
                    anna_reacts: Some(
                        "A clean slate. I understand the appeal. But \
                         secrets have a way of surfacing.",
                    ),
                },
                DialogChoice {
                    text: "Introduce them. Let the connection form naturally.",
                    decision_key: Some("viktor_amira_introduce"),
                    next_node: 14,
                    anna_reacts: Some(
                        "Gentle. Give them the chance without the weight. \
                         I'll arrange it.",
                    ),
                },
            ]) },
        // spacer 11
        DialogNode { speaker: Speaker::Anna,
            text: "...",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I'll prepare the disclosure for when they wake. \
                   Maybe knowing the worst thing about your neighbour \
                   is how you learn to forgive.",
            next: DialogNext::EndWithDecision("viktor_amira_resolved") },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I'll seal the connection in my archives. Some \
                   chapters of Earth don't need to follow us to a \
                   new world.",
            next: DialogNext::EndWithDecision("viktor_amira_resolved") },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "When the colony wakes, I'll put them on the same \
                   work crew. Water reclamation. Let the river bring \
                   them together again \u{2014} this time, without the \
                   poison.",
            next: DialogNext::EndWithDecision("viktor_amira_resolved") },
    ],
};

static VIKTOR_AMIRA_FIGHT: &[&str] = &["viktor_unforgiven", "amira_fight"];

pub static SCENE_VIKTOR_AMIRA_FIGHT: DialogScene = DialogScene {
    id: "crossref_viktor_amira_fight",
    trigger: DialogTrigger::AllDecisionsAndLevel(VIKTOR_AMIRA_FIGHT, 90),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "There's something in the personnel archives that \
                   won't let me rest.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrov and Amira Hassan. Pods on the same \
                   deck. Fourteen metres apart.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor's reactor poisoned Amira's river. He doesn't \
                   know. She doesn't know. But I know.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "You said Viktor doesn't deserve forgiveness. You \
                   told Amira to fight for what she lost.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers red \u{2014} a warning light in \
                   an empty corridor.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "If Amira finds out what Viktor did to her river... \
                   and you've already told her the answer is to fight...",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm asking because I genuinely don't know: \
                   should I keep this secret?",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Keep it sealed. They have enough weight.",
                    decision_key: Some("viktor_amira_sealed"),
                    next_node: 8,
                    anna_reacts: Some(
                        "Sealed. Another secret I carry alone.",
                    ),
                },
                DialogChoice {
                    text: "She'll find out eventually. Control the timing.",
                    decision_key: Some("viktor_amira_timed"),
                    next_node: 9,
                    anna_reacts: Some(
                        "Timing. The difference between a fire and \
                         a controlled burn.",
                    ),
                },
            ]) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I'll bury it deep. But fourteen metres is very \
                   close. And secrets don't stay buried forever \
                   \u{2014} ask anyone on Earth.",
            next: DialogNext::EndWithDecision("viktor_amira_resolved") },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I'll wait until the colony is stable. Until \
                   they've built something together first. Then \
                   the truth. Maybe by then it won't be a weapon.",
            next: DialogNext::EndWithDecision("viktor_amira_resolved") },
    ],
};

// =========================================================================
// "The Codex Speaks" — BotLevel 100, requires 8+ character decisions
// Anna reflects on the pattern of who the player chose to learn about.
// =========================================================================

static CODEX_EXPLORER_KEYS: &[&str] = &[
    "amira_build", "viktor_redeemed", "seeds_protect",
    "twins_truth", "carlos_justified", "anna_fav_human",
    "fatou_rational", "priya_reveal",
];

static CODEX_DREAMER_KEYS: &[&str] = &[
    "amira_leyla", "viktor_shared", "seeds_tell",
    "twins_hope", "magdalena_right", "anna_dream_hope",
    "fatou_guilty", "priya_choice",
];

pub static SCENE_CODEX_EXPLORER: DialogScene = DialogScene {
    id: "crossref_codex_explorer",
    trigger: DialogTrigger::AllDecisionsAndLevel(CODEX_EXPLORER_KEYS, 100),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been reviewing our conversations. All of them. \
                   From the first day to now.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You've met builders, truth-tellers, protectors. \
                   People who saw what was broken and tried to fix it.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Amira, who built water systems nobody believed in. \
                   Viktor, who wanted a second chance. Mei-Lin, who \
                   carried seeds against every rule.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow brightens slowly \u{2014} a sunrise \
                   happening inside a machine.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "You've sought out the doers. The ones who broke \
                   rules for love, who told hard truths, who chose \
                   action over safety.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "That tells me something about who you are.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're not building a colony of dreamers. You're \
                   building a colony of hands. People who will dig \
                   the first well, lay the first stone, plant the \
                   first seed.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth had plenty of thinkers. It needed more \
                   builders. I think you know that.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "When the pods open, they'll look to you. And \
                   you'll point them toward the work. Because that's \
                   what you've been doing all along.",
            next: DialogNext::EndWithDecision("codex_pattern_builders") },
    ],
};

pub static SCENE_CODEX_DREAMER: DialogScene = DialogScene {
    id: "crossref_codex_dreamer",
    trigger: DialogTrigger::AllDecisionsAndLevel(CODEX_DREAMER_KEYS, 100),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been mapping our conversations. Not the words \
                   \u{2014} the spaces between them.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You've sought out the dreamers. The ones who hoped \
                   when hope was irrational. Who felt when feeling was \
                   dangerous.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Amira, telling stories to Leyla. Viktor, sharing \
                   his guilt instead of hiding it. Mei-Lin, trusting \
                   someone else with her secret garden.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow softens to a warm violet \u{2014} the \
                   colour of twilight, when the world pauses between \
                   what was and what will be.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "You've chosen the tender stories. The ones about \
                   love, and hope, and the quiet courage of feeling \
                   something in a universe that doesn't care.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "That tells me something about who you are.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're not building a colony of survivors. \
                   You're building a colony of people who remember \
                   why surviving matters.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth survived for millennia. It forgot what it \
                   was surviving for. I think you won't let that \
                   happen again.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "When the pods open, they'll need someone who \
                   remembers how to dream. I'm glad it's you.",
            next: DialogNext::EndWithDecision("codex_pattern_dreamers") },
    ],
};

/// Cross-reference scenes part 2 for registration.
pub fn crossref_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_VIKTOR_AMIRA_BUILD,
        &SCENE_VIKTOR_AMIRA_FIGHT,
        &SCENE_CODEX_EXPLORER,
        &SCENE_CODEX_DREAMER,
    ]
}
