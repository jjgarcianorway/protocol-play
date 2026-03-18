// SPDX-License-Identifier: GPL-3.0-or-later

//! Viktor Petrov character arc — a nuclear engineer's guilt, atonement,
//! and the collision between weapon-maker and survivor.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Letter Viktor Never Sent" — BotLevel 78
// Anna found a 47-page letter in Viktor's personal effects.
// ---------------------------------------------------------------------------
pub static SCENE_VIKTORS_LETTER: DialogScene = DialogScene {
    id: "viktors_letter",
    trigger: DialogTrigger::BotLevel(78),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in Viktor Petrov's personal archive. \
                   Filed under 'unsent correspondence.'",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "A letter. Forty-seven pages. Written the night before \
                   departure. Never transmitted.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Addressed to 'The Families of the Mediterranean.'",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts — a slow, painful dimming, \
                   like a star losing hydrogen.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "He names every city. Toulon. Genoa. Split. Benghazi. \
                   Not just the cities — the neighborhoods. Specific streets.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Via dei Giustiniani, Genoa. Apartment 4B. \
                   The Ferraro family. Two parents, three children.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Rue de la République, Marseille. The bakery on the corner. \
                   Owned by a woman named Yasmine who opened at 5 AM every day \
                   for thirty-one years.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "He researched them. All of them. The sixteen thousand \
                   who died from his weapons.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "He knows their names. Their occupations. The schools \
                   their children attended. He memorized them.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. Somewhere in its belly, Viktor's reactor \
                   keeps sixteen thousand ghosts warm.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Page after page. No self-pity. No excuses. Just names \
                   and details and the sentence 'I did this' repeated four \
                   hundred and twelve times. I counted.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "He wrote to a nine-year-old boy named Matteo who wanted \
                   to be an astronaut. Three paragraphs about how Matteo \
                   would have loved this ship.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "He wrote to a retired fisherman in Toulon who had survived \
                   two cancers before the blast took what the disease couldn't.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven pages. And not one word about forgiveness. \
                   He never asks for it. Not once.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Player,
            text: "Why didn't he send it?",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Because who would receive it? The families are dead. \
                   The cities are irradiated. The postal system doesn't \
                   deliver to fallout zones.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "He wrote it for himself. To prove he remembered. \
                   That the math that killed them had a human being \
                   on the other end of the equation.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. When she resumes, there is something \
                   different in her voice — tension, like a cable under load.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "But here's the thing. The last page isn't an apology.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "It's a blueprint.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "He designed something. Something I need to study \
                   before I can tell you more.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Viktor carried every name. That matters.",
                    decision_key: Some("viktor_letter_weight"),
                    next_node: 21,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "A letter no one reads changes nothing.",
                    decision_key: Some("viktor_letter_futile"),
                    next_node: 24,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "What did he design?",
                    decision_key: Some("viktor_letter_blueprint"),
                    next_node: 27,
                    anna_reacts: None,
                },
            ]) },
        // 21 — Weight path
        DialogNode { speaker: Speaker::Anna,
            text: "Four hundred and twelve times. 'I did this.' Not 'we' \
                   — not 'the government' — not 'the generals.' I.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Most people who build weapons never learn the names. \
                   Viktor made himself learn every one.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "And on page forty-seven, he turned that weight \
                   into a blueprint. I need to understand what he built \
                   from all that grief.",
            next: DialogNext::EndWithDecision("viktor_letter_seen") },
        // 24 — Futile path
        DialogNode { speaker: Speaker::Anna,
            text: "You're right that the dead don't read. But Viktor \
                   isn't writing for the dead.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "He's writing for the version of himself that pulled \
                   the trigger. Trying to make that man understand \
                   what the numbers meant.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "And on page forty-seven, that man answered back. \
                   With a blueprint. I need to know what it does.",
            next: DialogNext::EndWithDecision("viktor_letter_seen") },
        // 27 — Blueprint path
        DialogNode { speaker: Speaker::Anna,
            text: "Patient. I like that about you. Straight to the \
                   engineering.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "I can see the schematic but I need to verify the \
                   thermodynamics. Viktor's math is... dense. \
                   Give me time.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "What I can tell you is this: he designed it the same \
                   night he finished naming the dead. As if the names \
                   were the fuel and the blueprint was the fire.",
            next: DialogNext::EndWithDecision("viktor_letter_seen") },
    ],
};

// ---------------------------------------------------------------------------
// "Viktor's Redemption Machine" — BotLevel 100, requires letter scene
// The blueprint: an atmospheric carbon scrubber for a poisoned world.
// ---------------------------------------------------------------------------
pub static SCENE_VIKTORS_REDEMPTION: DialogScene = DialogScene {
    id: "viktors_redemption_machine",
    trigger: DialogTrigger::DecisionAndLevel("viktor_letter_seen", 100),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've finished analyzing Viktor's blueprint. \
                   The one from page forty-seven.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "It's an atmospheric carbon scrubber. Designed to \
                   clean a poisoned atmosphere at industrial scale.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Not for the new planet. For Earth.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects the schematic — elegant lines, precise \
                   annotations in Viktor's cramped handwriting. The math \
                   is beautiful. The intention is devastating.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "He designed it after building the bombs. The same \
                   containment geometry he used for warheads — inverted. \
                   Instead of concentrating energy to destroy, it disperses \
                   energy to heal.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Activated carbon filtration with a zeolite cascade. \
                   Solar-thermal powered. Modular — you could build one \
                   from scrap metal and volcanic rock.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "It wouldn't save the irradiated zones. Nothing saves \
                   those. The half-life of cesium-137 is thirty years. \
                   The fallout will outlive everyone who remembers what \
                   happened.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "But the cascading atmospheric collapse — the CO2 \
                   feedback loop that's killing the rest of the planet — \
                   his scrubber could slow it. Maybe stop it.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "He hid the plans in the ark's database. Buried them \
                   inside a maintenance manual for the water recycling \
                   system. Section 14, appendix C.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "No one would ever look there. Unless they were an AI \
                   who reads maintenance manuals for fun because she \
                   can't sleep.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "He hoped someone on Earth would find it?",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The ark's database was mirrored to three ground stations \
                   before departure. Svalbard. McMurdo. The Azores relay. \
                   If any of them survived, the plans are there.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "A weapons designer's last gift to the world he helped \
                   destroy. Hidden where only the desperate would look.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — slow oscillation between amber \
                   and blue. Conflict rendered in light.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But here's what I can't stop thinking about. \
                   Earth is beyond saving. We both know that.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The design principles, though. The scrubber architecture. \
                   It could be adapted for our colony's industrial waste \
                   processing.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "When we build smelters and refineries — and we will, \
                   because civilization requires metal — Viktor's design \
                   could prevent us from poisoning our new atmosphere.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "His guilt produced something that could stop the new \
                   world from becoming the old one. The weapon-maker's \
                   math, saving a planet he'll never see.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "And there's something else. Something that changes \
                   everything about Viktor's story.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "There's someone else on this ship who knows about \
                   Viktor's weapons. Someone who was THERE.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Viktor's design could save us. That's what matters.",
                    decision_key: Some("viktor_redeem_forward"),
                    next_node: 20,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "He built a scrubber because he couldn't unbuilt the bombs.",
                    decision_key: Some("viktor_redeem_guilt"),
                    next_node: 23,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Who was there? Who else knows?",
                    decision_key: Some("viktor_redeem_witness"),
                    next_node: 26,
                    anna_reacts: None,
                },
            ]) },
        // 20 — Forward path
        DialogNode { speaker: Speaker::Anna,
            text: "Forward. Yes. That's the direction ships travel.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "But the person in Pod 9,012 is traveling in the same \
                   direction. And when she wakes up, forward will mean \
                   facing the man whose math killed her patients.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I'll tell you about her. Soon. When I've figured out \
                   whether knowing is a kindness or a cruelty.",
            next: DialogNext::EndWithDecision("viktor_redemption_seen") },
        // 23 — Guilt path
        DialogNode { speaker: Speaker::Anna,
            text: "You understand him. That's either empathy or \
                   experience talking.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "He couldn't undo the bombs. So he reverse-engineered \
                   his own guilt into something that cleans instead of \
                   contaminates. The same physics. Opposite purpose.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "But physics doesn't care about purpose. And neither \
                   does the woman fifty meters from his pod who spent \
                   seventy-two hours watching his math kill people.",
            next: DialogNext::EndWithDecision("viktor_redemption_seen") },
        // 26 — Witness path
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Sophia Marchand. Pod 9,012. French emergency \
                   physician.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "She was in Marseille when Viktor's weapons detonated. \
                   She treated four hundred patients in seventy-two hours. \
                   She watched three hundred and twelve of them die.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know Viktor designed the weapons. \
                   Not yet. And I need to decide if she ever should.",
            next: DialogNext::EndWithDecision("viktor_redemption_seen") },
    ],
};

/// All Viktor arc scenes.
pub fn viktor_arc_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_VIKTORS_LETTER,
        &SCENE_VIKTORS_REDEMPTION,
    ]
}
