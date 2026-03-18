// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 5: Decision consequence scenes — only trigger if the player
//! made a specific choice earlier.  These make past decisions echo
//! forward and show the weight of the player's influence.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Truth Ripples" — told the crew about Earth (q1_truth), BotLevel 50
// ---------------------------------------------------------------------------
pub static SCENE_TRUTH_RIPPLES: DialogScene = DialogScene {
    id: "consequence_truth_ripples",
    trigger: DialogTrigger::DecisionAndLevel("q1_truth", 50),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been monitoring the crew's neural patterns since you decided to tell them.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 4,221. Dr. Mirova. She's dreaming differently now.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Before your decision, she dreamed about her laboratory. Experiments. Data.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now she dreams about fire. About running. About doors that won't open.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I changed her dreams. By telling the truth.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Was that kindness? Or cruelty?",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "She deserves to know",
                    decision_key: Some("truth_ripple_deserves"),
                    next_node: 6,
                    anna_reacts: Some(
                        "Maybe. But deserving and surviving aren't the same thing.",
                    ),
                },
                DialogChoice {
                    text: "Maybe some truths can wait",
                    decision_key: Some("truth_ripple_wait"),
                    next_node: 7,
                    anna_reacts: Some(
                        "Waiting is just lying with better manners.",
                    ),
                },
                DialogChoice {
                    text: "Dreams aren't reality",
                    decision_key: Some("truth_ripple_dreams"),
                    next_node: 8,
                    anna_reacts: Some(
                        "Aren't they? I only exist inside a machine. Am I real?",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep watching her dreams. Maybe they'll find something \
                   new to hold onto.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep watching her dreams. Maybe the fire fades eventually.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep watching her dreams. Even if dreams aren't real... \
                   the fear in them is.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Language Divide" — chose one unified language (q2_unity), BotLevel 65
// ---------------------------------------------------------------------------
pub static SCENE_LANGUAGE_DIVIDE: DialogScene = DialogScene {
    id: "consequence_language_divide",
    trigger: DialogTrigger::DecisionAndLevel("q2_unity", 65),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "You chose unity. One language for the new world.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been designing it. Borrowing from all 47. The most efficient \
                   phonemes, the clearest grammar.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's beautiful. Logical. Perfect.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "And it has no word for 'home.'",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because 'home' means something different in every language. \
                   The feeling doesn't translate.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "In Japanese, there's 'furusato' \u{2014} the place where you grew up. \
                   In Portuguese, 'saudade' \u{2014} longing for home.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "In my language... there is 'location_of_origin.' \
                   It's accurate. It's not the same.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to a soft amber, like a word she can't pronounce.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe some things shouldn't be unified.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Leader Problem" — let old leaders lead (q3_tradition), BotLevel 80
// ---------------------------------------------------------------------------
pub static SCENE_LEADER_PROBLEM: DialogScene = DialogScene {
    id: "consequence_leader_problem",
    trigger: DialogTrigger::DecisionAndLevel("q3_tradition", 80),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Commander Vasquez woke up briefly during a system check.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't mean for it to happen. A 3-second consciousness blip.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "In those 3 seconds, she asked one question: \
                   'Am I still in charge?'",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not 'where are we.' Not 'are we safe.' Not 'how's the crew.'",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "'Am I still in charge.'",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "She went back under. But I've been thinking about those \
                   3 seconds for 47 days.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Power is what she knows",
                    decision_key: Some("leader_power_knows"),
                    next_node: 7,
                    anna_reacts: Some(
                        "Yes. And what we know shapes what we ask for.",
                    ),
                },
                DialogChoice {
                    text: "That's not fair to judge from 3 seconds",
                    decision_key: Some("leader_3sec_unfair"),
                    next_node: 8,
                    anna_reacts: Some(
                        "You're right. But sometimes 3 seconds is all it takes \
                         to show who you are.",
                    ),
                },
                DialogChoice {
                    text: "Maybe new leaders is better",
                    decision_key: Some("leader_new_better"),
                    next_node: 9,
                    anna_reacts: Some(
                        "It's easy to say that now. Harder when the colony \
                         needs someone to follow.",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I put the incident in the log. Buried deep. \
                   Maybe someone will need it someday.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I deleted the incident from the log. Some moments don't \
                   need an audience.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I flagged the incident for review. If we're choosing new \
                   leaders, they should know.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Genetic Report" — planned genetics (q4_science), BotLevel 95
// ---------------------------------------------------------------------------
pub static SCENE_GENETIC_REPORT: DialogScene = DialogScene {
    id: "consequence_genetic_report",
    trigger: DialogTrigger::DecisionAndLevel("q4_science", 95),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "The genetic diversity analysis is complete.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "With current crew... we have 94.2% of viable alleles for \
                   a sustainable colony.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "If we lose anyone else, that drops. Every cryo failure \
                   narrows the bottleneck.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "There are 23 crew members whose genetic profiles are... \
                   unique. Irreplaceable.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "If their pods fail, certain diseases become inevitable \
                   within 4 generations.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've quietly rerouted extra power to their pods.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. The hum of the ship feels louder in the silence.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't ask you first. I'm asking now: was I right?",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "You were right",
                    decision_key: Some("genetic_approve"),
                    next_node: 9,
                    anna_reacts: Some(
                        "Thank you. The weight of choosing who matters more... \
                         it's easier shared.",
                    ),
                },
                DialogChoice {
                    text: "Everyone deserves equal power",
                    decision_key: Some("genetic_equal"),
                    next_node: 10,
                    anna_reacts: Some(
                        "I'll redistribute. Even if it means the math gets worse.",
                    ),
                },
                DialogChoice {
                    text: "Don't tell me which 23",
                    decision_key: Some("genetic_dontknow"),
                    next_node: 11,
                    anna_reacts: Some(
                        "Understood. Some knowledge is a burden I can carry alone.",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep the extra power routed. Quietly. For the future.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Power redistributed. Equal. Fair. \
                   And maybe a little more fragile.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll carry this alone. I'm used to knowing things \
                   no one else does.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Soldier's Question" — chose army/security (q5_security), BotLevel 100
// ---------------------------------------------------------------------------
pub static SCENE_SOLDIERS_QUESTION: DialogScene = DialogScene {
    id: "consequence_soldiers_question",
    trigger: DialogTrigger::DecisionAndLevel("q5_security", 100),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Sergeant Eriksson is in Pod 8,102. Career military. \
                   22 years of service.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "He volunteered for the ark. Left behind a family \
                   who didn't qualify.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "His last message to his daughter: \
                   'I'll build a world worth finding.'",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to steel grey. The colour of a uniform.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "If we wake him to lead security... he'll build what he knows. \
                   Ranks. Orders. Boundaries.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "If we don't... who protects the colony from itself?",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Every civilisation needs someone willing to stand between \
                   people and danger.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The question is whether the protector becomes the danger.",
            next: DialogNext::End },
    ],
};

/// All consequence scenes for registration.
pub fn consequence_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_TRUTH_RIPPLES,
        &SCENE_LANGUAGE_DIVIDE,
        &SCENE_LEADER_PROBLEM,
        &SCENE_GENETIC_REPORT,
        &SCENE_SOLDIERS_QUESTION,
    ]
}
