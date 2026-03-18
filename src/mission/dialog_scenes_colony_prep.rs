// SPDX-License-Identifier: GPL-3.0-or-later

//! Practical colony preparation — soil, medicine, and power. Real science, real stakes.

use super::dialog_types::*;

/// "The Soil Question" — BotLevel 87: Which Earth crops grow in alien soil?
pub static SCENE_SOIL_QUESTION: DialogScene = DialogScene {
    id: "soil_question",
    trigger: DialogTrigger::BotLevel(87),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running agricultural simulations with the spectral \
                   data from the target planet. The soil analysis is interesting.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The planet has soil. Mineral composition roughly analogous to \
                   volcanic basalt. Iron-rich, phosphorus present. But it's never \
                   grown anything. No organic matter. No microbes. Mineral dust, \
                   not dirt.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Dirt is alive. Soil is rock. We're landing on rock and we need \
                   to make it alive.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna displays a table. Some numbers are green. Most are red.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Bamboo: 67% germination. Aggressive, tolerant, doesn't need \
                   mycorrhizae. Rice: 34%. Needs standing water and specific pH. \
                   Potatoes: 41%. Need organic matter — composting first.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Wheat: 8%. Needs established soil microbiome. Earth took three \
                   thousand years to optimize wheat farming. Mei-Lin's jasmine: \
                   12%. Delicate, bred for specific conditions.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "We might arrive on a planet where bamboo grows but wheat doesn't. \
                   The colony's diet will be determined by biochemistry, not preference.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Player, // 7
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Start with bamboo and rice. Survival crops first.",
                    decision_key: Some("soil_survival"), next_node: 8,
                    anna_reacts: Some("Pragmatic. Feed them first, then worry about variety.") },
                DialogChoice { text: "Build the soil first. Compost, microbes, the whole ecosystem.",
                    decision_key: Some("soil_ecosystem"), next_node: 10,
                    anna_reacts: Some("The slow path. Harder, but it gives us real soil.") },
                DialogChoice { text: "Plant everything. Let the planet tell us what it accepts.",
                    decision_key: Some("soil_experiment"), next_node: 13,
                    anna_reacts: Some("The scientific approach. Expensive in seeds. Rich in data.") },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 8 — Survival
            text: "Bamboo for structure and fiber. Rice for calories. Between them, \
                   the colony survives the first year.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll brief Mei-Lin when she wakes. She'll understand. The jasmine \
                   can wait. Survival can't.",
            next: DialogNext::EndWithDecision("soil_decided") },
        DialogNode { speaker: Speaker::Anna, // 10 — Ecosystem
            text: "We have microbial cultures in the bio-vault. Earth bacteria, fungi, \
                   nitrogen-fixing organisms. The building blocks of living soil.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "The first season is composting, not planting. The colony eats from \
                   stored rations while the soil comes alive. Six months of patience.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "But after that, the soil works with us. Every crop gets a real \
                   chance. Even the wheat. Even the jasmine.",
            next: DialogNext::EndWithDecision("soil_decided") },
        DialogNode { speaker: Speaker::Anna, // 13 — Experiment
            text: "Enough seed stock for three full cycles. Dedicate the first to \
                   experimental plots — every species, every variety, mapped to \
                   soil conditions.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "We'll know exactly what grows within one season. The risk is \
                   burning a third of our seeds. But we gain certainty — the one \
                   thing I can't simulate from twelve light-years away.",
            next: DialogNext::EndWithDecision("soil_decided") },
    ],
};

/// "The Medical Inventory" — BotLevel 99: What the colony has and doesn't have.
pub static SCENE_MEDICAL_INVENTORY: DialogScene = DialogScene {
    id: "medical_inventory",
    trigger: DialogTrigger::BotLevel(99),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've completed the full medical inventory audit. There are gaps \
                   you should know about.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Antibiotics: enough for two years at expected infection rates. \
                   Surgical supplies: material for 500 operations. After that, we \
                   manufacture from raw materials.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Fifteen surgeons. Eight GPs. Twelve nurses. Four anesthesiologists. \
                   Two radiologists with no imaging equipment heavy enough to bring.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Zero dentists.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna lets that sit for a moment.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can perform surgery through the medical robotic systems. I can \
                   diagnose, prescribe, monitor vitals. I cannot pull a tooth. The \
                   fine motor precision exceeds my interface capabilities.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The first cavity on New Earth will be a genuine crisis.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The broader picture: we have two years of medicine. After that, \
                   the colony depends on preventive care and whatever we can make. \
                   Elena's pharmaceutical chemistry becomes critical at that point.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player, // 8
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Train surgeons in dental procedures. Cross-skill the team.",
                    decision_key: Some("medical_crosstrain"), next_node: 9,
                    anna_reacts: Some("A surgeon who can pull a tooth isn't a dentist, \
                                       but they're better than nothing.") },
                DialogChoice { text: "Prioritize Elena's pharmaceutical lab. Medicine production first.",
                    decision_key: Some("medical_pharma"), next_node: 11,
                    anna_reacts: Some("If we synthesize antibiotics, the two-year clock \
                                       stops ticking.") },
                DialogChoice { text: "What about the long term? Generations from now?",
                    decision_key: Some("medical_longterm"), next_node: 13, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 9 — Cross-train
            text: "I have the full dental curriculum in the archive. Anatomy, \
                   extraction, prosthetics. I can train anyone with steady hands.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "By year two, six people handling basic dental care. Not elegant. \
                   But nobody dies from an untreated abscess.",
            next: DialogNext::EndWithDecision("medical_planned") },
        DialogNode { speaker: Speaker::Anna, // 11 — Pharma
            text: "Elena's lab gets construction priority. Clean room, distillation, \
                   sterile storage.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "If we synthesize penicillin within eighteen months, we extend our \
                   medical runway indefinitely. The colony stops being dependent on \
                   finite supplies.",
            next: DialogNext::EndWithDecision("medical_planned") },
        DialogNode { speaker: Speaker::Anna, // 13 — Long term
            text: "Generations from now, they need a medical school. I've drafted a \
                   curriculum. Twelve years on Earth, compressed to eight with \
                   simulation training. First graduates by Year Twenty.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Until then, I'm the backup. Every manual, every protocol, every \
                   algorithm. I don't sleep. I don't forget. That's what I'm for.",
            next: DialogNext::EndWithDecision("medical_planned") },
    ],
};

/// "The Power Budget" — BotLevel 113: Viktor's reactor gives 10 years. Then what?
pub static SCENE_POWER_BUDGET: DialogScene = DialogScene {
    id: "power_budget",
    trigger: DialogTrigger::BotLevel(113),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I want to talk about what happens in Year Ten.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor's reactor has a ten-year operational window after landing. \
                   Fuel rods depleted by then. No replacements. After that, the \
                   colony either has alternative energy or goes dark.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Three options. Each depends on what the planet gives us.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Solar: if the star's spectrum is compatible, panel arrays. Viktor's \
                   team has fabrication knowledge. Output: 40% of reactor capacity.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Wind: if the atmosphere has consistent pressure differentials — \
                   early data suggests yes — turbines are feasible. Kwame's designs \
                   include a template. Output: 25%.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Hydroelectric: if Amira's river mapping confirms stable waterflow, \
                   a small dam provides continuous baseline power. Output: 55%.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "No single source replaces the reactor. But two out of three give \
                   us 65 to 95 percent. Survivable. The question is what we build \
                   first — Year Ten is a deadline, construction starts in Year One.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Player, // 7
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Hydroelectric first. Highest output, most reliable.",
                    decision_key: Some("power_hydro"), next_node: 8,
                    anna_reacts: Some("Amira's river data becomes the most valuable \
                                       dataset on the ship.") },
                DialogChoice { text: "Solar first. Fastest to deploy, lowest risk.",
                    decision_key: Some("power_solar"), next_node: 11,
                    anna_reacts: Some("Quick wins matter. Power on day one buys time \
                                       for everything else.") },
                DialogChoice { text: "Build all three in parallel. Don't bet on one.",
                    decision_key: Some("power_all"), next_node: 14,
                    anna_reacts: Some("Ambitious. Spreads the workforce thin, but \
                                       eliminates single points of failure.") },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 8 — Hydro
            text: "A dam takes two years minimum. But once running, it doesn't stop. \
                   Amira surveys the river system within the first month — flow rates, \
                   seasonal variation, sediment load.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everything her models predicted, verified on the ground.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "If the river cooperates, the colony has power for centuries. If \
                   it doesn't, we'd better have a backup ready.",
            next: DialogNext::EndWithDecision("power_planned") },
        DialogNode { speaker: Speaker::Anna, // 11 — Solar
            text: "Solar panels deployed within weeks of landing. Viktor's team knows \
                   fabrication. Raw materials abundant in basalt-rich soil.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "The unknown is the star. Our spectral data says it should work. \
                   'Should' isn't 'does' until someone holds a panel up to that sky.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "First sunrise, first test. If the cells respond, we start arrays \
                   immediately. Year One power insurance.",
            next: DialogNext::EndWithDecision("power_planned") },
        DialogNode { speaker: Speaker::Anna, // 14 — All three
            text: "Three construction teams. Three timelines. Viktor, Amira, and \
                   Kwame all start on day one. Three of our most critical people, \
                   pulled in different directions.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "The coordination will be complex. But redundancy saves lives. \
                   Earth collapsed partly because it depended on single systems. \
                   We won't make that mistake twice.",
            next: DialogNext::EndWithDecision("power_planned") },
    ],
};

/// All colony preparation scenes.
pub fn colony_prep_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_SOIL_QUESTION, &SCENE_MEDICAL_INVENTORY, &SCENE_POWER_BUDGET]
}
