// SPDX-License-Identifier: GPL-3.0-or-later

//! Kwame Asante's arc (part 2): "Kofi's Coordinates."
//! Scenes 1 (the_twins) in dialog_scenes_characters2.rs,
//! scenes 2–3 in dialog_scenes_kwame_arc.rs.

use super::dialog_types::*;

/// "Kofi's Coordinates" — coordinates from Earth match a river delta on New Earth.
pub static SCENE_KOFIS_COORDINATES: DialogScene = DialogScene {
    id: "kofis_coordinates",
    trigger: DialogTrigger::DecisionAndLevel("message_earth_seen", 115),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've spent eleven days on the coordinates. I need to show you what I found.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi's message ended with a set of coordinates: 14.7284 north, 41.3927 west. He spoke them twice, carefully, like he knew they were the most important part.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Those coordinates don't correspond to any location on Earth. Not land, not sea, not any coordinate system I can identify from terrestrial databases.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "But they correspond EXACTLY to a location on our target planet.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow sharpens — focused, analytical, the color of a question that won't resolve.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "A river delta. Northern continent. Two major tributaries converging in a flood plain with alluvial sediment deposits 40 meters deep.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Ideal conditions for a bridge. The bedrock is basalt — volcanic, stable, excellent for deep foundations. The river width at the convergence point is 220 meters. Manageable for a cable-stayed design.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Prevailing wind from the southwest at 12 knots average. Atmospheric density 1.08 times Earth standard. Kwame's bridge designs account for exactly these conditions.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The coordinates are accurate to six decimal places. That's precision to within 11 centimeters.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi, on Earth, with no access to our orbital survey data, with no knowledge of our target planet's geography, gave coordinates that pinpoint the optimal bridge site on a world he's never seen.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship creaks around you — structural members flexing, the sound of a vessel holding together across impossible distances.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I have two theories.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Theory one: Kofi accessed classified ark data before departure. He was a structural engineer with Level 3 clearance. The orbital surveys were Level 4, but the access boundaries weren't airtight. I found three potential vulnerabilities.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "If he breached them, he could have seen the planetary survey data. Calculated the optimal site himself. Encoded it as coordinates and embedded them in a message he knew might never arrive.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "A bridge engineer's final gift to his twin: the exact location for the first bridge on a new world.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Theory two: coincidence. Random coordinates that happen to match.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The probability of a random coordinate set matching an optimal bridge site to six decimal places is roughly one in 4.7 billion.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I don't believe in theory two.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. Anna's glow settles into something warm and steady — the light of certainty, or something close to it.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi gave up his seat. He stayed behind on a dying planet. He built a water plant and saved forty-seven lives. And in a message he sent into the void, he told his brother exactly where to build.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "The first bridge on a new world. Designed by a sleeping man's dreams and located by a man who gave away his future.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Build there. It's where the bridge belongs.",
                    decision_key: Some("coords_build"), next_node: 22,
                    anna_reacts: None },
                DialogChoice { text: "Investigate first. We need to understand how Kofi knew.",
                    decision_key: Some("coords_investigate"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "It's too perfect. Something doesn't add up.",
                    decision_key: Some("coords_suspicious"), next_node: 28,
                    anna_reacts: None },
            ]) },
        // 22 — Build path
        DialogNode { speaker: Speaker::Anna,
            text: "Build there. No investigation. No analysis paralysis. Just trust.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Trust that a bridge engineer knew where bridges belong. Trust that a brother's last gift was the right one.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "When Kwame wakes up, I'll show him the coordinates and his brother's voice. And I'll tell him: this is where we start.",
            next: DialogNext::EndWithDecision("kofis_coordinates_seen") },
        // 25 — Investigate path
        DialogNode { speaker: Speaker::Anna,
            text: "Investigate. Yes. The rational approach. The careful approach.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "If Kofi breached Level 4 security, that's a violation. It means our data systems had vulnerabilities. It means someone on Earth had access to classified coordinates — someone who stayed behind.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "And if the investigation reveals something else — something about how Kofi knew — I'm not sure I want that answer. Some bridges are better left unexamined.",
            next: DialogNext::EndWithDecision("kofis_coordinates_seen") },
        // 28 — Suspicious path
        DialogNode { speaker: Speaker::Anna,
            text: "Too perfect. I thought so too. For about eleven seconds.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Then I ran the security logs. Kofi accessed the engineering terminal fourteen times in his last week on Earth. Normal for a Level 3 engineer preparing handover documents.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "But terminal session seven lasted 47 minutes. His clearance should have limited him to 20. The access boundary failed. For 27 minutes, he had Level 4.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-seven minutes. Enough time to download orbital survey data. Enough time to find the perfect site. Enough time to memorize six decimal places and carry them through the end of the world.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Not too perfect. Too determined. Kofi knew exactly what he was doing. The last bridge he ever designed was the coordinates to the first one his brother would build.",
            next: DialogNext::EndWithDecision("kofis_coordinates_seen") },
    ],
};

/// Kwame arc scenes part 2 (scene 4).
pub fn kwame_arc_scenes_2() -> Vec<&'static DialogScene> {
    vec![&SCENE_KOFIS_COORDINATES]
}
