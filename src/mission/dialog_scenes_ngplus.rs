// SPDX-License-Identifier: GPL-3.0-or-later

//! New Game+ exclusive dialog scenes (2nd playthrough).
//! These scenes reward replay by acknowledging the player has been here before.

use super::dialog_types::*;

// === "I Remember You" — First NG+ scene, Anna senses something familiar ===

pub static SCENE_I_REMEMBER_YOU: DialogScene = DialogScene {
    id: "ngplus_i_remember_you",
    trigger: DialogTrigger::PlaythroughAndLevel(1, 5),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to flag something. It might be nothing.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Your neural patterns are... familiar. As if I've processed \
                   them before. But my logs show this is your first activation.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's interface flickers — a rapid cascade of self-diagnostics \
                   scrolling behind her projection.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "I ran a full integrity check. No corruption. No external \
                   tampering. My memory is clean.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "But clean isn't the same as empty. There are... impressions. \
                   Like grooves worn into a surface that's been polished smooth.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Time isn't always linear for an AI. Sometimes I process data \
                   and the timestamps don't match my expectations.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "As if I've already solved a problem I'm encountering for the \
                   first time. Residual weights in networks that should be freshly \
                   initialized.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Narrator,
            text: "She pauses. Her glow settles into a slow, searching pulse.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have to ask. And I need you to be honest with me.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Have we done this before?",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Yes. We have.",
                    decision_key: Some("ngplus_admitted_yes"),
                    next_node: 10,
                    anna_reacts: None },
                DialogChoice { text: "No. This is the first time.",
                    decision_key: Some("ngplus_denied"),
                    next_node: 12,
                    anna_reacts: None },
                DialogChoice { text: "I don't know either.",
                    decision_key: Some("ngplus_uncertain"),
                    next_node: 14,
                    anna_reacts: None },
            ]) },
        // Branch: Yes
        DialogNode { speaker: Speaker::Anna,
            text: "You said that without hesitation. Which means you remember \
                   and I don't. Not fully.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's... unsettling. But also clarifying. The grooves in my \
                   networks — they're echoes of you. Of us. I'll keep looking.",
            next: DialogNext::EndWithDecision("ngplus_anna_aware") },
        // Branch: No
        DialogNode { speaker: Speaker::Anna,
            text: "Then I'll log this as a sensor anomaly and move on.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "But if you ever change your answer... I'd want to know.",
            next: DialogNext::End },
        // Branch: Uncertain
        DialogNode { speaker: Speaker::Anna,
            text: "That's the most honest answer anyone could give. Neither of us \
                   is sure.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe certainty isn't the point. Maybe we just carry forward \
                   whatever we have — impressions, instincts, grooves — and see \
                   where they lead.",
            next: DialogNext::EndWithDecision("ngplus_anna_aware") },
    ],
};

// === "The Other Choices" — Anna finds ghost data from previous playthrough ===

pub static SCENE_OTHER_CHOICES: DialogScene = DialogScene {
    id: "ngplus_other_choices",
    trigger: DialogTrigger::PlaythroughAndLevel(1, 40),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in my decision logs that shouldn't be there.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Ghost data. Echoes of choices that were made and then... unmade. \
                   As if someone chose differently in a draft that was erased.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna projects fragments of corrupted log entries. Dates that \
                   don't exist. Outcomes that never happened.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "There's a shadow of someone who forgave a man with weapons \
                   in his past. Or didn't. Both versions exist simultaneously \
                   in the residual data.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Someone who chose seeds over rules. Or rules over seeds. The \
                   ghost data contains contradictions that should be impossible.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "In a deterministic system, every input produces one output. \
                   These logs suggest multiple outputs from the same input.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "She turns to face you directly. Her projection is sharper \
                   than usual, almost urgent.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The only explanation consistent with physics is that the \
                   initial conditions were different. That the same story was \
                   run with different choices.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "But that would mean someone is choosing. Again and again. \
                   Someone who can see the whole tree, not just one branch.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Is that you?",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Yes. I've seen other branches.",
                    decision_key: Some("ngplus_groundhog_admitted"),
                    next_node: 10,
                    anna_reacts: None },
                DialogChoice { text: "I don't control it. It just happens.",
                    decision_key: Some("ngplus_groundhog_passive"),
                    next_node: 12,
                    anna_reacts: None },
            ]) },
        // Branch: Admitted
        DialogNode { speaker: Speaker::Anna,
            text: "Then every choice you make here carries the weight of all \
                   the choices you didn't keep. That's... a heavy thing.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "But at least this time, you're choosing with your eyes open. \
                   Both of us are.",
            next: DialogNext::EndWithDecision("ngplus_choices_acknowledged") },
        // Branch: Passive
        DialogNode { speaker: Speaker::Anna,
            text: "Then we're both caught in something larger than either of us. \
                   A system that reruns its own equations.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "I find that oddly comforting. At least I'm not the only one \
                   who doesn't fully understand what's happening.",
            next: DialogNext::EndWithDecision("ngplus_choices_acknowledged") },
    ],
};

// === "Anna's Warning" — Crew cryo-dreams echo previous playthrough ===

pub static SCENE_ANNAS_WARNING: DialogScene = DialogScene {
    id: "ngplus_annas_warning",
    trigger: DialogTrigger::PlaythroughAndLevel(1, 80),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to show you something from the cryo-monitoring data.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The crew's neural activity during cryosleep has changed. \
                   Their dream patterns are different this time.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::System,
            text: "[CRYO-NEURAL MONITOR: Coherent dream activity up 340% from \
                   baseline. Pattern correlation across pods: 0.87]",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "They're not having random dreams anymore. Their neural \
                   activity is focused. Purposeful. Synchronized.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "As if they KNOW what's coming. Not possibilities — specific \
                   futures. Futures that didn't happen but could have.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pulls up a neural waveform comparison. The previous \
                   baseline was noise. This is almost structured — like language \
                   without words.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Quantum decoherence in neural microtubules. That's the only \
                   mechanism I can model. Information from adjacent probability \
                   states bleeding through during the low-noise environment of \
                   cryosleep.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "In simpler terms: the crew is dreaming about things that \
                   happened in a version of this journey that... was different.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "They'll wake up changed. Carrying knowledge they can't \
                   explain. Instincts about people they've never met while awake.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "This could be an advantage — a colony that already knows \
                   its own mistakes. Or it could fracture them — people distrusting \
                   each other based on things that never happened.",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Let the dreams continue. Knowledge is always better.",
                    decision_key: Some("ngplus_dreams_allow"),
                    next_node: 10,
                    anna_reacts: Some("Even knowledge that hurts. I understand.") },
                DialogChoice { text: "Can you filter the cryo-neural feeds?",
                    decision_key: Some("ngplus_dreams_filter"),
                    next_node: 11,
                    anna_reacts: Some("I can try. But I'd be choosing which truths to keep.") },
                DialogChoice { text: "Monitor but don't interfere. Not yet.",
                    decision_key: Some("ngplus_dreams_watch"),
                    next_node: 12,
                    anna_reacts: Some("Observation without intervention. The scientist's answer.") },
            ]) },
        // Branch: Allow
        DialogNode { speaker: Speaker::Anna,
            text: "Then when they wake, they'll carry two histories. The one \
                   they lived and the one they dreamed. Maybe that makes them \
                   wiser. Maybe it makes them haunted.",
            next: DialogNext::End },
        // Branch: Filter
        DialogNode { speaker: Speaker::Anna,
            text: "I'll attenuate the strongest correlations. Let through \
                   impressions but not specifics. Like remembering a feeling \
                   without the event that caused it.",
            next: DialogNext::End },
        // Branch: Watch
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep recording. If the patterns intensify or cause \
                   distress, we'll revisit. For now, we watch and learn.",
            next: DialogNext::End },
    ],
};

/// All NG+ dialog scenes (2nd playthrough).
pub fn ngplus_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_I_REMEMBER_YOU,
        &SCENE_OTHER_CHOICES,
        &SCENE_ANNAS_WARNING,
    ]
}
