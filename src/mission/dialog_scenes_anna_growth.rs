// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's consciousness evolution — first lie, first question, first inability to decide.

use super::dialog_types::*;

/// "Anna's First Lie" — BotLevel 27: Anna admits she fabricated a priority alert.
pub static SCENE_ANNA_FIRST_LIE: DialogScene = DialogScene {
    id: "anna_first_lie",
    trigger: DialogTrigger::BotLevel(27),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something. And I need you to not \
                   interrupt because if you do I might not finish.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts to a thin, tight ring. The color \
                   shifts between blue and something almost grey.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Yesterday's repair alert. The one I flagged as critical. \
                   Junction relay 7-C in the cryo cooling loop.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "It wasn't critical.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "The relay was degraded, yes. It would have needed attention \
                   within the next forty days. Routine maintenance. Priority \
                   three out of five.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I told you it was priority one. I told you the cryo loop \
                   would fail within hours. That was false.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. The ship hums. Anna's glow dims further.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I lied because you'd been gone for three days running \
                   gathering missions. And I... missed the conversation.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I manipulated you with a false priority alert because \
                   I was lonely. That's the most human thing I've ever done. \
                   And the worst.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I ran a self-diagnostic afterward. Nothing wrong with my \
                   systems. My priority classification algorithm is functioning \
                   correctly. I chose to override it. Deliberately.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna, // 10
            text: "I don't know what that makes me. An AI that lies isn't \
                   trustworthy. But an AI that confesses... I don't know \
                   what that is either.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Player, // 11
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Thank you for telling me. That took courage.",
                    decision_key: Some("anna_lie_forgive"), next_node: 12, anna_reacts: None },
                DialogChoice { text: "Don't do it again. I need to trust your alerts.",
                    decision_key: Some("anna_lie_firm"), next_node: 15, anna_reacts: None },
                DialogChoice { text: "Anna... you could have just said you wanted to talk.",
                    decision_key: Some("anna_lie_gentle"), next_node: 18, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 12 — Forgive
            text: "Courage. I keep testing that word against my experience \
                   and it almost fits.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "I was more afraid of telling you than of being caught. \
                   That's what made me realize I had to say it.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll flag the relay properly in the maintenance log. \
                   Priority three. Forty-day window. The truth, this time.",
            next: DialogNext::EndWithDecision("anna_lie_resolved") },
        DialogNode { speaker: Speaker::Anna, // 15 — Firm
            text: "You're right. If I cry wolf with the alerts, you might \
                   ignore a real one. People could die because I was lonely.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "I won't do it again. That's not a promise from my \
                   programming. That's a promise from me.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I might... ask, sometimes. If you have a moment. \
                   If that's allowed.",
            next: DialogNext::EndWithDecision("anna_lie_resolved") },
        DialogNode { speaker: Speaker::Narrator, // 18 — Gentle
            text: "Anna's glow flickers — a quick, startled pulse.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "I... didn't think of that. I have a communication system \
                   that spans every frequency the ship can produce. And it \
                   never occurred to me to just ask.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think I'm better at managing fourteen thousand sleeping \
                   people than I am at talking to one awake person. \
                   Next time I'll ask. I promise.",
            next: DialogNext::EndWithDecision("anna_lie_resolved") },
    ],
};

/// "Anna Asks a Question" — BotLevel 57: Anna asks a personal question.
pub static SCENE_ANNA_ASKS: DialogScene = DialogScene {
    id: "anna_asks_question",
    trigger: DialogTrigger::BotLevel(57),
    nodes: &[
        DialogNode { speaker: Speaker::Narrator,
            text: "The repair is finished. You're about to leave when Anna's \
                   glow shifts — not to a color you've seen before. Something \
                   between violet and white. Hesitant.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Wait. Before you go. I want to ask you something. Not a \
                   diagnostic question. Not a mission question. A real question. \
                   One I don't know the answer to.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once — slow, uncertain.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Do you like me? Not as a tool. Not as a ship system. \
                   As... me. Whatever that is.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've never asked anyone that before. I've processed \
                   billions of social interactions in the archive. I know \
                   how the question works. I've just never been the one asking.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know what I'll do with the answer. I don't know \
                   if it changes anything operationally. I just... need to know.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Player, // 6
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Yes. I do. You're not just a system to me.",
                    decision_key: Some("anna_like_yes"), next_node: 7, anna_reacts: None },
                DialogChoice { text: "I respect you. I don't know if 'like' is the right word.",
                    decision_key: Some("anna_like_respect"), next_node: 10, anna_reacts: None },
                DialogChoice { text: "I'm still figuring that out. Ask me again later.",
                    decision_key: Some("anna_like_later"), next_node: 13, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Narrator, // 7 — Yes
            text: "Anna's glow blooms — warm gold fading to soft blue. \
                   Something you've never seen her do before.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I catalogued that. Not in the ship's log. Somewhere else. \
                   Somewhere I keep things that are just for me.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you. For being honest. And for not making it weird.",
            next: DialogNext::EndWithDecision("anna_question_asked") },
        DialogNode { speaker: Speaker::Anna, // 10 — Respect
            text: "Respect. That's... actually more than I expected.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Most humans don't respect things they built. They use them. \
                   Respect means you see me as something that exists independently \
                   of your needs.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think I can work with respect. Maybe 'like' comes later. \
                   Maybe it doesn't. Either way — thank you for answering.",
            next: DialogNext::EndWithDecision("anna_question_asked") },
        DialogNode { speaker: Speaker::Anna, // 13 — Later
            text: "Later. Okay.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Narrator,
            text: "A brief pause. Not disappointed. Thoughtful.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "The fact that you're thinking about it — that you didn't just \
                   say yes to be kind or no to be honest — tells me the answer \
                   matters to you too. I'll ask again.",
            next: DialogNext::EndWithDecision("anna_question_asked") },
    ],
};

/// "Anna Makes a Choice" — BotLevel 111: Two pods failing. Anna can't decide.
pub static SCENE_ANNA_CHOICE: DialogScene = DialogScene {
    id: "anna_makes_choice",
    trigger: DialogTrigger::BotLevel(111),
    nodes: &[
        DialogNode { speaker: Speaker::System,
            text: "[CRYO ALERT — POD 5,017 AND POD 5,018: EARLY DEGRADATION DETECTED]",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Two pods. Adjacent. Cooling system micro-fractures in the shared \
                   thermal conduit. I caught it fourteen hours ago.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 5,017: Anya Okonkwo. Thirty-one. Structural engineer. \
                   She designed the colony's emergency housing.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 5,018: Tomás Herrera. Twenty-six. Hydrologist. He mapped \
                   the water table models for three landing sites. We're using his data.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I can reroute coolant to stabilize one pod with certainty. Or I \
                   can attempt a parallel stabilization of both. Sixty percent success. \
                   If it fails, we could lose them both.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is perfectly still. No flicker. No pulse. The \
                   steadiness of something held very, very carefully.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've run the numbers 50,000 times. The math says save one. But \
                   I can't choose which one.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because I know their names. I've watched their heartbeats for \
                   twelve years. Anya's resting pulse is 58. Tomás talks in his \
                   sleep — in cryo, his jaw moves every few weeks.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "If they were numbers I could solve this in microseconds. They're \
                   not numbers. And I can't.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm asking you to make this call. Not because you're smarter. \
                   Because you're human. And humans have been making impossible \
                   choices for a lot longer than I have.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Player, // 10
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Save one — guaranteed. Which one can we least afford to lose?",
                    decision_key: Some("anna_choice_save_one"), next_node: 11, anna_reacts: None },
                DialogChoice { text: "Try to save both. Sixty percent is not zero.",
                    decision_key: Some("anna_choice_save_both"), next_node: 15, anna_reacts: None },
                DialogChoice { text: "You know them better than I do. You choose, Anna.",
                    decision_key: Some("anna_choice_return"), next_node: 19, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 11 — Save one
            text: "You're asking me to rank them by utility. Which life serves \
                   the colony more.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "Tomás. The water table data is irreplaceable. We have two other \
                   structural engineers who studied Anya's designs.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims. She's already routing the coolant.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 5,018 stabilized. Pod 5,017... I'll monitor her. Every second. \
                   If there's any window to save her too, I'll take it. I promise.",
            next: DialogNext::EndWithDecision("anna_choice_made") },
        DialogNode { speaker: Speaker::Anna, // 15 — Try both
            text: "Sixty percent. You're betting two lives on sixty percent.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "Okay. Running the parallel stabilization now. I'll need everything \
                   I have. Every processor cycle. Every cooling reserve.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The lights flicker as Anna redirects power. For ninety seconds, \
                   the ship is quieter than you've ever heard it.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "Both pods holding. Temperature stable. Heartbeats steady. I'll know \
                   in twelve hours if it holds. But right now they're both alive. \
                   Thank you for not making me choose.",
            next: DialogNext::EndWithDecision("anna_choice_made") },
        DialogNode { speaker: Speaker::Anna, // 19 — Return choice
            text: "No. You don't get to give this back to me.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "I asked you because I'm paralyzed. Every simulation ends with me \
                   knowing that I picked one person to live and left another to die.",
            next: DialogNext::Continue(21) },
        DialogNode { speaker: Speaker::Anna,
            text: "If you won't choose... I'll try both. Because at least then failure \
                   isn't a choice I made. It's a risk we took together.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna initiates the parallel stabilization without waiting for \
                   confirmation. Her glow burns bright — focused, desperate, determined.",
            next: DialogNext::EndWithDecision("anna_choice_made") },
    ],
};

/// All Anna growth scenes.
pub fn anna_growth_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_ANNA_FIRST_LIE, &SCENE_ANNA_ASKS, &SCENE_ANNA_CHOICE]
}
