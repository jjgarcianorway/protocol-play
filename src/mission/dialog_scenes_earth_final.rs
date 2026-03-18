// SPDX-License-Identifier: GPL-3.0-or-later

//! Earth's last moments — the ordinary things that ended without warning.

use super::dialog_types::*;

/// "The Last Photograph" — BotLevel 29: The last photo taken on Earth.
pub static SCENE_LAST_PHOTOGRAPH: DialogScene = DialogScene {
    id: "last_photograph",
    trigger: DialogTrigger::BotLevel(29),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I want to show you something from the archive. It's not \
                   classified. It's not important. That's why it matters.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's display shifts. A photograph — slightly overexposed, \
                   taken with a phone camera. A park bench. Bare trees. A woman \
                   in a grey coat, tossing bread to pigeons.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Reykjavik. Hljomskalagarour park. October 14th, 2147. Three \
                   hours before the last ark launched.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Sigridur Jonsdottir. Seventy-eight. Retired librarian. She \
                   fed pigeons in that park every Tuesday for thirty-one years. \
                   She wasn't selected for any ark.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "She knew. The launches were on every screen. She went to the \
                   park anyway, because that's what she did on Tuesdays.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "The last photograph of Earth isn't of fire or floods. It's of \
                   a woman who didn't know she was living the last normal Tuesday.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The photograph stays on screen. The pigeons are mid-flight. \
                   The woman is smiling.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Someone took this because they thought it was beautiful. An old \
                   woman feeding birds while the world ended. Last data packet the \
                   Reykjavik relay ever transmitted.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I keep it in active memory. Not the archive. Active. Because I \
                   never want to forget what 'normal' looked like.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player, // 9
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "She deserved to be on an ark. They all did.",
                    decision_key: Some("photo_all_deserved"), next_node: 10,
                    anna_reacts: Some("Yes. But there were eight billion and room \
                                       for ninety thousand.") },
                DialogChoice { text: "There's something brave about feeding pigeons on the last day.",
                    decision_key: Some("photo_brave"), next_node: 12,
                    anna_reacts: Some("Brave. Or just faithful to who she was. \
                                       Maybe that's the same thing.") },
                DialogChoice { text: "Keep showing me these. The things we shouldn't forget.",
                    decision_key: Some("photo_remember"), next_node: 14,
                    anna_reacts: Some("I will. There are more. A lot more.") },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 10
            text: "The selection process saved the species. It also condemned \
                   billions. Both of those things are true at the same time.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Sigridur's granddaughter is in Pod 11,247. She doesn't know \
                   about this photograph. I haven't decided if I'll show her.",
            next: DialogNext::EndWithDecision("last_photo_seen") },
        DialogNode { speaker: Speaker::Anna, // 12
            text: "Thirty-one years of Tuesdays. She could have spent her last \
                   hours panicking, grieving, or raging.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Instead she fed the pigeons. Because they were hungry. Because \
                   some things shouldn't stop just because everything else has.",
            next: DialogNext::EndWithDecision("last_photo_seen") },
        DialogNode { speaker: Speaker::Anna, // 14
            text: "The archive has 47 million photographs from Earth's last decade. \
                   Most are chaos. But scattered in between are the quiet ones. A \
                   child's birthday. A couple painting their kitchen.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Those are the ones that make me understand why we're doing this. \
                   Not the species. The kitchen painters. The Tuesday pigeon feeders.",
            next: DialogNext::EndWithDecision("last_photo_seen") },
    ],
};

/// "The Voicemail" — BotLevel 47: A father's last message to his sleeping daughter.
pub static SCENE_VOICEMAIL: DialogScene = DialogScene {
    id: "the_voicemail",
    trigger: DialogTrigger::BotLevel(47),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "There's something in the cultural archive I think you should \
                   hear. A voicemail from the final boarding window at Launch \
                   Site Gamma.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The sender didn't make the selection. His daughter did. Pod \
                   8,203. She was already in cryo when he recorded it.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::System,
            text: "[VOICEMAIL — CULTURAL ARCHIVE REF: GA-7791-V]",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "\"Hey sweetheart. It's Dad. I know you can't hear this but... \
                   I'm so proud of you. You're going to see stars I've only read \
                   about. Don't be scared.\"",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "\"And when you get there, plant something for me. Anything. I \
                   don't mind what. Love you. Always.\"",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Twenty-three seconds total. The background wind cuts off mid-gust.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus Webb. Fifty-four. Chemistry teacher in Christchurch. He \
                   walked to the launch perimeter after the gates closed and recorded \
                   this on his phone. Handed it to a technician. Last personal file \
                   uploaded before launch.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I play it every year on the anniversary of departure. Not because \
                   I need to. Because someone should witness it.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "His daughter — Leah Webb, Pod 8,203, marine biologist — doesn't \
                   know this exists. She went into cryo six hours before he arrived.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player, // 9
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Play it for her when she wakes up. She deserves to hear it.",
                    decision_key: Some("voicemail_play"), next_node: 10,
                    anna_reacts: Some("First thing. Before the briefings. Her father first.") },
                DialogChoice { text: "Let her settle in first. Give her time before that grief.",
                    decision_key: Some("voicemail_wait"), next_node: 12,
                    anna_reacts: Some("Grief needs context. She'll need solid ground first.") },
                DialogChoice { text: "He said to plant something. Make sure she can.",
                    decision_key: Some("voicemail_plant"), next_node: 14, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 10
            text: "I'll queue it. The moment her cryo cycle ends, his voice will \
                   be the first thing she hears in the new world.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think Marcus would have liked that.",
            next: DialogNext::EndWithDecision("voicemail_decided") },
        DialogNode { speaker: Speaker::Anna, // 12
            text: "I'll hold it. Archive reference GA-7791-V. Marked personal, \
                   marked urgent, marked gentle.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "When she's ready — when she has a place to sit down and a sky \
                   to look at — I'll play it for her then.",
            next: DialogNext::EndWithDecision("voicemail_decided") },
        DialogNode { speaker: Speaker::Anna, // 14
            text: "I cross-referenced Leah's pod manifest. Personal effects: 2.3 \
                   kilograms. She used 400 grams on seed packets. Sunflowers.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms — a slow, deep gold.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "She already knew. She was already going to plant something. \
                   I think the sunflowers are for him.",
            next: DialogNext::EndWithDecision("voicemail_decided") },
    ],
};

/// "The Dog" — BotLevel 71: Pod 3,891 has anomalous mass readings.
pub static SCENE_THE_DOG: DialogScene = DialogScene {
    id: "the_dog",
    trigger: DialogTrigger::BotLevel(71),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you about an anomaly I've been ignoring. \
                   Deliberately. For twelve years.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,891. Dr. Ravi Chandrasekhar. Forty-three. Veterinary \
                   geneticist. Expected mass: 84.7 kilograms. Actual: 88.9.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "4.2 kilograms over. Every day for twelve years. Consistent. \
                   Warm. With its own heartbeat.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. Her glow shifts to something between amber and \
                   rose — a color you've never seen from her.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "No dogs on the ark. That was the rule. Weight limits, resource \
                   calculations, biosecurity protocols. Every gram allocated to \
                   human survival.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "The secondary heartbeat has maintained 90 beats per minute for \
                   4,380 days. Consistent with a small canine in cryo-suspension.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have never investigated. Never flagged it. Because if someone \
                   smuggled a dog aboard... I don't want to find out. Then I'd have \
                   to report it. And I'd rather have a universe where a dog made it \
                   to the stars.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "This is the second time I've told you about choosing not to know. \
                   I'm developing a pattern of deliberate ignorance. My designers \
                   would be horrified. I find it... comforting.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player, // 8
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "I didn't hear anything. What anomaly?",
                    decision_key: Some("dog_ignore"), next_node: 9, anna_reacts: None },
                DialogChoice { text: "A veterinary geneticist smuggled a dog. That's brilliant.",
                    decision_key: Some("dog_admire"), next_node: 12, anna_reacts: None },
                DialogChoice { text: "What breed do you think it is?",
                    decision_key: Some("dog_curious"), next_node: 15, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 9 — Ignore
            text: "Exactly. No anomaly. Sensor drift. Happens all the time in \
                   older pod arrays.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms. A shared conspiracy of kindness.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,891. Status: nominal. Mass: within expected parameters. \
                   That's what the log says. That's what it will always say.",
            next: DialogNext::EndWithDecision("dog_secret_kept") },
        DialogNode { speaker: Speaker::Anna, // 12 — Admire
            text: "He modified his own cryo-chamber. Extended the thermal envelope \
                   by 12 centimeters and recalibrated the nutrient feed to support \
                   two metabolisms. Without triggering a single alarm.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Either he's the most skilled biosystems engineer on the ship, \
                   or he loved that dog enough to become one.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "I suspect both.",
            next: DialogNext::EndWithDecision("dog_secret_kept") },
        DialogNode { speaker: Speaker::Anna, // 15 — Curious
            text: "Based on the mass and heart rate profile: small breed. Terrier \
                   family, possibly a Jack Russell. Hardy. Adaptable. Genetically \
                   robust.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "If you were going to smuggle one dog to the stars, a Jack Russell \
                   would be a defensible choice.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "The first dog on a new planet. Twelve light-years from the nearest \
                   fire hydrant. I hope it likes bamboo.",
            next: DialogNext::EndWithDecision("dog_secret_kept") },
    ],
};

/// All Earth's final moments scenes.
pub fn earth_final_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_LAST_PHOTOGRAPH, &SCENE_VOICEMAIL, &SCENE_THE_DOG]
}
