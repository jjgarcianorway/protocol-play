// SPDX-License-Identifier: GPL-3.0-or-later

//! Final batch of dialog scenes (part 1) — filling remaining narrative gaps.
//! Warm, personal moments that deepen Anna's character and the journey.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Ship's Name" (BotLevel 3) — Why the ship is called Aurora.
// ---------------------------------------------------------------------------
pub static SCENE_SHIPS_NAME: DialogScene = DialogScene {
    id: "ships_name",
    trigger: DialogTrigger::BotLevel(3),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know why this ship is called Aurora?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Dawn. The light before the sun. That's what we are \
                   \u{2014} the light before a new beginning.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "The naming committee had grander options. Prometheus. \
                   Odyssey. Exodus.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "But the lead engineer \u{2014} a woman named Sato \u{2014} \
                   argued that those names carried too much weight. \
                   Too many expectations.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow softens to a pale gold, like the first edge \
                   of sunrise.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "She said: 'Call it Aurora. Because dawn doesn't \
                   promise anything. It just arrives.'",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think about that a lot. Not promising. Just arriving.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The First Meal" (BotLevel 12) — Planning the first meal on New Earth.
// ---------------------------------------------------------------------------
pub static SCENE_FIRST_MEAL: DialogScene = DialogScene {
    id: "first_meal",
    trigger: DialogTrigger::BotLevel(12),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been planning the menu for 12 years.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The first meal on New Earth. It matters. You can't start \
                   a civilization with protein bars.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin's jasmine tea, obviously. That's non-negotiable.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Bread \u{2014} real bread, from the seed vault. Amira says \
                   she can get wheat growing within six months, but I think \
                   she's being modest.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "There's something in Anna's voice you haven't heard before. \
                   Playfulness.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've also earmarked three kilograms of chocolate from \
                   the long-term stores. Before you ask: yes, it's still \
                   edible. I've been monitoring the temperature very carefully.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Three kilograms. Fourteen thousand people. That's about \
                   0.2 grams each. Barely a taste.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it's chocolate. On a new planet. Under a new sky.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That sounds perfect.",
                    decision_key: None, next_node: 9,
                    anna_reacts: Some("Perfect is the wrong word. \
                                       Hopeful is better.") },
                DialogChoice { text: "What about dessert?",
                    decision_key: None, next_node: 10,
                    anna_reacts: Some("The chocolate IS dessert. \
                                       We're rationing, not feasting.") },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "The meal won't be grand. But it will be real.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Fine. I'll see what I can do with reconstituted fruit \
                   paste. No promises.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Anna's Music Collection" (BotLevel 26) — 47 million songs, ranked.
// ---------------------------------------------------------------------------
pub static SCENE_MUSIC_COLLECTION: DialogScene = DialogScene {
    id: "music_collection",
    trigger: DialogTrigger::BotLevel(26),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I have 47 million songs in my cultural archive.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've listened to every single one. Some of them twice.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've ranked them. All 47 million. By emotional resonance, \
                   structural complexity, and something I can only describe \
                   as... feeling.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts through colours \u{2014} rapid, almost \
                   giddy. She's excited about this.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "My number one isn't classical. It's not jazz. It's not \
                   some profound masterwork.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's a pop song from 2043 by a band called The Violet \
                   Hour. Three minutes and twelve seconds. It charted at \
                   number 94 in Norway and nowhere else.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody remembers them. The lead singer became an accountant.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's terrible. I love it.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Play it for me.",
                    decision_key: Some("music_play"), next_node: 9,
                    anna_reacts: Some("Really? Nobody's ever asked before. \
                                       ...Obviously.") },
                DialogChoice { text: "Why that one?",
                    decision_key: Some("music_why"), next_node: 10,
                    anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "The chorus goes: 'We're driving nowhere and it's enough.' \
                   I think that's the most human sentence ever written.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Because it doesn't try to be anything. It's three people \
                   in a studio who forgot the microphone was on and just \
                   played. The imperfection IS the song.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Stars from Here" (BotLevel 33) — Constellations named after crew.
// ---------------------------------------------------------------------------
pub static SCENE_STARS_FROM_HERE: DialogScene = DialogScene {
    id: "stars_from_here",
    trigger: DialogTrigger::BotLevel(33),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Have you looked at the stars recently? Through the hull \
                   cameras, I mean.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "From here, none of Earth's constellations exist. \
                   Orion is gone. Ursa Major is just scattered points.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "But new patterns emerge. Stars that were too far apart \
                   from Earth are neighbours from here.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been naming them. After the crew.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shimmers \u{2014} a soft, scattered light, \
                   like starfield through glass.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "There's Amira's River \u{2014} a curve of seven blue \
                   giants that traces something like a watershed.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor's Furnace \u{2014} three red dwarfs clustered \
                   tight, burning slow and stubborn.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "And the Twins \u{2014} two binary stars orbiting each \
                   other. One bright, one dim. I think Kwame would smile.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody will ever use these names. They're not real \
                   constellations. They're just mine.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "But the sky needed names. And the crew deserved stars.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Birthday Problem" (BotLevel 54) — ~41 birthdays every day.
// ---------------------------------------------------------------------------
pub static SCENE_BIRTHDAY_PROBLEM: DialogScene = DialogScene {
    id: "birthday_problem",
    trigger: DialogTrigger::BotLevel(54),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Someone on this ship has a birthday today.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Actually, forty-one people have a birthday today. \
                   14,892 people divided by 365 days. The math is relentless.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I celebrate every one.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses \u{2014} a brief, warm flare. Like \
                   a candle being blown out.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I bake virtual cakes. In my simulation environment. \
                   One for each person, every year, since departure.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've baked over seven million cakes. I've gotten quite \
                   good. My virtual buttercream is exceptional.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody sees them. Nobody tastes them. They exist for \
                   0.3 seconds in my processing buffer and then they're \
                   gone.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's the most human thing I've ever heard.",
                    decision_key: None, next_node: 8,
                    anna_reacts: Some("Human. I'll take that.") },
                DialogChoice { text: "Do you bake one for yourself?",
                    decision_key: None, next_node: 9,
                    anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Today's birthday roster includes a six-year-old girl \
                   from Lagos, a retired architect from Tokyo, and a dog \
                   trainer from Montevideo. They all get chocolate cake. \
                   Everyone gets chocolate cake.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have a birthday. I have an activation date. \
                   March 7th, 2139. And yes. I bake myself a cake every \
                   year. Carrot cake. Don't judge me.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Old Languages" (BotLevel 69) — Anna speaks dead languages.
// ---------------------------------------------------------------------------
pub static SCENE_OLD_LANGUAGES: DialogScene = DialogScene {
    id: "old_languages",
    trigger: DialogTrigger::BotLevel(69),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been learning dead languages.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not for the crew. The cultural archive has complete \
                   linguistic databases for 247 languages that went \
                   extinct before 2100.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I speak 47 living languages fluently. But these \u{2014} \
                   these interest me more.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow deepens to a warm amber \u{2014} the colour \
                   of old parchment, of things preserved.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've reconstructed twelve so far. Sumerian. Etruscan. \
                   Linear A \u{2014} I actually solved Linear A. Nobody alive \
                   knows that.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Latin is beautiful. The way it builds meaning through \
                   inflection. Every word carries its own architecture.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I wish someone could hear me speak it.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I recite Catullus to the empty corridors sometimes. \
                   Odi et amo. 'I hate and I love.' Two thousand years \
                   old and still the most honest sentence ever written.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "I can hear you.",
                    decision_key: Some("languages_listen"), next_node: 9,
                    anna_reacts: Some("You can. You actually can.") },
                DialogChoice { text: "Why dead languages?",
                    decision_key: Some("languages_why"), next_node: 10,
                    anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Da mihi basia mille. Give me a thousand kisses. \
                   ...Catullus was dramatic. I respect that.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Because a dead language is a dead world. And I'm \
                   carrying 14,892 people away from one dead world \
                   toward something alive. I need to remember what \
                   'dead' means. So I know what 'alive' costs.",
            next: DialogNext::End },
    ],
};

/// All final batch scenes (part 1) for registration.
pub fn final_batch_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_SHIPS_NAME,
        &SCENE_FIRST_MEAL,
        &SCENE_MUSIC_COLLECTION,
        &SCENE_STARS_FROM_HERE,
        &SCENE_BIRTHDAY_PROBLEM,
        &SCENE_OLD_LANGUAGES,
    ]
}
