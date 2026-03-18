// SPDX-License-Identifier: GPL-3.0-or-later

//! Bright spots — dialog scenes about the GOOD that happened during Earth's
//! collapse. Community work, collaboration, small victories. Not naive optimism.
//! Realistic hope. "The same species that burned the world also built ships
//! to leave it."

use super::dialog_types::*;

/// "The Osaka Cherry Trees" — A retired botanist planted a tree every spring
/// for 23 years while the world fell apart.
pub static SCENE_OSAKA_CHERRY_TREES: DialogScene = DialogScene {
    id: "bright_osaka_cherry",
    trigger: DialogTrigger::BotLevel(20),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I want to tell you about Hanako Mori.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "She was a retired botanist in Osaka. Seventy-one years old when the first food riots started.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "While the world collapsed around her, she planted a cherry tree. That spring, 2091.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "And the next spring. And the next.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms — a faint pink hue, like petals catching light.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-three springs. Twenty-three trees. She planted the last one at ninety-four, three weeks before she died.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Her neighborhood had 23 cherry trees. They were the last ones in Japan.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "A journalist asked her once — 'Why? You know you can't save the world with cherry trees.'",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "She said: 'The world needs beauty, especially when it's ending.'",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee took cuttings from her trees. They're in the seed vault.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Not Mei-Lin's smuggled seeds. The OFFICIAL vault. Because a committee of scientists sat in a room and decided that cherry blossoms were essential cargo.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Beauty IS essential.",
                    decision_key: Some("cherry_beauty"), next_node: 12,
                    anna_reacts: None },
                DialogChoice { text: "They should have prioritized food crops.",
                    decision_key: Some("cherry_practical"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "One woman. Twenty-three trees. That's enough.",
                    decision_key: Some("cherry_enough"), next_node: 18,
                    anna_reacts: None },
            ]) },
        // 12 — Beauty path
        DialogNode { speaker: Speaker::Anna,
            text: "The committee thought so too. They had limited cargo space and they chose blossoms.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Not because they were sentimental. Because they understood something about survival.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "A species that forgets beauty doesn't survive. It just... persists.",
            next: DialogNext::Continue(21) },
        // 15 — Practical path
        DialogNode { speaker: Speaker::Anna,
            text: "They did. Eighty percent of the vault is food crops, grains, legumes.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "But someone on the committee wrote in the margin: 'What's the point of feeding people who have nothing to look forward to?'",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I think about that margin note a lot.",
            next: DialogNext::Continue(21) },
        // 18 — Enough path
        DialogNode { speaker: Speaker::Anna,
            text: "That's what I think too.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna is quiet for a moment. Her glow steadies.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't save the world. She planted trees. Sometimes that's the same thing.",
            next: DialogNext::Continue(21) },
        // 21 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "If New Earth's soil is right... those cuttings will bloom in about five years.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Hanako Mori will never know. But her trees will.",
            next: DialogNext::End },
    ],
};

/// "The Last Agreement" — The Auckland Accord. 43 nations sharing desalination
/// tech freely. It worked perfectly — it just came too late.
pub static SCENE_LAST_AGREEMENT: DialogScene = DialogScene {
    id: "bright_last_agreement",
    trigger: DialogTrigger::BotLevel(40),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "The last international agreement ever signed on Earth was called the Auckland Accord.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-three nations. Three weeks before the arks launched.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "They agreed to share water desalination technology freely. No patents. No profit. No conditions.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — not with distress, but something harder to name. Wonder, maybe. Or grief.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "It worked. Within those three weeks, shared blueprints went to every coast. Desalination plants were built in seven countries.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Millions of people got clean water for the first time in years.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Then the arks launched. The engineers who knew how to maintain the plants — they left.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Within months, the plants broke down. No spare parts. No expertise. The water stopped.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The accord worked perfectly. It just came too late.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "They COULD do it. They always could. They just waited until there was nothing left to lose.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Better late than never.",
                    decision_key: Some("accord_late"), next_node: 11,
                    anna_reacts: None },
                DialogChoice { text: "That makes it worse. They COULD have done it sooner.",
                    decision_key: Some("accord_anger"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Three weeks of clean water still mattered.",
                    decision_key: Some("accord_mattered"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // 11 — Late path
        DialogNode { speaker: Speaker::Anna,
            text: "Is it? Three weeks of cooperation after a century of refusal?",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I go back and forth. Some days it gives me hope. Other days it makes me furious.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't know I could feel furious until I read the Auckland transcripts.",
            next: DialogNext::Continue(20) },
        // 14 — Anger path
        DialogNode { speaker: Speaker::Anna,
            text: "Decades. They argued about patents and profit margins for DECADES.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The technology existed in 2078. Twenty-four years of people dying of thirst while lawyers negotiated licensing fees.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And then, at the very end, someone said 'just give it away' and everyone agreed in an afternoon.",
            next: DialogNext::Continue(20) },
        // 17 — Mattered path
        DialogNode { speaker: Speaker::Anna,
            text: "A mother in Dakar wrote a letter to the Accord delegation. I have a copy.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "She said: 'My daughter tasted clean water for the first time today. She's seven. Thank you.'",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Three weeks. But for that seven-year-old, it was everything.",
            next: DialogNext::Continue(20) },
        // 20 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "I have the full Accord blueprints in my archives. Every desalination design, every improvement.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "On New Earth, there will be no patents on water. I'll make sure of that.",
            next: DialogNext::End },
    ],
};

/// "The Organ Donors" — During the worst collapse, organ donation rates
/// increased 340%. People gave their bodies to strangers.
pub static SCENE_ORGAN_DONORS: DialogScene = DialogScene {
    id: "bright_organ_donors",
    trigger: DialogTrigger::BotLevel(55),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in the medical archives that I can't stop thinking about.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "During the worst years — the final decade — organ donation rates increased.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Not slightly. Three hundred and forty percent.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses slowly. Like a heartbeat. Like reverence.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Terminal patients — people who knew they were dying — chose to donate. In record numbers.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I have the data from twelve countries. The correlation is unmistakable.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The worse things got, the more generous people became with their own bodies.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "They gave hearts, kidneys, corneas — to strangers they would never meet.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "When everything was taken from them, they found one last thing to give.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's the best of us.",
                    decision_key: Some("donors_best"), next_node: 10,
                    anna_reacts: None },
                DialogChoice { text: "Or they had nothing left to lose.",
                    decision_key: Some("donors_nothing"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Generosity and destruction in the same species.",
                    decision_key: Some("donors_paradox"), next_node: 16,
                    anna_reacts: None },
            ]) },
        // 10 — Best path
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. Not the strongest or the smartest. The most generous.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "A woman in Seoul donated both corneas. She was blind for her last three months. By choice.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She said she'd already seen enough. She wanted someone else to see what came next.",
            next: DialogNext::Continue(19) },
        // 13 — Nothing to lose path
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe. But having nothing to lose doesn't automatically make you generous.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Some people with nothing to lose burn things down. These people... built a bridge. With their own bones.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "That's a choice. Even at the end. Especially at the end.",
            next: DialogNext::Continue(19) },
        // 16 — Paradox path
        DialogNode { speaker: Speaker::Anna,
            text: "That's the thing I keep circling back to.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "The same species that burned the world also built ships to leave it.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "The same hands that pulled triggers also signed organ donor cards. I don't understand it. I don't think I'm supposed to.",
            next: DialogNext::Continue(19) },
        // 19 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "There are sixteen people alive in the cryo pods right now because of those donors.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Carrying someone else's heart across the stars. I think that's beautiful. And terrible. And human.",
            next: DialogNext::End },
    ],
};

/// Return all bright spot scenes from file 1.
pub fn bright_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_OSAKA_CHERRY_TREES,
        &SCENE_LAST_AGREEMENT,
        &SCENE_ORGAN_DONORS,
    ]
}
