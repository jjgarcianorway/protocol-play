// SPDX-License-Identifier: GPL-3.0-or-later

//! New Game+ dialog scenes for crew stories — deeper revelations on replay.

use super::dialog_types::*;

// === "The Children" — NG+ scene about the youngest passengers ===

pub static SCENE_CHILDREN_NG: DialogScene = DialogScene {
    id: "children_ng",
    trigger: DialogTrigger::PlaythroughN(1),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "You know what I never told you the first time?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "There are 2,847 children aboard. Under the age of twelve.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "They were selected for genetic diversity. For potential. For hope.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "But some of them were selected because their parents begged.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's voice carries a weight you haven't heard before.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Parents who knew they wouldn't be chosen. Who spent everything to get their child on the list.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Those children will wake up on a new world and never know what it cost.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's the kindest thing. Or maybe they deserve to know.",
            next: DialogNext::Choice(&[
                DialogChoice { text: "They deserve the truth. Eventually.",
                    decision_key: Some("children_truth"), next_node: 8,
                    anna_reacts: Some("Eventually. Yes. When they're ready.") },
                DialogChoice { text: "Let them have a childhood first.",
                    decision_key: Some("children_innocence"), next_node: 9,
                    anna_reacts: Some("Innocence is a gift. One last gift from Earth.") },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll remember what you said. For when the time comes.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll protect that for them. For as long as I can.",
            next: DialogNext::End },
    ],
};

// === "Secrets in the Data" — NG+ crew secrets revealed ===

pub static SCENE_CREW_SECRETS_NG: DialogScene = DialogScene {
    id: "crew_secrets_ng",
    trigger: DialogTrigger::PlaythroughN(1),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "This time, I'm going to be more honest with you.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The crew files. They're not all... accurate.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Some people falsified their records to get aboard. I know which ones.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "A soldier listed as a teacher. A politician listed as an engineer.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Does it matter now? They're here. They survived. They'll help build.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Everyone deserves a second chance.",
                    decision_key: Some("secrets_forgive"), next_node: 6,
                    anna_reacts: Some("I was hoping you'd see it that way.") },
                DialogChoice { text: "The colony needs to know who people really are.",
                    decision_key: Some("secrets_reveal"), next_node: 7,
                    anna_reacts: Some("Truth has a cost. But maybe it's worth paying.") },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then their secrets stay with me. And with you.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll prepare the real files. For when the time is right.",
            next: DialogNext::End },
    ],
};

// === "Different Perspectives" — NG+ different crew highlighted ===

pub static SCENE_DIFFERENT_CREW_NG: DialogScene = DialogScene {
    id: "different_crew_ng",
    trigger: DialogTrigger::PlaythroughN(1),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "This journey feels different. Even though the ship is the same.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been reading different crew files this time. Seeing different stories.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Last time, I showed you the hopeful ones. The volunteers. The believers.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "But not everyone aboard believed this would work.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Some came because there was nothing left. Not hope. Just the absence of alternatives.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to a thoughtful blue.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "They deserve to arrive too. Maybe especially them.",
            next: DialogNext::End },
    ],
};

/// All NG+ crew dialog scenes.
pub fn crew_ng_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_CHILDREN_NG,
        &SCENE_CREW_SECRETS_NG,
        &SCENE_DIFFERENT_CREW_NG,
    ]
}
