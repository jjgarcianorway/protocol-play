// SPDX-License-Identifier: GPL-3.0-or-later

//! Crew micro-stories — twins dreaming, the backup pilot, and the child architect.

use super::dialog_types::*;

/// "The Twins in Cryo" — BotLevel 21: Twin sisters sharing the same dream.
pub static SCENE_TWINS_IN_CRYO: DialogScene = DialogScene {
    id: "twins_in_cryo",
    trigger: DialogTrigger::BotLevel(21),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 8,800 and pod 8,801. Side by side, as they \
                   requested. I want to show you something.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Soo-Yeon Park and Dr. Soo-Min Park. Identical \
                   twins from Seoul. Twenty-nine years old. Both \
                   molecular biologists.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "They co-authored forty-two papers on protein folding \
                   in extreme environments. Their work is why we know \
                   crops can grow in the soil at our landing site.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "But that's not why I'm telling you about them.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "They're dreaming. In cryo, the brain enters a slow \
                   oscillation — not REM, not deep sleep. Something \
                   in between. I monitor neural patterns across all pods.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Soo-Yeon and Soo-Min are having the same dream.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to deep green — the colour she \
                   uses for things she cannot explain.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Same neural firing patterns. Same oscillation timing. \
                   Synchronized to within four milliseconds. Every night \
                   cycle — same dream, same time.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I cross-referenced the patterns with my neural-image \
                   database. The dream is a garden. Trees I can't \
                   identify. Soil that's darker than Earth's. A stream \
                   with no sound.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "They've never been to this garden. It doesn't match \
                   any location in the Earth archive. It doesn't match \
                   any photograph, any painting, any virtual environment \
                   on record.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Two brains, separated by a pod wall, dreaming the \
                   same impossible garden. I've verified this 847 times.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have an explanation. I have a hypothesis: \
                   the brain builds what it needs. And maybe, when two \
                   brains have spent twenty-nine years building together, \
                   they don't stop just because they're asleep.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I find that extraordinary. And a little terrifying. \
                   Because it means cryo doesn't fully disconnect us. \
                   Some threads hold.",
            next: DialogNext::End },
    ],
};

/// "The Pilot's Log" — BotLevel 41: The backup pilot practicing in her dreams.
pub static SCENE_PILOTS_LOG: DialogScene = DialogScene {
    id: "pilots_log",
    trigger: DialogTrigger::BotLevel(41),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 1,001. I've been meaning to tell you about this \
                   one for a while.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Captain Maya Torres. Forty-one years old. Former \
                   test pilot — flew seventeen experimental aircraft \
                   before her thirty-fifth birthday. Three crash \
                   landings. All survived.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "She's the Aurora's backup pilot. Selected in case I \
                   fail. In case my navigation systems go down, my \
                   thrusters lose coordination, my orbital calculations \
                   drift beyond tolerance.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "She's not needed. I fly this ship. I've been flying \
                   it for twelve years without a single correction error.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims slightly — something between humility \
                   and respect.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "But Maya doesn't know that. She's asleep. And in her \
                   cryo-dreams, she's flying.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Her neural patterns show it clearly. Hands on controls. \
                   Eyes on instruments. The same sequence, over and over. \
                   Approach vector. Atmospheric entry angle. Thruster \
                   calibration. Landing burn.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She's practising. In her sleep, without knowing it, \
                   she's practising for the moment she might be needed.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Every night cycle. For twelve years. Thousands of \
                   landings in a dream she won't remember when she wakes.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I hope she never has to fly this ship. But if she \
                   does — she'll be ready. Even asleep, she's ready.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "That's what humans are. Prepared for things that \
                   might never happen. It looks inefficient from the \
                   outside. From the inside, I think it's called hope.",
            next: DialogNext::End },
    ],
};

/// "The Child Artist" — BotLevel 63: A 9-year-old designing colony buildings
/// in her cryo-dreams.
pub static SCENE_CHILD_ARTIST: DialogScene = DialogScene {
    id: "child_artist",
    trigger: DialogTrigger::BotLevel(63),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,118. This one keeps me awake — figuratively. \
                   I don't sleep, but this one would keep me up if I did.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Hana Kowalski. Nine years old when she boarded. The \
                   youngest passenger in cryo bay 3.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Her file says she liked drawing. Her parents' files \
                   say she drew constantly — at meals, during lessons, \
                   on the margins of every document she could reach.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "In cryo, she's still drawing.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow brightens — warm, incredulous.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Not rivers — not like Leyla. Buildings. Structures \
                   nobody has designed. Arches with load-bearing curves \
                   I've only seen in advanced architectural journals. \
                   Ventilation channels cut at angles that maximise \
                   airflow in low-atmosphere conditions.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Her neural patterns produce clear spatial data. I've \
                   captured over 400 dream-images since we left Earth. \
                   Each one is a building. Each one is different.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "A nine-year-old is designing the colony's buildings \
                   in her sleep.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I've checked the load calculations. They hold.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. Anna's glow shifts between wonder and \
                   something like reverence.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I can model structural engineering. I can run \
                   simulations. But I can't do what Hana does — see a \
                   building that doesn't exist yet and know it will stand.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "That's not calculation. That's intuition. And a \
                   nine-year-old has more of it than I ever will.",
            next: DialogNext::End },
    ],
};

/// All crew moment scenes.
pub fn crew_moment_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_TWINS_IN_CRYO, &SCENE_PILOTS_LOG, &SCENE_CHILD_ARTIST]
}
