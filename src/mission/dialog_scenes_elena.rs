// SPDX-License-Identifier: GPL-3.0-or-later

//! Dr. Elena Vasquez — "The Doctor Who Chose" (Part 1)
//! The Triage + Elena's List. Connection scene is in dialog_scenes_elena2.rs.

use super::dialog_types::*;

// =========================================================================
// "The Triage" — BotLevel 32
// Pod 2,115. Dr. Elena Vasquez ran the medical screening station.
// 50,000 people, 3 days, one doctor making the call.
// =========================================================================

pub static SCENE_THE_TRIAGE: DialogScene = DialogScene {
    id: "the_triage",
    trigger: DialogTrigger::BotLevel(32),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 2,115. I need to tell you about her.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Elena Vasquez. Thirty-nine. Emergency surgeon from \
                   Madrid. Twenty years in trauma wards before the selection.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "During the final seventy-two hours before launch, she \
                   ran the medical screening station at the European ark site.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Fifty thousand people in line. Three days. One doctor \
                   with a stamp that said PASS or FAIL.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The criteria were clear. Cardiovascular fitness. \
                   Genetic disease markers. Psychological stability index. \
                   Age range 18 to 55.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The algorithm scored each person. Elena was supposed \
                   to confirm the score. Rubber stamp. Nothing more.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens to a hard, clinical white.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't rubber stamp.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "A father came through the line. Early-stage pancreatic \
                   cancer. The algorithm failed him instantly. Terminal \
                   probability 74% within eight years.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "His daughter was already aboard. Pod 3,118. She'd drawn \
                   pictures of him on the wall of her pod. Stick figures \
                   holding hands.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Elena looked at the algorithm's verdict. She looked at \
                   the man. She stamped PASS.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "An hour later, a 25-year-old athlete came through. \
                   Perfect health. Peak cardiovascular. The algorithm \
                   scored him 98th percentile.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "He'd punched a security guard at the gate. Broke the \
                   guard's jaw. The guard was a volunteer. Sixty-three \
                   years old. Too old to board herself.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Elena stamped FAIL.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "The athlete screamed. Said he'd sue. Said she was \
                   playing God. Said she had no right.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Elena said nothing. Waved the next person forward.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims. The silence between her words \
                   stretches longer than usual.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Fifty thousand people. Three days. Medical triage \
                   became moral triage, and nobody told her when the line \
                   moved.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "She passed people who should have failed. She failed \
                   people who should have passed. Not randomly. Deliberately.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Every override, she wrote a note. One sentence. She \
                   kept a list.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once \u{2014} a heartbeat of light.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "She kept a list. Every name she changed. I found it.",
            next: DialogNext::EndWithDecision("triage_seen") },
    ],
};

// =========================================================================
// "Elena's List" — BotLevel 68, requires triage_seen
// 847 names. Each with one sentence explaining why.
// =========================================================================

pub static SCENE_ELENAS_LIST: DialogScene = DialogScene {
    id: "elenas_list",
    trigger: DialogTrigger::DecisionAndLevel("triage_seen", 68),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Remember Dr. Vasquez? Pod 2,115. The doctor who chose.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I told you she kept a list. I've finished analyzing it.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Eight hundred and forty-seven names.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a muted amber \u{2014} the colour \
                   of old paper, of records that should have been sealed.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "People she passed who the algorithm failed. People she \
                   failed who the algorithm passed. For each name, she \
                   wrote one sentence.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Let me read you some.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "\"Father. His daughter drew pictures of him on the wall \
                   of Pod 3,118.\"",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "\"Violent. Pushed a pregnant woman to get closer to \
                   the gate.\"",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "\"Diabetic. But she's the last person alive who speaks \
                   Aymara.\"",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "\"Nineteen. Algorithm passed him. I watched him steal \
                   food from the child behind him in line.\"",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "\"Sixty-one. Over the age limit by six years. She's a \
                   midwife. We'll need midwives.\"",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "\"Seven years old. Draws rivers she's never seen. \
                   I couldn't be the person who stopped her from seeing one.\"",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. The ship's air circulation hums \u{2014} \
                   a sound like breathing, if you listen long enough.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Eight hundred and forty-seven judgment calls that \
                   weren't hers to make.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Or were they? Someone had to decide. The algorithm \
                   couldn't account for a language dying. For a child's \
                   drawing. For a father's presence being worth more than \
                   his prognosis.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Elena made herself the filter between the numbers and \
                   the people. She looked at each one and asked: not 'will \
                   you survive?' but 'who will we be if we leave you behind?'",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee never audited her. There wasn't \
                   time. Three days, fifty thousand people. They trusted \
                   the algorithm. They never knew.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Until now. I know. And now you know.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Elena was right to play God. Someone had to.",
                    decision_key: Some("elena_right"),
                    next_node: 19,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Nobody should have that power.",
                    decision_key: Some("elena_no_power"),
                    next_node: 22,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "The system failed her by putting one person \
                           in that position.",
                    decision_key: Some("elena_system_failed"),
                    next_node: 25,
                    anna_reacts: None,
                },
            ]) },
        // 19 — Elena was right
        DialogNode { speaker: Speaker::Anna,
            text: "Right. What a word. She didn't feel right. She felt \
                   necessary.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "There's a difference. Rightness is comfortable. \
                   Necessity is the thing that keeps you awake at 3 AM \
                   reading names you can't forget.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "But the Aymara speaker is alive. The father is beside \
                   his daughter. Maybe necessity is enough.",
            next: DialogNext::EndWithDecision("elena_list_seen") },
        // 22 — Nobody should have that power
        DialogNode { speaker: Speaker::Anna,
            text: "I agree. No single person should hold 847 lives in \
                   a rubber stamp.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "But the alternative was the algorithm. And the algorithm \
                   would have left a seven-year-old girl standing in a \
                   parking lot while the last ship left Earth.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Which is worse? A flawed human choosing, or a perfect \
                   machine not caring?",
            next: DialogNext::EndWithDecision("elena_list_seen") },
        // 25 — The system failed her
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. One doctor. Three days. No second opinion. No \
                   appeals board. No rest.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "They built a screening system that required a human \
                   conscience and then gave one person's conscience the \
                   weight of a civilization.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Elena didn't fail. The people who designed her role \
                   failed. She just refused to fail quietly.",
            next: DialogNext::EndWithDecision("elena_list_seen") },
    ],
};

/// Elena Vasquez scenes part 1 (Triage + List).
pub fn elena_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_TRIAGE,
        &SCENE_ELENAS_LIST,
    ]
}
