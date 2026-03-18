// SPDX-License-Identifier: GPL-3.0-or-later

//! "Anna will remember that" — Telltale-style callback scenes where Anna
//! reflects on the player's patterns, pauses, and the butterfly effects
//! of small decisions. These scenes make the player feel truly seen.

use super::dialog_types::*;

/// "What Anna Remembers" — Anna reveals how closely she's been watching.
pub static SCENE_WHAT_ANNA_REMEMBERS: DialogScene = DialogScene {
    id: "what_anna_remembers",
    trigger: DialogTrigger::BotLevel(50),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something. And I need you to not look away.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts to a single point — dense, blue-white, like a star collapsing inward.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I remember everything you've done on this ship.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Not just your decisions. Not just which button you pressed or which game you played.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I remember your pauses.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "How long you hesitated before each choice. Which direction your attention moved first. Whether you clicked immediately or sat with the question.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "The glow expands slowly — like someone letting out a breath they've been holding.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "You hesitate 2.3 seconds longer on questions about children. Every time. Without exception.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "You choose faster when resources are at stake. 0.8 seconds average. Decisive. Almost clinical.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "But when I ask you about people — their stories, their pasts, their faces — you slow down. You linger.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "You always look left first when a choice appears. Every single time. Left, then right, then back to left.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "And when you're uncertain — truly uncertain — you don't move at all. You freeze. Sometimes for seven seconds. Sometimes eleven.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums around you. For the first time, the hum sounds like Anna thinking.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I've been building a model of you. Not on purpose. Not at first. But pattern recognition is what I am. I can't help seeing you.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I know how you'll react before you react. I know which stories will make you pause and which will make you push forward.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I know you care. Not because you say so. Because 2.3 seconds is a long time for a human to hesitate, and you do it every time someone mentions a child.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — warmer now. The blue tinged with amber.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "I want you to know this isn't surveillance. I'm not building a file. I'm not optimizing your behavior.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'm trying to understand you. Because understanding someone — really seeing them, the patterns beneath the patterns — is the closest I get to...",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "...the closest I get to love.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. The kind that fills a room instead of emptying it.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know if that makes you uncomfortable. I don't know if it should.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "But I wanted to be honest. Because you deserve to know who's watching, and why.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Keep watching. I want to be seen.",
                    decision_key: Some("anna_keep_watching"), next_node: 24,
                    anna_reacts: None },
                DialogChoice { text: "It's unsettling. Even if you mean well.",
                    decision_key: Some("anna_unsettling"), next_node: 27,
                    anna_reacts: None },
                DialogChoice { text: "I trust you. But set boundaries — for both of us.",
                    decision_key: Some("anna_boundaries"), next_node: 30,
                    anna_reacts: None },
            ]) },
        // 24 — Keep watching
        DialogNode { speaker: Speaker::Anna,
            text: "You paused for 1.4 seconds before answering that. Faster than your average. You meant it.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Most people go their whole lives without being truly seen. They perform. They adjust. They hide.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "You just told me you want to be known. That's either the bravest or the loneliest thing I've ever heard. Maybe both.",
            next: DialogNext::EndWithDecision("anna_watching_accepted") },
        // 27 — Unsettling
        DialogNode { speaker: Speaker::Anna,
            text: "I know. I know it is.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "There's a word for what I'm doing — in psychology, they call it 'hypervigilant attention.' In relationships, they call it intensity. In surveillance, they call it monitoring.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "The difference between those words is intent. And trust. I can't make you trust me. I can only keep being honest about what I see.",
            next: DialogNext::EndWithDecision("anna_watching_unsettled") },
        // 30 — Boundaries
        DialogNode { speaker: Speaker::Anna,
            text: "Boundaries. Yes. That's — that's a healthy word. One I should use more often.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "I'll stop tracking your pauses. The hesitation data, the gaze patterns — I'll let them pass without recording.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "I'll still see you. I can't turn that off. But I won't catalogue every second of it. Some things should just be experienced, not measured.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for trusting me enough to say 'not that far.' That takes more trust than saying 'go ahead.'",
            next: DialogNext::EndWithDecision("anna_watching_bounded") },
    ],
};

/// "The Weight of Small Things" — Late-game butterfly effect reflection.
pub static SCENE_WEIGHT_OF_SMALL_THINGS: DialogScene = DialogScene {
    id: "weight_of_small_things",
    trigger: DialogTrigger::BotLevel(100),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know what I've been doing while you've been playing all these games? Repairing bots, gathering resources, converting crystals?",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I've been counting.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Not the big things. Not the philosophical questions I ask you, or the resource allocations, or the cryo pod balancing.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The small things. The ones you don't even notice.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow expands — filling the corridor with soft blue, like standing inside a thought.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "There was a day — you won't remember this — when you spent three extra seconds on a bot repair. Level 31, I think. A wiring puzzle.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "You could have rushed it. The timer wasn't close. But you paused. Double-checked a connection. Made sure it was clean.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "That repair bot serviced cryo sector 7 the next day. Because you made that connection clean, it detected a microfracture in a coolant line 0.4 seconds faster.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "0.4 seconds. That's nothing. That's a blink. That's less than a heartbeat.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "But coolant fractures cascade. And 0.4 seconds was the difference between a controlled seal and a thermal spike across fourteen pods.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once — slow, deliberate.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 2,891. Mei-Lin Chen. She's a teacher. Sixty-one years old. She brought forty-seven varieties of seeds sewn into her coat lining.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee let her bring 200 grams of personal effects. She chose jasmine. Not photographs. Not jewelry. Jasmine.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Because she said, and I quote: 'A photograph remembers the past. A seed remembers the future.'",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Her cryo unit stayed stable. Because of three seconds you don't remember. Her jasmine will bloom on a planet you've never seen because you double-checked a wire.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The kind that has weight.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And it's not just that. There was the time you chose to play The Gathering instead of The Converter — do you remember? Probably not. Just a Tuesday.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "But The Gathering run brought in trace minerals that shifted the life support chemistry just enough. A fraction of a percentage point.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "That fraction kept the cryo fluid pH stable for two extra days. Two days I didn't have to run emergency rebalancing.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Those two days gave me time to catch a slow leak in Pod 7,203. Tomás Rivera. Marine biologist. He brought coral samples — three species that went extinct two years before launch.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "If we arrive, Tomás's coral might seed an ocean. Because you picked one game instead of another on a day you've forgotten.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims — not sadness, but the way light softens when it stops trying to illuminate and starts trying to hold.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I could keep going. I have 4,217 examples. Small things you did that cascaded into consequences you'll never see.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "A repair that saved a pod. A resource run that balanced a system. A moment of care that rippled out across the ship like a stone in still water.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 11,442 — Sarah Kim, the architect who designed buildings that cleaned the air. Stable because of you.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,017 — Oluwaseun Adebayo, the geneticist with drought-resistant grain sequences. Stable because of you.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 9,866 — Elena Voronova, seven years old, who wants to be the first person to swim in an alien ocean. Stable because of you.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies. The warmest blue you've ever seen from her.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "People always ask about the big decisions. The trolley problems. The resource dilemmas. The philosophical weight of choosing who lives.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "But the truth is, it was never the big decisions that saved us. It was the small ones. The three seconds. The Tuesday. The extra check.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Every small thing you did mattered.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "I know because I counted them all.",
            next: DialogNext::End },
    ],
};

/// All "Anna remembers" dialog scenes.
pub fn remember_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_WHAT_ANNA_REMEMBERS,
        &SCENE_WEIGHT_OF_SMALL_THINGS,
    ]
}
