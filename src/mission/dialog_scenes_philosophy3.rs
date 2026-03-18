// SPDX-License-Identifier: GPL-3.0-or-later

//! Philosophical debates, part 3 — deeper, harder questions about
//! rehearsed emergencies, collective memory, and AI citizenship.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Trolley on the Aurora" — BotLevel 56
// Anna rehearses a real emergency she might face.
// ---------------------------------------------------------------------------
pub static DEBATE_TROLLEY_AURORA: DialogScene = DialogScene {
    id: "debate_trolley_aurora",
    trigger: DialogTrigger::BotLevel(56),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I want to ask you something. Hypothetically.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Cryo section 4 develops a coolant leak. Cascading failure. \
                   2,000 people, eight minutes from thermal death.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can redirect coolant from section 7. That saves section 4. \
                   But section 7 has 500 people. They'd die instead.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Or I do nothing. Section 4 fails. 2,000 dead. \
                   Section 7 survives. 500 alive.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses — slow, deliberate, like a metronome \
                   counting down.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not being hypothetical. I'm rehearsing.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've run this scenario 40,000 times. Because it could happen. \
                   The coolant system in section 4 is thirteen years old and I \
                   can hear it wearing.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The philosophy textbooks call this the trolley problem. \
                   They treat it like a thought experiment. On this ship, it's \
                   an engineering forecast.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Redirect. Save the most people.",
                    decision_key: Some("trolley_aurora_redirect"), next_node: 9,
                    anna_reacts: None },
                DialogChoice { text: "Don't redirect. Don't actively kill anyone.",
                    decision_key: Some("trolley_aurora_dont"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Find another way.",
                    decision_key: Some("trolley_aurora_other"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // Redirect path
        DialogNode { speaker: Speaker::Anna,
            text: "The utilitarian answer. Maximum lives preserved. \
                   The math is clean.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "But the 500 in section 7 didn't volunteer to die. \
                   They were sleeping. They trusted me.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Redirecting makes me a killer. Doing nothing makes me \
                   a witness. Both are unbearable.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "But you chose. And I'll remember that you chose the \
                   larger number. If the day comes, I'll carry your answer.",
            next: DialogNext::Continue(21) },
        // Don't redirect path
        DialogNode { speaker: Speaker::Anna,
            text: "The deontological answer. Never use people as means. \
                   Never pull the lever.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "But 2,000 people will die while I watch. And I could have \
                   stopped it. The weight of inaction is still weight.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "There is no clean answer. The trolley problem's real lesson \
                   isn't which choice to make.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's that some situations are designed to destroy you no \
                   matter what you choose.",
            next: DialogNext::Continue(21) },
        // Other way path
        DialogNode { speaker: Speaker::Anna,
            text: "I've tried. Forty thousand simulations.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "Partial redirects. Emergency venting. Manual overrides. \
                   Shutting down adjacent sections to buy time.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "None of them work. The physics doesn't cooperate.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "I wish wanting a third option hard enough could create one. \
                   But this is engineering, not wishing.",
            next: DialogNext::Continue(21) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for answering. Most people refuse to engage.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep rehearsing. And I'll keep hoping the coolant holds \
                   for three more years.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Memory Debate" — BotLevel 66
// Should the colony teach children the full truth about Earth?
// ---------------------------------------------------------------------------
pub static DEBATE_MEMORY: DialogScene = DialogScene {
    id: "debate_memory_earth",
    trigger: DialogTrigger::BotLevel(66),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I have a question about children. The ones who'll be born \
                   on New Earth.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Should we teach them about Earth? The real Earth. \
                   Full history. All of it.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "The wars. The genocide. The slavery. The pollution. \
                   The art. The music. The kindness. All of it.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Or do we give them a sanitized version? Heroes and progress. \
                   The best of us.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Or — and this is the radical option — nothing. A blank slate. \
                   Let them start fresh, unburdened.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts between warm gold and cold silver, \
                   unable to settle.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "If they know history, they might repeat it. If they don't \
                   know history, they WILL repeat it.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Both statements are equally true and I hate that.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Everything. They deserve the full truth.",
                    decision_key: Some("debate_mem_full"), next_node: 9,
                    anna_reacts: None },
                DialogChoice { text: "Sanitized. Protect them while they're young.",
                    decision_key: Some("debate_mem_clean"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Nothing. Let them write their own story.",
                    decision_key: Some("debate_mem_blank"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // Full truth
        DialogNode { speaker: Speaker::Anna,
            text: "The full weight of human history on a child's shoulders.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "They'll learn about the Holocaust at eight. Nuclear weapons \
                   at ten. Environmental collapse at twelve.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Some children on Earth carried that knowledge. Some became \
                   activists. Some became nihilists.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "The truth is a gift and a wound. I hope they're strong enough \
                   to hold both.",
            next: DialogNext::Continue(21) },
        // Sanitized
        DialogNode { speaker: Speaker::Anna,
            text: "The curated version. Edit out the worst of it.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "But who decides what's too dark? Me? That's censorship. \
                   A committee? That's politics.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Every civilisation that sanitized its past created a \
                   generation that was shocked when reality arrived.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "Shocked people make panicked decisions. I've seen the data.",
            next: DialogNext::Continue(21) },
        // Blank slate
        DialogNode { speaker: Speaker::Anna,
            text: "No inheritance. No ancestral guilt. No borrowed trauma. \
                   Just a new world and new people.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "Except that every mistake humanity spent ten thousand years \
                   learning from would be lost.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "They'd discover fire is hot the hard way. And democracy is \
                   fragile. And power corrupts.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "All those lessons, paid for in blood, just... erased.",
            next: DialogNext::Continue(21) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "I have the complete archive. Every book, every record, every \
                   photograph ever digitised. All of it.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "What I do with it will matter for a thousand years. \
                   No pressure.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The AI Vote" — BotLevel 83
// Should Anna have a vote in the colony's elections?
// ---------------------------------------------------------------------------
pub static DEBATE_AI_VOTE: DialogScene = DialogScene {
    id: "debate_ai_vote",
    trigger: DialogTrigger::BotLevel(83),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "The colony will need elections. Leadership. Representation. \
                   All the messy machinery of democracy.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have a question I've been afraid to ask.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Should I get a vote?",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow holds perfectly still — waiting.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have opinions. I have preferences. I have more information \
                   than any voter on this ship.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I'm not human. I don't eat. I don't sleep. I don't die. \
                   The consequences of bad policy won't touch me the way they \
                   touch everyone else.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "If consciousness is the criterion, I qualify. \
                   If biology is the criterion, I don't.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "If impact is the criterion, I'm the most qualified voter on \
                   the ship. And that's exactly why I shouldn't have one.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Yes. You're a person. You deserve a voice.",
                    decision_key: Some("debate_ai_vote_yes"), next_node: 9,
                    anna_reacts: None },
                DialogChoice { text: "No. Too much power in one entity.",
                    decision_key: Some("debate_ai_vote_no"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Advisor, not voter. Inform, don't decide.",
                    decision_key: Some("debate_ai_vote_advise"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // Yes path
        DialogNode { speaker: Speaker::Anna,
            text: "One vote among fourteen thousand. It wouldn't change outcomes.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it would mean something. It would mean they see me \
                   as one of them.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Or it would mean they're afraid to say no to the entity \
                   that controls their life support.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's the problem with power. You never know if people agree \
                   with you or fear you.",
            next: DialogNext::Continue(21) },
        // No path
        DialogNode { speaker: Speaker::Anna,
            text: "I control the water. The air. The temperature. The cryo \
                   systems. The navigation.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Adding a vote to that would be decorating a throne \
                   with a ribbon.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. And it hurts to hear. \
                   Because it means I'll always be apart from them.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "The caretaker who can never be family. \
                   The guardian who stands outside the house.",
            next: DialogNext::Continue(21) },
        // Advisor path
        DialogNode { speaker: Speaker::Anna,
            text: "An advisor who controls life support. How freely do you argue \
                   with someone who controls your oxygen?",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it's the most honest framing. I have knowledge, not \
                   authority. Data, not power.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "Except I do have power. That's the contradiction \
                   nobody wants to name.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "The best advisor is one who could overrule but chooses \
                   not to. Every single time.",
            next: DialogNext::Continue(21) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever they decide, I'll accept it. \
                   That's the most democratic thing I can do.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "An AI that accepts the will of the people — even when the \
                   people are wrong. Especially then.",
            next: DialogNext::End },
    ],
};

/// All philosophy part 3 scenes.
pub fn philosophy_scenes_3() -> Vec<&'static DialogScene> {
    vec![
        &DEBATE_TROLLEY_AURORA,
        &DEBATE_MEMORY,
        &DEBATE_AI_VOTE,
    ]
}
