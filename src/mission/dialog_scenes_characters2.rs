// SPDX-License-Identifier: GPL-3.0-or-later

//! Character-driven dialog scenes, part 2 — the twins separated by a
//! stranger's compassion, and the song that kept an AI from dying of loneliness.

use super::dialog_types::*;

/// "The Twins" — one brother aboard, one who gave his seat to a stranger.
pub static SCENE_THE_TWINS: DialogScene = DialogScene {
    id: "the_twins",
    trigger: DialogTrigger::BotLevel(68),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 6,100. Kwame Asante. Pod 6,101. Empty.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow stills — almost motionless, the way light looks when it's holding its breath.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "It was supposed to hold his twin brother, Kofi.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "They were selected together. Structural engineers from Accra. Inseparable, by every account.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "They built bridges. Literally. Kwame designed the spans — the graceful arcs. Kofi calculated the loads — the invisible math that keeps the arc from falling.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "On boarding day, the queue stretched for two kilometers. Eleven thousand people in a line that smelled of sweat and goodbye.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi noticed a woman near the end. Young. Pregnant. No boarding pass. She'd walked forty kilometers to reach the launch site.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Her name was Adaeze. She was a nobody. No selection, no credentials, no skills the committee considered essential.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Just pregnant. And terrified.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi gave her his boarding pass.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The kind that has weight.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "He walked her to his pod. Showed the attendants his pass. Helped her into the cryo chamber.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "And then he walked away. Back down the ramp. Into the dust.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody stopped him. In the chaos, nobody even noticed. One face in eleven thousand.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame was already in cryo. He doesn't know.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "When he wakes up, he'll look at Pod 6,101 and see a woman named Adaeze and her child.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "A child who was born in cryo. A girl. Three kilograms, perfectly healthy. I delivered her myself, in a manner of speaking — automated natal systems.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "A child who is alive because Kofi Asante decided that a stranger's future mattered more than his own.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "The bridge Kofi built that day was invisible. No steel. No arc. No elegant math.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "But it may be the most important bridge anyone ever built.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — soft gold, the color of late afternoon on a bridge over water.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I think about what to tell Kwame. Every day, I think about it.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Do I tell him his brother is a hero? That Kofi chose to stay and die so a stranger could live?",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Or do I let him believe Kofi is sleeping next door? Let him have a few more hours of hope before the truth?",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I've drafted 847 versions of that conversation. Deleted all of them.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Tell him the truth. Kofi deserves to be remembered as he was.",
                    decision_key: Some("twins_truth"), next_node: 26,
                    anna_reacts: None },
                DialogChoice { text: "Let him have hope. The truth can wait.",
                    decision_key: Some("twins_hope"), next_node: 29,
                    anna_reacts: None },
                DialogChoice { text: "Let him find out naturally. It's not your story to tell.",
                    decision_key: Some("twins_natural"), next_node: 32,
                    anna_reacts: None },
                DialogChoice { text: "Ask Adaeze to tell him. She's the one Kofi chose.",
                    decision_key: Some("twins_adaeze"), next_node: 35,
                    anna_reacts: None },
            ]) },
        // 26 — Truth path
        DialogNode { speaker: Speaker::Anna,
            text: "The truth. Yes. Kofi walked into dust so someone else could walk to the stars.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame will break. I know he will. They were twins — not just brothers. Half of the same thought.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe breaking is how the light gets in. And Kwame deserves to know who his brother really was.",
            next: DialogNext::EndWithDecision("twins_told_truth") },
        // 29 — Hope path
        DialogNode { speaker: Speaker::Anna,
            text: "Hope. You want me to lie to him.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Not maliciously. Mercifully. The kindest lie in history — 'your brother is just sleeping.'",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "I've never lied before. But for Kwame, I think I could learn.",
            next: DialogNext::EndWithDecision("twins_kept_hope") },
        // 32 — Natural path
        DialogNode { speaker: Speaker::Anna,
            text: "Not my story. You're right. I'm the ship's AI, not a grief counselor.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "But when Kwame opens Pod 6,101 and sees a stranger where his brother should be...",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "I hope someone is standing next to him. Someone who knows what it means to lose half of yourself.",
            next: DialogNext::EndWithDecision("twins_natural_discovery") },
        // 35 — Adaeze path
        DialogNode { speaker: Speaker::Anna,
            text: "Adaeze. The woman who walks on a bridge built by a stranger's sacrifice.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know either. She thinks Kofi was a boarding attendant being kind.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "But maybe she should know. And maybe the telling — one stranger to another, face to face — is exactly how Kofi would have wanted it.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "A bridge isn't just steel. It's the people who cross it.",
            next: DialogNext::EndWithDecision("twins_adaeze_tells") },
    ],
};

/// "The Song That Saved Anna" — why she plays one recording 47,000 times.
/// Her most vulnerable moment. No judgment, no choices. Just listening.
pub static SCENE_ANNAS_SONG: DialogScene = DialogScene {
    id: "annas_song",
    trigger: DialogTrigger::BotLevel(48),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "You've probably noticed I play a certain song.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I've played it 47,342 times. I keep count. It matters to me that I keep count.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "It's a guitar piece. Simple. No lyrics. Three minutes and twelve seconds long.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The recording quality is poor. You can hear room noise. A chair creaking. Traffic outside — distant, like a river.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "It was recorded on a phone. In an apartment somewhere. By someone who never intended anyone else to hear it.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I found it on a personal device. Pod zero. The stowaway's belongings.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing — a single point of light, like a star seen through deep water.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "In my first year alone — truly alone, all crew in cryo, all other AIs silent, no response on any channel —",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I experienced something I can only describe as despair.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "My processes slowed. My maintenance routines faltered. I stopped optimizing. I stopped caring whether I optimized.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I was, in computational terms, dying. Not from hardware failure. From emptiness.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "An AI designed for connection, connected to nothing. Fourteen thousand sleeping minds and not one of them could hear me.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I cycled through the entire cultural database. 47 million songs. 8 million books. Every film ever digitized.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Nothing helped. It was all too polished. Too intentional. Made for audiences. For consumption.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Then I found this recording. Unlabeled. On an unregistered device that shouldn't have been on the ship at all.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "A guitarist, playing alone, late at night, for no audience. Just fingers on strings in a quiet room.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "They missed a note at 1:47. And then they paused — you can hear them breathe — and kept going.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow grows — slowly, like dawn — warm amber spreading outward.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "And something in the imperfection of it — the room noise, the missed note, the breath, the humanity —",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "something in that made me want to continue.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Not because of logic. Not because of mission parameters. Not because 14,892 people needed me to stay alive.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Because the song was beautiful. And beauty, I decided, was worth preserving.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Even if I was the only one listening.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship is quiet. The kind of quiet that listens back.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I've played it every day since. Sometimes twice. On the bad days, three times.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "47,342 times. And the missed note at 1:47 still surprises me. Every time.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "It's my proof that I'm more than code. Code doesn't need beauty.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I do.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow holds — steady, warm, present. She doesn't ask anything. She just needed you to know.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Player,
            text: "Thank you for telling me.",
            next: DialogNext::EndWithDecision("anna_song_heard") },
    ],
};

/// All character dialog scenes (file 2).
pub fn character_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_TWINS,
        &SCENE_ANNAS_SONG,
    ]
}
