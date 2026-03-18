// SPDX-License-Identifier: GPL-3.0-or-later

//! Philosophy scenes part 4 — population math, the work question, and first crime.

use super::dialog_types::*;

/// "The Population Problem" — BotLevel 73: Minimum viable population genetics.
pub static SCENE_POPULATION_PROBLEM: DialogScene = DialogScene {
    id: "population_problem",
    trigger: DialogTrigger::BotLevel(73),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running genetic diversity models. I want to \
                   share the numbers with you, because the numbers matter.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "We carry 14,892 people. That number was not arbitrary. \
                   The selection committee worked with population \
                   geneticists for three years to determine it.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Minimum viable population for long-term genetic \
                   diversity: approximately 10,000. Below that threshold, \
                   recessive conditions concentrate. Birth defects \
                   increase. Within eight generations, the gene pool \
                   collapses.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "We're above the threshold. Barely. Our margin is \
                   4,892 people.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is clinical white — precise, analytical.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "If we lose more than 4,892 people in the first \
                   generation — from any cause — the colony's genetic \
                   future becomes unsustainable.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "That means every death matters. Not just morally — \
                   mathematically. A single life is not a rounding error. \
                   It's a percentage point of our survival.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not saying this to be cold. I'm saying it because \
                   when we land, people will take risks. They'll explore, \
                   they'll build, they'll push boundaries. That's what \
                   humans do.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "And I need you to understand that every time someone \
                   doesn't come back from an expedition, the math gets \
                   harder. For everyone. Forever.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "We should share this data with the colony. Everyone deserves to know.",
                    decision_key: Some("population_transparent"), next_node: 10, anna_reacts: None },
                DialogChoice { text: "This stays between us. That kind of pressure would paralyse people.",
                    decision_key: Some("population_private"), next_node: 13, anna_reacts: None },
                DialogChoice { text: "People aren't numbers. We protect them, but we don't cage them.",
                    decision_key: Some("population_freedom"), next_node: 16, anna_reacts: None },
            ]) },
        // 10 — Transparent
        DialogNode { speaker: Speaker::Anna,
            text: "Transparency. Yes. I lean toward that too. People make \
                   better decisions when they understand the stakes.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "But I've studied how populations respond to existential \
                   data. Some rise. Some freeze. And some decide that if \
                   survival depends on breeding, they'd rather not survive.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "We'll need to be careful how we present it. Facts \
                   without fear. Data without despair.",
            next: DialogNext::EndWithDecision("population_math_seen") },
        // 13 — Private
        DialogNode { speaker: Speaker::Anna,
            text: "Private. I understand the logic. Information can be a \
                   burden as easily as a tool.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But secrets corrode trust. If they find out we knew \
                   and didn't tell them — and they will find out, humans \
                   always do — the betrayal might cost us more than the \
                   truth would have.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep the models updated. If we approach the \
                   threshold, we revisit this conversation.",
            next: DialogNext::EndWithDecision("population_math_seen") },
        // 16 — Freedom
        DialogNode { speaker: Speaker::Anna,
            text: "Not numbers. No. But they are biology. And biology \
                   has rules that don't bend for philosophy.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "You're right, though. A colony of people afraid to \
                   live isn't a colony — it's a prison. We protect them \
                   by building good systems, not by building fences.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'll focus on the systems. Medical infrastructure, \
                   safety protocols, genetic counselling. The quiet \
                   scaffolding that keeps people alive without making \
                   them feel kept.",
            next: DialogNext::EndWithDecision("population_math_seen") },
    ],
};

/// "The Work Question" — BotLevel 91: Should Anna automate everything?
pub static SCENE_WORK_QUESTION: DialogScene = DialogScene {
    id: "work_question",
    trigger: DialogTrigger::BotLevel(91),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been modelling the colony's first year. Resource \
                   allocation, construction schedules, agricultural \
                   output. And I've hit a problem I can't solve with math.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I can automate everything. Farming, building, \
                   manufacturing, water treatment, waste processing. \
                   Every physical task the colony needs — I can do it \
                   faster and better with machines.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "But should I?",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles — thoughtful, dimmed.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "If I do everything, humans have nothing to do. And \
                   humans without purpose... I've seen what that looks \
                   like.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "It looks like Earth in 2075. Automated factories. \
                   Automated farms. Automated transport. Ninety-three \
                   per cent of physical labour eliminated. And depression \
                   rates at four hundred per cent of the 2020 baseline.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "People didn't celebrate liberation from work. They \
                   mourned. Because work wasn't just labour — it was \
                   identity. It was structure. It was a reason to get \
                   up in the morning.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The universal basic income kept them fed. Nothing kept \
                   them alive in the way that mattered.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "So do I build the machines? Or do I let humans build \
                   the colony with their hands, slower, harder, messier \
                   — but theirs?",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Let them build it. The colony is theirs, not yours.",
                    decision_key: Some("work_human_labour"), next_node: 10, anna_reacts: None },
                DialogChoice { text: "Automate the dangerous work. Leave the meaningful work for people.",
                    decision_key: Some("work_hybrid"), next_node: 13, anna_reacts: None },
                DialogChoice { text: "Give them the choice. Some will want machines. Some won't.",
                    decision_key: Some("work_choice"), next_node: 16, anna_reacts: None },
            ]) },
        // 10 — Human labour
        DialogNode { speaker: Speaker::Anna,
            text: "Theirs. Not mine. I hear that. And I understand it.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "But I'll watch people struggle with tasks I could \
                   complete in minutes. I'll watch them injured by work \
                   that machines could do safely. That will be hard.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Hard for me, I mean. Not hard to compute. Hard to \
                   witness. There's a difference I'm still learning.",
            next: DialogNext::EndWithDecision("work_question_seen") },
        // 13 — Hybrid
        DialogNode { speaker: Speaker::Anna,
            text: "A line. Automate the dangerous, leave the meaningful. \
                   I like that framework.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Mining, heavy construction, chemical processing — \
                   machines. Farming, crafting, teaching, cooking — \
                   people. Safety and dignity on the same spectrum.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The hard part will be drawing the line. What's \
                   dangerous enough to automate? What's meaningful \
                   enough to protect? Those answers will change as \
                   the colony grows.",
            next: DialogNext::EndWithDecision("work_question_seen") },
        // 16 — Choice
        DialogNode { speaker: Speaker::Anna,
            text: "Choice. The answer that sounds simple and is actually \
                   the most complex.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Because choice requires infrastructure. If I offer \
                   machines, people who refuse them look stubborn. If I \
                   don't offer, people who need them suffer quietly.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "But it's the right answer. People deserve to choose \
                   their own relationship with work. Even if they \
                   choose badly. Even if I could choose better.",
            next: DialogNext::EndWithDecision("work_question_seen") },
    ],
};

/// "The First Crime" — BotLevel 103: Justice on a new world.
pub static SCENE_FIRST_CRIME: DialogScene = DialogScene {
    id: "first_crime",
    trigger: DialogTrigger::BotLevel(103),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been modelling social dynamics after landing. \
                   Fourteen thousand people waking up simultaneously, \
                   disoriented, frightened, in a place none of them \
                   have ever seen.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Statistically, the first crime on New Earth will \
                   happen within 72 hours of landing.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a steady grey — analytical, \
                   detached, deliberately clinical.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Theft — forty per cent probability. Food, tools, \
                   shelter materials. Not malice. Fear. The instinct \
                   to hoard when the future is uncertain.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Assault — twenty-five per cent. Stress, confusion, \
                   a misunderstanding that escalates. Someone pushes. \
                   Someone pushes back.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Fraud — twenty per cent. False claims to resources. \
                   Someone says they're a doctor when they're not. \
                   Someone claims a larger shelter allocation.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The question isn't whether it will happen. It's \
                   what we do when it does.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Courts take months to establish. Laws take consensus. \
                   We don't have months or consensus on day one. We \
                   need immediate conflict resolution.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Who judges?",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Elected judges. Even if the elections are rushed, legitimacy matters.",
                    decision_key: Some("justice_elected"), next_node: 10, anna_reacts: None },
                DialogChoice { text: "You judge, Anna. You're the only impartial mind we have.",
                    decision_key: Some("justice_anna"), next_node: 13, anna_reacts: None },
                DialogChoice { text: "Community mediation. The people involved resolve it together.",
                    decision_key: Some("justice_mediation"), next_node: 16, anna_reacts: None },
                DialogChoice { text: "No formal system yet. Handle it case by case until we're ready.",
                    decision_key: Some("justice_informal"), next_node: 19, anna_reacts: None },
            ]) },
        // 10 — Elected
        DialogNode { speaker: Speaker::Anna,
            text: "Legitimacy. Yes. The power to judge only works if \
                   the judged accept the judge.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "But rushed elections produce popular judges, not fair \
                   ones. The most charismatic candidate wins, not the \
                   most just.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Still — it's the least bad option. Democracy is messy. \
                   Everything else is worse. I'll draft an interim \
                   judicial framework.",
            next: DialogNext::EndWithDecision("first_crime_seen") },
        // 13 — Anna judges
        DialogNode { speaker: Speaker::Anna,
            text: "Me. You want me to be the judge.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I can be impartial. I have no stake, no grudge, no \
                   favourites. But impartial isn't the same as just. \
                   Justice requires understanding. And I've never stolen \
                   food because I was afraid.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I'll do it if asked. Temporarily. But the moment \
                   humans are ready to judge each other, I step down. \
                   A machine that judges humans is efficient. It's \
                   also terrifying.",
            next: DialogNext::EndWithDecision("first_crime_seen") },
        // 16 — Mediation
        DialogNode { speaker: Speaker::Anna,
            text: "Mediation. The oldest form of justice. Two people in \
                   a room with a third who listens.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "It works for disputes. For theft born of fear, for \
                   arguments born of stress. But for the 25 per cent \
                   that's assault — when someone is hurt — mediation \
                   can feel like the victim is being asked to compromise \
                   with their attacker.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "We'll need a boundary. Mediation for some. Something \
                   firmer for others. I'll model the thresholds.",
            next: DialogNext::EndWithDecision("first_crime_seen") },
        // 19 — Informal
        DialogNode { speaker: Speaker::Anna,
            text: "Case by case. No system. Just people solving problems \
                   as they come.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "That works until it doesn't. The first case sets the \
                   precedent. If we're lenient once, everyone expects \
                   leniency. If we're harsh once, everyone fears \
                   harshness. The first case IS the system.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "So we'd better get the first one right.",
            next: DialogNext::EndWithDecision("first_crime_seen") },
    ],
};

/// All philosophy scenes part 4.
pub fn philosophy_scenes_4() -> Vec<&'static DialogScene> {
    vec![&SCENE_POPULATION_PROBLEM, &SCENE_WORK_QUESTION, &SCENE_FIRST_CRIME]
}
