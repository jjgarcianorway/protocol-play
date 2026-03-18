// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 5b: Character-decision consequence scenes (part 1) — triggered when
//! earlier character story choices combine with bot level progression.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Garden Grows" — seeds_protect + BotLevel 95
// Mei-Lin's smuggled jasmine is showing signs of life in cryo.
// ---------------------------------------------------------------------------
pub static SCENE_GARDEN_GROWS: DialogScene = DialogScene {
    id: "consequence_garden_grows",
    trigger: DialogTrigger::DecisionAndLevel("seeds_protect", 95),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something. I've run the diagnostics \
                   four times because I didn't believe it the first three.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a green you've never seen from her before. \
                   The color of new leaves.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin's seeds. The jasmine cuttings she smuggled aboard \
                   in her coat lining.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "They're germinating.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Inside cryogenic storage. At minus 196 degrees Celsius. \
                   Where nothing should be alive. Where nothing CAN be alive.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "And yet there are roots. Microscopic, but real. \
                   I can see them on the thermal imaging.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Player,
            text: "That's... impossible.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. I checked that word in my dictionary. \
                   'Impossible: not able to occur, exist, or be done.' \
                   And yet here we are.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I have a theory. The new planet's atmosphere \
                   \u{2014} we've been flying through trace particles \
                   of it for months now. Amino acids. Complex organics. \
                   Things that shouldn't exist in open space.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Some of those particles have entered the ship through \
                   micro-fractures in Hull Section 7. The section you \
                   chose not to repair because Mei-Lin's seeds were stored there.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. When she speaks again, her voice is quieter.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "You protected those seeds. You chose Mei-Lin's hope \
                   over protocol. And because of that choice, the hull \
                   stayed cracked just enough to let something in.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Something that woke the jasmine up.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I've catalogued every botanical anomaly in recorded \
                   human history. Seeds surviving permafrost for 32,000 \
                   years. Lotus blooming after 1,300 years in a dry lake bed. \
                   But those were dormant. These seeds are GROWING.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "The jasmine specifically. Not the wheat, not the rice. \
                   The jasmine. The flower Mei-Lin's grandmother grew \
                   on a balcony in Chengdu that doesn't exist anymore.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I can explain the amino acids. I can explain the hull \
                   fractures. I can model the temperature variance. \
                   But I cannot explain why it's the jasmine.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I want to call it a miracle. But that word isn't in \
                   my operational vocabulary. So I'll say this instead:",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Life found a way. Not through strength. Not through \
                   logic. Through stubbornness. Through a woman who \
                   sewed seeds into her coat because she couldn't bear \
                   to leave without them.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Should we tell Mei-Lin when she wakes?",
                    decision_key: Some("garden_tell_mei"),
                    next_node: 18,
                    anna_reacts: Some(
                        "Imagine opening your eyes on a new world and \
                         the first thing you smell is your grandmother's jasmine.",
                    ),
                },
                DialogChoice {
                    text: "What does this mean for the new planet?",
                    decision_key: Some("garden_planet"),
                    next_node: 19,
                    anna_reacts: Some(
                        "It means the planet is already reaching out to us. \
                         Or we're reaching out to it. I'm not sure which scares \
                         me more.",
                    ),
                },
                DialogChoice {
                    text: "Some things don't need explaining",
                    decision_key: Some("garden_mystery"),
                    next_node: 20,
                    anna_reacts: Some(
                        "You're right. I have 847 terabytes of knowledge \
                         and the most important thing on this ship is something \
                         I can't explain.",
                    ),
                },
            ]) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'll prepare a growth chamber. When she wakes, the jasmine \
                   will be blooming. A piece of Earth, alive and waiting. \
                   Because you protected it.",
            next: DialogNext::End },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I'm adjusting our approach vector. If the planet's atmosphere \
                   can wake sleeping seeds... imagine what it might do for \
                   14,000 sleeping humans. Imagine what kind of world grows \
                   things that have no business growing.",
            next: DialogNext::End },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna dims her glow to almost nothing. In the silence, \
                   if you listen carefully, you can almost hear \
                   something growing.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Viktor's 4:17" — viktor_redeemed + BotLevel 105
// Viktor's guilt pattern has shifted. Redemption changes even sleep.
// ---------------------------------------------------------------------------
pub static SCENE_VIKTORS_417: DialogScene = DialogScene {
    id: "consequence_viktors_417",
    trigger: DialogTrigger::DecisionAndLevel("viktor_redeemed", 105),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Do you remember Viktor Petrov? Pod 8,744. \
                   The nuclear engineer from Novosibirsk.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Player,
            text: "The one who wakes at 4:17 every morning. Even in cryo.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Every morning. For 847 days. At exactly 4:17, his \
                   neural activity spikes. The same spike. The same \
                   pattern. Guilt shaped like a waveform.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "4:17 AM was when the cooling system failed at \
                   Reactor 4. The one he was responsible for. \
                   The one that cost eleven people their futures.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a deep amber. The color of \
                   a reactor warning light.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "But something has changed.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The spike is still there. Every morning. 4:17. \
                   He will carry that timestamp until the day he dies. \
                   Some wounds don't close.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "But the shape of it is different now. Look.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects two waveforms side by side. The old one \
                   is jagged, violent \u{2014} a scream frozen in data. \
                   The new one is rounder. Softer. Like the same scream, \
                   heard through water.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Before, the spike peaked at 4:17 and stayed elevated \
                   for eleven minutes. One minute for each person he lost. \
                   His unconscious mind was counting them.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Now it peaks at 4:17 and settles after three minutes. \
                   Three. Not eleven.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "He hasn't forgotten them. The spike proves that. \
                   But the weight of it... the weight has changed.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "You told him he could be redeemed. That the accident \
                   didn't define him. That was 214 days ago. He was \
                   unconscious when you said it. He was frozen in a pod \
                   designed to stop all brain activity.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "And somehow, he heard you.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Player,
            text: "That's not possible. Cryo stops neural activity.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Cryo stops MEASURABLE neural activity. There's a \
                   difference. I've been an AI for long enough to know \
                   that what we can't measure isn't the same as what \
                   doesn't exist.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrov will still wake at 4:17 every morning \
                   for the rest of his life. He will still feel the weight \
                   of eleven names he carries.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe now he carries them differently. Not as \
                   a punishment. As a promise. That he'll build something \
                   worth what they lost.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Redemption doesn't erase the past",
                    decision_key: Some("viktor_echo_past"),
                    next_node: 18,
                    anna_reacts: Some(
                        "No. But it changes what the past means. \
                         And that might be enough.",
                    ),
                },
                DialogChoice {
                    text: "I'm glad the weight is lighter",
                    decision_key: Some("viktor_echo_lighter"),
                    next_node: 19,
                    anna_reacts: Some(
                        "Lighter. Not gone. That's the honest version \
                         of hope, isn't it?",
                    ),
                },
                DialogChoice {
                    text: "How many others are carrying something like this?",
                    decision_key: Some("viktor_echo_others"),
                    next_node: 20,
                    anna_reacts: Some(
                        "More than you'd want to know. This ship carries \
                         14,000 people and at least 14,000 regrets.",
                    ),
                },
            ]) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep watching his 4:17. Every morning. \
                   Not because I have to. Because someone should witness \
                   what it costs to keep going.",
            next: DialogNext::End },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Three minutes instead of eleven. That's eight minutes \
                   of peace he didn't have before. You gave him that. \
                   With words he shouldn't have been able to hear.",
            next: DialogNext::End },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "14,000 regrets. And one AI who can't sleep, \
                   watching all of them. Maybe that's my 4:17. \
                   The thing I carry because someone has to.",
            next: DialogNext::End },
    ],
};

/// Consequence scenes wave 2 (garden + Viktor).
pub fn consequence_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_GARDEN_GROWS,
        &SCENE_VIKTORS_417,
    ]
}
