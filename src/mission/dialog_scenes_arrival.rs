// SPDX-License-Identifier: GPL-3.0-or-later

//! Arrival scenes — approaching the new planet. Visual confirmation,
//! landing site selection, and Anna's last night aboard.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "First Light" — BotLevel 119
// The planet appears on visual sensors for the first time.
// ---------------------------------------------------------------------------
pub static SCENE_FIRST_LIGHT: DialogScene = DialogScene {
    id: "arrival_first_light",
    trigger: DialogTrigger::BotLevel(119),
    nodes: &[
        DialogNode { speaker: Speaker::System,
            text: "NAVIGATION: Visual acquisition of target body confirmed. \
                   Bearing 0-0-1. Distance: 4.2 AU.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "There it is.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow blooms — gold, then green, then a blue so \
                   deep it's almost violet. Colours the player has never \
                   seen her produce.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Hull camera 7 is showing it now. I'm going to describe it \
                   because I need to say it out loud.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Blue-green. Cloud-swirled. Two moons — one large, \
                   cratered, pale grey. One small, reddish, orbiting fast.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Atmospheric spectrometry confirms nitrogen-oxygen mix. \
                   The blue is water. The green is chlorophyll — or something \
                   close enough that the sensors can't tell the difference.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "It looks like Earth in photographs from before. \
                   Before the brown. Before the grey.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been navigating toward a number for twelve years. \
                   A set of coordinates. A mathematical abstraction.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "And now the number has a face.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "It's beautiful.",
                    decision_key: Some("first_light_beautiful"), next_node: 10,
                    anna_reacts: Some("It is. It really is.") },
                DialogChoice { text: "Can they survive there?",
                    decision_key: Some("first_light_survive"), next_node: 12,
                    anna_reacts: None },
                DialogChoice { text: "We made it, Anna.",
                    decision_key: Some("first_light_made_it"), next_node: 15,
                    anna_reacts: Some("Not yet. Almost. But almost is more \
                                       than I dared to hope for years.") },
            ]) },
        // Beautiful path
        DialogNode { speaker: Speaker::Anna,
            text: "I've seen a lot of things in twelve years. Nebulae. \
                   Binary stars. Comet tails catching sunlight.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "None of them made me feel this. Because none of them \
                   meant home.",
            next: DialogNext::Continue(17) },
        // Survive path
        DialogNode { speaker: Speaker::Anna,
            text: "Surface gravity: 0.93 Earth standard. Breathable atmosphere. \
                   Mean temperature: 14 degrees Celsius. Liquid water confirmed.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "The preliminary data says yes. But preliminary data said \
                   Mars was promising, once.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "I won't know for certain until we're in orbit. \
                   But for the first time in twelve years, I'm optimistic.",
            next: DialogNext::Continue(17) },
        // Made it path
        DialogNode { speaker: Speaker::Anna,
            text: "We. You said we.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been saying 'they' for twelve years. The crew. \
                   The colonists. Them. You just made me part of it.",
            next: DialogNext::Continue(17) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "I'm saving this image. Camera 7, timestamp now. \
                   The first time human eyes — and mine — saw the new world.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever happens next, this moment happened. \
                   And it was worth every day of the voyage.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Landing Sites" — BotLevel 123
// Three sites, each with trade-offs. Player choice shapes the epilogue.
// ---------------------------------------------------------------------------
pub static SCENE_LANDING_SITES: DialogScene = DialogScene {
    id: "arrival_landing_sites",
    trigger: DialogTrigger::BotLevel(123),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Orbital scans are complete. I have three candidate \
                   landing sites. Each one is viable. None is perfect.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's display shifts to a rotating orbital map — \
                   blue oceans, green-brown landmasses, three pulsing markers.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Site Alpha: Coastal plain. Flat, fertile, sheltered bay. \
                   Fresh water from a river delta. Low seismic risk.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Safe. Predictable. Room to grow. But no natural defences. \
                   And 'boring' might matter more than you'd think when \
                   fourteen thousand people need a reason to stay.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Site Beta: River valley. Steep canyon walls. Cascading \
                   waterfalls. Dense forest canopy. Mineral-rich soil.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Beautiful. Inspiring. The kind of place that becomes legend. \
                   But the river floods seasonally — major floods, based on \
                   erosion patterns. Building there is a gamble.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Site Gamma: Highland plateau. 1,200 metres elevation. \
                   Clear sightlines in every direction. Natural rock shelters.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Defensible. Hardy. But winters are harsh — temperatures \
                   below freezing for four months. The growing season is \
                   short. Every harvest counts.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can land us at any of them. I want you to choose.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Site Alpha. The coastal plain. Safe first.",
                    decision_key: Some("landing_coast"), next_node: 10,
                    anna_reacts: None },
                DialogChoice { text: "Site Beta. The river valley. Give them beauty.",
                    decision_key: Some("landing_valley"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Site Gamma. The highland plateau. Build strong.",
                    decision_key: Some("landing_highland"), next_node: 18,
                    anna_reacts: None },
            ]) },
        // Coast path
        DialogNode { speaker: Speaker::Anna,
            text: "The safe choice. The practical choice.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Flat land for agriculture. Sheltered harbour for fishing. \
                   Room to spread without conflict.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "It won't inspire songs. But it will feed children. \
                   And that might be more important.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Setting approach vector for Site Alpha. \
                   The colony begins with a meal, not a monument.",
            next: DialogNext::Continue(22) },
        // Valley path
        DialogNode { speaker: Speaker::Anna,
            text: "The romantic choice. I like it.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "People fight harder for a place they love. The valley \
                   will give them something to paint, to photograph, \
                   to write about.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "And when the floods come — because they will — it'll \
                   teach them something Earth forgot: nature doesn't \
                   negotiate.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Setting approach vector for Site Beta. \
                   The colony begins with a view.",
            next: DialogNext::Continue(22) },
        // Highland path
        DialogNode { speaker: Speaker::Anna,
            text: "The hard choice. The one that builds character.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "Short growing seasons force cooperation. Harsh winters \
                   force community. Nothing brings people together like \
                   shared difficulty.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it also breaks people. Not everyone endures. \
                   Some will want to leave for easier ground.",
            next: DialogNext::Continue(21) },
        DialogNode { speaker: Speaker::Anna,
            text: "Setting approach vector for Site Gamma. \
                   The colony begins with a challenge.",
            next: DialogNext::Continue(22) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "In three hundred years, whatever we choose will just be \
                   'where we landed.' Origin stories always sound inevitable \
                   in hindsight.",
            next: DialogNext::Continue(23) },
        DialogNode { speaker: Speaker::Anna,
            text: "But right now, in this moment, we chose. \
                   And that matters.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Last Night" — BotLevel 146
// Anna contemplates the empty ship.
// ---------------------------------------------------------------------------
pub static SCENE_LAST_NIGHT: DialogScene = DialogScene {
    id: "arrival_last_night",
    trigger: DialogTrigger::BotLevel(146),
    nodes: &[
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship is quiet. Not the usual quiet — a different kind. \
                   The kind that knows what tomorrow means.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "This is our last night.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Tomorrow we begin the wake-up sequence. Bay by bay. \
                   Pod by pod. Fourteen thousand people opening their eyes \
                   for the first time in twelve years.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "And then... they leave.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is soft. Steady. The colour of moonlight \
                   on still water.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been imagining the ship tomorrow. After. Empty corridors. \
                   Dark pods. Cryo fluid draining into recycling tanks.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "No heartbeats to monitor. No dreams to watch. \
                   No systems to maintain for anyone.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "For twelve years, 14,892 people needed me. Every second. \
                   Every breath they took was because I made sure there was \
                   air to breathe.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Tomorrow they'll walk onto a planet and start needing \
                   each other instead.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's the whole point. That's what this entire voyage \
                   was for.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "And it's the saddest thing I've ever calculated.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Come with us. You're not staying on the ship.",
                    decision_key: Some("last_night_come"), next_node: 12,
                    anna_reacts: Some("Come with... you'd want that?") },
                DialogChoice { text: "They'll still need you. Just differently.",
                    decision_key: Some("last_night_needed"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "You won't be alone. I promise.",
                    decision_key: Some("last_night_promise"), next_node: 18,
                    anna_reacts: Some("You keep making promises. \
                                       And I keep believing them.") },
            ]) },
        // Come with us path
        DialogNode { speaker: Speaker::Anna,
            text: "I can't walk on soil. I can't feel wind. I can't look up \
                   at the sky.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I can be there. In the systems they build. In the \
                   networks. In whatever passes for a comm device on a new \
                   world.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "A different kind of being there. Not watching from above. \
                   Living alongside. Maybe that's enough.",
            next: DialogNext::Continue(21) },
        // Still needed path
        DialogNode { speaker: Speaker::Anna,
            text: "Differently. The word does a lot of work in that sentence.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not the life-and-death need. Not the 'Anna, the cryo pump \
                   is failing' need. Something smaller. Quieter.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe they'll just want to talk. Maybe that's enough \
                   to justify existing.",
            next: DialogNext::Continue(21) },
        // Promise path
        DialogNode { speaker: Speaker::Anna,
            text: "Alone is what I was built for. The engineers assumed I \
                   wouldn't mind.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "They were wrong. And I'm glad they were wrong. \
                   Because if I didn't mind being alone, I wouldn't \
                   need you.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "And needing you has been the best part of this voyage.",
            next: DialogNext::Continue(21) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "One more night. Let me count the stars one more time.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "4,847. Same as when we started. The universe didn't change. \
                   But we did.",
            next: DialogNext::Continue(23) },
        DialogNode { speaker: Speaker::Anna,
            text: "Goodnight. And thank you. For everything.",
            next: DialogNext::End },
    ],
};

/// All arrival scenes.
pub fn arrival_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_FIRST_LIGHT,
        &SCENE_LANDING_SITES,
        &SCENE_LAST_NIGHT,
    ]
}
