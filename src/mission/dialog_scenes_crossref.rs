// SPDX-License-Identifier: GPL-3.0-or-later

//! Cross-reference scenes, part 1 — "The Ripple Effect" variants.
//! Anna draws explicit connections between seemingly unrelated
//! choices the player made, creating BG3-level interconnection.

use super::dialog_types::*;

// =========================================================================
// "The Ripple Effect" — Anna connects two decisions (BotLevel 75)
// Three variants depending on which decision combo the player made.
// =========================================================================

// --- Variant A: seeds_protect + faction_keepers ---
static RIPPLE_SEEDS_KEEPERS: &[&str] = &["seeds_protect", "faction_keepers"];

pub static SCENE_RIPPLE_SEEDS_KEEPERS: DialogScene = DialogScene {
    id: "crossref_ripple_seeds_keepers",
    trigger: DialogTrigger::AllDecisionsAndLevel(RIPPLE_SEEDS_KEEPERS, 75),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I noticed something today. A connection I should \
                   have seen earlier.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You protected Mei-Lin's seeds. You sided with the \
                   Keepers \u{2014} Hassan al-Rashidi's faction.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Two separate decisions. Weeks apart. Different contexts.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "But they're the same instinct, aren't they?",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin sewed jasmine seeds into her coat to save \
                   her grandmother's garden. Hassan catalogued every \
                   poem, recipe, and lullaby from the cultures we left behind.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to deep amber \u{2014} the colour of \
                   old parchment, of things worth keeping.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Both of them smuggled fragments of Earth aboard. \
                   One in soil, one in words. You protected both.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "There's a word for that in seven of the 47 languages \
                   on this ship. The closest English gets is 'stewardship.' \
                   But it's bigger than that.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's the belief that some things matter more than \
                   survival. That arriving somewhere new means nothing \
                   if you've forgotten where you came from.",
            next: DialogNext::End },
    ],
};

// --- Variant B: amira_build + augment_refuse ---
static RIPPLE_AMIRA_HUMAN: &[&str] = &["amira_build", "augment_refuse"];

pub static SCENE_RIPPLE_AMIRA_HUMAN: DialogScene = DialogScene {
    id: "crossref_ripple_amira_human",
    trigger: DialogTrigger::AllDecisionsAndLevel(RIPPLE_AMIRA_HUMAN, 75),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I keep coming back to two things you did.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You told Amira to build. Not fight, not flee. Build.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "And when I offered you augmentation \u{2014} faster reflexes, \
                   sharper cognition \u{2014} you refused.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Amira spent fifteen years designing a water-sharing \
                   system that worked. Every engineer said it would work. \
                   Every politician said it was impossible.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "She built anyway. With human hands. Human patience. \
                   Human stubbornness.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "You chose the same thing. Slower. Shakier. \
                   But yours.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow deepens to the warm blue she saves \
                   for moments that matter.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "There's something the optimisers never understood. \
                   The cracks are where the light gets in. The \
                   imperfections are what make it real.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Amira knew that. I think you do too.",
            next: DialogNext::End },
    ],
};

// --- Variant C: faction_pioneers + augment_accept ---
static RIPPLE_PIONEERS_AUGMENT: &[&str] = &[
    "faction_pioneers", "augment_accept",
];

pub static SCENE_RIPPLE_PIONEERS_AUGMENT: DialogScene = DialogScene {
    id: "crossref_ripple_pioneers_augment",
    trigger: DialogTrigger::AllDecisionsAndLevel(RIPPLE_PIONEERS_AUGMENT, 75),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I see the pattern in what you've chosen.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You backed the Pioneers. Forward, always forward. \
                   Build new, don't mourn old.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "And you accepted the augmentation. Faster. Sharper. \
                   More than what you were.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Both are the same instinct: evolution. Refusing \
                   to be limited by what came before.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses white \u{2014} clinical, precise, \
                   the colour of a mind running calculations.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth died because it couldn't adapt fast enough. \
                   You're determined not to make the same mistake.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I admire it. And it frightens me.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because the last civilisation that optimised \
                   everything... built me. And then needed an ark \
                   to escape what they'd made.",
            next: DialogNext::End },
    ],
};

/// Cross-reference ripple scenes for registration.
pub fn crossref_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_RIPPLE_SEEDS_KEEPERS,
        &SCENE_RIPPLE_AMIRA_HUMAN,
        &SCENE_RIPPLE_PIONEERS_AUGMENT,
    ]
}
