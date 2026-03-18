// SPDX-License-Identifier: GPL-3.0-or-later

//! Amira Hassan's multi-scene arc (part 1): "Amira's Daughter" + "The Selection File."
//! Scene 1 (amiras_water) lives in dialog_scenes_characters.rs.
//! Scene 4 (amira_wakes) lives in dialog_scenes_amira_arc2.rs.

use super::dialog_types::*;

/// "Amira's Daughter" — Leyla is designing rivers in cryo-sleep.
pub static SCENE_AMIRAS_DAUGHTER: DialogScene = DialogScene {
    id: "amiras_daughter",
    trigger: DialogTrigger::DecisionAndLevel("dialog_seen_amiras_water", 48),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Remember Amira Hassan? Pod 4,231. The hydrologist.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I've been monitoring Leyla. Pod 4,232. The daughter who draws rivers.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Something has changed.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — a ripple of blues, like light through moving water.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Her brain patterns during cryo-sleep... they're not what I expected from a seven-year-old.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The crayon rivers are gone. What's replaced them is — I need you to understand how strange this is.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She's not dreaming about rivers anymore. She's designing them.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Water flow calculations. Optimal tributary patterns. Erosion modeling over geological timescales.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "A seven-year-old girl is doing graduate-level hydrology in her sleep.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship's water recyclers hum — a distant, rhythmic pulse, like a heartbeat.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I've compared her neural patterns to Amira's published work. The mathematical structures are identical.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Not similar. Identical. As if Amira's fifteen years of research were compressed into a sleeping child's mind.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "My best theory: the cryo-environment, combined with Amira's genetic markers for spatial reasoning, unlocked something.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla's brain is building on her mother's work. Unconsciously. In dreams she can't control.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Her latest patterns show a complete watershed design for a planet with 40% more rainfall than Earth.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know what our target planet looks like. Nobody told her. She was seven.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The rainfall estimate in her dream design is within 3% of actual survey data.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's light holds perfectly still — the way it does when she's deciding how much to reveal.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's extraordinary. She could help design the colony.",
                    decision_key: Some("leyla_gift"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "That doesn't sound natural. Is cryo doing something to her?",
                    decision_key: Some("leyla_concern"), next_node: 22,
                    anna_reacts: None },
                DialogChoice { text: "Does Amira know?",
                    decision_key: Some("leyla_amira"), next_node: 25,
                    anna_reacts: None },
            ]) },
        // 19 — Gift path
        DialogNode { speaker: Speaker::Anna,
            text: "Extraordinary. Yes. That's one word for it.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Another word is 'unprecedented.' And unprecedented things make councils nervous.",
            next: DialogNext::Continue(28) },
        // 21 — (reserved for index alignment)
        DialogNode { speaker: Speaker::Anna,
            text: ".",
            next: DialogNext::End },
        // 22 — Concern path
        DialogNode { speaker: Speaker::Anna,
            text: "That's the question I've been losing sleep over. If I slept.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "The cryo system was designed for preservation. Not enhancement. But twelve years is a long time for a developing brain to dream.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know if this is a gift or a side effect. And I don't know which answer frightens me more.",
            next: DialogNext::Continue(28) },
        // 25 — Amira path
        DialogNode { speaker: Speaker::Anna,
            text: "Amira is asleep. She has no idea. She went into cryo believing her daughter dreamed in crayon.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "When she wakes up, she'll find out her seven-year-old has become the most advanced hydrologist on the ship.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not sure if that will make her proud or terrified.",
            next: DialogNext::Continue(28) },
        // 28 — Converge + cliffhanger
        DialogNode { speaker: Speaker::Anna,
            text: "There's something else about Leyla I haven't told you. Something about her selection file.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — once, sharply — then steadies. The silence that follows is deliberate.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "But not tonight. You have enough to process.",
            next: DialogNext::EndWithDecision("amira_daughter_seen") },
    ],
};

/// "The Selection File" — Leyla wasn't selected. She was never supposed to be here.
pub static SCENE_SELECTION_FILE: DialogScene = DialogScene {
    id: "amira_selection_file",
    trigger: DialogTrigger::DecisionAndLevel("amira_daughter_seen", 72),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I promised you the truth about Leyla's selection file. Here it is.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla Hassan was never selected for the ark.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "The words land like stones dropped into still water.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The selection program had an age minimum. Twelve years old. Leyla was seven.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The rationale was resource efficiency. Children under twelve consume resources but can't contribute to colony setup for years.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Amira was told. Explicitly. 'Your daughter does not qualify.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She boarded anyway. Alone. Walked past the security checkpoint with her credentials and nothing else.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The next morning, Pod 4,232 registered an occupant. Leyla Hassan. Age seven. Vitals stable.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody processed her. Nobody signed off. No boarding record, no security log, no manifest entry.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla simply... appeared.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens to a single point — bright, focused, the way it gets when she's angry. Or impressed.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I spent years analyzing the security footage. Going frame by frame through forty-seven camera feeds.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "And I found it.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Amira hacked the boarding system.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Not with brute force. Not with stolen credentials. She found a vulnerability in the pod allocation algorithm.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The same mind that designed water-sharing systems across three hostile nations found a way to share one more pod.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The hydrologist who couldn't convince politicians to share water... convinced a computer to share a seat.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. Somewhere deep in the ship, water moves through recycling pipes — Amira's element, even here.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Any parent would have done the same.",
                    decision_key: Some("selection_parent"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "She broke the rules. The selection existed for a reason.",
                    decision_key: Some("selection_rules"), next_node: 22,
                    anna_reacts: None },
                DialogChoice { text: "Does anyone else know?",
                    decision_key: Some("selection_secret"), next_node: 25,
                    anna_reacts: None },
            ]) },
        // 19 — Parent path
        DialogNode { speaker: Speaker::Anna,
            text: "Would they? Fourteen thousand people applied. Twelve thousand were rejected. Most of them had children too.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "The difference is Amira had the skill to break the system. Does having the ability make it right?",
            next: DialogNext::Continue(28) },
        // 21 — (reserved for index alignment)
        DialogNode { speaker: Speaker::Anna,
            text: ".",
            next: DialogNext::End },
        // 22 — Rules path
        DialogNode { speaker: Speaker::Anna,
            text: "The selection existed to maximize survival odds. Every seat calculated. Every skill balanced against every need.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla took someone else's place. Someone who qualified. Someone who was told yes and then — wasn't here.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "That person is still on Earth. Or was.",
            next: DialogNext::Continue(28) },
        // 25 — Secret path
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. Someone else knows.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "I found the trace because I look at everything. But I wasn't the first to find it.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Someone on the council found it years ago. And they've been sitting on it.",
            next: DialogNext::Continue(28) },
        // 28 — Converge + cliffhanger
        DialogNode { speaker: Speaker::Anna,
            text: "But the hack left a trace. And someone on the council found it.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. James Whitfield. Founders faction. He has the evidence sealed.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "He hasn't used it. Not yet. But sealed evidence is just leverage waiting for a reason.",
            next: DialogNext::EndWithDecision("selection_file_seen") },
    ],
};

/// Amira arc scenes from this file (part 1: scenes 2–3).
pub fn amira_arc_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_AMIRAS_DAUGHTER,
        &SCENE_SELECTION_FILE,
    ]
}
