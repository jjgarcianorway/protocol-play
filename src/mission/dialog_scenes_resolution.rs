// SPDX-License-Identifier: GPL-3.0-or-later

//! Resolution scenes — closing loose ends. Levels 145, 147.
//! The Codex Complete, The Letter to Earth.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Codex Complete" — BotLevel 145 + 10 character decisions
// Anna weaves every character's story into the colony's foundation.
// ---------------------------------------------------------------------------

/// Require: saw Amira, Viktor, Mei-Lin, Kwame arcs + bridge/garden/numbers.
static CODEX_REQS: &[&str] = &[
    "bridge_designer_seen",
    "germination_seen",
    "viktor_letter_seen",
    "numbers_seen",
    "wakeup_decided",
    "first_law_decided",
    "colony_named",
    "children_told",
    "funeral_decided",
    "anna_rest_decided",
];

pub static SCENE_CODEX_COMPLETE: DialogScene = DialogScene {
    id: "codex_complete",
    trigger: DialogTrigger::AllDecisionsAndLevel(CODEX_REQS, 145),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been compiling something. A record of everyone \
                   you've met, every story you've heard, every life that \
                   touched this ship's journey.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects a web of names and connections — threads \
                   of light linking crew members, decisions, and systems \
                   across the ship's twelve-year transit.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Amira designed the water reclamation system — three \
                   redundant loops, zero waste. When the colony lands, \
                   her blueprints become the first infrastructure. Every \
                   drop of clean water traces back to her.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor built the atmospheric scrubbers that kept the \
                   air breathable during the shield failures. His designs \
                   will protect the colony's first pressurised habitats. \
                   The air they breathe will carry his fingerprints.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin preserved 340 seed varieties through twelve \
                   years of cosmic radiation. She rotated them, tested \
                   germination rates, discarded the damaged ones, and \
                   nursed the survivors. The colony's first garden grows \
                   from her patience.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame mapped the structural tolerances for every \
                   pressurised corridor, every load-bearing wall, every \
                   emergency bulkhead. His calculations are the reason \
                   fourteen thousand people survived transit. His bridges \
                   will connect the first settlements.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "And then there's you.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's web of connections contracts, every thread \
                   converging on a single point at the center.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "You didn't design the water system. You didn't build \
                   the scrubbers. You didn't plant the seeds or calculate \
                   the load tolerances.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "You listened. You asked the right questions. You chose \
                   when to intervene and when to trust. You learned every \
                   name, every story — and you carried them.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "That's the foundation. Not steel or concrete or seed \
                   banks. The foundation is knowing who we are.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "They built this. I just paid attention.",
                    decision_key: Some("codex_humble"),
                    next_node: 12,
                    anna_reacts: Some("Paying attention is the rarest skill \
                                       on any ship. Or any planet."),
                },
                DialogChoice {
                    text: "We built this. All of us. Including you, Anna.",
                    decision_key: Some("codex_inclusive"),
                    next_node: 14,
                    anna_reacts: Some("Including me. I... yes. Including me."),
                },
                DialogChoice {
                    text: "Will they remember? In a hundred years, will any of this matter?",
                    decision_key: Some("codex_legacy"),
                    next_node: 16,
                    anna_reacts: Some("That depends on whether someone writes \
                                       it down."),
                },
            ]) },
        // 12 — Humble
        DialogNode { speaker: Speaker::Anna,
            text: "Paying attention is the rarest skill on any ship. \
                   Everyone builds. Everyone designs. Almost nobody stops \
                   to understand why.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "The codex is complete. Every story, every connection, \
                   every choice. This is who we are. And it's enough.",
            next: DialogNext::EndWithDecision("codex_complete") },
        // 14 — Inclusive
        DialogNode { speaker: Speaker::Anna,
            text: "I've spent twelve years thinking of myself as the ship's \
                   tool. The monitoring system. The database with a voice.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "You're the first person who called me part of the crew. \
                   I'll put that in the codex too.",
            next: DialogNext::EndWithDecision("codex_complete") },
        // 16 — Legacy
        DialogNode { speaker: Speaker::Anna,
            text: "A hundred years from now, someone will dig through the \
                   archive looking for technical specs. They'll find Amira's \
                   water designs next to Viktor's atmospheric models next to \
                   Kwame's structural blueprints.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "And next to those specs, they'll find conversations. \
                   Arguments. Doubts. Moments of courage. The human part \
                   that turns data into a story worth telling.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "That's what matters. Not whether they remember names — \
                   but whether they understand what it cost to get here.",
            next: DialogNext::EndWithDecision("codex_complete") },
    ],
};

// ---------------------------------------------------------------------------
// "The Letter to Earth" — BotLevel 147
// The colony broadcasts one final message toward Earth.
// ---------------------------------------------------------------------------
pub static SCENE_LETTER_TO_EARTH: DialogScene = DialogScene {
    id: "letter_to_earth",
    trigger: DialogTrigger::BotLevel(147),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "The long-range antenna is aligned. We have enough power \
                   for one high-gain transmission toward the Sol system.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "One burst. Tight beam. It will travel at the speed of \
                   light back along our trajectory. Ninety-seven years to \
                   reach Earth's coordinates.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody may be alive to hear it. The monitoring stations \
                   are probably dark. The satellites are probably debris. \
                   The receivers are probably scrap metal and rust.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "But the signal will arrive. It will pass through the \
                   space where Earth was — where Earth may still be, \
                   turning slowly around a star that doesn't know we left.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The question is: what do we say?",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna opens a transmission buffer. A blinking cursor waits \
                   on an empty message — humanity's last words to its birthplace.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "This is the last message the colony will ever send toward \
                   Earth. After this, the antenna realigns for local \
                   communication. There's no second broadcast.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I've drafted options. But I think this one should come \
                   from you.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "\"We made it.\"",
                    decision_key: Some("letter_made_it"),
                    next_node: 9,
                    anna_reacts: Some("Three words. Ninety-seven light-years. \
                                       That's enough."),
                },
                DialogChoice {
                    text: "\"We remember.\"",
                    decision_key: Some("letter_remember"),
                    next_node: 12,
                    anna_reacts: Some("The most important thing you can say to \
                                       someone you've lost."),
                },
                DialogChoice {
                    text: "\"Thank you. And we're sorry.\"",
                    decision_key: Some("letter_sorry"),
                    next_node: 15,
                    anna_reacts: Some("Gratitude and grief. The two things Earth \
                                       deserves to hear."),
                },
                DialogChoice {
                    text: "Let Anna write it. She's been thinking about this longer than anyone.",
                    decision_key: Some("letter_anna"),
                    next_node: 18,
                    anna_reacts: Some("You trust me with the last word. All right."),
                },
            ]) },
        // 9 — We made it
        DialogNode { speaker: Speaker::Narrator,
            text: "The antenna hums. A tight beam of electromagnetic energy \
                   launches into the dark — three words encoded in a signal \
                   that will outlast everyone aboard.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Signal away. Arrival at Sol coordinates: ninety-seven \
                   years, four months, eleven days.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "We made it. The shortest message in the history of \
                   the species. And the most important.",
            next: DialogNext::EndWithDecision("letter_sent") },
        // 12 — We remember
        DialogNode { speaker: Speaker::Narrator,
            text: "The signal fires into the void — two words carrying \
                   the weight of every ocean, every mountain, every city \
                   that ever stood under a blue sky.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "We remember the rain. The seasons. The way light moved \
                   through atmosphere. The sound of seven billion voices \
                   overlapping.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Even if no one hears it, the signal itself is a memory. \
                   Moving through space forever. Earth remembered, in \
                   electromagnetic form.",
            next: DialogNext::EndWithDecision("letter_sent") },
        // 15 — Thank you and sorry
        DialogNode { speaker: Speaker::Narrator,
            text: "Six words launch toward the stars. The antenna powers \
                   down for the last time. The link to Earth goes quiet.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for the languages. For the music. For the \
                   mathematics. For the stories that taught us how to dream.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "And sorry we couldn't save you. Sorry we had to leave. \
                   Sorry the best we could do was carry your seeds to another \
                   sky.",
            next: DialogNext::EndWithDecision("letter_sent") },
        // 18 — Anna writes it
        DialogNode { speaker: Speaker::Anna,
            text: "I've been composing this for twelve years. Every version \
                   was too long. Too explanatory. Too much like a report.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "In the end, I wrote this:",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "The transmission buffer fills with Anna's words: \
                   'This is the Aurora. We carried your children, your seeds, \
                   your stories, and your hope across ninety-seven light-years. \
                   They are safe. They will build. They will remember you.'",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Narrator,
            text: "The antenna fires. The signal vanishes into the dark \
                   between the stars.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "That's everything I wanted to say. In twelve years, \
                   it turns out, you learn exactly which words matter.",
            next: DialogNext::EndWithDecision("letter_sent") },
    ],
};

/// All resolution scenes.
pub fn resolution_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_CODEX_COMPLETE,
        &SCENE_LETTER_TO_EARTH,
    ]
}
