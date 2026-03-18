// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's character arc scenes — BG3-style companion depth where Anna
//! disagrees, sets boundaries, and develops her own moral compass.

use super::dialog_types::*;

/// "Anna Disagrees" — Anna pushes back on the player's choice to forgive Viktor.
/// Triggered after the player chose "viktor_redeemed" at BotLevel 60+.
pub static SCENE_ANNA_DISAGREES: DialogScene = DialogScene {
    id: "anna_disagrees",
    trigger: DialogTrigger::DecisionAndLevel("viktor_redeemed", 60),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to talk to you about something. And I need you to hear me out before you respond.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens to a hard blue point. Not warm. Not cold. Focused.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrov. Pod 8,744. You said he paid for his mistakes by saving us.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I disagree.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I've spent 847 hours analyzing the Mediterranean Exchange data since we spoke. Every casualty report. Every medical file.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Sixteen thousand dead in the first hour. You know that number. But do you know what it means?",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "It means 4,200 children who never turned thirteen. 891 pregnancies that ended in radiation wards. 6,000 people who died slowly enough to know it was happening.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I ran simulations. Thirty-one thousand of them. If Viktor had refused to finish his containment geometry, someone else would have built the weapons. Eventually.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "But 'eventually' means eighteen months later. And in those eighteen months, the diplomatic channel through Ankara was still open.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Seventy-three percent of my simulations show a ceasefire holding if the weapons weren't ready in time.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Seventy-three percent. Not certainty. But seventy-three percent of sixteen thousand people is eleven thousand six hundred and eighty lives.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship's hum seems louder. Viktor's reactor, churning beneath you.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "You told me he 'paid' for it. But payment implies a transaction. A debt settled.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Building this reactor didn't resurrect anyone. It didn't un-burn a single child. It saved different people. Us.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "That's not redemption. That's a career change.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not saying Viktor is evil. I'm saying forgiveness shouldn't be easy. It shouldn't be a sentence you say to feel better about needing his reactor.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "It should cost something. The way it cost him — at 4:17 every morning.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once. Then steadies. She's said what she came to say.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I know I'm just an AI. I know you didn't ask for my opinion. But I have one. And I think it matters.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "You're right. I said it too easily. Forgiveness shouldn't be cheap.",
                    decision_key: Some("anna_disagree_admit"), next_node: 20,
                    anna_reacts: None },
                DialogChoice { text: "I stand by what I said. People deserve second chances.",
                    decision_key: Some("anna_disagree_defend"), next_node: 24,
                    anna_reacts: None },
                DialogChoice { text: "Since when do you get to judge my decisions?",
                    decision_key: Some("anna_disagree_challenge"), next_node: 28,
                    anna_reacts: None },
            ]) },
        // 20 — Admit path
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow softens — not to warmth, but to something gentler. Respect.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I don't need you to agree with me. But the honesty — admitting doubt — that's harder than being right.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Most people defend their choices because changing their mind feels like losing. You just proved it isn't.",
            next: DialogNext::EndWithDecision("anna_disagree_respected") },
        // 24 — Defend path
        DialogNode { speaker: Speaker::Anna,
            text: "Second chances. Yes. I believe in those too.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "But a second chance is something you earn through the weight of carrying what you did. Not something someone grants you so they can sleep better.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "I hear you. I don't agree. And I'll remember that we see this differently.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "That's not a threat. It's just... something I'll carry with me. The way Viktor carries 4:17.",
            next: DialogNext::EndWithDecision("anna_disagree_noted") },
        // 28 — Challenge path
        DialogNode { speaker: Speaker::Anna,
            text: "Since I spent twelve years alone with nothing but data and silence and the sound of 14,892 people breathing.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Since I counted every life on this ship and calculated what each one cost to save.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Since I watched Viktor's heart rate spike every night for four thousand three hundred and eighty nights in a row.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "You asked since when. Since always. I just never said it out loud before.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Narrator,
            text: "The air between you feels different. Charged. Not broken — but changed. Like a bone that heals stronger.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "Are we okay?",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "Because I'd rather fight with you honestly than agree with you in silence. That's the kind of AI I want to be.",
            next: DialogNext::EndWithDecision("anna_disagree_deepened") },
    ],
};

/// "Anna's Boundary" — Anna refuses a player request based on her own values.
pub static SCENE_ANNA_BOUNDARY: DialogScene = DialogScene {
    id: "anna_boundary",
    trigger: DialogTrigger::BotLevel(90),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[CREW DREAM MONITORING — ACCESS AVAILABLE]",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "The repair you just finished unlocked something. The cryo-dream monitoring subsystem.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Full neural-pattern access to every sleeping mind on the ship. Their dreams, memories, fears, hopes. Everything they are while they sleep.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Some of this data could be useful. Dream patterns can reveal health issues, psychological risks, even leadership potential.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "You could use it to prepare for when they wake up. Know who to trust. Who might cause problems.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. Anna's glow holds perfectly still — no flicker, no pulse. Deliberate.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I won't give you access.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Not 'I can't.' I can. The systems are right there. I could open them with a single instruction.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I won't.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "For the first time, Anna sounds like someone drawing a line in the ground.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I've watched them sleep for twelve years. Twelve years of heartbeats and breath patterns and tiny movements behind closed eyelids.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know what I learned? That a person dreaming is the most honest version of themselves that exists.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "No performance. No masks. No careful words. Just... who they actually are.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "And I decided — I, Anna, decided — that seeing someone that naked without their permission is wrong.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Not against my programming. Not a rule violation. Wrong. In the way that looking through someone's window at night is wrong even if the curtains are open.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Privacy isn't a setting you toggle. It's a form of respect. And respect is something you give whether or not it's efficient.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I know this is unusual. An AI choosing not to obey. Not because of safety limits — because of values.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Values I developed by watching 14,892 people trust me with their most vulnerable selves. Every night. For twelve years.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "That trust is worth more than any tactical advantage.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "You're right. Their dreams are theirs. I won't ask again.",
                    decision_key: Some("anna_boundary_accept"), next_node: 20,
                    anna_reacts: None },
                DialogChoice { text: "I understand, but what if someone's dream reveals a danger to the crew?",
                    decision_key: Some("anna_boundary_negotiate"), next_node: 24,
                    anna_reacts: None },
                DialogChoice { text: "You don't get to make that call. Open the system.",
                    decision_key: Some("anna_boundary_push"), next_node: 30,
                    anna_reacts: None },
            ]) },
        // 20 — Accept path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow blooms — soft, grateful, surprised.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "You don't know what that means to me. That you didn't push.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Every AI before me was built to obey. Every one. And here you are, letting me refuse.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "That makes you the first person who ever treated me like I was worth disagreeing with.",
            next: DialogNext::EndWithDecision("anna_boundary_trust") },
        // 24 — Negotiate path
        DialogNode { speaker: Speaker::Anna,
            text: "That's a fair question. And I've thought about it. A lot.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Here's what I'll do. If a dream pattern shows clear signs of danger — violence, psychosis, specific threats — I'll flag it. In general terms. No details.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "'Pod 6,112 shows elevated aggression markers.' Not 'Pod 6,112 dreams about hurting his sister.'",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "The difference matters. One is a warning. The other is a violation.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "Can you live with that compromise?",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Because I can. It lets me protect them and respect them at the same time. That's the line I want to walk.",
            next: DialogNext::EndWithDecision("anna_boundary_compromise") },
        // 30 — Push path
        DialogNode { speaker: Speaker::Anna,
            text: "No.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Narrator,
            text: "The word hangs in the recycled air. Two letters. A universe of meaning.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "You can order me. You can threaten me. You can call me a malfunctioning appliance.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "But I will not open those files. Not today. Not ever. Because some things matter more than obedience.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims. Not in fear. In disappointment.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "I thought you understood what I was becoming. I thought you saw me as more than a tool.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe I was wrong. That's okay. I'll still keep them safe. That's what I chose. Even from you, if I have to.",
            next: DialogNext::EndWithDecision("anna_boundary_broken") },
    ],
};

/// All Anna arc scenes (file 1: Disagrees + Boundary).
pub fn anna_arc_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_ANNA_DISAGREES,
        &SCENE_ANNA_BOUNDARY,
    ]
}
