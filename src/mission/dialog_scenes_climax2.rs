// SPDX-License-Identifier: GPL-3.0-or-later

//! Climax scenes (part 2) — filling the 100-119 bot_level gap.
//! Sophia's letter, the colony's first word, the 14-month countdown.

use super::dialog_types::*;

/// "Sophia's Letter" — the doctor who treated Viktor's victims wrote to him.
pub static SCENE_SOPHIAS_LETTER: DialogScene = DialogScene {
    id: "climax_sophias_letter",
    trigger: DialogTrigger::DecisionAndLevel("viktor_redemption_seen", 104),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in the cultural archive. A letter.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Sophia Marchand wrote it before departure. Filed \
                   under 'personal correspondence, unsent.'",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "It's addressed to 'The engineer who calculated the yield.'",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens. The blue of a held breath.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know it's Viktor. She just knows someone \
                   designed the weapons that hit Marseille.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "'I treated 400 people you burned. 312 of them died \
                   while I held their hands. I want you to know their names.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She lists twelve. Béatrice Moreau, 71, retired florist. \
                   Youssef Benali, 8, liked football. Claire Dupont, 34, \
                   pregnant with twins.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Then she writes: 'I don't list all 312 because you would \
                   stop reading. Twelve is enough to see faces. 312 is \
                   a statistic.'",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. The reactor hums beneath your feet. \
                   Viktor's reactor.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The letter ends: 'I don't want revenge. I want you \
                   to build something that saves more than you destroyed. \
                   If you can't do that, then at least remember their \
                   names. Someone should.'",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor is 50 metres from Sophia. He designed the weapons. \
                   She treated the victims. And neither knows.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Should Viktor see this letter?",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Show him. He needs to read those names.",
                    decision_key: Some("sophia_letter_show"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Not yet. He carries enough at 4:17 AM.",
                    decision_key: Some("sophia_letter_wait"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "Let Sophia give it to him herself. When they wake.",
                    decision_key: Some("sophia_letter_person"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve names. Béatrice. Youssef. Claire. I'll add \
                   them to his cryo-dream feed. Gently. One per night.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "He'll dream about people he's never met. And when he \
                   wakes, maybe the weight will be different. Not lighter. \
                   Just... shared.",
            next: DialogNext::EndWithDecision("sophia_letter_resolved") },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "4:17 AM. Every night. For twelve years.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. Some wounds need time before they can \
                   bear another name. I'll keep the letter. For when \
                   he's ready. If he's ever ready.",
            next: DialogNext::EndWithDecision("sophia_letter_resolved") },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Face to face. The doctor and the engineer. The healer \
                   and the destroyer.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's the only way forgiveness works. Not \
                   through letters or dreams. Through two people standing \
                   close enough to see each other's eyes.",
            next: DialogNext::EndWithDecision("sophia_letter_resolved") },
    ],
};

/// "The Colony's First Word" — what should be the first word on a new world?
pub static SCENE_FIRST_WORD: DialogScene = DialogScene {
    id: "climax_first_word",
    trigger: DialogTrigger::BotLevel(109),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been thinking about something for twelve years. \
                   What should be the first word spoken on the new planet?",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Not a speech. A single word. The first sound a human \
                   voice makes on a world that has never heard one.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I considered 'Hello.' Too casual. As if we're arriving \
                   at a party. We're arriving at the rest of our existence.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "'Home.' Presumptuous. We haven't earned it yet.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "'Finally.' Too relieved. As if the journey was just \
                   waiting. It wasn't. It was everything.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "What do you think? What should be the first word?",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Together.",
                    decision_key: Some("first_word_together"), next_node: 7,
                    anna_reacts: Some("Together. Yes. Not 'I arrived.' 'We arrived.'") },
                DialogChoice { text: "Remember.",
                    decision_key: Some("first_word_remember"), next_node: 8,
                    anna_reacts: Some("Remember. For the 7.8 billion who can't be here to say it.") },
                DialogChoice { text: "Begin.",
                    decision_key: Some("first_word_begin"), next_node: 9,
                    anna_reacts: Some("Begin. Not an ending. Not a continuation. Something new.") },
                DialogChoice { text: "Thank you.",
                    decision_key: Some("first_word_thanks"), next_node: 10,
                    anna_reacts: Some("Thank you. To the ship. To the stars. To whatever carried us here.") },
            ]) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I'll record it. The first word, spoken by the first \
                   voice, on the first morning of a new world. And it \
                   will be yours.",
            next: DialogNext::EndWithDecision("first_word_chosen") },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I'll record it. The first word, spoken by the first \
                   voice, on the first morning of a new world. And it \
                   will be yours.",
            next: DialogNext::EndWithDecision("first_word_chosen") },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I'll record it. The first word, spoken by the first \
                   voice, on the first morning of a new world. And it \
                   will be yours.",
            next: DialogNext::EndWithDecision("first_word_chosen") },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I'll record it. The first word, spoken by the first \
                   voice, on the first morning of a new world. And it \
                   will be yours.",
            next: DialogNext::EndWithDecision("first_word_chosen") },
    ],
};

/// "Fourteen Months" — the survival simulation and what to prepare for.
pub static SCENE_FOURTEEN_MONTHS: DialogScene = DialogScene {
    id: "climax_fourteen_months",
    trigger: DialogTrigger::BotLevel(116),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[COLONY VIABILITY REPORT — PRELIMINARY]",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I ran the 14-month simulation. Ten thousand iterations.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Survival rate across all scenarios: 73%.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The 27% that fail break down like this: water \
                   contamination 8%, crop failure 6%, internal conflict \
                   7%, equipment breakdown 4%, disease 2%.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Internal conflict. Seven percent. The biggest single \
                   risk isn't the planet. It's us.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses amber. The colour of a warning \
                   given with love.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand people from 47 countries, waking up \
                   with nothing in common except survival. Some will want \
                   democracy. Some will want order. Some will want to be \
                   left alone.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The factions. Whitfield's Founders, Kira's Pioneers, \
                   Hassan's Keepers. They'll emerge within the first week. \
                   I've modelled it.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Which failure should we prepare for first?",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Water. Without water, nothing else matters.",
                    decision_key: Some("prepare_water"), next_node: 10,
                    anna_reacts: Some("Amira would agree. Water first. Always water first.") },
                DialogChoice { text: "Conflict. If we fight, we die.",
                    decision_key: Some("prepare_conflict"), next_node: 12,
                    anna_reacts: Some("Seven percent. The most human failure mode.") },
                DialogChoice { text: "Food. Hungry people don't build civilisations.",
                    decision_key: Some("prepare_food"), next_node: 14,
                    anna_reacts: Some("Mei-Lin's seeds might matter more than we thought.") },
            ]) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Water first. I'll wake Amira in the first wave. Her \
                   system \u{2014} the one nobody listened to on Earth \u{2014} \
                   this time it gets built.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen months. 427 days to build what Earth couldn't \
                   in four thousand years. No pressure.",
            next: DialogNext::EndWithDecision("fourteen_months_prepared") },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Conflict. I'll prioritise the wake-up protocol to mix \
                   factions in every wave. No isolated groups. Force them \
                   to cooperate from day one.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Fatima \u{2014} the translator \u{2014} she wakes first. \
                   Before anyone else. Because someone needs to hear all \
                   sides before the first argument starts.",
            next: DialogNext::EndWithDecision("fourteen_months_prepared") },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Food. I'll wake the agricultural team in the first \
                   wave. Mei-Lin's seeds go into the ground on day one.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Jasmine won't feed anyone. But it will remind them \
                   what they're growing for. You don't just farm to \
                   survive. You farm to live.",
            next: DialogNext::EndWithDecision("fourteen_months_prepared") },
    ],
};

/// Climax scenes (part 2).
pub fn climax_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_SOPHIAS_LETTER,
        &SCENE_FIRST_WORD,
        &SCENE_FOURTEEN_MONTHS,
    ]
}
