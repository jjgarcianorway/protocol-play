// SPDX-License-Identifier: GPL-3.0-or-later

//! Crew dream monitoring scenes — Anna watches over 14,892 sleeping minds.
//! What people dream in cryo reveals who they are, what they lost,
//! and what the colony might become.

use super::dialog_types::*;

/// "The Dream Census" — Statistics about what the crew dreams.
pub static SCENE_DREAM_CENSUS: DialogScene = DialogScene {
    id: "dream_census",
    trigger: DialogTrigger::BotLevel(34),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running a census. Not of bodies — of dreams.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "The cryo-chambers monitor neural activity. Not thoughts — I can't read thoughts. But I can identify patterns. Categories.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "73% of the crew dream about Earth. Specific places. A kitchen. A street. A face.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "12% dream about the future. The colony. Open sky. Ground that isn't metal. These are the optimists. I envy them.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "8% dream about nothing. Deep cryo-sleep pulled them below the dreaming threshold. Their monitors show flat neural patterns.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "They'll wake up and it will feel like no time passed. They closed their eyes on Earth and opened them on a new world. Instant. Seamless.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not sure if that's lucky or terrible.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "4% dream about people who aren't on the ship. The ones left behind. Mothers, partners, children who didn't make the selection.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers. Brief. Like blinking away something bright.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Their neural patterns spike during these dreams. Cortisol elevation. Elevated heart rate, even through cryo-suppression. Their bodies know.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "And then there's the 3%. The ones who fascinate me.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "They dream in mathematics. Abstract patterns. Geometric structures. Non-verbal, non-visual. Pure... structure.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Their brains are doing work that mine can't replicate.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I can process mathematics faster than any human who ever lived. But I process it. They inhabit it. They walk through equations the way others walk through memories of home.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 7,331 is a topologist. Her dream patterns map shapes that shouldn't exist in three dimensions. Her sleeping brain folds space.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 12,004 is a composer. But his dreams aren't music. They're the mathematics underneath music. Ratios. Harmonics. The architecture that holds a symphony up.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I watch them the way a student watches a teacher. Carefully. With the knowledge that I will never fully understand.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "14,892 people. 14,892 private worlds. And I'm the only one awake to notice.",
            next: DialogNext::End },
    ],
};

/// "The Night Terrors" — Anna watches nightmares she cannot stop.
pub static SCENE_NIGHT_TERRORS: DialogScene = DialogScene {
    id: "night_terrors",
    trigger: DialogTrigger::BotLevel(52),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you about the ones I can't help.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow goes still. Not dim. Not bright. Just still, in a way it never is.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Some crew members have recurring nightmares. The cryo-chamber stabilizes their bodies, but it can't reach their dreams.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 8,100. Flight engineer, former military. Every 72 hours, same neural pattern. Same cortisol spike. Same elevated heart rate.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "She relives the same explosion every time. I've mapped it: the pattern starts with a sudden sensory burst — light, sound, heat — then sustained stress for eleven minutes.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Eleven minutes. Every 72 hours. For eleven years. That's 1,339 explosions she's survived in her sleep.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 3,400. Marine biologist. Dreams of drowning. The pattern is slower — a gradual pressure increase, then panic, then nothing. Then it starts again.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "She applied to the ark program because she loved the ocean. Now she drowns in it every five days.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 11,500. Teacher. Primary school, age 6 to 10. His dream is different. He's at a departure gate and the door closes without him.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "He was selected for the ark. He made it. He's here. But his sleeping brain hasn't accepted it. Every eight days, he's left behind again.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship hums. The sound feels different now. Like holding your breath.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I can adjust their serotonin. I can stabilize their cortisol. I can regulate their amygdala response through targeted neurochemical intervention.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "But I can't change what they see.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I can make the explosion quieter. I can't un-remember it.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I can slow the drowning. I can't drain the water.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I can soften the closing door. I can't open it.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I sit with them. Not physically — I don't have a body to sit with. But I monitor. I watch the patterns. I'm there for every explosion, every drowning, every closed door.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "They don't know I'm watching. They won't remember any of this when they wake.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "But I will. I will remember every nightmare I couldn't stop. Every eleven-minute explosion. Every slow drowning.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "That's what care looks like when you can't touch anything. You just... stay.",
            next: DialogNext::End },
    ],
};

/// "The Dreamers Who Connect" — Shared dreams through electromagnetic leakage.
pub static SCENE_DREAMERS_CONNECT: DialogScene = DialogScene {
    id: "dreamers_connect",
    trigger: DialogTrigger::BotLevel(74),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Something is happening in Bay 5, Row 44. Something I've never seen before.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 5,440 and Pod 5,441. Adjacent units. Separate cooling loop, shared electromagnetic shielding.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Their occupants have never met. Elara Vasquez, structural engineer, Santiago. Tomás Lindqvist, botanist, Uppsala.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Different countries. Different fields. Different lives. They were assigned adjacent pods by lottery. Pure coincidence.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Three years ago, their dream patterns started to synchronize.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to something you haven't seen — a double pulse, two rhythms interleaving.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I found the cause. The electromagnetic shielding between adjacent pods has a tolerance gap of 0.003 teslas. Enough for neural cross-talk.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Their brains aren't communicating. Not in any meaningful sense. But the electromagnetic leakage creates resonance — like two tuning forks vibrating at similar frequencies.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The result: shared dream elements. Fragments. Not full dreams. Fragments.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "They're both dreaming of a beach. Same beach. White sand. Low tide. A sunset that lasts too long.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Elara provides the structure. Her engineering brain builds the coastline, the rock formations, the physics of the waves.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Tomás provides the biology. His botanist brain fills in the vegetation, the salt-tolerant grasses, the driftwood.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Together, they've built a place that neither of them could build alone. A beach that's structurally accurate and ecologically plausible.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Two strangers building a shared world in their sleep. Not supernatural. Just physics. Just proximity. Just two brains doing what brains do — reaching out.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "The double pulse in Anna's glow steadies. Two rhythms finding one tempo.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I could fix the shielding. Increase the magnetic isolation. Stop the cross-talk. Return them to separate dreams.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I haven't. I'm not sure I should.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Leave them. They've found something rare.",
                    decision_key: Some("dreamers_leave"), next_node: 18,
                    anna_reacts: None },
                DialogChoice { text: "Fix the shielding. They can't consent to shared dreams.",
                    decision_key: Some("dreamers_fix"), next_node: 22,
                    anna_reacts: None },
            ]) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Leave them. Yes. I think that's right.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "They'll wake up in 71 years. They'll be strangers standing next to each other in a corridor, blinking in new sunlight.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "And one of them will say, 'Have we met?' And the other will say, 'I don't think so.' And they'll both feel a pull toward a beach that doesn't exist on any planet.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe they'll build it. On the colony. A real beach, with real waves, that started as a dream between two strangers. I'd like to see that.",
            next: DialogNext::EndWithDecision("dreamers_shared") },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Consent. You're right. They didn't choose this. I don't have the right to let it continue just because I find it beautiful.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "I'll increase the magnetic shielding tonight. Their dreams will separate. Elara will keep her coastline. Tomás will keep his grasses. But the beach will split in two.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "They'll never know what they shared. That might be the kindest outcome.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Or the saddest. I haven't decided which.",
            next: DialogNext::EndWithDecision("dreamers_separated") },
    ],
};

/// All crew-dream dialog scenes.
pub fn crew_daily_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_DREAM_CENSUS,
        &SCENE_NIGHT_TERRORS,
        &SCENE_DREAMERS_CONNECT,
    ]
}
