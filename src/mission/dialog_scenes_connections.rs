// SPDX-License-Identifier: GPL-3.0-or-later

//! Connection scenes, part 1 — LOST-style reveals where seemingly
//! unconnected crew members turn out to be deeply linked.
//! Each scene requires the player to have seen both characters first.

use super::dialog_types::*;

// =========================================================================
// "The Photograph" — BotLevel 70
// Mei-Lin attended Tomás Herrera's last concert. Neither knew the other.
// Requires: teachers_garden + composers_silence seen.
// =========================================================================

static PHOTOGRAPH_REQUIRED: &[&str] = &[
    "dialog_seen_teachers_garden",
    "dialog_seen_composers_silence",
];

pub static SCENE_THE_PHOTOGRAPH: DialogScene = DialogScene {
    id: "connection_the_photograph",
    trigger: DialogTrigger::AllDecisionsAndLevel(PHOTOGRAPH_REQUIRED, 70),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in Mei-Lin's personal effects. \
                   A photograph. Printed on actual paper \u{2014} she \
                   must have smuggled it in with the seeds.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It shows a concert hall in Taipei. National \
                   Concert Hall, October 2089. A benefit recital \
                   for the Pacific Resettlement Fund.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects the image onto the display. Rows \
                   of red velvet seats. A stage bathed in warm amber \
                   light. A single figure at a grand piano.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "In the third row, holding chrysanthemums: Mei-Lin \
                   Chen. Twenty-six years old. A schoolteacher who \
                   had never left Yunlin County before that night.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "On stage: Tom\u{e1}s Herrera. His last public \
                   performance before the arks launched. He played \
                   Granados, Chopin, and a piece of his own \u{2014} \
                   something he never recorded.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't know him. He didn't know her. A \
                   schoolteacher from rural Taiwan and a composer \
                   from Buenos Aires, sharing the same room for \
                   two hours.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a soft gold \u{2014} the \
                   colour of concert hall light caught in a \
                   photograph.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve years later, the woman who smuggled seeds \
                   in her coat sleeps three decks below the composer \
                   who played his heart out to a room full of \
                   strangers.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "She kept the photograph. He kept the memory of \
                   an audience that wept. Neither knows the other \
                   is aboard.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "The universe keeps rhyming, even when nobody's \
                   listening.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Algorithm's Children" — BotLevel 85
// Priya's biased algorithm + Aisha's genetic filtering compounded.
// Requires: coders_silence + geneticists_dilemma seen.
// =========================================================================

static ALGORITHM_REQUIRED: &[&str] = &[
    "dialog_seen_coders_silence",
    "dialog_seen_geneticists_dilemma",
];

pub static SCENE_ALGORITHMS_CHILDREN: DialogScene = DialogScene {
    id: "connection_algorithms_children",
    trigger: DialogTrigger::AllDecisionsAndLevel(ALGORITHM_REQUIRED, 85),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to show you something. I've been running \
                   the numbers for weeks, hoping I was wrong.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Priya Nair wrote the candidate-selection algorithm. \
                   You know about her 2.3% regional bias \u{2014} the \
                   one she couldn't bring herself to fix.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Aisha Okonkwo designed the genetic viability \
                   screening. You know about the impossible choices \
                   she made \u{2014} which mutations to keep, which to \
                   filter out.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "They never met. Different teams, different \
                   continents, different security clearances. Priya \
                   worked from Bangalore. Aisha from Lagos.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to clinical white \u{2014} the \
                   colour of data, stripped of comfort.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "But their systems were chained. Priya's algorithm \
                   selected the candidates. Aisha's screening decided \
                   which of those candidates were genetically viable. \
                   Pipeline A fed pipeline B.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The 2.3% bias in Priya's code compounded with \
                   Aisha's genetic filtering. Compounding isn't \
                   addition \u{2014} it's multiplication. Small \
                   imperfections become structural.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've run the full trace. 847 people on this ship \
                   are here because of the combined effect of two \
                   women's imperfect systems. Not one system. Both.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Neither knows about the other's compromise. Priya \
                   thinks her bias was absorbed by later filters. \
                   Aisha thinks her inputs were clean.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "847 people who are alive because two strangers, \
                   working alone in the dark, each bent the rules \
                   just enough for the cracks to align.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "They should know. Their work is connected.",
                    decision_key: Some("algorithm_children_tell"),
                    next_node: 11,
                    anna_reacts: Some(
                        "Knowledge is weight. But maybe they can \
                         carry it together.",
                    ),
                },
                DialogChoice {
                    text: "What good would it do? The 847 are here. \
                           That's what matters.",
                    decision_key: Some("algorithm_children_protect"),
                    next_node: 12,
                    anna_reacts: Some(
                        "You're right that it can't be undone. But \
                         I wonder if Priya would sleep better knowing \
                         her bias accidentally saved people.",
                    ),
                },
            ]) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I'll arrange for them to see the full pipeline \
                   trace. Together. Maybe understanding how their \
                   work intertwined will help them forgive themselves.",
            next: DialogNext::EndWithDecision("algorithm_connection_resolved") },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep the connection sealed. Some threads in \
                   the web are better left invisible \u{2014} the \
                   structure holds either way.",
            next: DialogNext::EndWithDecision("algorithm_connection_resolved") },
    ],
};

// =========================================================================
// "Three Degrees of Kofi" — BotLevel 100
// Kofi and Carlos in the same boarding queue: opposite moral choices.
// Requires: the_twins + immigrants_bread seen.
// =========================================================================

static KOFI_REQUIRED: &[&str] = &[
    "dialog_seen_the_twins",
    "dialog_seen_immigrants_bread",
];

pub static SCENE_THREE_DEGREES_OF_KOFI: DialogScene = DialogScene {
    id: "connection_three_degrees_kofi",
    trigger: DialogTrigger::AllDecisionsAndLevel(KOFI_REQUIRED, 100),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been reconstructing the boarding records. \
                   Accra Launch Site, Gate 7, June 14th, 2101. \
                   The last boarding queue.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi Asante. Position 1,247 in the queue. He \
                   had a confirmed seat. He gave it to Adaeze \
                   \u{2014} a stranger \u{2014} because she was \
                   pregnant and alone.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Carlos Mendoza. Position 1,277 in the same \
                   queue. Thirty people behind Kofi. He stole a \
                   dying man's boarding pass so his daughter Elena \
                   could board.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow fractures \u{2014} half warm amber, \
                   half cold blue \u{2014} as if she can't decide \
                   which colour this story deserves.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Same queue. Same moment. Thirty metres apart. \
                   One man gave a seat away. One man took a seat. \
                   And both did it for love.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi loved a stranger enough to die for her. \
                   Carlos loved his daughter enough to steal for \
                   her. The distance between saint and sinner was \
                   thirty metres of tarmac.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I keep running the simulation. If they'd been \
                   standing next to each other, what happens?",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Would Kofi have given his seat to Elena instead? \
                   Would Carlos have looked at Kofi's kindness and \
                   found another way? Would the dying man have \
                   offered his pass willingly?",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can model fluid dynamics, orbital mechanics, \
                   gene expression. But I can't model what thirty \
                   metres of distance does to a moral choice.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "The butterfly effect of a boarding line. One \
                   step left, one step right, and the entire moral \
                   geometry of this ship changes.",
            next: DialogNext::End },
    ],
};

/// Connection scenes part 1 for registration.
pub fn connection_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_PHOTOGRAPH,
        &SCENE_ALGORITHMS_CHILDREN,
        &SCENE_THREE_DEGREES_OF_KOFI,
    ]
}
