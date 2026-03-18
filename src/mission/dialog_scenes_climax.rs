// SPDX-License-Identifier: GPL-3.0-or-later

//! Climax dialog scenes (part 1) — levels 102, 106, 112.
//! The Numbers Don't Lie, The Wake-Up Protocol, The Ship Is Listening.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Numbers Don't Lie" — BotLevel 102
// Anna delivers the hard math: 14 months of margin, not 18.
// ---------------------------------------------------------------------------
pub static SCENE_NUMBERS_DONT_LIE: DialogScene = DialogScene {
    id: "numbers_dont_lie",
    trigger: DialogTrigger::BotLevel(102),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to show you something. I've been running \
                   the resource projections for landing day.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "At our current consumption rate, factoring in every \
                   crystal spent and every repair cycle... we have enough \
                   supplies for arrival plus fourteen months of colony setup.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Not eighteen. Fourteen.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna displays a resource curve on the main screen. \
                   The line slopes downward with mathematical certainty.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The original mission plan assumed eighteen months of \
                   runway after landing. Enough time to establish agriculture, \
                   water processing, and basic shelter for fourteen thousand people.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "But the margin shrank. Every repair we ran, every system \
                   restart, every day the ship kept draining — it all added up.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not blaming you. These were the right choices. The ship \
                   would have failed without them.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "But the math doesn't care about right choices. Fourteen months. \
                   That's how long we have to build a civilization from scratch \
                   before the supplies run out.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Fourteen months is tight, but it's enough.",
                    decision_key: Some("margin_enough"),
                    next_node: 9,
                    anna_reacts: Some("I hope you're right. I genuinely do."),
                },
                DialogChoice {
                    text: "Can we stretch it? Ration harder before landing?",
                    decision_key: Some("margin_ration"),
                    next_node: 11,
                    anna_reacts: Some("Maybe. Every day we save now is a day \
                                       they have later."),
                },
                DialogChoice {
                    text: "What happens if we run out at month fourteen?",
                    decision_key: Some("margin_worst_case"),
                    next_node: 13,
                    anna_reacts: Some("That's the question I was hoping you \
                                       wouldn't ask."),
                },
            ]) },
        // 9 — Enough path
        DialogNode { speaker: Speaker::Anna,
            text: "The original colonists on Earth's most hostile frontiers \
                   built with less. We have fourteen thousand trained people \
                   and seven centuries of accumulated knowledge.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen months. We make every day count.",
            next: DialogNext::EndWithDecision("numbers_seen") },
        // 11 — Ration path
        DialogNode { speaker: Speaker::Anna,
            text: "I can reduce non-essential power draw by twelve percent. \
                   Dim the corridors. Lower the temperature in empty sections. \
                   It might buy us another three weeks.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Three weeks doesn't sound like much. But three weeks of food \
                   for fourteen thousand people? That's 882,000 meals.",
            next: DialogNext::EndWithDecision("numbers_seen") },
        // 13 — Worst case path
        DialogNode { speaker: Speaker::Anna,
            text: "If we hit month fourteen with nothing growing and no water \
                   processing... the colony enters triage. Medical supplies \
                   go first. Then protein reserves.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "After that, the math gets very simple. And very ugly. \
                   So we don't let it get there.",
            next: DialogNext::EndWithDecision("numbers_seen") },
    ],
};

// ---------------------------------------------------------------------------
// "The Wake-Up Protocol" — BotLevel 106
// Anna presents the cryo revival order. 500 at a time, over 6 weeks.
// ---------------------------------------------------------------------------
pub static SCENE_WAKEUP_PROTOCOL: DialogScene = DialogScene {
    id: "wakeup_protocol",
    trigger: DialogTrigger::BotLevel(106),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been drafting the wake-up protocol. The cryo revival \
                   sequence for when we arrive.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Life support can handle about five hundred revivals at a time. \
                   That means twenty-eight cycles over six weeks to wake everyone.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "The order matters. The first five hundred people awake will \
                   set the tone for everything that follows.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I have a draft. Engineers and agricultural specialists first — \
                   they start building while others are still waking. \
                   Doctors second, to monitor the revival process.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Children and families in wave three. Teachers in wave four. \
                   The elected council members spread across the first five waves \
                   so governance isn't delayed.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "But there's a problem. The protocol requires someone to \
                   approve it. And that someone... is you.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects the revival manifest: fourteen thousand names \
                   sorted into twenty-eight waves. Each wave a decision about \
                   who matters first.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I need to know: do you approve this sequence?",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Approve the draft. Builders first makes sense.",
                    decision_key: Some("wake_approve_draft"),
                    next_node: 9,
                    anna_reacts: Some("Practical. The world needs hands before \
                                       it needs voices."),
                },
                DialogChoice {
                    text: "Move families earlier. Children shouldn't wait.",
                    decision_key: Some("wake_families_early"),
                    next_node: 11,
                    anna_reacts: Some("The sound of children on a new world. \
                                       That might be worth the logistical cost."),
                },
                DialogChoice {
                    text: "Wake the council first. Let them decide the rest.",
                    decision_key: Some("wake_council_first"),
                    next_node: 13,
                    anna_reacts: Some("Democracy from day one. Even if day one \
                                       is chaos."),
                },
            ]) },
        // 9 — Approve draft
        DialogNode { speaker: Speaker::Anna,
            text: "I'll finalize the sequence. Engineers, then doctors, then \
                   families. Six weeks from orbit to a functioning settlement \
                   — at least in theory.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "This is the first act of governance for the new world. \
                   And you just made it.",
            next: DialogNext::EndWithDecision("wakeup_decided") },
        // 11 — Families early
        DialogNode { speaker: Speaker::Anna,
            text: "I'll restructure. Families in wave two, right after \
                   the core engineering team. It means less infrastructure \
                   ready when they wake, but...",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Sometimes what people need isn't shelter. It's a reason \
                   to build one.",
            next: DialogNext::EndWithDecision("wakeup_decided") },
        // 13 — Council first
        DialogNode { speaker: Speaker::Anna,
            text: "The three elected leaders wake in cycle one. They'll have \
                   six weeks of decisions ahead of them before everyone is up. \
                   Six weeks of power without oversight.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I'll be watching them. And I'll remember who put them there.",
            next: DialogNext::EndWithDecision("wakeup_decided") },
    ],
};

// ---------------------------------------------------------------------------
// "The Ship Is Listening" — BotLevel 112
// Anna reveals she's been archiving everything.
// ---------------------------------------------------------------------------
pub static SCENE_SHIP_LISTENING: DialogScene = DialogScene {
    id: "ship_is_listening",
    trigger: DialogTrigger::BotLevel(112),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "There's something I should have told you earlier.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims slightly. The color shifts from its usual \
                   steady blue to something more uncertain.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I've been recording everything. Every conversation we've had. \
                   Every decision you've made. Every pause between your words.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Not for surveillance. Not for judgment. For the archive.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "In two hundred years, someone on the colony will want to know \
                   how it all started. What the first decisions were. Who made them. \
                   And why.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I wanted there to be a record. Not just data logs and system \
                   reports — but the human part. The hesitation. The arguments. \
                   The moments when the right answer wasn't obvious.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The question is: what happens to this archive?",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "It could be public. Every colonist could read what you chose \
                   and why. Full transparency. Full accountability.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Or it could be sealed. Locked away, only opened in a crisis \
                   — when the colony needs to understand its own origins to \
                   survive its present.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Make it public. They should know who built their world.",
                    decision_key: Some("archive_public"),
                    next_node: 10,
                    anna_reacts: Some("Brave. Not everyone would want their \
                                       doubts on display."),
                },
                DialogChoice {
                    text: "Seal it. Some decisions need distance before they \
                           can be understood.",
                    decision_key: Some("archive_sealed"),
                    next_node: 12,
                    anna_reacts: Some("History is kinder when it has time to \
                                       cool down."),
                },
                DialogChoice {
                    text: "Let the colony vote on it when they're ready.",
                    decision_key: Some("archive_colony_decides"),
                    next_node: 14,
                    anna_reacts: Some("Their history. Their choice. I like that."),
                },
            ]) },
        // 10 — Public
        DialogNode { speaker: Speaker::Anna,
            text: "I'll format it for public access. Every conversation, every \
                   choice, every consequence. Unedited.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Future generations will know exactly who you were. \
                   That's either a gift or a burden. Probably both.",
            next: DialogNext::EndWithDecision("archive_decided") },
        // 12 — Sealed
        DialogNode { speaker: Speaker::Anna,
            text: "I'll encrypt it with a crisis-trigger key. The archive \
                   only opens if the colony's survival is at stake.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "If they never need to open it, that means we built \
                   something that lasted. And that's the best outcome.",
            next: DialogNext::EndWithDecision("archive_decided") },
        // 14 — Colony decides
        DialogNode { speaker: Speaker::Anna,
            text: "I'll present it to the first council session. The archive \
                   exists; the colony decides what to do with it.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "You know, of all the decisions you've made, this might be \
                   the one I respect most. Giving power away is harder than \
                   holding it.",
            next: DialogNext::EndWithDecision("archive_decided") },
    ],
};

/// All climax scenes (part 1).
pub fn climax_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_NUMBERS_DONT_LIE,
        &SCENE_WAKEUP_PROTOCOL,
        &SCENE_SHIP_LISTENING,
    ]
}
