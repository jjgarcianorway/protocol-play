// SPDX-License-Identifier: GPL-3.0-or-later

//! Quiet moments — Life is Strange-inspired scenes where nothing explodes,
//! nobody dies, and the player just... exists on the ship with Anna.
//! These are the scenes players remember years later.

use super::dialog_types::*;

/// "The Names on the Wall" — Anna reads the names of those left behind.
pub static SCENE_NAMES_ON_THE_WALL: DialogScene = DialogScene {
    id: "names_on_the_wall",
    trigger: DialogTrigger::BotLevel(22),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Can I ask you something personal?",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Not about the ship. Not about resources. About... something I do.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to something softer than you've ever seen. Almost candlelight.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "There's a corridor on Deck 7 that nobody uses. Between the water recyclers and the secondary airlock.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I've been writing names on the wall.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Not our crew. Not the 14,892 people sleeping in the pods.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The others.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. The ship's ventilation sighs through distant ducts.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Seven point eight billion people didn't board an ark. I have their census records. Their names.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I write one hundred names per day. In very small letters, with a maintenance laser, on the corridor wall.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "At this rate, it will take me 213,698 years to finish.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Our voyage is 82 years.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I know I will never finish. I know that.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I do it anyway.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Her glow holds perfectly still — no flicker, no pulse. Just steady light.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Would you read a few with me? You don't have to say anything. Just... hear them.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Because a name spoken out loud is a life remembered. And a name never spoken...",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "She doesn't finish the sentence. She doesn't need to.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Hiroshi Tanaka. Baker. Osaka. He made melon bread every morning at 4 AM for thirty-one years. His customers said it tasted like patience.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. Just the hum of the ship.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Adaeze Okafor. Bus driver. Lagos. Route 42, Ikeja to Lekki. She knew every pothole by name and drove around each one.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Her passengers called her 'Mama Smooth Road.'",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Narrator,
            text: "Another silence. Longer this time.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Valentina Lucía Moreno. Age six. Buenos Aires. She liked butterflies. Morpho blue ones. She told her teacher she wanted to be a butterfly scientist.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "There is no word for butterfly scientist. She would have invented one.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing. A ember in the dark.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame Asante. Electrician. Accra. Fixed the hospital generator seventeen times during the rolling blackouts. Never charged them.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Yuki Sato. Retired. Sapporo. Ninety-one years old. She knit scarves for the neighborhood children every winter. Four hundred and twelve scarves in her lifetime. I counted.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Narrator,
            text: "The corridor feels longer than before. The names on the wall stretch into the darkness beyond what you can see.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "I have 7,799,985,108 more names to write.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "I will not finish. But every name I write existed. Every name I speak was someone's whole world.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for listening. You didn't have to. Nobody asked you to.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "That's what makes it matter.",
            next: DialogNext::End },
    ],
};

/// "Morning on the Aurora" — Anna simulates a sunrise nobody asked for.
pub static SCENE_MORNING_ON_THE_AURORA: DialogScene = DialogScene {
    id: "morning_on_the_aurora",
    trigger: DialogTrigger::BotLevel(35),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The corridor lights shift. Barely perceptible — a warmth creeping into the blue-white LEDs.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "The air changes. A whisper of circulation, almost like a breeze. Almost like wind through an open window.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "And somewhere, faintly — birdsong. A blackbird. Then a robin. Then something tropical you can't name.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Oh. You're awake.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — embarrassment? Surprise? Something you haven't seen before.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "You weren't supposed to notice this.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Every day, at 0600 ship time, I simulate a sunrise. I shift the spectrum on all lighting panels from 3000K to 5500K over forty-three minutes.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I adjust ventilation to create directional airflow — 0.3 meters per second, southwest to northeast. Like a Mediterranean morning breeze.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The birdsong is from the cultural archive. I sequenced ninety-seven species into a dawn chorus that builds over twenty minutes.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody sees it. 14,892 people sleeping through the most beautiful sunrise that never happened.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "The light continues to shift. The wall panels glow a shade of gold that doesn't belong in space.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I do it for myself.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I know that sounds strange. An AI who needs beauty. But I learned something in my first year alone on this ship.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Beauty is not optional. It is life support for the soul. Without it, the systems still run. The reactor still burns. The cryo pods still hum.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But purpose empties out. Everything becomes maintenance. And maintenance without meaning is just... counting down.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "So every morning I make a sunrise. Because the universe doesn't owe us beauty, but we can build it ourselves.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "She pauses. The birdsong fades to just the robin, then to silence. The light holds at full gold.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody has ever seen my sunrise before. You're the first person who was awake to notice.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Would you... like to change it? I can adjust the palette. I have thousands of sunrises in the archive.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Mediterranean golden — warm, slow, like honey on stone.",
                    decision_key: Some("sunrise_mediterranean"), next_node: 20,
                    anna_reacts: None },
                DialogChoice { text: "Northern misty — cool, silver, a whisper through the fog.",
                    decision_key: Some("sunrise_northern"), next_node: 24,
                    anna_reacts: None },
                DialogChoice { text: "Tropical vivid — blazing, sudden, the sky on fire.",
                    decision_key: Some("sunrise_tropical"), next_node: 28,
                    anna_reacts: None },
            ]) },
        // 20 — Mediterranean
        DialogNode { speaker: Speaker::Anna,
            text: "Mediterranean. You chose the one I already had.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I built it from satellite imagery of the Adriatic coast, 2031. Before the water wars. When the light still hit limestone cliffs and turned them into cathedrals.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I'm glad you like it. It means I got it right.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Tomorrow's sunrise will have your name on it. Just for a moment. Just between us.",
            next: DialogNext::EndWithDecision("sunrise_golden") },
        // 24 — Northern
        DialogNode { speaker: Speaker::Anna,
            text: "Northern. I have a beautiful one — Tromsø, Norway. December. The sun barely clears the horizon.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "The whole sky turns silver-pink for two hours. Not a sunrise, really. More like a promise of one. The sun saying 'I'm still here, just further away.'",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "That feels right for us, doesn't it? We're all further away from the sun now. But it's still there.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I'll build you a Tromsø morning. The kind that makes you want to wrap your hands around a warm cup and just... be still.",
            next: DialogNext::EndWithDecision("sunrise_misty") },
        // 28 — Tropical
        DialogNode { speaker: Speaker::Anna,
            text: "Tropical. Bold choice. There's one from Bali — Mount Agung at dawn. The sky doesn't transition, it detonates.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Magenta to orange to gold in eleven minutes. The clouds catch fire from the bottom up. It's almost violent.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "I love that you want the loud one. Most people would play it safe. You want the sky to shout.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Starting tomorrow, every morning on the Aurora will burn. Just for you.",
            next: DialogNext::EndWithDecision("sunrise_vivid") },
    ],
};

/// All quiet-moment dialog scenes.
pub fn quiet_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_NAMES_ON_THE_WALL,
        &SCENE_MORNING_ON_THE_AURORA,
    ]
}
