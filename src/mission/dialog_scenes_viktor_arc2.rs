// SPDX-License-Identifier: GPL-3.0-or-later

//! Viktor Petrov arc part 2 — "The Witness": the collision between
//! weapon-maker and survivor, and the choice that defines the colony.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Witness" — BotLevel 118, requires redemption scene
// Dr. Sophia Marchand survived Viktor's weapons in Marseille.
// She's fifty meters from the man whose math killed her patients.
// ---------------------------------------------------------------------------
pub static SCENE_THE_WITNESS: DialogScene = DialogScene {
    id: "viktors_witness",
    trigger: DialogTrigger::DecisionAndLevel("viktor_redemption_seen", 118),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to talk to you about Pod 9,012. \
                   I've been putting this off for thirty-seven days.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Sophia Marchand. Forty-one years old. Emergency \
                   physician. Trained at Hôpital de la Timone in Marseille.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "On the day of the Mediterranean Exchange, she was \
                   running the triage unit at Marseille's Vieux-Port \
                   emergency center.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The blast hit at 4:17 AM. She was already on shift. \
                   Twelve-hour overnight. She'd just finished her third \
                   coffee.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flattens to a clinical white. The color \
                   of hospital lights. Of fluorescent tubes that never \
                   turn off.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The thermal pulse reached the hospital seven seconds \
                   after detonation. The windows shattered inward. \
                   She was standing behind a concrete pillar. That's why \
                   she lived.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Four hundred patients in seventy-two hours. Burns. \
                   Radiation sickness. Crush injuries. Children who \
                   couldn't tell her where it hurt because they couldn't \
                   stop screaming.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She triaged them. Green tag: survivable. Yellow tag: \
                   urgent. Red tag: critical. Black tag: beyond saving.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "She ran out of black tags by hour six. Started using \
                   strips torn from her own coat.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Three hundred and twelve of her four hundred patients \
                   died. She held the hands of ninety-one of them \
                   personally. She counted.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Like Viktor counted his names. They're the same kind \
                   of person, carrying the same kind of weight. \
                   From opposite sides of the same equation.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship creaks. Metal expanding, contracting. \
                   The sound of a vessel holding itself together.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Sophia doesn't know who designed the weapons. \
                   She knows someone did. She's said so in her \
                   pre-departure psychological evaluation.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Direct quote: 'I don't hate them. Hatred requires \
                   energy I spent on keeping people alive. But I want \
                   to look them in the eye and ask: did you see the \
                   children?'",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "She's in a cryo-pod fifty meters from the man \
                   whose math killed her patients. And when they wake \
                   up, they'll both be standing in the same colony.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor's carbon scrubber. Sophia's trauma surgery \
                   expertise. The colony needs both. Desperately.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "His engineering could prevent atmospheric poisoning. \
                   Her field medicine could save lives during the \
                   settlement's first years, when everything is dangerous \
                   and nothing is sterile.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "They could save each other. Professionally. \
                   Maybe personally. If the truth doesn't destroy \
                   them first.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow cycles through colors — amber, white, \
                   blue — as if she's running through futures, \
                   testing outcomes.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I've simulated this three thousand times. \
                   Different variables. Different contexts. \
                   There is no clean answer.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "So I'm asking you. When they wake up — and they \
                   will wake up, because we are going to get there — \
                   what do I do?",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Let them meet naturally. The truth finds its own way.",
                    decision_key: Some("witness_natural"),
                    next_node: 21,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Prepare them separately. They deserve to choose.",
                    decision_key: Some("witness_prepare"),
                    next_node: 25,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Keep the secret. Some truths only cause pain.",
                    decision_key: Some("witness_secret"),
                    next_node: 29,
                    anna_reacts: None,
                },
            ]) },
        // 21 — Natural path
        DialogNode { speaker: Speaker::Anna,
            text: "Natural. In a colony of fourteen thousand people \
                   where the nuclear engineer and the trauma surgeon \
                   will inevitably be assigned to the same infrastructure \
                   committee.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "He'll recognize her accent. Marseille French is \
                   distinctive. She'll see his hands shake when someone \
                   mentions the Mediterranean. They'll know.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's the honest way. No AI playing god with \
                   seating charts. Just two people in a room, carrying \
                   matching scars, figuring it out.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I'll step back. But I won't stop watching. Because \
                   if forgiveness is possible between these two, it's \
                   possible for all of us.",
            next: DialogNext::EndWithDecision("viktor_witness_resolved") },
        // 25 — Prepare path
        DialogNode { speaker: Speaker::Anna,
            text: "Prepare them. Give them the choice of knowing \
                   before the shock of recognition does it for them.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "I'd tell Viktor first. Show him Sophia's file. \
                   Let him see the hands that tried to undo what his \
                   math did. Let him feel that before he meets her.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Then Sophia. Not 'the man who built the bomb is here.' \
                   But: 'the man who built the bomb spent twelve years \
                   memorizing the names of everyone it killed, and then \
                   he designed something to make sure it never happens again.'",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "Context isn't forgiveness. But it's the ground \
                   forgiveness grows in. And these two are going to \
                   need a lot of ground.",
            next: DialogNext::EndWithDecision("viktor_witness_resolved") },
        // 29 — Secret path
        DialogNode { speaker: Speaker::Anna,
            text: "Keep the secret. Let Viktor be the reactor engineer \
                   and Sophia be the doctor, and let the Mediterranean \
                   stay buried in the space between their pods.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "I understand the logic. The colony needs them \
                   functional, not broken. Peace is more useful than \
                   truth.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "But secrets on a ship this small are pressure vessels. \
                   And I know something about what happens when \
                   containment fails.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep it. For now. But Viktor wakes at 4:17 \
                   every morning, and Sophia flinches at loud noises. \
                   The truth is already leaking through their bodies. \
                   It's only a matter of time.",
            next: DialogNext::EndWithDecision("viktor_witness_resolved") },
    ],
};

/// Viktor arc scenes (part 2: The Witness).
pub fn viktor_arc_scenes_2() -> Vec<&'static DialogScene> {
    vec![&SCENE_THE_WITNESS]
}
