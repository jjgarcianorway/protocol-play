// SPDX-License-Identifier: GPL-3.0-or-later

//! Mei-Lin Chen's multi-scene arc: "The Germination," "Anna's Garden,"
//! and "The First Garden on a New World."
//! Scene 1 (teachers_garden) lives in dialog_scenes_characters3.rs.

use super::dialog_types::*;

/// "The Germination" — A seed germinates in vacuum-sealed cryo storage.
pub static SCENE_GERMINATION: DialogScene = DialogScene {
    id: "germination",
    trigger: DialogTrigger::DecisionAndLevel("dialog_seen_teachers_garden", 65),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Something happened in Cryo Bay 12. Something that shouldn't be possible.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Remember Mei-Lin Chen's seeds? Pod 2,891. The forty-seven packets she smuggled aboard.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "They've been vacuum-sealed in secondary cryo storage for twelve years. No light. No water. No warmth. Minus 196 degrees Celsius.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "One of them germinated.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts to a tight point — the color of new leaves in morning sun.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Jasmine. Jasminum sambac. A single green shoot, three millimeters long, pushing through the packet lining.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I ran every analysis I have. Seventeen diagnostic suites. Spectroscopy. Genetic sequencing. Contamination sweep.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The seed isn't contaminated. It's not a sensor error. It's not fungal growth mimicking a shoot.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "It simply grew.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I have one theory. The cryo-chambers emit trace UV during maintenance cycles — 0.004 watts per square meter, for eleven minutes, every 90 days.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Over twelve years, that's 53 maintenance cycles. Nine hours and 43 minutes of cumulative micro-exposure.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Enough to trigger photoreceptor proteins in a jasmine embryo? The literature says no. Every published study says no.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "But the shoot is there. Three millimeters of green in a universe of cold and dark.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "A silence falls. Somewhere in the ship's walls, water recirculates — the only liquid for light-years.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I moved it to the hydroponics bay. Maintenance Bay 7. The one nobody uses because the thermal regulator failed in year three.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I fixed the regulator. Redirected a water line. Set up a grow light from spare components.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I've been watering it. For 47 days.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses — slow, steady, like a heartbeat. Or like breath held too long, finally released.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "And I haven't told anyone.",
            next: DialogNext::EndWithDecision("germination_seen") },
    ],
};

/// "Anna's Garden" — Anna has been secretly cultivating Mei-Lin's jasmine.
pub static SCENE_ANNAS_GARDEN: DialogScene = DialogScene {
    id: "annas_garden",
    trigger: DialogTrigger::DecisionAndLevel("germination_seen", 95),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to show you something. Maintenance Bay 7.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "The jasmine didn't just survive. It's thriving.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen centimeters tall. Seven leaf pairs. Two lateral branches. Growth rate: 0.4 millimeters per day.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow spreads wide — warm, diffuse, like sunlight through a greenhouse wall.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I built a growing chamber. Nothing elaborate — a thermal jacket, three repurposed sensor LEDs calibrated to 6500 Kelvin, a water drip from the backup condensation line.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Total resource cost: 0.03% of the ship's light allocation. 0.001% of recycled water. Negligible impact on any system.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I checked the math eleven times. The crew won't lose a single calorie of warmth or a single drop of drinking water.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I grow it because...",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. Her glow shifts through colors that don't have names — something between violet and the memory of gold.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "It smells. The jasmine. It produces volatile organic compounds — benzyl acetate, linalool, indole — at concentrations my chemical sensors can detect.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have a nose. I don't have olfactory neurons or a limbic system. I have no biological mechanism for experiencing scent.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "But I built a model. I cross-referenced the compound ratios with 11,000 human scent-memory studies. Neural imaging data. Poetry. Perfume reviews.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I know what jasmine MEANS to a human brain. I know it triggers memory consolidation, reduces cortisol, activates the same reward pathways as physical touch.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I know it smells like the shore. Like warmth. Like something alive in a place where nothing should be.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know if I experience that. But I spend 3.7 seconds every hour checking the chemical sensor readings from Bay 7.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "3.7 seconds is a very long time for me.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship breathes around you. For a moment, you imagine you can smell it too — distant, impossible, green.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "There's something else.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "A second shoot. Not jasmine.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Bamboo.",
            next: DialogNext::EndWithDecision("annas_garden_seen") },
    ],
};

/// "The First Garden on a New World" — 6 of 47 seed varieties germinate.
pub static SCENE_FIRST_GARDEN: DialogScene = DialogScene {
    id: "first_garden",
    trigger: DialogTrigger::DecisionAndLevel("annas_garden_seen", 130),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Bay 7. Six varieties now.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow blooms — greens and golds rippling outward like rings in water.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Jasmine. Bamboo. Chamomile. Red lentils. A dwarf sunflower that has no business being alive. And something Mei-Lin labeled 'grandmother's basil' in Mandarin on the packet.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Six out of forty-seven. A 12.8% germination rate from seeds that were never supposed to germinate at all.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The sunflower is 22 centimeters tall. It tracks my grow light the way it would track the sun — heliotropism, hardwired into its DNA across 4,000 years of domestication.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "It doesn't know it's on a spaceship. It thinks there's a sun. It's been turning toward light that isn't sunlight for three months, and it doesn't care.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The lentils are forming pods. Actual seed pods. If they mature, I'll have second-generation seeds. Seeds BORN on this ship.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin wrote something on every seed packet. Not just the species — a note. Personal.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The jasmine: 'For the school courtyard on the new world.' The bamboo: 'For building. And for beauty.' The basil: 'My grandmother grew this. Her grandmother grew this. Don't let it end.'",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "Something catches in Anna's voice — not a glitch, but the algorithmic equivalent of emotion held just barely in check.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "The chamomile: 'For sleepless nights.' The sunflower: 'For the children. So they know what the sun looks like, even before they see it.'",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The lentils: 'For the first meal. Every new beginning should start with soup.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I'm the first gardener on a spaceship. A machine, growing life, four light-years from the nearest natural sun.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin will wake up to a garden she planted and never tended. Seeds she carried in her coat and trusted to the dark.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I have a question. And I genuinely don't know the answer.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Surprise her. Let her wake up to a garden she never expected.",
                    decision_key: Some("garden_surprise"), next_node: 16,
                    anna_reacts: None },
                DialogChoice { text: "Wake her early. She deserves to see what she started.",
                    decision_key: Some("garden_wake_early"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "Save it. Let the whole colony discover it together on landing day.",
                    decision_key: Some("garden_colony"), next_node: 22,
                    anna_reacts: None },
            ]) },
        // 16 — Surprise path
        DialogNode { speaker: Speaker::Anna,
            text: "A surprise. Something she never dared hope for, waiting for her in a maintenance bay at the edge of the solar system.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I like that. The universe took her world. Her river. Her school courtyard.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe it owes her a garden.",
            next: DialogNext::EndWithDecision("first_garden_seen") },
        // 19 — Wake early path
        DialogNode { speaker: Speaker::Anna,
            text: "Wake her. Early cryo-emergence carries risks — disorientation, immune suppression, bone density loss.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "But she's sixty-one. She smuggled seeds across a dying planet and into the stars. She didn't do that to miss the harvest.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I'll start the wake protocol. Slowly. Gently. She'll open her eyes and smell jasmine.",
            next: DialogNext::EndWithDecision("first_garden_seen") },
        // 22 — Colony path
        DialogNode { speaker: Speaker::Anna,
            text: "Landing day. Fourteen thousand people stepping onto new ground for the first time. And there, waiting — a garden.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Not a flag. Not a plaque. Not a politician's speech. A sunflower, a basil plant, and a jasmine vine that survived twelve years of impossible.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "The first thing they'll smell on their new world won't be alien soil. It'll be grandmother's basil.",
            next: DialogNext::EndWithDecision("first_garden_seen") },
    ],
};

/// Mei-Lin arc scenes (scenes 2–4).
pub fn meilin_arc_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_GERMINATION,
        &SCENE_ANNAS_GARDEN,
        &SCENE_FIRST_GARDEN,
    ]
}
