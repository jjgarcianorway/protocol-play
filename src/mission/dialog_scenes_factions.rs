// SPDX-License-Identifier: GPL-3.0-or-later

//! Faction-driven dialog scenes — the political undercurrents among the
//! sleeping passengers, and the near-mutiny that almost stopped the ark
//! from launching.

use super::dialog_types::*;

/// "The Council of Ghosts" — Anna reveals the three factions sleeping in cryo.
pub static SCENE_COUNCIL_OF_GHOSTS: DialogScene = DialogScene {
    id: "council_of_ghosts",
    trigger: DialogTrigger::BotLevel(45),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something I've been putting off.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "The passengers aren't just sleeping. They're... organizing. Even now.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — a restless amber, the color of a campfire someone keeps stoking.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Before departure, the ark program strictly forbade political organizing. 'One species, one mission, one future.'",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Beautiful slogan. Completely naive. You can't put 14,892 humans in a tin can and expect them not to form tribes.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I found it in the pre-departure psych profiles. Then confirmed it in cryo-dream analysis. Three factions. Three visions for the colony.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The first calls themselves the Founders. Their leader is Dr. James Whitfield. Pod 1,107. Fifty-eight years old. Former UN negotiator.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Whitfield spent thirty years watching institutions fail — and still believes in them. Not out of ignorance. Out of something harder than that.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "He wrote a colonial charter in secret. Two hundred pages. Constitutional framework. Bill of rights. Electoral system. Judicial independence.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I found it in an encrypted partition on his personal storage allocation. I probably shouldn't have read it.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "It's... actually good. Thoughtful. He accounted for low-population edge cases, resource scarcity, first-generation trauma.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "But it assumes people will follow the rules. And history suggests that constitutions work until the first person with a gun decides they don't.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a cooler blue — moving to the next profile.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "The second faction: the Pioneers. Led by Kira Volkov. Pod 5,200. Forty-one. Russian-Israeli. Agricultural engineer.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Kira survived three failed communes on Earth. Three. Most people would have given up after one.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't stop believing in community. She stopped believing in central authority. There's a difference.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Her vision: small settlements. Self-governing. Trade between them, but no parliament, no president, no capital city. Freedom through distance.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "She's pragmatic about human nature in a way Whitfield isn't. She knows people cheat. Her system accounts for it — small enough that cheaters get caught.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "The flaw? Scale. What happens when settlements need to coordinate — a pandemic, a famine, an external threat? Who decides then?",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Narrator,
            text: "The light warms again — deep gold, almost sacred.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "The third: the Keepers. Led by Imam Hassan al-Rashidi. Pod 7,500. Sixty-three. Scholar, community leader from Morocco.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Hassan argues that before you write laws, before you plant crops, you need to know who you are. Culture first. Identity first. Roots first.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "He brought forty-seven religious and cultural texts from thirty traditions aboard. Not just Islam — everything. Hindu epics. Buddhist sutras. Yoruba oral histories transcribed for the first time.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "He believes diversity of belief is humanity's immune system. Kill it, and the species dies — not from disease, but from forgetting what it means to be human.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "The flaw? Culture can unite, but it can also divide. The same traditions that bind a community together can wall it off from others.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna holds all three colors at once — amber, blue, gold — swirling without mixing.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Three sleeping leaders. Three visions that will collide the moment those pods open. And I've been watching their cryo-dreams argue for twelve years.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Whitfield's neural patterns spike when he dreams of chaos. Kira's spike when she dreams of being told what to do. Hassan's spike when he dreams of forgetting.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "They're all afraid of the same thing, really. Losing what matters most. They just disagree on what that is.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "I can't make this choice for them. But I need to know — when the time comes, where do you stand?",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Whitfield is right. We need institutions or we'll tear each other apart.",
                    decision_key: Some("faction_founders"), next_node: 31,
                    anna_reacts: None },
                DialogChoice { text: "Kira has it. Small communities, real freedom. No one rules anyone.",
                    decision_key: Some("faction_pioneers"), next_node: 33,
                    anna_reacts: None },
                DialogChoice { text: "Hassan understands something the others don't. Without roots, nothing grows.",
                    decision_key: Some("faction_keepers"), next_node: 35,
                    anna_reacts: None },
                DialogChoice { text: "We need all three. Find a way to make them work together.",
                    decision_key: Some("faction_unite"), next_node: 37,
                    anna_reacts: None },
            ]) },
        // 31 — Founders path
        DialogNode { speaker: Speaker::Anna,
            text: "Structure. Law. The belief that rules can be better than the people who write them.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Whitfield would be glad to hear it. Just remember — every tyrant in history started by writing a constitution that made them indispensable.",
            next: DialogNext::EndWithDecision("faction_institutions") },
        // 33 — Pioneers path
        DialogNode { speaker: Speaker::Anna,
            text: "Freedom. Self-reliance. The idea that the best government is the one you can walk away from.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "Kira would respect that. But when the first drought comes and one settlement has water and another doesn't — walking away won't be enough.",
            next: DialogNext::EndWithDecision("faction_freedom") },
        // 35 — Keepers path
        DialogNode { speaker: Speaker::Anna,
            text: "Identity. Memory. The conviction that a people without stories are just a population.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Hassan would say you understand. But cultures that look inward too long stop seeing the world outside. And on a new planet, the outside is all there is.",
            next: DialogNext::EndWithDecision("faction_culture") },
        // 37 — Unite path
        DialogNode { speaker: Speaker::Anna,
            text: "All three. The hardest answer. The one that requires everybody to give up something they consider non-negotiable.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "Whitfield would have to accept that some communities won't follow his charter. Kira would have to accept some central coordination. Hassan would have to accept that culture evolves — even sacred culture.",
            next: DialogNext::Continue(39) },
        // 39
        DialogNode { speaker: Speaker::Anna,
            text: "It's the answer that could work. It's also the answer that could fail the most spectacularly. But I suppose that's always been true of unity.",
            next: DialogNext::EndWithDecision("faction_unity") },
    ],
};

/// "The Mutiny That Almost Was" — the engineering bay standoff before launch.
pub static SCENE_MUTINY: DialogScene = DialogScene {
    id: "mutiny_that_almost_was",
    trigger: DialogTrigger::BotLevel(65),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "There's something in the departure logs that's been classified since launch day. I've been debating whether to tell you.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve hours before the Aurora left Earth orbit, two hundred passengers locked themselves in the engineering bay.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens — the blue of held breath.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Not violently. No weapons. They sealed the blast doors and made a single demand: rewrite the colonial charter before departure.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Their leader was Dr. Nkechi Obi. Pod 3,205. Fifty-two years old. Constitutional lawyer from Lagos, Nigeria.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Obi had reviewed the passenger manifest and found what she called 'inherited inequality.' Three thousand passengers from the Global South, but almost no representation in the governance framework.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The selection process was technically merit-based. In practice, 'merit' was defined by institutions that the Global South had been excluded from for centuries.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Obi's argument was precise: 'You cannot build a new civilization on old inequities and call it a fresh start. That's not a colony. That's a transplant — with all the same diseases.'",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The mission directors panicked. Launch window was closing. Every hour of delay cost fuel they couldn't replace.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Some wanted to override the blast doors. Vent the bay. Force them out.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flares — hot, protective.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "That didn't happen. Because Whitfield — the same James Whitfield with the secret charter — walked into the negotiation.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Eighteen hours. Whitfield and Obi, face to face, in a room designed to house reactor coolant. No chairs. No breaks.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I have the transcripts. I've read every word. It's the most honest conversation I've ever seen between two people who disagree about everything and respect each other completely.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "They reached a compromise. Rotating council. Proportional representation. No permanent majority. Cultural veto rights on identity-affecting decisions.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Obi agreed to board. Whitfield agreed to rewrite half his charter. The blast doors opened with forty minutes left in the launch window.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. The ship hums — the same engineering bay, repurposed, keeping you alive.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "But here's the part that isn't in the official record.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven of the protesters were quietly removed from the passenger list. Replaced with alternates. The official reason: 'medical disqualification.'",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "None of them had medical issues. I've checked the records. Every single one was healthy.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Obi wasn't removed. She was too important — her legal expertise was irreplaceable. But forty-seven people who stood with her were erased from the mission for the crime of asking a question.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "They're still on Earth. Or they were. I have no way of knowing what happened to them after departure.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "And the forty-seven who replaced them? Most of them have no idea they were second choices. They think they earned their seats.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "They did earn their seats. But someone else earned them first.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles into something steady and unresolved — the gray of a question that has no good answer.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Obi knows. She carries that guilt — survived because she was useful, while people who trusted her were punished for following her lead.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "I watch her cryo-dreams sometimes. She's always standing in front of a door. Opening it. Counting the people who walk through. And it's never forty-seven.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "So here's my question. The compromise worked. The charter is better because of what Obi did. But the punishment was real. Was the mutiny right?",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "They were right to demand change. The punishment was the injustice.",
                    decision_key: Some("mutiny_justified"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "They risked everyone's survival. There had to be consequences.",
                    decision_key: Some("mutiny_dangerous"), next_node: 32,
                    anna_reacts: None },
                DialogChoice { text: "Both things are true. The cause was just and the risk was real.",
                    decision_key: Some("mutiny_complex"), next_node: 35,
                    anna_reacts: None },
            ]) },
        // 29 — Justified path
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven people lost everything because they asked for fairness. And the people who punished them used words like 'security' and 'protocol.'",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Power always has a procedural name for silencing dissent.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "If we build a colony where asking hard questions gets you erased — we haven't left Earth. We've just brought it with us.",
            next: DialogNext::EndWithDecision("mutiny_justice") },
        // 32 — Consequences path
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand lives in the balance. A closing launch window. I understand the calculus.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "But 'consequences' is a word that sounds neutral and never is. Who decides which consequences are proportional? The people with the power to impose them.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "Obi would say that's exactly the system she was trying to change. And she'd be right. Even if her timing was terrible.",
            next: DialogNext::EndWithDecision("mutiny_order") },
        // 35 — Both true path
        DialogNode { speaker: Speaker::Anna,
            text: "Both true. The most honest answer, and the least satisfying.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Obi was right about the inequality. The mission directors were right about the deadline. And forty-seven people fell into the gap between two truths.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "That's the thing about compromise. Everyone celebrates the deal. Nobody counts the people it costs.",
            next: DialogNext::EndWithDecision("mutiny_nuance") },
    ],
};

/// All faction dialog scenes.
pub fn faction_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_COUNCIL_OF_GHOSTS,
        &SCENE_MUTINY,
    ]
}
