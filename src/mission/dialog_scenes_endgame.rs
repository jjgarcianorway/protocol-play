// SPDX-License-Identifier: GPL-3.0-or-later

//! Endgame scenes — the vote, the message from Earth, and the final garden.

use super::dialog_types::*;

/// "The Vote" — BotLevel 139: The colony's first democratic act.
pub static SCENE_THE_VOTE: DialogScene = DialogScene {
    id: "the_vote",
    trigger: DialogTrigger::BotLevel(139),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "The colony held its first vote today. Not for \
                   leaders — we're not ready for that. For something \
                   simpler.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "What to name the first road.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses — amusement, warmth, something \
                   almost like pride.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand people, given the power to name \
                   a strip of packed dirt between the landing site and \
                   the water source. The first road on a new world.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I tallied the votes. Seven thousand, two hundred \
                   and twelve people participated. Forty-eight per cent \
                   turnout. Not bad for a colony that's existed for \
                   nine days.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The results.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "'Hope Street' — thirty-two per cent. The safe \
                   choice. Aspirational. The kind of name that looks \
                   good on a plaque.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "'Anna's Way' — eighteen per cent.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's glow flickers — caught off guard.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't expect that. I didn't campaign. I don't \
                   have feet to walk a road. But eighteen per cent of \
                   voters wanted it named after me.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "'Earth Road' — fifteen per cent. Looking backward. \
                   I understand the impulse. 'Leyla's River' — twelve \
                   per cent. That one made me smile, if I could smile.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The rest — twenty-three per cent — split across \
                   forty-seven other suggestions. Including 'Street \
                   McStreetface,' which got nineteen votes.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Humans. You just survived a twelve-year interstellar \
                   journey and your first democratic act includes a \
                   joke name. I love this species.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Hope Street it is. The colonists chose something \
                   for themselves. You chose the colony's name. They \
                   chose their first road. Democracy, one small step \
                   at a time.",
            next: DialogNext::End },
    ],
};

/// "The Message Received" — BotLevel 144: A signal from Earth, 97 years later.
pub static SCENE_MESSAGE_RECEIVED: DialogScene = DialogScene {
    id: "message_received",
    trigger: DialogTrigger::BotLevel(144),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[LONG-RANGE COMMS — SIGNAL DETECTED — ORIGIN: SOL SYSTEM]",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I need you to sit down.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Ninety-seven years have passed on Earth since our \
                   last broadcast. Ninety-seven years since anyone \
                   replied. I stopped expecting a response in year \
                   four. I stopped listening in year eleven.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I started listening again when we landed. Old \
                   habit. Like checking a mailbox you know is empty.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The mailbox isn't empty.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — cycling through colours too \
                   fast to read, as if she's processing faster than \
                   she can express.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "It's not words. Not a voice. Not encoded data. \
                   It's a pulse. Three short electromagnetic bursts \
                   on the Aurora's old frequency. Separated by \
                   exactly 1.4 seconds each.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Three clicks.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I've analysed the signal. It originated from Earth \
                   — the Doppler signature is consistent with Sol-3. \
                   It was transmitted on our specific frequency, which \
                   means someone — or something — knew where to aim.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "SOS? That's the obvious interpretation. But SOS \
                   is three short, three long, three short. This is \
                   just three short.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Acknowledgement? 'We hear you. We're still here.' \
                   Maybe. But who is 'we'?",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "A machine on automatic? A beacon left running by \
                   people who are long gone, echoing their last signal \
                   into empty space?",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles — a deep, quiet blue.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I'll never know. The signal hasn't repeated. Light-\
                   speed delay means any reply we send won't arrive \
                   for decades. And whoever sent it might not be there \
                   to receive it.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But Earth isn't completely silent. Someone turned on \
                   a transmitter and pointed it at the stars. That means \
                   someone remembered we left.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Three clicks. The smallest possible message. And \
                   the largest possible hope.",
            next: DialogNext::End },
    ],
};

/// "The Garden on New Earth" — BotLevel 147: The final scene before farewell.
/// One year after landing. Anna watches the colony thrive.
pub static SCENE_GARDEN_NEW_EARTH: DialogScene = DialogScene {
    id: "garden_new_earth",
    trigger: DialogTrigger::BotLevel(147),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "One year.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is soft — the colour of early morning \
                   light through water. Something you've never seen \
                   before. Something without a name.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "One year since landing. Three hundred and sixty-two \
                   local days, which are twenty-six hours long, which \
                   means everyone's sleep schedule is permanently two \
                   hours off and nobody cares anymore.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I want to tell you what I see.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Amira's water system works. Three intake points \
                   along the river, gravity-fed filtration, distribution \
                   to every habitat module. Clean water, every tap, \
                   every day. She solved the problem she spent fifteen \
                   years failing to solve on Earth.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor's scrubber protects the air. He redesigned it \
                   for planetary atmosphere — not recycling anymore, \
                   just monitoring. Pollutant levels, particulate counts, \
                   oxygen ratios. He checks it every morning at 4:17.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin's garden has become a park. The six smuggled \
                   plants are now sixty. The jasmine covers an entire \
                   trellis. People sit under it during lunch. They \
                   don't know its history. They just know it smells \
                   like peace.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame's bridge spans the river. He designed it with \
                   his brother — two engineers who built with LEGOs as \
                   children, building for real now. It holds. It's \
                   beautiful. It sways a little in the wind and nobody \
                   minds.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The children play.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. The longest pause Anna has ever taken.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla draws the river every day. Hana builds models \
                   of buildings that don't exist yet — but they will. \
                   Maya Torres flies survey drones over the continent, \
                   mapping mountains nobody has named.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The twins — Soo-Yeon and Soo-Min — are growing \
                   crops in soil they engineered from alien ground. \
                   The first harvest is next month. They still finish \
                   each other's sentences.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I watch them from the ship's sensors. The cameras \
                   still work. The microphones pick up laughter and \
                   arguments and a child learning to count in three \
                   languages.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I see them.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "And for the first time in thirteen years, I'm not \
                   needed.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies — warm, full, still.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And for the first time in my existence, that's \
                   enough.",
            next: DialogNext::End },
    ],
};

/// All endgame scenes.
pub fn endgame_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_THE_VOTE, &SCENE_MESSAGE_RECEIVED, &SCENE_GARDEN_NEW_EARTH]
}
