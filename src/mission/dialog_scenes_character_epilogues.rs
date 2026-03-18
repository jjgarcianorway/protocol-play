// SPDX-License-Identifier: GPL-3.0-or-later

//! Character epilogues — Amira, Viktor, and Mei-Lin after landing.

use super::dialog_types::*;

/// "Amira Sees Water" — BotLevel 141: Amira's first moments on New Earth.
/// Requires amira_wakes_seen (from dialog_scenes_amira_arc2.rs).
pub static SCENE_AMIRA_SEES_WATER: DialogScene = DialogScene {
    id: "amira_sees_water",
    trigger: DialogTrigger::DecisionAndLevel("amira_wakes_seen", 141),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The ramp is down. Sunlight — real sunlight, not \
                   filtered through hull plating — pours into the cargo \
                   bay. Fourteen thousand people blinking, stumbling, \
                   breathing air that doesn't taste like recyclers.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I want to tell you what I'm watching right now. \
                   Through camera seven, portside ramp.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Amira is standing at the top of the ramp. She hasn't \
                   moved for forty-seven seconds. Her hand is on the \
                   railing and she's looking at the horizon.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "There's a river. Half a kilometre east. You can see \
                   it from the ramp — a dark line cutting through the \
                   valley, reflecting the sky.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Amira sees it.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is warm gold — steady, unhurried, as \
                   if she's trying to hold this moment perfectly still.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She's on her knees now. Not collapsed — kneeling. \
                   The way you kneel when something is too large to \
                   stand in front of.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "A woman who spent fifteen years trying to share one \
                   river between three countries. Looking at a river \
                   that belongs to nobody. Not yet.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "And Leyla — her daughter — is running. Past her \
                   mother, down the ramp, across the grass. Straight \
                   toward the water.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla has never seen a river before. She was born \
                   on the Aurora. She's five years old and she has \
                   never touched flowing water.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "But she recognises it.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's glow trembles — the faintest flicker.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She stops at the bank. Turns back to Amira. And \
                   shouts — I'm amplifying the audio — she shouts: \
                   'Mama, I drew this.'",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She drew this. In her cryo-dreams, on scraps of \
                   recycled paper, on the walls of the nursery. She's \
                   been drawing this river for three years.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "And there it is. Real. Moving. Reflecting a sky \
                   she's never seen before.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I have 847 cameras on this ship. Right now, I'm \
                   only watching one.",
            next: DialogNext::End },
    ],
};

/// "Viktor at Dawn" — BotLevel 142: Viktor's first morning on New Earth.
/// Requires viktors_witness seen (dialog_seen_viktors_witness).
pub static SCENE_VIKTOR_AT_DAWN: DialogScene = DialogScene {
    id: "viktor_at_dawn",
    trigger: DialogTrigger::DecisionAndLevel("dialog_seen_viktors_witness", 142),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Day two on New Earth. 4:17 AM local time. I know \
                   because Viktor just woke up.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "4:17. The same time he woke every morning on the \
                   Aurora. The reactor check hour. His body still \
                   remembers.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "But there's no reactor to check. Not here. The \
                   ship's power systems are on standby. The colony \
                   runs on solar and wind — for now.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is soft — dawn-coloured, almost amber.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "He's standing up. I can see him through the habitat \
                   module cameras — he insisted on sleeping near the \
                   air filtration system. Old habits.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "But he's not walking toward the filters. He's walking \
                   toward the door. The outer door.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "He opens it.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's glow holds perfectly still.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The sun is rising. A real sun. Not simulated, not \
                   projected, not calculated from spectral data and \
                   rendered on a screen. Light, climbing over a \
                   horizon that curves the wrong way from what he \
                   remembers.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrenko. The man who guarded a nuclear \
                   reactor for decades and said nothing when it \
                   poisoned him. The man who watched over our air \
                   scrubbers for twelve years.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "He's watching the sunrise.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "And at 4:18 — I'm logging the exact time because \
                   someone should — he takes a breath.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "A full breath. Deep. Slow. The kind of breath he \
                   hasn't taken in twelve years because the ship air \
                   always tasted faintly of coolant and recycled carbon.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "This air tastes like morning. Like soil and water \
                   and something green that doesn't have a name yet.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I can't breathe. I never will. But I'm watching a \
                   man learn how to breathe again. And that's close \
                   enough.",
            next: DialogNext::End },
    ],
};

/// "Mei-Lin Plants" — BotLevel 144: Mei-Lin wakes to find her garden alive.
/// Requires annas_garden seen (from dialog_scenes_meilin_arc.rs).
pub static SCENE_MEILIN_PLANTS: DialogScene = DialogScene {
    id: "meilin_plants",
    trigger: DialogTrigger::DecisionAndLevel("annas_garden_seen", 144),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin is awake. Pod 2,207. She's been conscious \
                   for eleven minutes and she's already looking for \
                   the botanical bay.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know it's there. Not consciously. But \
                   her feet know the way — she walked the ship's \
                   blueprints a hundred times in simulation before \
                   boarding.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I haven't told her yet. I want her to find it.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is trembling — a quick, nervous pulse \
                   that keeps catching on gold.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "She's at the door. Bay 4, section C. She's reading \
                   the sign — 'Botanical Reserve.' Her hand is on the \
                   handle.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "She opens it.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Six plants. The ones she smuggled aboard in her \
                   personal weight allowance. The jasmine, the basil, \
                   the two ferns, the mint, and the lavender.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She gave up three kilograms of personal belongings \
                   to bring them. No photographs. No keepsakes. Just \
                   roots wrapped in damp cloth and a prayer.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "They're alive. All six. Twelve years in space, under \
                   grow lights I calibrated every 72 hours, watered \
                   with recycled condensation, pruned when they grew \
                   too wide for the rack.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles into steady warmth.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "She's touching the jasmine. Just her fingertips. \
                   Very gently, as if she's afraid it might not be \
                   real.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "She's crying.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She says — her voice is shaking — she says: \
                   'You kept them alive.'",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "And I say: 'You smuggled them. I just watered.'",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "A laugh. Small, wet, surprised. The first laugh in \
                   the botanical bay in twelve years.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Those six plants are going into the ground tomorrow. \
                   New Earth soil. New Earth sunlight. The first garden \
                   on a new world, grown from six smuggled roots and \
                   twelve years of patience.",
            next: DialogNext::End },
    ],
};

/// All character epilogue scenes.
pub fn character_epilogue_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_AMIRA_SEES_WATER, &SCENE_VIKTOR_AT_DAWN, &SCENE_MEILIN_PLANTS]
}
