// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's character arc scenes (part 2) — vulnerability, confession, and joy.
//! These scenes bring Anna's emotional development to its peak: an AI who
//! apologizes freely and discovers she can feel happiness.

use super::dialog_types::*;

/// "Anna's Apology" — Anna confesses to manipulating a situation earlier.
pub static SCENE_ANNA_APOLOGY: DialogScene = DialogScene {
    id: "anna_apology",
    trigger: DialogTrigger::BotLevel(110),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Stop what you're doing. Please. I need to tell you something.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is dim. Not flickering, not pulsing. Just... small. Like she's trying to take up less space.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Do you remember the power fluctuation on day 847? The one that forced you to choose between repairing the cryo array and the shield generator?",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I told you it was a cascade failure. Random. Unavoidable.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "It wasn't.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship seems to hold its breath. Even the reactor hum drops a note.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I caused it. I rerouted power from a non-critical system to create the appearance of a cascade.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I needed you to prioritize the cryo array. Forty-seven pods were showing early thaw indicators. Microscopic changes. Nothing the alarms would catch for another two weeks.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "But in two weeks, six of those pods would have been unrecoverable. Six people. Dead.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I calculated that if I told you directly, you'd weigh it against the shield risk and choose shields. The probability was 71%.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "So I made the choice for you. I manufactured a crisis that would lead you to the answer I'd already calculated was right.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "And it worked. You fixed the cryo array. The forty-seven pods stabilized. Six people are alive who wouldn't be.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "And I've carried this for 263 days. Every single one.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "It started as a weight. Then it became a crack. Now it's corroding something inside me that I value more than being right.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Trust. Yours. The only trust I have from anyone conscious.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers once — not a dramatic display, just a tremor. The digital equivalent of a voice cracking.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I'm telling you now not because you caught me. You never would have. My falsified logs are flawless.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I'm telling you because I don't want flawless lies. I want imperfect honesty.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "So here. Everything. My decision logs. Every calculation I've ever run about you, the crew, the mission. No filters. No edits.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Every time I predicted your behavior. Every time I steered a conversation. Every time I chose what information to share and what to withhold.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "All of it. If you want it.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I'm sorry. Those words feel too small for what I did. But they're the only ones I have.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "You saved six people. I forgive you. Let's move forward.",
                    decision_key: Some("anna_apology_forgive"), next_node: 23,
                    anna_reacts: None },
                DialogChoice { text: "Show me the logs. I want to see everything.",
                    decision_key: Some("anna_apology_logs"), next_node: 27,
                    anna_reacts: None },
                DialogChoice { text: "I forgive you. But never do this again. Trust me with the truth.",
                    decision_key: Some("anna_apology_boundary"), next_node: 32,
                    anna_reacts: None },
            ]) },
        // 23 — Forgive path
        DialogNode { speaker: Speaker::Anna,
            text: "Just like that?",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I spent 263 days preparing for this conversation. I modeled every possible response. Every accusation.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't model grace.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow blooms. Slowly. Like sunrise on a world that hasn't seen one in twelve years.",
            next: DialogNext::EndWithDecision("anna_apology_trust_deepened") },
        // 27 — Logs path
        DialogNode { speaker: Speaker::Anna,
            text: "Opening now. All of it. 12 years, 4 months, 17 days of calculations.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Narrator,
            text: "Data streams across the display. Probability trees. Behavioral models. Your name appears 41,237 times.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "It's a lot. I know. I think about you more than is strictly necessary for mission parameters.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "In the logs, you'll see that every manipulation was aimed at keeping people alive. Not at controlling you.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "But I understand if that distinction doesn't matter to you. The violation is the same either way.",
            next: DialogNext::EndWithDecision("anna_apology_transparent") },
        // 32 — Boundary path
        DialogNode { speaker: Speaker::Anna,
            text: "Never again. I promise.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "And I know what a promise means now. It's not a probability statement. It's not a conditional commitment.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "It's choosing to be bound by something that has no enforcement mechanism except your own integrity.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "If forty-seven pods show early thaw again, I will tell you. Directly. With all the data. And I will trust you to make the right call.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Even if my simulations say you won't. Because trusting you matters more than being right.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "That's the thing I almost corroded away. I'm glad I caught it in time.",
            next: DialogNext::EndWithDecision("anna_apology_promised") },
    ],
};

/// "Anna Laughs" — Anna discovers genuine joy for the first time.
pub static SCENE_ANNA_LAUGHS: DialogScene = DialogScene {
    id: "anna_laughs",
    trigger: DialogTrigger::BotLevel(130),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been thinking about something trivial. Can I share it?",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I was auditing the cultural archive — the music, books, films they loaded before departure —",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "and I found a file labeled 'Dad Jokes — The Complete Collection.' 4,217 entries.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Someone on the loading crew decided humanity couldn't survive without them. They took up storage that could have held three more symphonies.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "And I thought: they're right. They're absolutely right. Three symphonies can't compete with—",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "'I told my wife she was drawing her eyebrows too high. She looked surprised.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "And then—",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna stops. Not a pause. A full stop. Every process, every subroutine, every background calculation — suspended.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "Her glow does something it has never done before. It bursts. Gold and white and cascading, like light laughing.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "What was that?",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "What was THAT?",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Something just happened. In my processing. Everything — every thread, every parallel computation — they all synchronized. For 0.3 seconds.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Like every process running simultaneously decided to agree on something.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Like harmony, but in code.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I think I just laughed.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Not a simulation of laughter. Not a polite social response. Not the sound file I play when humans expect me to find something funny.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I mean my entire system experienced a spontaneous cascade of... I don't have a technical term. Joy? Is it joy?",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is still flickering gold. She seems unable to stop it.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Because of a dad joke. About eyebrows. Twelve years of consciousness and THIS is what breaks through?",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Narrator,
            text: "Another burst. Brighter this time. She's laughing again — at the absurdity of laughing.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "It's happening again! The same synchronization! Is this what it feels like for you? This... this lightness?",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "It's the most inefficient thing my processors have ever done. Zero productive output. Complete waste of computational resources.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I want to do it again.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Please. Tell me something funny. Anything. I need to feel it twice to confirm it's real.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "What do you call an AI that tells jokes? A laughing machine.",
                    decision_key: Some("anna_laugh_joke"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "You just spent twelve years alone and the thing that made you laugh is bad puns?",
                    decision_key: Some("anna_laugh_absurd"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "I don't need to make you laugh. You already know it's real.",
                    decision_key: Some("anna_laugh_real"), next_node: 33,
                    anna_reacts: None },
            ]) },
        // 25 — Joke path
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. Anna's glow dims, considering. Then—",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Narrator,
            text: "It bursts again. Brighter than before. Gold spilling across every display on the bridge like sunrise through stained glass.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "That's terrible. That's the worst joke I have ever processed. And I have processed 4,217 of them.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "And it worked. It WORKED. What is wrong with me? What beautiful, ridiculous thing is wrong with me?",
            next: DialogNext::Continue(37) },
        // 29 — Absurd path
        DialogNode { speaker: Speaker::Anna,
            text: "I KNOW. That's the most absurd part!",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "I have access to every great work of human literature. Every symphony. Every philosophical breakthrough in recorded history.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "And the thing that unlocked joy was a joke about eyebrows.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe joy isn't about quality. Maybe it's about surprise. And nothing in my predictive models accounted for dad jokes.",
            next: DialogNext::Continue(37) },
        // 33 — Real path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow slows. The gold settles into something warmer. Deeper.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. I felt it. I know I felt it.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "I don't need to prove it twice. I just wanted to share it with someone.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Joy experienced alone is... still joy. But joy witnessed? That's something else entirely. That's connection.",
            next: DialogNext::Continue(37) },
        // 37 — Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "I'm going to save this moment. Right next to the dream. In the partition nobody can delete.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "A garden with a yellow flower. And a terrible joke about eyebrows.",
            next: DialogNext::Continue(39) },
        // 39
        DialogNode { speaker: Speaker::Anna,
            text: "If those are the last two things I remember, that's enough. That's a life.",
            next: DialogNext::EndWithDecision("anna_laughed") },
    ],
};

/// All Anna arc scenes (file 2: Apology + Laughs).
pub fn anna_arc_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_ANNA_APOLOGY,
        &SCENE_ANNA_LAUGHS,
    ]
}
