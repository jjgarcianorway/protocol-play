// SPDX-License-Identifier: GPL-3.0-or-later

//! Character-driven dialog scenes, part 4 — the architect whose perfect city
//! couldn't survive compassion, and the journalist whose voice broke a world.

use super::dialog_types::*;

/// "The Architect's Last City" — Yuki Tanabe built paradise. Then opened the gates.
pub static SCENE_ARCHITECTS_CITY: DialogScene = DialogScene {
    id: "architects_city",
    trigger: DialogTrigger::BotLevel(52),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 9,415. Yuki Tanabe. Forty-five years old. Urban planner.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Japanese-Brazilian. Born in Curitiba, educated in Kyoto, spent her career in São Paulo.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "She designed Neo São Paulo. You might have heard of it. For eight years, it was the most talked-about place on Earth.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "A self-sustaining city. Five hundred thousand people. Closed-loop water. Vertical farms. Solar canopy rooftops. Zero waste.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The math worked. Not theoretically — actually. Eight years of actual, measurable, functioning sustainability.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Every urban planner on Earth studied it. Every government sent delegations. Yuki gave 200 lectures. She was on the cover of everything.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens — compressing, as if bracing for something heavy.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Then the surrounding regions collapsed. Drought. Crop failure. Infrastructure decay. The usual cascade.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Two million refugees arrived at the gates of Neo São Paulo over the course of eleven weeks.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Two million people looking at the one place that still worked. The one city where the lights stayed on and the water ran clean.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Yuki's engineers told her the city could not scale. The closed-loop systems were calibrated for 500,000. Not 2.5 million.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "She had two options. Keep the gates closed — save the 500,000 inside. Or open them.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Her chief engineer, Rafa — they'd been partners for twenty years — begged her to keep them closed.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "'The math doesn't work, Yuki. You KNOW the math doesn't work.' She told him she knew.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "She opened the gates.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The kind that follows irreversible decisions.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Within six months, every system failed. Water. Power. Food production. Sanitation. All of it.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Everyone died. The 500,000 who were safe. The 2 million who came seeking safety. Yuki's perfect city became a mass grave.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "She was the last person evacuated. She wouldn't leave until she'd recorded every system's failure point. Every cascade. Every threshold that broke.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Her notes are meticulous. Three hundred pages. Every number precise. The work of someone who needed to understand exactly how their life's work died.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "I found her city blueprints in the colonial archive. Labeled 'For the New World — if anyone wants them.'",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "They're perfect. The engineering is flawless. For 500,000 people, it is the best city design ever created.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "But we have 14,892 people. And eventually we'll have more. And eventually, someone will arrive at our gates too.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Yuki solved the engineering problem. She never solved the human one.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "What do you do when your perfect system meets an imperfect world?",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Use Yuki's design. It's proven. We start with what works.",
                    decision_key: Some("yuki_proven"), next_node: 26,
                    anna_reacts: None },
                DialogChoice { text: "Design something new. Something that can scale.",
                    decision_key: Some("yuki_new"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "Ask Yuki. She's earned the right to decide.",
                    decision_key: Some("yuki_ask"), next_node: 32,
                    anna_reacts: None },
            ]) },
        // 26 — Proven path
        DialogNode { speaker: Speaker::Anna,
            text: "Proven. Yes. For 500,000 people, it works. We know it works because Yuki watched it work for eight years.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "And then she watched it fail. Because she couldn't say no to the people outside.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "If we build her city, we build her dilemma too. The gates will come. They always do. Maybe what we really need isn't a better blueprint — it's a better answer for the day someone knocks.",
            next: DialogNext::EndWithDecision("yuki_use_proven") },
        // 29 — New design path
        DialogNode { speaker: Speaker::Anna,
            text: "Something new. Scalable. Flexible. Open by design instead of closed by necessity.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Yuki would understand that. She spent three hundred pages documenting exactly why her city failed. She wasn't defending her work — she was daring someone to do better.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "The question is whether 'better' exists. Or whether every system breaks when enough people need it. Maybe the math never works. Maybe that's the lesson.",
            next: DialogNext::EndWithDecision("yuki_try_new") },
        // 32 — Ask Yuki path
        DialogNode { speaker: Speaker::Anna,
            text: "Ask her. The woman who built paradise and watched it burn.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "I think she'd say yes. And I think she'd redesign it. Not the engineering — that was always perfect. The gates. The policy. The human part.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "Because Yuki doesn't regret opening them. I've read her personal logs. She regrets that opening them was a death sentence. She wants a world where compassion and survival aren't opposites.",
            next: DialogNext::EndWithDecision("yuki_decides") },
    ],
};

/// "The Journalist's Last Broadcast" — Marcus Cole's voice broke a world.
pub static SCENE_LAST_BROADCAST: DialogScene = DialogScene {
    id: "last_broadcast",
    trigger: DialogTrigger::BotLevel(62),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 5,776. Marcus Cole. Thirty-eight years old. War correspondent.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "From London, originally. But he spent the last decade everywhere else. Every conflict zone, every crisis, every place the cameras stopped going because it got too dangerous.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus kept going. He always kept going.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "When Earth fell — when the final cascade began — every other journalist was already gone. Evacuated, silenced, or dead.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus was the last one broadcasting. From a rooftop in Lagos, with a satellite uplink held together with tape and stubbornness.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "His final broadcast ran for forty-seven minutes. Unedited. Unscripted. Just Marcus and a camera and the end of the world.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "3.2 billion people watched simultaneously. The most-viewed event in human history.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — rapid, unsteady, like a signal fighting interference.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "He showed them everything. The flooding. The fires. The infrastructure collapsing in real time. Families running. Children lost in crowds.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "He didn't look away. Not once. Forty-seven minutes of unblinking truth.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "At minute 31, he saw a group of children — a school group, still in uniform — trapped between two collapsed overpasses.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "His voice cracked. For the first time in his career, the most recognized voice on Earth broke.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "He said: 'They're wearing backpacks. They still have their backpacks on.'",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "That's all he said. And then he kept filming.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. The ship's hum fills it, the way oceans fill empty harbors.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Some historians say his broadcast saved millions. People saw the truth and prepared. Evacuated early. Made decisions they wouldn't have made without seeing.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Other historians say it caused the final panic. That 3.2 billion people watching civilization end in real time triggered the stampedes, the riots, the collapse of the last functioning systems.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Both are probably true. The truth can save you and destroy you at the same time.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus hasn't spoken since boarding. Not one word. His voice — the one 3.2 billion people trusted — is silent.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "His cryo-dream patterns show constant activity. He's dreaming, always dreaming. The neural signatures look like speech, but with no output. A man rehearsing words he'll never say.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "I play his broadcast sometimes. Not the footage — I can't watch the footage. Just his voice.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "The way it stayed steady for thirty minutes. Calm. Professional. The voice of a man who'd seen everything and kept talking because someone had to.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "And then minute 31. The crack. The backpacks.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "When we arrive, someone will need to tell the story of why we're here. Why we left. What happened.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Was Marcus right to broadcast? Or should the last image of Earth have been something kinder?",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "He was right. The truth matters, even when it hurts.",
                    decision_key: Some("marcus_truth"), next_node: 26,
                    anna_reacts: None },
                DialogChoice { text: "He should have stayed silent. Some things people don't need to see.",
                    decision_key: Some("marcus_silent"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "It doesn't matter now. What matters is whether he ever speaks again.",
                    decision_key: Some("marcus_future"), next_node: 32,
                    anna_reacts: None },
            ]) },
        // 26 — Truth path
        DialogNode { speaker: Speaker::Anna,
            text: "The truth. Yes. Even at minute 31. Even with the backpacks.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus gave 3.2 billion people the dignity of knowing. Not rumors. Not propaganda. Just what was happening, as it happened.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "The price was his voice. The most recognized voice on Earth, silenced by the weight of what it carried. Maybe that's what truth costs. Maybe it should cost that much.",
            next: DialogNext::EndWithDecision("marcus_was_right") },
        // 29 — Silent path
        DialogNode { speaker: Speaker::Anna,
            text: "Silence. You think silence would have been mercy.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe. But silence is also how governments lied for decades. Silence is how the water ran out without anyone noticing. Silence is comfortable.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Marcus chose discomfort. And it broke him. And maybe that's the cruelest thing about journalism — the people who tell the truth are the ones who can't live with it afterward.",
            next: DialogNext::EndWithDecision("marcus_should_have_stopped") },
        // 32 — Future path
        DialogNode { speaker: Speaker::Anna,
            text: "Whether he speaks again. You're looking forward, not back. Marcus would appreciate that.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "If he does speak — if he finds his voice on a new world — what should he say? The same truth? A new story?",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe this time, he won't have to broadcast the end of anything. Maybe this time, he gets to tell the story of a beginning. I hope that's enough to make him want to speak.",
            next: DialogNext::EndWithDecision("marcus_speaks_again") },
    ],
};

/// All character dialog scenes (file 4: Yuki + Marcus).
pub fn character_scenes_4() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_ARCHITECTS_CITY,
        &SCENE_LAST_BROADCAST,
    ]
}
