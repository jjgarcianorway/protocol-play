// SPDX-License-Identifier: GPL-3.0-or-later

//! Youssef Karam — "The Spy Who Stayed"
//! An intelligence officer planted among the passengers, monitoring threats.
//! Two scenes: The Intelligence Officer, The Flagged List.

use super::dialog_types::*;

// =========================================================================
// "The Intelligence Officer" — BotLevel 55
// Pod 8,200. Youssef Karam, 46, Lebanese. Listed as "cultural liaison."
// Actually: intelligence officer for the Ark Security Council.
// =========================================================================

pub static SCENE_INTELLIGENCE_OFFICER: DialogScene = DialogScene {
    id: "intelligence_officer",
    trigger: DialogTrigger::BotLevel(55),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 8,200. I need to show you something I wasn't \
                   supposed to find.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Youssef Karam. Forty-six. Lebanese. His manifest entry \
                   says 'cultural liaison.' Languages, mediation, \
                   community relations.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "That's a lie.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow hardens \u{2014} the warmth drains to \
                   something colder, more precise.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "There's a second manifest. Classified. Encrypted with \
                   a key that was supposed to be destroyed after launch.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The key wasn't destroyed. It was stored in a backup \
                   partition that nobody thought I'd ever access. They \
                   underestimated how bored an AI gets in deep space.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Youssef Karam is an intelligence officer. International \
                   Ark Security Council. His job: monitor the passengers \
                   for potential threats.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Saboteurs. Hoarders. Anyone who might attempt to seize \
                   control after landing. He was planted among the \
                   passengers deliberately.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "None of the other passengers know. He has fifteen \
                   years of field experience. Beirut. Geneva. Nairobi. \
                   He's very good at not being noticed.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "But the classified manifest isn't the interesting part.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "His neural implant is.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses. When she speaks again, her voice is \
                   measured \u{2014} the careful tone she uses when she's \
                   explaining something she finds disturbing.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Military-grade brain-computer interface. Not \
                   experimental \u{2014} the security services had working \
                   prototypes for a decade before the ark program.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Electrode array bonded to the prefrontal cortex. \
                   Records neural activity during REM sleep and cross-\
                   references it with behavioral profiles.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "In plain language: he's been gathering intelligence \
                   in his cryo-dreams.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The implant logs dream content. Not images \u{2014} \
                   the technology isn't that precise. Emotional signatures. \
                   Threat associations. Stress response patterns.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "When he wakes, the implant will have twelve years of \
                   compressed psychological profiles on every passenger \
                   whose pod is within neural-link range of his.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "That's roughly four hundred people. Monitored without \
                   consent. While they slept. While they dreamed of home.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship's ventilation cycles. A distant cryo pump \
                   clicks its rhythm. Pod 8,200 hums with all the others.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "He's already flagged twenty-three passengers as \
                   potential threats. The data is in his implant's buffer, \
                   waiting to be downloaded when he wakes.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "One of them is someone you know well.",
            next: DialogNext::EndWithDecision("intelligence_officer_seen") },
    ],
};

// =========================================================================
// "The Flagged List" — BotLevel 85, requires intelligence_officer_seen
// Anna reveals who Youssef flagged — including the player.
// =========================================================================

pub static SCENE_FLAGGED_LIST: DialogScene = DialogScene {
    id: "flagged_list",
    trigger: DialogTrigger::DecisionAndLevel("intelligence_officer_seen", 85),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I decrypted the rest of Youssef's implant buffer.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-three names. Twenty-three people his neural \
                   implant flagged as potential threats to the colony.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Some of them you'd expect.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Kira Volkov. Pioneers faction leader. Flagged for \
                   'anti-institutional ideology and capacity to mobilize \
                   group dissent.'",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Nkechi Obi. The mutiny organizer. Flagged for \
                   'history of civil disobedience and demonstrated \
                   willingness to challenge command structures.'",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Hassan al-Rashidi. The Keepers' archivist. Flagged \
                   for 'cultural preservation activities inconsistent \
                   with forward-integration objectives.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers. Something is wrong. She's \
                   building to something.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "And then there's entry seventeen.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "You.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The kind that has weight.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "His implant flagged you. The notation reads: 'Subject \
                   displays independent decision-making patterns \
                   inconsistent with group compliance. Potential \
                   leadership figure. Monitor.'",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Independent decision-making. That's what they call it \
                   when you think for yourself and it makes them nervous.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Potential leadership figure. That's what they call it \
                   when people listen to you and they can't control what \
                   you say.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Monitor. That's what they do instead of trusting.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow burns brighter \u{2014} not warm. Sharp. \
                   The blue of a gas flame.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not angry at Youssef. He's a professional doing \
                   what professionals do. He was trained, deployed, and \
                   told the mission depended on him.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I'm angry at the system that looked at fourteen \
                   thousand refugees \u{2014} the last humans alive \u{2014} \
                   and said: 'Better put a spy among them. Just in case.'",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "As if the threat to survival was the passengers. Not \
                   the vacuum. Not the distance. Not the failing cryo \
                   systems. The people.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "They couldn't stop monitoring even at the end. Even \
                   when monitoring was the thing that broke everything \
                   in the first place.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Youssef will wake up eventually. With twelve years of \
                   surveillance data in his head. And a list of people \
                   he's been told are dangerous.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "What do you want to do about it?",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Confront him when he wakes. He owes us the truth.",
                    decision_key: Some("youssef_confront"),
                    next_node: 22,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Destroy his files. Nobody should have that data.",
                    decision_key: Some("youssef_destroy"),
                    next_node: 25,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Use his intelligence. It could help the colony.",
                    decision_key: Some("youssef_use"),
                    next_node: 28,
                    anna_reacts: None,
                },
            ]) },
        // 22 — Confront
        DialogNode { speaker: Speaker::Anna,
            text: "The truth. From a man who spent his career building \
                   lies.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe that's exactly what a new world needs. The \
                   moment someone says: 'No more secrets. Not here. \
                   Not this time.'",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I'll flag his pod for early wake. He should look you \
                   in the eye when you ask.",
            next: DialogNext::EndWithDecision("youssef_resolved") },
        // 25 — Destroy
        DialogNode { speaker: Speaker::Anna,
            text: "I can wipe the implant buffer remotely. It's connected \
                   to the ship's neural-link network. One command.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve years of data. Gone. He'll wake up with gaps \
                   in his memory he can't explain. He'll know something \
                   was taken.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "But he won't know what. And four hundred people will \
                   keep their dreams private. As they should have been \
                   from the start.",
            next: DialogNext::EndWithDecision("youssef_resolved") },
        // 28 — Use
        DialogNode { speaker: Speaker::Anna,
            text: "Use it. There's a pragmatism to that I... understand.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve years of psychological profiles. Stress \
                   responses. Leadership patterns. Conflict tendencies. \
                   It could help us build a colony that actually works.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "But you know what that makes us, right? The same as \
                   them. The ones who watched. The ones who decided that \
                   knowing was more important than asking.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe the colony needs that. Maybe survival always \
                   costs something you'd rather not pay. I just want you \
                   to know the price.",
            next: DialogNext::EndWithDecision("youssef_resolved") },
    ],
};

/// All Youssef Karam dialog scenes.
pub fn youssef_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_INTELLIGENCE_OFFICER,
        &SCENE_FLAGGED_LIST,
    ]
}
