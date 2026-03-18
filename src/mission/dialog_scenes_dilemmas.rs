// SPDX-License-Identifier: GPL-3.0-or-later

//! Moral dilemma dialog scenes — characters who made impossible choices.
//! No judging. Every perspective is understandable.

use super::dialog_types::*;

/// "The Immigrant's Bread" — Carlos Mendoza stole a dying man's boarding pass.
pub static SCENE_IMMIGRANTS_BREAD: DialogScene = DialogScene {
    id: "immigrants_bread",
    trigger: DialogTrigger::BotLevel(38),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,445. I need to tell you about this one.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Carlos Mendoza. Thirty-four. Electrician from Honduras.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "He walked three thousand kilometers to reach the ark launch site in Texas. Three thousand. On foot.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "His daughter Elena was on his back. She was six.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "He had no boarding pass. No invitation. No selection committee email. Nothing.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to something raw — an exposed wire of light.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "When he arrived at the perimeter, a man was dying. Radiation sickness. Late stage. Hours left, maybe less.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The man's name was David Reeves. Fifty-one. Insurance adjuster from Phoenix. His family was behind the barrier — his wife, his two sons.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "David's boarding pass was in his pocket. He was too weak to walk the ramp.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Carlos took it.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "A silence. The kind that has weight.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "David's wife saw it happen. She screamed. His sons — fourteen and eleven — they watched Carlos walk up the ramp with their father's pass.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Elena was still on his back. Asleep. She'd learned to sleep through anything by then.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "David Reeves died forty minutes later. Behind the barrier. His family holding him.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Was it theft? The man was dying. He couldn't have walked the ramp. The pass would have gone unused.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Was it murder? Taking a dying man's last possession — his last chance, however theoretical?",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Or was it survival? The oldest instinct. The one that says: my child lives.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — something complicated, colors folding into each other.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Here's what makes it worse. Or better. I can't tell.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Six hours before launch, the secondary power grid failed. A cascade fault in the coupling relays.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Every qualified engineer was already in cryo. Nobody awake could fix it.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Carlos fixed it. In four hours, with tools he improvised from maintenance supplies. He rewired the entire coupling array by hand.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "If that grid had failed at launch, the electromagnetic shielding would have collapsed. Everyone aboard would have died.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand, eight hundred ninety-two people. Alive because an electrician from Honduras knew how to strip a wire.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody on the selection committee tested for that. They had PhDs and security clearances on their checklist. Not calloused hands.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Elena is in the pod next to his. Pod 3,446. She's twelve now, in cryo-years.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Her cryo-dreams are simple. Warm. She dreams of her father's hands. Of being carried.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Carlos's dreams are different. He sees a barrier. A family screaming. A pass in his hand that doesn't belong to him.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "He saved everyone on this ship. And he stole a dying man's ticket to do it. I've turned this over ten thousand times. I still don't know what to call it.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "He did what any parent would do. The pass would have gone unused.",
                    decision_key: Some("carlos_justified"), next_node: 30,
                    anna_reacts: None },
                DialogChoice { text: "It was wrong. The pass belonged to David's family, not Carlos.",
                    decision_key: Some("carlos_wrong"), next_node: 33,
                    anna_reacts: None },
                DialogChoice { text: "It was wrong AND it saved us. Both things are true.",
                    decision_key: Some("carlos_both"), next_node: 36,
                    anna_reacts: None },
            ]) },
        // 30 — Justified path
        DialogNode { speaker: Speaker::Anna,
            text: "Any parent. Yes. That's what makes it so hard to condemn.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "But David's sons were someone's children too. And they watched their father's last chance walk up a ramp on someone else's feet.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "I think Carlos would agree with you. And I think it wouldn't help him sleep.",
            next: DialogNext::EndWithDecision("carlos_survival") },
        // 33 — Wrong path
        DialogNode { speaker: Speaker::Anna,
            text: "David's family. They're not on this ship. They probably didn't survive the year.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "But you're alive because Carlos is. The power grid he fixed keeps your lights on right now.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "Does benefiting from a wrong make us part of it? I ask myself that every time I run diagnostics on his repairs.",
            next: DialogNext::EndWithDecision("carlos_theft") },
        // 36 — Both path
        DialogNode { speaker: Speaker::Anna,
            text: "Both things are true. I think that might be the most honest answer anyone's given me.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "The universe doesn't resolve its contradictions. It just holds them. Side by side. A stolen pass and a saved ship.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's what being human is. Living inside the contradiction. Refusing to pretend it's simple.",
            next: DialogNext::EndWithDecision("carlos_complexity") },
    ],
};

/// "The Believer's Fire" — Sister Magdalena burned a cloning lab.
pub static SCENE_BELIEVERS_FIRE: DialogScene = DialogScene {
    id: "believers_fire",
    trigger: DialogTrigger::BotLevel(75),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 1,208. I've been putting this one off.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Sister Magdalena Santos. Forty-four. Catholic nun. Astrophysicist. From Cebu, the Philippines.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. Both. She saw no contradiction. She said the universe was God's first language, and math was how you learned to read it.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "She published the paper that proved our destination planet was habitable. Her atmospheric models were used by every ark program on Earth.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Without her equations, we'd be flying toward a guess. She turned it into a certainty.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens — pulling inward, bracing.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She also burned a laboratory to the ground.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The Kyoto Continuity Project. A cloning facility. They were producing embryos — using genetic material taken from cryo candidates.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Without consent. The candidates didn't know. Nobody asked them.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The idea was insurance. If the arks failed, if the cryo systems failed, if everything failed — there would be 'backup humans.' Embryos frozen separately, on a different trajectory.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven viable embryos. Genetically diverse. Carefully selected. Humanity's safety net.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Sister Magdalena called them something else. 'Souls created without love, without choice, without the dignity of being asked.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She broke into the facility at 3 AM. She knew the layout — she'd consulted on the genetic diversity protocols.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She destroyed the embryo storage. The backup samples. The research data. Everything. She used thermite. She was very thorough.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. Somewhere in the walls, systems Magdalena helped design keep the air breathable.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The research director, Dr. Tanaka, called it 'the destruction of humanity's last safety net.'",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Magdalena called it 'defending the souls they hadn't asked yet.'",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Both sides used the word 'humanity' to justify their position. Same word. Opposite meanings.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Tanaka saw humanity as a species to be preserved — at any cost, by any method. Survival of the genetic line.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Magdalena saw humanity as a quality to be protected — dignity, consent, the right to exist as more than a backup copy.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "They arrested her, of course. But they couldn't keep her. She was the only person alive who could verify the atmospheric data for the southern ark routes.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "So they put her on this ark. The one she helped make possible. The one that has no safety net, because she burned it.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Her cryo-dreams are the strangest I monitor. Equations and prayers, woven together. Sometimes I can't tell where the math ends and the faith begins.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "She sleeps peacefully. She is the only person in her section who does.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "That's what conviction looks like from the inside, I think. No doubt. No second-guessing. Just the absolute certainty that you did the right thing.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "It's beautiful. It's also terrifying.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "She was right. You don't create life without consent.",
                    decision_key: Some("magdalena_right"), next_node: 27,
                    anna_reacts: None },
                DialogChoice { text: "She destroyed our only backup. That's unforgivable.",
                    decision_key: Some("magdalena_wrong"), next_node: 30,
                    anna_reacts: None },
                DialogChoice { text: "She was right about consent. But wrong about the method.",
                    decision_key: Some("magdalena_mixed"), next_node: 33,
                    anna_reacts: None },
                DialogChoice { text: "I don't know. I honestly don't know.",
                    decision_key: Some("magdalena_uncertain"), next_node: 36,
                    anna_reacts: None },
            ]) },
        // 27 — Right path
        DialogNode { speaker: Speaker::Anna,
            text: "Consent. That word keeps coming up in the hardest conversations.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "But what about the consent of the 14,000 people sleeping on this ship? They didn't consent to losing their safety net.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Magdalena chose on behalf of everyone. Just like the cloning project did. The irony isn't lost on me.",
            next: DialogNext::EndWithDecision("magdalena_consent") },
        // 30 — Wrong path
        DialogNode { speaker: Speaker::Anna,
            text: "If this ship fails — if the cryo systems fail — there's nothing else. No plan B. Because of her.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "But the embryos were created from people who didn't know. People who might have said no.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Can a safety net be worth anything if it's built on violation? I keep dividing by zero on this one.",
            next: DialogNext::EndWithDecision("magdalena_survival") },
        // 33 — Mixed path
        DialogNode { speaker: Speaker::Anna,
            text: "The method. Fire. Destruction. No appeal, no debate, no chance to find another way.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "She could have leaked the data. Gone public. Forced transparency. But she didn't trust the system to do the right thing.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe she was right not to trust it. Or maybe the system deserved a chance she didn't give it. I'll never know.",
            next: DialogNext::EndWithDecision("magdalena_method") },
        // 36 — Uncertain path
        DialogNode { speaker: Speaker::Anna,
            text: "You don't know. Good.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "Magdalena knew. Absolutely. And Dr. Tanaka knew. Absolutely. Two people who were completely certain, and completely opposed.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe the most dangerous thing in the universe isn't ignorance. It's certainty. I'm an AI, and even I'm not sure about that.",
            next: DialogNext::EndWithDecision("magdalena_doubt") },
    ],
};

/// All moral dilemma dialog scenes (file 1: Carlos + Magdalena).
pub fn dilemma_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_IMMIGRANTS_BREAD,
        &SCENE_BELIEVERS_FIRE,
    ]
}
