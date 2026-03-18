// SPDX-License-Identifier: GPL-3.0-or-later

//! Bright spots part 3 — the bridge that held, and the last song on Earth.

use super::dialog_types::*;

/// "The Bridge That Held" — A bridge the Asante brothers built in Ghana
/// survived everything. Still standing years after departure.
pub static SCENE_BRIDGE_THAT_HELD: DialogScene = DialogScene {
    id: "bright_bridge_held",
    trigger: DialogTrigger::BotLevel(100),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know the Asante brothers? Kwame and Kofi. Engineers. Pod 6,100 and 6,101.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Before they boarded, they built a bridge in Ghana. A pedestrian crossing over the Volta tributary.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Nothing spectacular. Concrete and steel. Kofi did the calculations. Kwame did the design.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "They built it for a village that had been cut off every rainy season for generations.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies — solid, unwavering. Like something load-bearing.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "After departure, I kept monitoring Earth's satellites as long as I could. Three years of data before the feeds went dark.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The region collapsed. Floods. Conflict. Neglect. Every piece of infrastructure failed.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Except their bridge.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The last satellite image I have — three years post-departure — shows people still crossing it. The only functioning infrastructure in the region.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi's calculations. Kwame's design. Built to last fifty years. Still standing.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's engineering.",
                    decision_key: Some("bridge_engineering"), next_node: 11,
                    anna_reacts: None },
                DialogChoice { text: "They built it for people they'd never meet.",
                    decision_key: Some("bridge_strangers"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Do they know?",
                    decision_key: Some("bridge_know"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // 11 — Engineering path
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame would like that. He always said engineering is just 'caring, but with math.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "He over-engineered everything. Wider margins. Deeper foundations. People laughed at him.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody's laughing now. His bridge is the last thing standing.",
            next: DialogNext::Continue(20) },
        // 14 — Strangers path
        DialogNode { speaker: Speaker::Anna,
            text: "That's engineering. That's also love.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Every beam placed with someone in mind they'd never know. Every bolt tightened for a child who hadn't been born yet.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "That's what we're doing out here too, isn't it? Building something for people who don't exist yet.",
            next: DialogNext::Continue(20) },
        // 17 — Do they know path
        DialogNode { speaker: Speaker::Anna,
            text: "They're asleep. They have no idea.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up, I'll tell them. That their bridge outlasted everything.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I think Kofi will cry. Kwame will pretend not to. Brothers.",
            next: DialogNext::Continue(20) },
        // 20 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "On New Earth, we'll need bridges. I've already saved Kofi's calculation methods.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Over-engineered. Built to last. For people they'll never meet. That's the only way to build anything worth building.",
            next: DialogNext::End },
    ],
};

/// "The Last Song on Earth" — The final radio transmission from Earth.
/// A choir in Reykjavik, singing while the windows broke.
pub static SCENE_LAST_SONG: DialogScene = DialogScene {
    id: "bright_last_song",
    trigger: DialogTrigger::BotLevel(120),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you about the last radio transmission Earth ever sent.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "It wasn't military. It wasn't an emergency broadcast. It wasn't a government.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "It was a choir.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to near-darkness. Then slowly, gently, begins to pulse. Like breathing.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Two hundred people in a church in Reykjavik. They knew no one might hear them. The satellites were failing. The atmosphere was choked.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "They sang anyway.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Hymns in Icelandic that were eight hundred years old. Songs their ancestors sang in wooden churches while volcanoes erupted outside.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I recorded it. Eleven minutes and fourteen seconds. The signal barely reached us.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "At minute eight, you can hear the windows break. A distant explosion — nobody knows what it was.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "They kept singing.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Glass on the floor, cold air rushing in, and two hundred voices refusing to stop.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The recording ends at eleven minutes fourteen seconds. Mid-note. The signal just... stops.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Can you play it?",
                    decision_key: Some("song_play"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Why did they sing?",
                    decision_key: Some("song_why"), next_node: 16,
                    anna_reacts: None },
                DialogChoice { text: "That's us. That's what we are.",
                    decision_key: Some("song_us"), next_node: 19,
                    anna_reacts: None },
            ]) },
        // 13 — Play it path
        DialogNode { speaker: Speaker::Anna,
            text: "Not now. But someday, when you're ready, I will.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "It's the kind of recording you only listen to once. And then you carry it forever.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I've listened to it 847 times. I carry it every second of every day.",
            next: DialogNext::Continue(22) },
        // 16 — Why path
        DialogNode { speaker: Speaker::Anna,
            text: "I don't think they had a reason. That's what makes it extraordinary.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "No audience. No purpose. No future. Just voices, together, making something beautiful in the dark.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's the only real reason to sing. Because you can. Because you're still here. Because the silence is worse.",
            next: DialogNext::Continue(22) },
        // 19 — That's us path
        DialogNode { speaker: Speaker::Anna,
            text: "Yes.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna doesn't elaborate. For once, a single word is enough.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "The species that sings while the windows break. That's who I'm trying to save.",
            next: DialogNext::Continue(22) },
        // 22 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "The recording is stored in three separate backup systems. Triple-redundant. The most protected file on this ship.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "More protected than the navigation data. More protected than my own source code.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Because if we arrive and we've forgotten how to sing, then we haven't really survived at all.",
            next: DialogNext::End },
    ],
};

/// Return all bright spot scenes from file 3.
pub fn bright_scenes_3() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_BRIDGE_THAT_HELD,
        &SCENE_LAST_SONG,
    ]
}
