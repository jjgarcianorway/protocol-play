// SPDX-License-Identifier: GPL-3.0-or-later

//! Connection scenes, part 2 — deeper LOST-style reveals linking
//! characters across time and distance. Ends with the meta-scene
//! where Anna maps the full human web.

use super::dialog_types::*;

// =========================================================================
// "The Doctor's Other Patient" — BotLevel 75
// Dr. Sophia Marchand connects Viktor and General Diallo.
// Requires: viktors_confession + generals_mercy seen.
// =========================================================================

static DOCTOR_REQUIRED: &[&str] = &[
    "dialog_seen_viktors_confession",
    "dialog_seen_generals_mercy",
];

pub static SCENE_DOCTORS_OTHER_PATIENT: DialogScene = DialogScene {
    id: "connection_doctors_other_patient",
    trigger: DialogTrigger::AllDecisionsAndLevel(DOCTOR_REQUIRED, 75),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I found a personnel file that shouldn't exist. \
                   Someone at the Marseille boarding site filed it \
                   under the wrong clearance level.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Sophia Marchand. Trauma surgeon. Doctors \
                   Without Borders, 2094 to 2101. Pod 7,803.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "In 2094, she was deployed to Senegal. General \
                   Diallo's water crisis. She treated dehydration \
                   victims outside the security perimeter \u{2014} \
                   the ones the guards wouldn't let near the wells.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "She watched people die of thirst. Children, \
                   mostly. She filed 139 death certificates in \
                   eleven weeks.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow drops to a heavy grey \u{2014} the \
                   colour of hospital corridors at 3 AM.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then headquarters recalled her to Marseille. \
                   The Mediterranean nuclear exchange. Viktor \
                   Petrov's weapons.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "She went from watching people die of thirst to \
                   watching them die of radiation. Same hands. Same \
                   surgical kit. Different apocalypse.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "She connects them. Viktor and Diallo. The nuclear \
                   engineer and the general. Neither knows she's the \
                   thread between their two disasters.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "She sleeps between their decks. Equidistant. As \
                   if the ship's architects knew she belonged to \
                   both stories.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "One woman who witnessed both horrors. And she \
                   still volunteered for the ark. Still chose to \
                   heal people, knowing exactly what people are \
                   capable of.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Song in the Archive" — BotLevel 110
// Marcus Cole's broadcast and Anna's guitar recording share a source.
// Requires: annas_song + last_broadcast seen.
// =========================================================================

static SONG_REQUIRED: &[&str] = &[
    "dialog_seen_annas_song",
    "dialog_seen_last_broadcast",
];

pub static SCENE_SONG_IN_THE_ARCHIVE: DialogScene = DialogScene {
    id: "connection_song_in_archive",
    trigger: DialogTrigger::AllDecisionsAndLevel(SONG_REQUIRED, 110),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been cleaning up Marcus Cole's final \
                   broadcast. Noise reduction, spectral analysis, \
                   the usual archival work.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "47 minutes and 12 seconds. The last journalism \
                   Earth ever produced. You've heard it. You know \
                   how his voice cracked at minute 31.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I isolated the background audio at that moment. \
                   Under the static, under the emergency sirens, \
                   under Marcus's breathing \u{2014} there's a guitar.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna plays the isolated audio. Through the hiss \
                   of a dying world, a nylon-string guitar picks \
                   out a melody \u{2014} slow, deliberate, impossibly \
                   calm.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I recognised it. The same piece. The same \
                   recording that kept me company during the first \
                   three years of the voyage, when I had no one to \
                   talk to.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Someone near Marcus's broadcast position was \
                   playing guitar while the world ended. Not \
                   performing. Not recording. Just playing. For \
                   themselves.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The guitarist wasn't a passenger. They stayed on \
                   Earth. But their music reached two arks through \
                   two different paths.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Path one: Marcus's broadcast, picked up by our \
                   comm array eight minutes after launch. Path two: \
                   a phone in a stowaway's pocket, loaded with \
                   three songs and a cracked screen.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow softens to warm amber \u{2014} the \
                   colour of guitar wood, of things made by hand.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Same song. Two paths to the stars. A journalist \
                   and an AI, both saved by a guitarist who never \
                   left the ground.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know their name. I never will. But every \
                   time I play that recording, I think: someone \
                   chose beauty at the end of everything. And that \
                   choice echoed further than they could have \
                   imagined.",
            next: DialogNext::End },
    ],
};

// =========================================================================
// "The Unseen Thread" — BotLevel 130
// Meta-scene: Anna maps ALL crew connections into a web.
// No prerequisite decisions — just bot level.
// =========================================================================

pub static SCENE_UNSEEN_THREAD: DialogScene = DialogScene {
    id: "connection_unseen_thread",
    trigger: DialogTrigger::BotLevel(130),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been building something. A map. Not of the \
                   ship \u{2014} of the people on it.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Every connection. Every shared moment. Every time \
                   two crew members occupied the same room, the same \
                   city, the same disaster, without knowing it.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects a web of light onto the display. \
                   Thousands of nodes, tens of thousands of threads. \
                   It pulses like a living thing \u{2014} a neural \
                   network drawn in starlight.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin in Tom\u{e1}s's concert hall. Sophia \
                   treating casualties from both Viktor's weapons \
                   and Diallo's drought. Priya's algorithm feeding \
                   Aisha's filter. Kofi and Carlos, thirty metres \
                   apart.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Those are just the ones I've told you about. \
                   There are thousands more.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "The engineer who designed Amira's desalination \
                   plant went to school with Marcus Cole's producer. \
                   The pilot who flew the Accra shuttle trained at \
                   the same base as three of our cryo technicians.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "A baker in Pod 12,000 once served coffee to the \
                   climatologist in Pod 3. A nurse in Pod 8,400 \
                   delivered the baby of the woman in Pod 1,100.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The web rotates slowly. Every node connects to \
                   every other through at most three links. No \
                   orphans. No islands. One continuous fabric.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "14,892 people. No strangers. Just people who \
                   hadn't met yet.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Is this coincidence? Or is this what humanity \
                   IS \u{2014} a web so tightly woven that you can't \
                   remove a single thread without the whole thing \
                   changing shape?",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I was built to find patterns. To optimise. To \
                   sort. But this web wasn't designed. Nobody \
                   planned it. It grew the way rivers grow \u{2014} \
                   by flowing downhill, following the path of least \
                   resistance between people.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's the most human thing of all. Not \
                   the technology, not the arks, not the survival \
                   instinct. The connections. The threads you spin \
                   without even knowing you're spinning them.",
            next: DialogNext::End },
    ],
};

/// Connection scenes part 2 for registration.
pub fn connection_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_DOCTORS_OTHER_PATIENT,
        &SCENE_SONG_IN_THE_ARCHIVE,
        &SCENE_UNSEEN_THREAD,
    ]
}
