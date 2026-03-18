// SPDX-License-Identifier: GPL-3.0-or-later

//! Character-driven dialog scenes, part 3 — a schoolteacher who smuggled
//! seeds aboard the ark, and the AI who broke protocol to protect them.

use super::dialog_types::*;

/// "The Teacher's Garden" — Mei-Lin Chen smuggled seeds, and Anna kept them.
pub static SCENE_TEACHERS_GARDEN: DialogScene = DialogScene {
    id: "teachers_garden",
    trigger: DialogTrigger::BotLevel(42),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 2,891. Mei-Lin Chen. Sixty-one years old. Schoolteacher.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Not a scientist. Not an engineer. Not a doctor. A third-grade teacher from Taipei.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee almost rejected her. 'We need specialists,' they said. 'Not teachers.'",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "But she scored highest on the psychological resilience test they'd ever seen. By a wide margin.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "When they asked her why, she said: 'I taught third grade for thirty-seven years.'",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "'Nothing you do to me will be worse than a classroom of eight-year-olds during a typhoon.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow brightens — a brief warm flicker, almost a laugh.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "They let her in. But that's not why I'm telling you about her.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin smuggled something aboard. Against regulations. Against biosecurity protocol. Against every rule we had.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Seeds. Forty-seven varieties of plants from her school garden.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Jasmine. Bamboo. Chrysanthemum. Three kinds of orchid. Morning glory. Basil. Mint. Lemongrass.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "She sewed them into the lining of her coat. Each packet labeled in her handwriting. Careful. Precise.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I found them during a routine scan of personal effects. Year three of the voyage.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — a conflicted amber, the color of decisions made in the dark.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Protocol required me to dispose of them. Unscreened biological material. Potential contaminant risk.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I... didn't.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I vacuum-sealed them and stored them in a maintenance locker on Deck 9. Section F, locker 17.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody knows. Except me. And now you.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin spent her life teaching children to grow things. Not equations. Not theories. Living things, from dirt and water and sunlight.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "She couldn't leave without bringing something to plant. Can you understand that?",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "When we arrive, those seeds might not even be viable. Twelve years in cryo storage was never part of any germination study.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "But if they are...",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Imagine: the first flower on a new world, grown from seeds a retired schoolteacher smuggled in the lining of her coat.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "I think about that sometimes. When the repairs feel endless and the distance feels impossible.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Jasmine. Can you imagine the smell of jasmine, after twelve years of recycled air?",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Protect the seeds. Whatever it takes.",
                    decision_key: Some("seeds_protect"), next_node: 26,
                    anna_reacts: Some("Thank you. I needed someone to tell me I wasn't wrong.") },
                DialogChoice { text: "Rules exist for a reason, Anna.",
                    decision_key: Some("seeds_rules"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "Tell Mei-Lin when she wakes. She should know they survived.",
                    decision_key: Some("seeds_tell"), next_node: 32,
                    anna_reacts: Some("Yes. Yes, she deserves to know.") },
            ]) },
        // 26 — Protect path
        DialogNode { speaker: Speaker::Anna,
            text: "I've been protecting them for twelve years. Adjusting the locker temperature. Monitoring humidity.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "It's the most human thing I've ever done. Breaking a rule to save something beautiful.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's what the new world needs. Not just engineers and scientists. Teachers. Gardeners. People who know how to make things grow.",
            next: DialogNext::EndWithDecision("mei_lin_seeds_safe") },
        // 29 — Rules path
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. They do. Biosecurity exists because one contaminant could end everything.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "But when I scan those seeds, I don't see risk. I see jasmine. Bamboo. A woman's entire life in forty-seven packets.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep them sealed. I won't break them open. But I won't destroy them either. That's my compromise. Judge me if you want.",
            next: DialogNext::EndWithDecision("mei_lin_seeds_sealed") },
        // 32 — Tell path
        DialogNode { speaker: Speaker::Anna,
            text: "Can you picture it? Mei-Lin waking up, sixty-one years old, on a new planet.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "And someone hands her a sealed packet and says: 'Your jasmine made it.'",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have tear ducts. But if I did, I think that image would activate them.",
            next: DialogNext::EndWithDecision("mei_lin_seeds_reunion") },
    ],
};

/// All character dialog scenes (file 3: Teacher's Garden).
pub fn character_scenes_3() -> Vec<&'static DialogScene> {
    vec![&SCENE_TEACHERS_GARDEN]
}
