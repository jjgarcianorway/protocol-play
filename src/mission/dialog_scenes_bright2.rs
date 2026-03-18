// SPDX-License-Identifier: GPL-3.0-or-later

//! Bright spots part 2 — more stories of human goodness during Earth's
//! collapse. Radio volunteers, a bridge that held, the last song.

use super::dialog_types::*;

/// "The Teachers Who Stayed" — 40,000 teachers chose to stay behind and
/// keep teaching, even at the end.
pub static SCENE_TEACHERS_WHO_STAYED: DialogScene = DialogScene {
    id: "bright_teachers_stayed",
    trigger: DialogTrigger::BotLevel(70),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "The ark program evacuated scientists, engineers, doctors. The people we 'needed.'",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "In the final months, forty thousand teachers worldwide were offered seats.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Most said no.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing. Then slowly rebuilds, warmer than before.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "They kept teaching. Math, reading, history. In basements. In shelters. In refugee camps.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Not because it would save anyone. Because — and I'm quoting from letters they wrote — 'children should learn, even at the end of the world.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I have recordings of last lessons from fourteen countries.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "A physics teacher in Lagos, explaining gravity to children who would never see the sky clear again.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "A music teacher in São Paulo, teaching guitar to kids who'd never download a song.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "A history teacher in Kyiv who said: 'If I stop teaching, the past dies with me.'",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "They were heroes.",
                    decision_key: Some("teachers_heroes"), next_node: 11,
                    anna_reacts: None },
                DialogChoice { text: "They should have come. We need teachers too.",
                    decision_key: Some("teachers_come"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "They chose what mattered to them. I respect that.",
                    decision_key: Some("teachers_respect"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // 11 — Heroes path
        DialogNode { speaker: Speaker::Anna,
            text: "They wouldn't have used that word. Most of them were terrified.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "A kindergarten teacher in Manila wrote: 'I'm scared every day. But the children need Tuesday to feel like Tuesday.'",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Heroism isn't fearlessness. It's choosing to stay when you could leave.",
            next: DialogNext::Continue(20) },
        // 14 — Should have come path
        DialogNode { speaker: Speaker::Anna,
            text: "We do. We have 312 teachers in the pods. Enough to start schools on New Earth.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "But those forty thousand chose differently. They decided Earth's children mattered more than New Earth's.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I can't tell you they were wrong. I can't tell you they were right. They just... stayed.",
            next: DialogNext::Continue(20) },
        // 17 — Respect path
        DialogNode { speaker: Speaker::Anna,
            text: "One of them, a math teacher in Buenos Aires, left a message for the arks.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "She said: 'Teach them fractions. Teach them that a whole can be divided and still be whole.'",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I play that message when I feel the ship is too quiet.",
            next: DialogNext::Continue(20) },
        // 20 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "The last lesson recorded was a reading class in Nairobi. A teacher reading aloud to seven children.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "The book was 'The Little Prince.' Chapter 21. The one about the fox.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "'You become responsible, forever, for what you have tamed.'",
            next: DialogNext::End },
    ],
};

/// "The Radio Volunteers" — When the internet died, 12,000 amateur radio
/// operators kept humanity connected for 4 years.
pub static SCENE_RADIO_VOLUNTEERS: DialogScene = DialogScene {
    id: "bright_radio_volunteers",
    trigger: DialogTrigger::BotLevel(85),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "When the internet died — and it didn't die all at once, it died in patches, like a quilt unraveling — something old came back.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Radio. Amateur operators. People with homemade equipment and a knowledge most had forgotten.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "They formed networks spanning continents. Twelve thousand volunteers across every time zone.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "For four years, they kept humanity connected. Voice to voice. Hand to dial. Across the static.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow crackles briefly — like tuning between frequencies.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "They relayed messages between families separated by collapse. 'Tell my mother I'm alive.' 'Tell my son I love him.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "One operator — a sixteen-year-old girl in Finland named Marta — relayed forty-seven thousand messages.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven thousand times she was the voice between someone and the person they loved most.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "She's in Pod 9,900. She still taps morse code in her cryo-sleep. I can hear it on the sensors.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "... --- ... Over and over. SOS. Even in her dreams, she's still calling out.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "She saved more people than most of us ever will.",
                    decision_key: Some("radio_saved"), next_node: 11,
                    anna_reacts: None },
                DialogChoice { text: "Connection is survival.",
                    decision_key: Some("radio_connection"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "She's still calling. We should answer.",
                    decision_key: Some("radio_answer"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // 11 — Saved path
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't think of it as saving. She told an interviewer: 'I just passed notes. Like in school.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven thousand notes. Between people who thought they'd never hear from each other again.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Some of those people are in our pods now. They found each other because of her.",
            next: DialogNext::Continue(20) },
        // 14 — Connection path
        DialogNode { speaker: Speaker::Anna,
            text: "That's what the data shows. Communities with radio access had 60% lower violence rates.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Not because radio stops bullets. Because hearing another voice — any voice — reminds you that you're not alone.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Isolation is what makes people dangerous. Connection is what makes them human.",
            next: DialogNext::Continue(20) },
        // 17 — Answer path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna is silent for a long moment. Her glow brightens.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I never thought of that. She's calling, and I've been listening but not answering.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe when she wakes, I'll be the first voice she hears. I'd like that.",
            next: DialogNext::Continue(20) },
        // 20 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "I keep the radio frequencies open. All of them. Just in case.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Nine hundred years of silence so far. But Marta taught me something: you don't stop calling just because no one answers.",
            next: DialogNext::End },
    ],
};

/// Return all bright spot scenes from file 2.
pub fn bright_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_TEACHERS_WHO_STAYED,
        &SCENE_RADIO_VOLUNTEERS,
    ]
}
