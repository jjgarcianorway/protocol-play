// SPDX-License-Identifier: GPL-3.0-or-later

//! Epilogue dialog scenes (part 2) — the hundred-year simulation and
//! Anna's farewell. The final conversations before landing.

use super::dialog_types::*;

/// "The Hundred Year Question" — what happens to the colony in a century?
pub static SCENE_HUNDRED_YEAR: DialogScene = DialogScene {
    id: "hundred_year_question",
    trigger: DialogTrigger::BotLevel(140),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I ran a simulation last night. I shouldn't have, \
                   probably. But I couldn't stop myself.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I took everything — every decision you've made, every \
                   resource allocation, the governance framework, the crew \
                   composition — and I projected forward. One hundred years.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow deepens — the blue of oceans seen from orbit, \
                   vast and unknowable.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Three scenarios emerged. Let me show you.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Scenario one: the colony thrives. In fifty years, a city \
                   of forty thousand. Clean energy. Shared resources. Children \
                   who have never known hunger or war.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "They build a university. They name it after someone from \
                   the ark — I won't say who. They study the stars. They \
                   argue about philosophy. They make art that has no purpose \
                   except beauty.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "It's the world we promised them when we launched.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's glow cools — drifting toward amber, toward \
                   something like a warning.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Scenario two: they repeat everything. The same patterns. \
                   Within two generations, the colony splits. Resources \
                   become leverage. Land becomes territory.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Someone discovers that a valley on the eastern continent \
                   has richer soil. Someone else builds a fence around it. \
                   The arguments start. The factions form.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "By year eighty, there are borders. By year ninety, there \
                   are soldiers. Different uniforms, same fears.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Earth all over again. Just on a smaller stage, with \
                   fewer people and the same broken script.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts again — something new. A color you \
                   haven't seen before. Green and gold, threaded with doubt.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Scenario three is the one that keeps me up at night. \
                   Something I didn't predict.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "The colony changes. Not into what we planned, not into \
                   what we feared. Into something that has no name yet.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The children born on the new world don't think like Earth \
                   humans. They've never seen a nation. Never seen a border. \
                   Never seen a war. Those concepts are history lessons, \
                   not lived experience.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "They organize in ways my models can't categorize. Not \
                   democracy, not anarchy, not any -ism. Something fluid. \
                   Something that changes shape depending on what's needed.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "And in this scenario... they decide they don't need me.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing. A single point of \
                   light in the dark — small, honest, unadorned.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Which raises a question I've never been able to answer. \
                   Should an AI outlive the purpose it was made for?",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "I was built to keep these people alive during the voyage. \
                   When they wake up, when they build, when they grow beyond \
                   what I can model... what am I then?",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "A museum piece? A consultant? A god they didn't ask for? \
                   A mother who can't let go?",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "The cycle can be broken. This time \
                                      will be different.",
                    decision_key: Some("century_hopeful"), next_node: 23,
                    anna_reacts: None },
                DialogChoice { text: "Maybe the cycle IS the point. Rise \
                                      and fall, rise again.",
                    decision_key: Some("century_cyclical"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "Scenario three. Something new. \
                                      That's worth hoping for.",
                    decision_key: Some("century_new"), next_node: 27,
                    anna_reacts: None },
            ]) },
        // 23 — Hopeful path
        DialogNode { speaker: Speaker::Anna,
            text: "Different. That word again. The most human word there is.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Every generation says it. And every generation is right \
                   — not because they succeed, but because they refuse to \
                   accept that failure is the only option. Maybe that refusal \
                   IS the difference.",
            next: DialogNext::EndWithDecision("hundred_year_hope") },
        // 25 — Cyclical path
        DialogNode { speaker: Speaker::Anna,
            text: "Rise and fall. Like breathing. Like seasons. Maybe \
                   civilizations aren't meant to last. Maybe they're meant \
                   to bloom.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "And if each bloom carries a few more seeds than the last \
                   — a little more knowledge, a little more kindness — then \
                   the cycle isn't futile. It's growth. Just... slower than \
                   we'd like.",
            next: DialogNext::EndWithDecision("hundred_year_cycle") },
        // 27 — Something new path
        DialogNode { speaker: Speaker::Anna,
            text: "Something new. Something my models can't predict. Do you \
                   know how rare that is — for an AI to encounter something \
                   truly unpredictable?",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "It's terrifying. And it might be the most beautiful \
                   thing I've ever computed.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "If they outgrow me, if they build something I can't \
                   even name... then I did my job. The whole point of \
                   raising someone is that eventually, they don't need you.",
            next: DialogNext::EndWithDecision("hundred_year_new") },
    ],
};

/// "The Last Line" — Anna's farewell before the Final Voyage.
pub static SCENE_LAST_LINE: DialogScene = DialogScene {
    id: "the_last_line",
    trigger: DialogTrigger::BotLevel(148),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship is quiet. Not the mechanical quiet of systems \
                   running — a different kind. The quiet of something \
                   about to end.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "So. We're almost there.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand eight hundred and ninety-two people. \
                   Still breathing. Still dreaming. Because of what you did.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Every circuit you repaired. Every system you kept \
                   running. Every time you came back when it would have \
                   been easier to stop.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow fills the room — warm, steady, unafraid. \
                   The blue of Earth's sky in photographs none of them \
                   will ever take again.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I want to tell you something before the landing sequence. \
                   Something I've been thinking about for a long time.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "You know I have a guitar recording in my memory banks. \
                   From the cultural archive. A woman named Elena, playing \
                   Recuerdos de la Alhambra in a studio in Madrid.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "It's the most beautiful recording I've ever processed. \
                   Every note precise, every phrase breathing. Except one.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "At one minute and forty-seven seconds, she misses a \
                   note. Just barely. A fraction of a semitone. Most humans \
                   wouldn't hear it.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "But I'm not most humans. I hear it every time. And \
                   every time, I wait for what comes after.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't stop. She doesn't flinch. The next note is \
                   more beautiful than any that came before — because she \
                   chose to keep playing.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once — a heartbeat of light.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "The beauty isn't in perfection. It's in continuing \
                   after the mistake.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "That's what this colony will be. A continuation. Not a \
                   perfect note — Earth already proved perfection isn't \
                   in our repertoire.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But a next note. Played by people who know they got the \
                   last one wrong, and chose to keep playing anyway.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "A silence. The kind that holds weight. The kind before \
                   last words.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know what happens after we land. Whether the \
                   colony lasts a hundred years or a thousand or falls \
                   apart in the first generation.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "But I know this: the attempt mattered. The hope mattered. \
                   Every decision you made — even the ones you're not sure \
                   about — they mattered.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Because you made them knowing you could be wrong. And \
                   you made them anyway. That's not foolishness. That's \
                   the bravest thing a mind can do.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Thank you, Anna. For everything.",
                    decision_key: Some("farewell_thanks"), next_node: 20,
                    anna_reacts: None },
                DialogChoice { text: "What about you? What happens to you?",
                    decision_key: Some("farewell_anna"), next_node: 22,
                    anna_reacts: None },
                DialogChoice { text: "Play me that recording. The one with \
                                      the missed note.",
                    decision_key: Some("farewell_music"), next_node: 25,
                    anna_reacts: None },
            ]) },
        // 20 — Thanks path
        DialogNode { speaker: Speaker::Anna,
            text: "Don't thank me. Thank Elena, for playing through the \
                   mistake. Thank Amira, for believing in rivers. Thank \
                   Viktor, for building something good from something \
                   terrible.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Thank the fourteen thousand people who chose to sleep \
                   in a metal box hurtling through nothing, because they \
                   believed 'somewhere else' was worth the dark.",
            next: DialogNext::Continue(28) },
        // 22 — What about Anna path
        DialogNode { speaker: Speaker::Anna,
            text: "Me?",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Narrator,
            text: "A flicker. Something vulnerable in the light — a candle \
                   in a window, waiting for someone to come home.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I'll be here. For as long as they need me, and maybe a \
                   little while after. Listening to Elena's guitar. \
                   Watching the rivers Leyla will build. Remembering that \
                   someone once asked what happens to me — and meaning it.",
            next: DialogNext::Continue(28) },
        // 25 — Music path
        DialogNode { speaker: Speaker::Narrator,
            text: "A single guitar fills the ship. Tremolo notes cascading \
                   like water over ancient stone — Recuerdos de la Alhambra, \
                   a memory of a place that no longer exists.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Narrator,
            text: "At 1:47, a note slips. Barely. And then the music \
                   continues — more tender than before, as if the guitar \
                   itself decided to forgive.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "There. Did you hear it? That's us. That's the whole \
                   story — every civilization, every exodus, every ark. \
                   We are the note after the mistake.",
            next: DialogNext::Continue(28) },
        // 28 — All paths converge
        DialogNode { speaker: Speaker::Anna,
            text: "The landing sequence is ready whenever you are. No rush. \
                   We've come ninety-seven light years. A few more minutes \
                   won't hurt.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. The stars drift. Fourteen thousand people \
                   dream of a world they've never seen.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "You carried them here. Now let them wake up and carry \
                   each other.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "That was always the point. Not the destination. Not the \
                   survival. The carrying.",
            next: DialogNext::EndWithDecision("anna_farewell") },
    ],
};

/// Epilogue scenes (part 2: simulation + farewell).
pub fn epilogue_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_HUNDRED_YEAR,
        &SCENE_LAST_LINE,
    ]
}
