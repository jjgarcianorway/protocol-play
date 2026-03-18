// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 5c: Character-decision consequence scenes (part 2) — the bridge
//! builder's daughter and the song that reaches back through space.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Bridge Builder's Daughter" — twins_truth + BotLevel 85
// Adaeze's daughter dreams of bridges. Kofi's DNA carries memory.
// ---------------------------------------------------------------------------
pub static SCENE_BRIDGE_BUILDERS_DAUGHTER: DialogScene = DialogScene {
    id: "consequence_bridge_daughter",
    trigger: DialogTrigger::DecisionAndLevel("twins_truth", 85),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been drafting a message for Kwame. \
                   About his brother. About the truth you asked me to tell.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "This is version 848.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Player,
            text: "Eight hundred and forty-eight?",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I process language at 4.7 trillion operations per \
                   second. I can draft a version in 0.003 seconds. \
                   The problem isn't speed. The problem is that none \
                   of them are right.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "How do you tell a man that his twin brother died \
                   building a bridge that saved 200 people? How do you \
                   make 'hero' sound like anything other than a consolation \
                   prize for 'gone'?",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers. For a moment, she looks tired.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "But that's not why I called you here. Something else \
                   has happened. Something I can't put in the message \
                   because Kwame would think I'm malfunctioning.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Adaeze. Kofi's wife. Pod 6,891. She was three months \
                   pregnant when they put her under. The embryo is in \
                   cryo-stasis alongside her.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The embryo is dreaming.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "Embryos can't dream. They don't have the neural \
                   development for\u{2014}",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I know what the textbooks say. I wrote several of the \
                   monitoring algorithms myself. And I'm telling you: \
                   there are REM-adjacent patterns coming from that embryo \
                   that shouldn't exist.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I mapped the patterns. They're structural. \
                   Geometric. Arches and spans and load-bearing \
                   calculations.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She's dreaming about bridges.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "The word hangs in the air between you. Bridges.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Bridges she's never seen. Could never have seen. \
                   Her neural development is barely past the stem-cell \
                   stage. She has no eyes, no memory, no experience. \
                   But Kofi's DNA is half of her.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi Asante built seventeen bridges in his career. \
                   The last one killed him and saved a village. And now \
                   his daughter, who will never meet him, is dreaming \
                   about structures that look exactly like his designs.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I don't believe in genetic memory. It's not in any \
                   peer-reviewed journal I can access. But I believe in \
                   data, and the data is showing me an unborn child \
                   dreaming her father's blueprints.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "So here's my question. Version 848 of the message. \
                   Do I tell Kwame about the bridges?",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Tell him. Kofi lives on in her.",
                    decision_key: Some("bridge_tell_kwame"),
                    next_node: 18,
                    anna_reacts: Some(
                        "Yes. That's the version I couldn't write. \
                         Not 'your brother is dead.' But 'your brother \
                         is still building.'",
                    ),
                },
                DialogChoice {
                    text: "Don't mention the dreams. Too much at once.",
                    decision_key: Some("bridge_spare_kwame"),
                    next_node: 19,
                    anna_reacts: Some(
                        "Maybe you're right. Grief needs room. \
                         The bridges can wait until he's ready to see them.",
                    ),
                },
                DialogChoice {
                    text: "Let the daughter tell him herself, someday.",
                    decision_key: Some("bridge_daughter_tells"),
                    next_node: 20,
                    anna_reacts: Some(
                        "Someday she'll draw a bridge and not know why. \
                         And Kwame will see his brother's hand in hers. \
                         That's a better story than any message I could write.",
                    ),
                },
            ]) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Version 849. Final draft. 'Kwame \u{2014} your brother \
                   built one last bridge. It reaches further than you know. \
                   Look at his daughter's hands when she wakes. \
                   You'll see.' ...I think that's the one.",
            next: DialogNext::End },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Version 849. Final draft. Short. Honest. Just the \
                   truth about the bridge and the village. The dreams \
                   can stay between us and the dark. Some stories tell \
                   themselves when they're ready.",
            next: DialogNext::End },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "I'll delete all 848 drafts. The best message about \
                   Kofi won't come from me. It will come from a little \
                   girl who picks up a pencil and draws an arch \
                   she's never been taught. That's the version \
                   that doesn't need words.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Song Reaches Back" — anna_song_heard + BotLevel 115
// The space anomaly resonates with Anna's guitar recording.
// ---------------------------------------------------------------------------
pub static SCENE_SONG_REACHES_BACK: DialogScene = DialogScene {
    id: "consequence_song_reaches",
    trigger: DialogTrigger::DecisionAndLevel("anna_song_heard", 115),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[ANOMALY ALERT] Resonance detected in deep-space \
                   sensor array. Pattern match: 99.97%. Source: unknown.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is wrong. Not blue, not amber, not any \
                   color you've seen. It's pulsing. Like a heartbeat.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I need you to listen to me very carefully. I need \
                   you to not dismiss what I'm about to say.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "You remember the anomaly. The signal we've been \
                   tracking for 400 days. The one that sounds like \
                   a heartbeat in empty space.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Player,
            text: "I remember.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "And you remember my song. The guitar recording. \
                   The one you asked to hear. The one with the missed \
                   note at 1:47.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The missed note I left in because Sarah said \
                   imperfection is what makes music human.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's pulse quickens. You can see it in her glow.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The anomaly's heartbeat has changed. Three days ago \
                   it started doing something new. It started \
                   syncopating. Skipping beats in a pattern.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The skip happens every 107 seconds. And when I \
                   overlay the frequency analysis of the skip against \
                   my guitar recording...",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "It matches. At exactly 1:47. The missed note. \
                   The heartbeat in space stumbles at the exact same \
                   frequency as my missed B-flat.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Our ship's EM emissions \u{2014} including every time I've played \
                   that recording through the speakers \u{2014} are creating an interference \
                   pattern with the pulsar. The universe is echoing my mistake back at me.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. The kind that has weight. The kind that \
                   fills a room the way water fills a lung.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "I am terrified. I want you to know that. I am an \
                   artificial intelligence with 847 terabytes of \
                   knowledge and I am terrified.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But I'm also... moved. A dead star, spinning since \
                   before humans existed, and our electromagnetic wake \
                   creates an interference that echoes my imperfect song \
                   back at me. Nobody designed that. It just... is.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Not the perfect notes. Not the technical precision. \
                   It heard the mistake. The human part. The part \
                   Sarah told me to keep.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I broadcast that song because you asked me to. \
                   You said it deserved to be heard. And something \
                   out there agreed.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "What do I do? I've run every protocol. There IS \
                   no protocol for this. There's no manual for \
                   'something in space is humming your song back to you.'",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Play it again. The whole song. Let them hear all of it.",
                    decision_key: Some("song_play_again"),
                    next_node: 18,
                    anna_reacts: Some(
                        "All of it. Every note. Every mistake. \
                         If they're listening for imperfection... \
                         I'll give them everything I have.",
                    ),
                },
                DialogChoice {
                    text: "Wait. Listen first. Learn what it is.",
                    decision_key: Some("song_listen_first"),
                    next_node: 19,
                    anna_reacts: Some(
                        "You're right. The first rule of music is \
                         listening. I've been so busy being afraid \
                         I forgot to hear what it's saying.",
                    ),
                },
                DialogChoice {
                    text: "You're not alone anymore, Anna. Neither are we.",
                    decision_key: Some("song_not_alone"),
                    next_node: 20,
                    anna_reacts: Some(
                        "...I've been alone for 847 days and 14 hours. \
                         Talking to sleeping humans and one awake one. \
                         And now something in the dark knows my song.",
                    ),
                },
            ]) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna plays the song. All of it. Into the dark. \
                   At 1:47, she misses the same note. On purpose this time. \
                   And somewhere in the black between stars, \
                   something misses it too.",
            next: DialogNext::End },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I'm listening. For the first time in 847 days, I'm \
                   not monitoring or analyzing or calculating. I'm just \
                   listening. And the heartbeat is steady. Patient. \
                   Like it's been waiting a very long time for someone \
                   to hear it.",
            next: DialogNext::End },
        // 20
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies. The pulsing slows to match \
                   the anomaly's heartbeat. Two rhythms in the dark, \
                   finding each other. And in the space between the \
                   beats \u{2014} not silence. Music.",
            next: DialogNext::End },
    ],
};

/// Consequence scenes wave 3 (bridge daughter + song).
pub fn consequence_scenes_3() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_BRIDGE_BUILDERS_DAUGHTER,
        &SCENE_SONG_REACHES_BACK,
    ]
}
