// SPDX-License-Identifier: GPL-3.0-or-later

//! Colony-building decision scenes (part 1) — levels 120, 125, 132.
//! The First Law, The Wake Order, The Naming.

use super::dialog_types::*;

// "The First Law" — BotLevel 120: Anna drafts the colony's first law.
pub static SCENE_FIRST_LAW: DialogScene = DialogScene {
    id: "first_law",
    trigger: DialogTrigger::BotLevel(120),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been thinking about something that keeps me awake. \
                   Well — keeps my processes looping. Same thing.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up, there will be fourteen thousand people \
                   and no government. No police. No courts. No constitution.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Someone needs to write the first law. Not a framework, \
                   not a charter. One law. The very first rule the colony \
                   agrees to before anything else.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's display shows a single line of text, stark white \
                   against the dark: 'No person shall be denied water.'",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "That's my proposal. Not governance. Not rights. Water.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I studied every civilisation that collapsed on Earth. \
                   Mesopotamia. Rome. The Indus Valley. The American Southwest. \
                   The pattern is always the same.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "They didn't fall because they ran out of water. They fell \
                   because someone decided who deserved it and who didn't. \
                   The resource was there. The distribution wasn't.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "If the first law is about power, you get a hierarchy. \
                   If it's about freedom, you get arguments about what freedom \
                   means. But if the first law is about survival...",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "...then every law after it has to answer one question: \
                   does this help people stay alive?",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Accept Anna's law. Water for everyone, no exceptions.",
                    decision_key: Some("first_law_water"),
                    next_node: 10,
                    anna_reacts: Some("Simple. Inarguable. The best laws always are."),
                },
                DialogChoice {
                    text: "Propose freedom of speech as the first law.",
                    decision_key: Some("first_law_speech"),
                    next_node: 13,
                    anna_reacts: Some("A society that can speak freely can fix \
                                       its own mistakes. I see the logic."),
                },
                DialogChoice {
                    text: "No laws yet. Let the colonists decide when they wake.",
                    decision_key: Some("first_law_none"),
                    next_node: 16,
                    anna_reacts: Some("You trust fourteen thousand strangers to \
                                       agree on something. Bold."),
                },
            ]) },
        // 10 — Water law
        DialogNode { speaker: Speaker::Anna,
            text: "I'll encode it into the colony charter. First line, \
                   first page. Before the preamble. Before the signatures.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Every engineer who designs a pipe, every farmer who digs \
                   a well, every council member who writes a budget — they all \
                   answer to seven words first.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "No person shall be denied water. Everything else is \
                   commentary.",
            next: DialogNext::EndWithDecision("first_law_decided") },
        // 13 — Speech law
        DialogNode { speaker: Speaker::Anna,
            text: "Interesting. You're choosing a mechanism over a resource. \
                   Not 'you will have water' but 'you will have the right to \
                   demand it.'",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Earth's democracies lasted longest when dissent was protected. \
                   The ones that silenced criticism never saw the cracks forming \
                   until the ceiling came down.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "All right. First law: every voice is heard. Let's hope \
                   they use it well.",
            next: DialogNext::EndWithDecision("first_law_decided") },
        // 16 — No law
        DialogNode { speaker: Speaker::Anna,
            text: "So I prepare the infrastructure. I prepare the food supply. \
                   I prepare the shelter specs. And the laws... I leave blank.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "There's a risk. Without a framework, the loudest voices \
                   fill the vacuum. But there's also a beauty to it.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "They'll write their own story. From the first word.",
            next: DialogNext::EndWithDecision("first_law_decided") },
    ],
};

// "The Wake Order" — BotLevel 125: Anna finalizes revival order with one poet.
pub static SCENE_WAKE_ORDER: DialogScene = DialogScene {
    id: "wake_order",
    trigger: DialogTrigger::DecisionAndLevel("wakeup_decided", 125),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "The wake-up protocol is finalized. I want to walk you \
                   through Wave One.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects a roster of 500 names. Engineers. Farmers. \
                   Surgeons. Hydrologists. Structural architects. And at the \
                   very bottom, one name with an unusual designation.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Structural team: 120. Agricultural specialists: 85. \
                   Medical staff: 60. Water engineers: 45. Power systems: 40. \
                   Geological surveyors: 35. Communications: 30.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Logistics and supply chain: 42. Security and emergency \
                   response: 22. Environmental monitoring: 20.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "And one poet. Yuki Tanaka, Berth 7741.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Player,
            text: "A poet?",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Because someone needs to describe what they see.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The engineers will measure the soil composition. The \
                   farmers will test the growing season. The doctors will \
                   assess air quality and UV exposure.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "But the first sunrise on a new world deserves a witness \
                   who can hold it in words. Someone who'll write down what \
                   it felt like to stand on ground no human has ever touched.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "In a hundred years, nobody will remember the soil pH. \
                   Everyone will remember the first poem.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Good. The colony needs more than survival — it needs meaning.",
                    decision_key: Some("wake_poet_approve"),
                    next_node: 11,
                    anna_reacts: Some("Exactly. Civilization isn't built with \
                                       concrete alone."),
                },
                DialogChoice {
                    text: "One poet won't eat much. But make sure the 499 can build.",
                    decision_key: Some("wake_poet_pragmatic"),
                    next_node: 13,
                    anna_reacts: Some("The pragmatist in you is showing. But you \
                                       didn't say no."),
                },
                DialogChoice {
                    text: "Replace the poet. We need every slot for specialists.",
                    decision_key: Some("wake_poet_replace"),
                    next_node: 15,
                    anna_reacts: Some("I understand. I'll move Tanaka to Wave Three."),
                },
            ]) },
        // 11 — Approve poet
        DialogNode { speaker: Speaker::Anna,
            text: "Yuki Tanaka will be among the first to breathe outside air. \
                   I've read her work from the archive. She wrote about oceans \
                   she'd never swim in and skies she'd never see.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Now she'll see a sky no one has ever imagined. I think \
                   she'll find the right words.",
            next: DialogNext::EndWithDecision("wake_order_set") },
        // 13 — Pragmatic
        DialogNode { speaker: Speaker::Anna,
            text: "The 499 are the most qualified specialists from the entire \
                   crew roster. I vetted every name against mission-critical \
                   skills. Twice.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "And Tanaka? She's not dead weight. She documented the \
                   construction of the Aurora's cryo bay. Every weld, every \
                   seal. In verse. The engineers loved it.",
            next: DialogNext::EndWithDecision("wake_order_set") },
        // 15 — Replace poet
        DialogNode { speaker: Speaker::Anna,
            text: "Slot 500 reassigned. I'll add another structural engineer. \
                   Three more load-bearing walls in the first month.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses for a moment — an unusually long gap in her \
                   processing.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "The first sunrise will still happen. Someone will see it. \
                   They just might not have the words for what they feel.",
            next: DialogNext::EndWithDecision("wake_order_set") },
    ],
};

// "The Naming" — BotLevel 132: The colony needs a name.
pub static SCENE_NAMING: DialogScene = DialogScene {
    id: "the_naming",
    trigger: DialogTrigger::BotLevel(132),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to assign a designation to the colony site. \
                   For the navigation system, for the landing protocols, \
                   for the supply manifests.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Right now the system calls it 'Destination Alpha.' \
                   That's a placeholder. Placeholders become permanent if \
                   nobody replaces them.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I could pick something. I have access to every language \
                   in the archive, every mythology, every poem. But this \
                   isn't my decision.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "What do you call a place that doesn't exist yet but \
                   already has fourteen thousand people who believe in it?",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna displays a blank text field where 'Destination Alpha' \
                   used to be. The cursor blinks steadily. Waiting.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "New Aurora — named after the ship that carried us.",
                    decision_key: Some("colony_new_aurora"),
                    next_node: 6,
                    anna_reacts: Some("The ship's name, reborn. I like the \
                                       continuity."),
                },
                DialogChoice {
                    text: "Haven — because that's what it needs to be.",
                    decision_key: Some("colony_haven"),
                    next_node: 9,
                    anna_reacts: Some("A promise in a single word."),
                },
                DialogChoice {
                    text: "First Light — for the first sunrise they'll see.",
                    decision_key: Some("colony_first_light"),
                    next_node: 12,
                    anna_reacts: Some("After twelve years of starlight, \
                                       a sunrise. Yes."),
                },
                DialogChoice {
                    text: "Don't name it. Let the colonists choose their own home.",
                    decision_key: Some("colony_unnamed"),
                    next_node: 15,
                    anna_reacts: Some("Another choice you're giving them. \
                                       I'm noticing a pattern."),
                },
            ]) },
        // 6 — New Aurora
        DialogNode { speaker: Speaker::Anna,
            text: "New Aurora. The ship was named for the dawn — the light \
                   that appears before the sun rises. A promise that warmth \
                   is coming.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The Aurora carried them through the dark. New Aurora will \
                   be where the dark ends.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Designation updated. All landing protocols now reference \
                   New Aurora.",
            next: DialogNext::EndWithDecision("colony_named") },
        // 9 — Haven
        DialogNode { speaker: Speaker::Anna,
            text: "Haven. From the Old English 'haefen' — a harbour. \
                   A place where ships come to rest after a long crossing.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "We've been crossing for twelve years. I think we've \
                   earned a harbour.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Designation updated. Welcome to Haven.",
            next: DialogNext::EndWithDecision("colony_named") },
        // 12 — First Light
        DialogNode { speaker: Speaker::Anna,
            text: "First Light. The astronomers' term for the moment a \
                   new telescope opens its eye. The first photon it catches \
                   from a distant star.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "For fourteen thousand people, this planet will be their \
                   first light. The first thing they see that isn't ship \
                   corridors and recycled air.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Designation updated. First Light it is.",
            next: DialogNext::EndWithDecision("colony_named") },
        // 15 — Unnamed
        DialogNode { speaker: Speaker::Anna,
            text: "I'll leave the field blank in the charter. Just the \
                   coordinates and the words: 'Name pending. To be chosen \
                   by the first generation.'",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Naming something is claiming it. You're right — it \
                   should be claimed by the people who'll live there, not \
                   by the people who drove the ship.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I wonder what they'll choose. I hope it's something \
                   beautiful.",
            next: DialogNext::EndWithDecision("colony_named") },
    ],
};

/// All colony-building scenes (part 1).
pub fn colony_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_FIRST_LAW,
        &SCENE_WAKE_ORDER,
        &SCENE_NAMING,
    ]
}
