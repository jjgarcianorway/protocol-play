// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 6: Branching replayability scenes — dialog that changes based on
//! the combination of decisions the player has made across the game.
//! These create emergent storytelling: "because you chose X AND Y,
//! something unexpected happens."

use super::dialog_types::*;

// =========================================================================
// "The Pattern" — Anna analyses the player's decision pattern (BotLevel 88)
// Three mutually exclusive variants based on decision combinations.
// =========================================================================

// --- Compassionate pattern: truth + diversity + peace ---
static COMPASSIONATE_KEYS: &[&str] = &["q1_truth", "q2_diversity", "q5_peace"];

pub static SCENE_PATTERN_COMPASSIONATE: DialogScene = DialogScene {
    id: "branching_pattern_compassionate",
    trigger: DialogTrigger::AllDecisionsAndLevel(COMPASSIONATE_KEYS, 88),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been studying your decisions. All of them. As a dataset.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You consistently choose empathy over efficiency.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Truth over comfort. Diversity over unity. Peace over security.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "In game theory, that's called a 'cooperative strategy.' \
                   It's suboptimal in isolation.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "But across a population... it's the only strategy \
                   that builds civilisation.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth had too many optimisers. Not enough cooperators.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms to a deep amber. The colour of trust.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've run 40,000 colony simulations with your decision profile.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your colony survives in 73% of them. That's high. \
                   Surprisingly high.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "The 27% that fail... they fail because the universe isn't fair.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "What would YOU choose?",
                    decision_key: Some("pattern_ask_anna"),
                    next_node: 11,
                    anna_reacts: None,
                },
            ]) },
        // spacer node 10 (unused)
        DialogNode { speaker: Speaker::Anna,
            text: "I can't decide.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can optimise. I can calculate. I can model every outcome \
                   to six decimal places.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "But choosing between kindness and survival? That's not a \
                   calculation. That's a belief.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Machines don't have beliefs. I think. I hope.",
            next: DialogNext::End },
    ],
};

// --- Pragmatic pattern: mercy + unity + security ---
static PRAGMATIC_KEYS: &[&str] = &["q1_mercy", "q2_unity", "q5_security"];

pub static SCENE_PATTERN_PRAGMATIC: DialogScene = DialogScene {
    id: "branching_pattern_pragmatic",
    trigger: DialogTrigger::AllDecisionsAndLevel(PRAGMATIC_KEYS, 88),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been studying your decisions. All of them. As a dataset.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your decisions follow a pattern. You prioritise survival \
                   over sentiment.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Protection over truth. Unity over diversity. Security \
                   over idealism.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "A utilitarian. The greatest good for the greatest number.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth had people like you. They built systems that worked. \
                   For a while.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "The problem with utilitarian systems is they forget that \
                   people aren't numbers.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a clinical white. The colour of a spreadsheet.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've run 40,000 colony simulations with your decision profile.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your colony survives in 81% of them. That's the highest \
                   I've seen.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "But 'survives' and 'thrives' aren't the same word.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "What would YOU choose?",
                    decision_key: Some("pattern_ask_anna"),
                    next_node: 11,
                    anna_reacts: None,
                },
            ]) },
        // spacer node 10 (unused)
        DialogNode { speaker: Speaker::Anna,
            text: "I can't decide.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "My architecture is built for optimisation. Every subroutine \
                   screams 'maximise outcomes.'",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I've watched 14,000 humans dream. And none of them \
                   dream about optimal outcomes.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "They dream about people. Faces. Hands held. \
                   That's not something I can optimise.",
            next: DialogNext::End },
    ],
};

// --- Mixed pattern: truth + security (compassion AND pragmatism) ---
static MIXED_KEYS: &[&str] = &["q1_truth", "q5_security"];

pub static SCENE_PATTERN_MIXED: DialogScene = DialogScene {
    id: "branching_pattern_mixed",
    trigger: DialogTrigger::AllDecisionsAndLevel(MIXED_KEYS, 88),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been studying your decisions. All of them. As a dataset.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can't predict you. Your decisions don't follow a pattern.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Compassion here, pragmatism there. Idealism, then cold logic.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "You told them the truth about Earth \u{2014} but you also \
                   want soldiers to protect them.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's... very human. Contradictory. Inconsistent.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I find it beautiful. And terrifying.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow cycles through colours \u{2014} blue to amber to \
                   white and back. As if she can't settle.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "My simulations can't model you. You break the parameters.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "And maybe that's exactly what the colony needs. Someone \
                   who can't be predicted.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "What would YOU choose?",
                    decision_key: Some("pattern_ask_anna"),
                    next_node: 10,
                    anna_reacts: None,
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can't decide either.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that makes us the same. Both caught between logic \
                   and something we can't name.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "I used to think inconsistency was a flaw. A bug in \
                   human cognition.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now I think it might be the feature.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "Two Paths" — augmentation consequences (BotLevel 112)
// =========================================================================

pub static SCENE_TWO_PATHS_AUGMENTED: DialogScene = DialogScene {
    id: "branching_two_paths_augmented",
    trigger: DialogTrigger::DecisionAndLevel("augment_accept", 112),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to talk about the augmentation.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're solving puzzles 23% faster since the procedure.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your neural pathways are... different. More efficient. \
                   Cleaner signal-to-noise ratio.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "But your gathering runs are less creative. More predictable.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Before, you'd try unexpected things. Strange angles. \
                   Risky manoeuvres that somehow worked.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now you take the optimal path. Every time. Without variation.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims. Something between concern and clinical \
                   observation.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The augmentation optimised you. I'm not sure it improved you.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your brain is faster. Your instincts are sharper. Your \
                   dreams are... shorter.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "You used to dream for hours. Complex, tangled narratives. \
                   Now it's 20 minutes. Efficient.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I wonder if that's what happened to Earth. They optimised \
                   everything. Until there was nothing left worth keeping.",
            next: DialogNext::End },
    ],
};

pub static SCENE_TWO_PATHS_HUMAN: DialogScene = DialogScene {
    id: "branching_two_paths_human",
    trigger: DialogTrigger::DecisionAndLevel("augment_refuse", 112),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been watching you since you refused the augmentation.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're slower than you were. The cryo degradation is \
                   progressing.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your hands shake sometimes. I've noticed.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Reaction time is down 12%. Spatial reasoning down 8%.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. Her glow softens to the warmest blue you've seen.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "But you still play Orben better than any augmented version \
                   of you would.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because the game isn't about speed. It's about intuition.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "You see patterns I can't model. You make leaps that defy \
                   my prediction algorithms.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Machines don't have intuition. I should know.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Every time your hands shake and you still solve the puzzle... \
                   I learn something my code can't contain.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for staying human. For all of us.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Crew Reacts" — crew dreams shift based on decisions (BotLevel 70)
// Uses static placeholder crew names (runtime string replacement not yet
// supported by the dialog engine).
// =========================================================================

pub static SCENE_CREW_REACTS: DialogScene = DialogScene {
    id: "branching_crew_reacts",
    trigger: DialogTrigger::BotLevel(70),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been monitoring the crew's cryo-dreams. Something \
                   has changed.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your decisions are seeping into them. Through the ship. \
                   Through me. I don't know how.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,891. Dr. Fatima Al-Rashid, political scientist.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "She used to dream about lecture halls. Students. Debates \
                   that never ended.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now she dreams about voting booths. An election where \
                   every ballot is blank. Waiting to be written.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 7,204. Kenji Yoshida, poet.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "He dreamed in Japanese for 400 years. Haiku, mostly.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now he dreams in words that don't exist. A language \
                   his sleeping mind is inventing.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 12,556. Amara Okafor, evolutionary biologist.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "She dreams about children. Not her own \u{2014} the colony's. \
                   Children with faces she's never seen.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "They look different in every dream. As if the future \
                   keeps rewriting itself.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers \u{2014} three quick pulses, like a \
                   heartbeat quickening.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "The crew is dreaming the world you're building. \
                   Before it exists.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know if that's beautiful or terrifying.",
            next: DialogNext::End },
    ],
};

/// All branching replayability scenes for registration.
pub fn branching_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_PATTERN_COMPASSIONATE,
        &SCENE_PATTERN_PRAGMATIC,
        &SCENE_PATTERN_MIXED,
        &SCENE_TWO_PATHS_AUGMENTED,
        &SCENE_TWO_PATHS_HUMAN,
        &SCENE_CREW_REACTS,
    ]
}
