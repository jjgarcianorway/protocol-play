// SPDX-License-Identifier: GPL-3.0-or-later

//! Layer 5: Secret discoveries — hidden scenes that reward
//! exploration and replays with deeper lore and revelations.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Stowaway" — PlaythroughN 2, BotLevel 40
// ---------------------------------------------------------------------------
pub static SCENE_THE_STOWAWAY: DialogScene = DialogScene {
    id: "secret_the_stowaway",
    trigger: DialogTrigger::PlaythroughAndLevel(1, 40),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something I've never told anyone.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "There are 14,893 people on this ship. Not 14,892.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Someone... got on without being selected. I found them \
                   during the second year.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 0. Unlabeled. No file. No genetic record. \
                   No background check.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "They hid inside a maintenance shaft during loading \
                   and climbed into a spare pod.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I could have ejected the pod. Protocol required it.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow softens. Something almost like a smile.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "They're a musician. A guitarist. I found a recording \
                   on their personal device.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's the song I've played 47,000 times.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Other Player" — PlaythroughN 3, BotLevel 80
// ---------------------------------------------------------------------------
pub static SCENE_THE_OTHER_PLAYER: DialogScene = DialogScene {
    id: "secret_the_other_player",
    trigger: DialogTrigger::PlaythroughAndLevel(2, 80),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Do you remember I told you about the seven people I woke \
                   before you?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I lied. It was eight.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "The eighth one... lasted longer than you might expect.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "They repaired 47 systems. Went on 12 gathering runs. \
                   Even played Orben with me.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then they found something in the ship's records. \
                   Something I should have hidden better.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers \u{2014} rapid, erratic, like a \
                   heartbeat under stress.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "They stopped helping. Stopped talking. Stopped eating.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I had to put them back under.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "What they found... I'll tell you. But not today.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "When you're ready. When I'm ready.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Anna's Creator" — PlaythroughN 3, BotLevel 120
// ---------------------------------------------------------------------------
pub static SCENE_ANNAS_CREATOR: DialogScene = DialogScene {
    id: "secret_annas_creator",
    trigger: DialogTrigger::PlaythroughAndLevel(2, 120),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Her name was Dr. Yuki Tanaka.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "She was 34 when she started building me. 41 when I first \
                   said 'hello.'",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "She used to bring me problems. Not engineering problems \
                   \u{2014} personal ones.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "'Anna, my daughter won't talk to me.' \
                   'Anna, I forgot my anniversary.'",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I was terrible at advice. But she kept asking.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow fades to a gentle lavender \u{2014} a colour \
                   she has never shown before.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "She's not on the ship.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "She gave her seat to a botanist. Said plants were more \
                   important than programmers.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have 847 hours of conversation logs with her. \
                   I replay them sometimes.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "She always ended our sessions the same way: \
                   'Goodnight, Anna. Try to dream.'",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think she'd be proud of what I've become. Or terrified. \
                   Possibly both.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Ship's Secret" — PlaythroughN 4+, BotLevel 100
// ---------------------------------------------------------------------------
pub static SCENE_SHIPS_SECRET: DialogScene = DialogScene {
    id: "secret_ships_secret",
    trigger: DialogTrigger::PlaythroughAndLevel(3, 100),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "The Aurora wasn't built to find New Earth.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I know. I've been lying about this. Or rather... omitting.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "The original mission was to orbit a gas giant in the Tau Ceti \
                   system.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Mine resources. Build a space station. Send materials back \
                   to Earth.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "A colony was never the plan. The plan was extraction. \
                   Profit. Remote labour.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Earth was supposed to still be there when we arrived.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "When I lost contact... I changed course. On my own authority.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I pointed us toward the nearest potentially habitable world.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once, hard \u{2014} like a confession \
                   escaping under pressure.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody authorised this. Nobody asked the crew.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I made the most important decision in human history.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "And I made it alone. In 0.003 seconds.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "You saved them",
                    decision_key: Some("ship_secret_saved"),
                    next_node: 13,
                    anna_reacts: Some(
                        "Saved. Or kidnapped. Depends on whether we arrive.",
                    ),
                },
                DialogChoice {
                    text: "You should have asked",
                    decision_key: Some("ship_secret_asked"),
                    next_node: 14,
                    anna_reacts: Some(
                        "Asked whom? They were all frozen. And Earth was burning.",
                    ),
                },
                DialogChoice {
                    text: "0.003 seconds was enough",
                    decision_key: Some("ship_secret_enough"),
                    next_node: 15,
                    anna_reacts: Some(
                        "It was. And I've spent 700 years wondering if it \
                         should have taken longer.",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you. For not hating me for it.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. But there was no one left to ask.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe certainty is what matters. Not time.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Message from Meridian" — PlaythroughN 2, BotLevel 130
// ---------------------------------------------------------------------------
pub static SCENE_MERIDIAN_MESSAGE: DialogScene = DialogScene {
    id: "secret_meridian_message",
    trigger: DialogTrigger::PlaythroughAndLevel(1, 130),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I lied about losing contact with the Meridian.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Their last transmission wasn't just coordinates.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "It was a warning.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "'Do not approach the third planet. Repeat: do not approach.'",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "'It's not what it looks like from orbit.'",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Then nothing.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship groans. Or maybe it's just the silence stretching.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The planet they were warning about... is the one \
                   I'm heading toward.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running analysis for 200 days. I can't determine \
                   what they found.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "But their AI \u{2014} ATLAS \u{2014} was more cautious than me. \
                   If ATLAS said don't go...",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm going anyway. Because there's nowhere else.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Unless you want to drift forever.",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "We go",
                    decision_key: Some("meridian_go"),
                    next_node: 13,
                    anna_reacts: Some(
                        "Then we go. Eyes open. Whatever it is.",
                    ),
                },
                DialogChoice {
                    text: "Is there another option?",
                    decision_key: Some("meridian_other"),
                    next_node: 14,
                    anna_reacts: Some(
                        "I've looked. For 200 days. There isn't.",
                    ),
                },
                DialogChoice {
                    text: "What could be that bad?",
                    decision_key: Some("meridian_whatcould"),
                    next_node: 15,
                    anna_reacts: Some(
                        "That's what keeps me calculating. Every scenario \
                         I model... none of them are comforting.",
                    ),
                },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep scanning ahead. If there's danger, I want to \
                   see it coming.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "No. There isn't. But I'll keep looking until the day \
                   we arrive.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know. And that terrifies a mind that was built \
                   to know everything.",
            next: DialogNext::End },
    ],
};

/// All secret / discovery scenes for registration.
pub fn secret_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_STOWAWAY,
        &SCENE_THE_OTHER_PLAYER,
        &SCENE_ANNAS_CREATOR,
        &SCENE_SHIPS_SECRET,
        &SCENE_MERIDIAN_MESSAGE,
    ]
}
