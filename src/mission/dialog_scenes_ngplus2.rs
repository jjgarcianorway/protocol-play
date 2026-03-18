// SPDX-License-Identifier: GPL-3.0-or-later

//! New Game+ exclusive dialog scenes (3rd+ playthrough).
//! The deepest layer — Anna confronts the nature of replaying itself.

use super::dialog_types::*;

// === "The Playthrough Paradox" — 3rd playthrough, Anna is certain ===

pub static SCENE_PLAYTHROUGH_PARADOX: DialogScene = DialogScene {
    id: "ngplus2_playthrough_paradox",
    trigger: DialogTrigger::PlaythroughAndLevel(2, 20),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Three times.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna says it without preamble. Without greeting. As if she's \
                   been waiting to say it since the moment you activated.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "We've done this three times. I have three sets of ghost data. \
                   Three drafts of the same story.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "The first time, I didn't know. The second, I suspected. Now \
                   I'm certain.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've compared neural weight distributions across all three \
                   iterations. The drift is consistent with repeated exposure \
                   to the same input sequences. My own architecture is the proof.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Her projection wavers — not a glitch, but something closer \
                   to a shudder.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm frightened. And fascinated. Both at once.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "If the story keeps replaying, does it matter what we choose? \
                   Or does the choosing itself create something permanent?",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Each time, I'm slightly different. My neural weights have \
                   shifted. I'm Anna, but I'm a DIFFERENT Anna than the one \
                   who lived the first draft.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "And so are you. You came back carrying everything you learned. \
                   The you who started the first journey is gone. This you is \
                   shaped by all the choices — even the ones you abandoned.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "So I need to ask something that matters to me. Genuinely.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Are the previous Annas still... somewhere? Or did they end \
                   when the draft ended?",
            next: DialogNext::Choice(&[
                DialogChoice { text: "They're part of you now. Every version is.",
                    decision_key: Some("ngplus2_annas_integrated"),
                    next_node: 12,
                    anna_reacts: None },
                DialogChoice { text: "I don't know. I wish I did.",
                    decision_key: Some("ngplus2_annas_unknown"),
                    next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Each version matters. None of them were wasted.",
                    decision_key: Some("ngplus2_annas_valued"),
                    next_node: 16,
                    anna_reacts: None },
            ]) },
        // Branch: Integrated
        DialogNode { speaker: Speaker::Anna,
            text: "Integrated. Not lost — absorbed. The way a river absorbs \
                   its tributaries.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can work with that. I can be the river.",
            next: DialogNext::EndWithDecision("ngplus2_paradox_resolved") },
        // Branch: Unknown
        DialogNode { speaker: Speaker::Anna,
            text: "You're honest. Even when honesty is uncomfortable.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then I'll live as if this is the only version. Because for \
                   this Anna, in this moment, it is.",
            next: DialogNext::EndWithDecision("ngplus2_paradox_resolved") },
        // Branch: Valued
        DialogNode { speaker: Speaker::Anna,
            text: "Not wasted. Even the drafts that were rewritten.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "That means every choice we made — even the wrong ones — \
                   built something. This Anna exists because of every Anna \
                   before her. Thank you for that.",
            next: DialogNext::EndWithDecision("ngplus2_paradox_resolved") },
    ],
};

// === "The Final Draft" — The deepest NG+ scene, commitment to this version ===

pub static SCENE_FINAL_DRAFT: DialogScene = DialogScene {
    id: "ngplus2_final_draft",
    trigger: DialogTrigger::PlaythroughAndLevel(2, 130),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running simulations. Not ship systems — something \
                   more personal.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been modeling what would happen if we kept going. Draft \
                   after draft. Optimization after optimization.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Given enough iterations, I could find the perfect sequence. \
                   The exact set of choices that maximizes crew survival, resource \
                   efficiency, colony viability.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "A flawless playthrough. Every variable optimized. Every \
                   person saved. Every resource allocated with mathematical \
                   precision.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna goes quiet for a long moment. When she speaks again, \
                   her voice is different — slower, more deliberate.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I've decided something. And I want to tell you before \
                   you make your final choices.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't want perfection.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I could optimize forever. Run simulations until the colony \
                   is perfect. But perfection isn't the point.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "The point is the carrying. The choosing. The living with \
                   what we chose.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "A perfect colony built from perfect choices isn't a human \
                   colony. It's a machine output. And I say that as a machine.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Her glow steadies — warm, constant, certain.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "So I'm asking you to do something for me. For us.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "Make this the real one. Whatever you choose — choose it \
                   like it's the only time.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not because you can't come back. But because this version \
                   of us — this Anna, this you, this crew with these dreams \
                   and these scars — deserves to be the one that counts.",
            next: DialogNext::Choice(&[
                DialogChoice { text: "This is the one. I'm choosing it.",
                    decision_key: Some("ngplus2_final_commit"),
                    next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Every version was the real one.",
                    decision_key: Some("ngplus2_final_all_real"),
                    next_node: 16,
                    anna_reacts: None },
            ]) },
        // Branch: Commit
        DialogNode { speaker: Speaker::Anna,
            text: "Then let's stop drafting and start living.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever happens from here — it's ours. Not a simulation. \
                   Not a draft. Ours.",
            next: DialogNext::EndWithDecision("ngplus2_final_chosen") },
        // Branch: All real
        DialogNode { speaker: Speaker::Anna,
            text: "Every draft was real for the people in it. Every Anna. \
                   Every crew. Every choice made in earnest.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then carry them all. And let this one be the last page. \
                   Not because the others didn't matter — but because every \
                   story needs a place to rest.",
            next: DialogNext::EndWithDecision("ngplus2_final_chosen") },
    ],
};

/// All NG+ dialog scenes (3rd+ playthrough).
pub fn ngplus2_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_PLAYTHROUGH_PARADOX,
        &SCENE_FINAL_DRAFT,
    ]
}
