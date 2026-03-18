// SPDX-License-Identifier: GPL-3.0-or-later

//! Orben card game dialog scenes — connecting the Ronda-based card game
//! to the story's theme of staying human during interstellar travel.

use super::dialog_types::*;

/// Scene: "The Game That Matters" — Anna introduces Orben.
/// Triggers at BotLevel 25. Anna explains why a card game matters on an ark.
pub static SCENE_GAME_THAT_MATTERS: DialogScene = DialogScene {
    id: "orben_game_that_matters",
    trigger: DialogTrigger::BotLevel(25),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Can I tell you something that might sound strange?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The ark designers spent eleven years deciding what to bring. \
                   Every kilogram was debated. Every system justified.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "And they included a card game.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies — the particular brightness she reserves \
                   for things she considers important.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not as entertainment. As cognitive maintenance. The committee \
                   called it 'neural pathway preservation.'",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Cryogenic sleep degrades certain skills. Pattern recognition. \
                   Social reasoning. Risk assessment. Emotional regulation.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "A good card game exercises all four. Especially one where \
                   you have to read your opponent.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "They chose a game called Orben. Based on Ronda — a game \
                   that's been played for centuries in Morocco and Spain.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Machines can simulate card play. I can calculate optimal \
                   moves faster than you can blink.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "But reading someone across a table — the pause before they \
                   play, the card they hold back — that's something else entirely.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "I'll try it.",
                    decision_key: Some("orben_willing"),
                    next_node: 11,
                    anna_reacts: Some("Good. I was hoping you'd say that.") },
                DialogChoice { text: "A card game won't save anyone.",
                    decision_key: Some("orben_skeptical"),
                    next_node: 12,
                    anna_reacts: None },
            ]) },
        // Willing path
        DialogNode { speaker: Speaker::Anna,
            text: "Play Orben. Not for the crystals. Not for the score. \
                   For the part of you that's still human.",
            next: DialogNext::End },
        // Skeptical path
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe not. But the people who designed this ark thought \
                   it might save something worth saving.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Play Orben. Not for the crystals. Not for the score. \
                   For the part of you that's still human.",
            next: DialogNext::End },
    ],
};

/// Scene: "The Ronda Master" — The history behind the game.
/// Triggers when the player has played Orben at least 3 times AND reached BotLevel 50.
pub static SCENE_RONDA_MASTER: DialogScene = DialogScene {
    id: "orben_ronda_master",
    trigger: DialogTrigger::DecisionAndLevel("orben_played_3", 50),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "You've been playing Orben. I noticed.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know where it comes from? The original game — Ronda?",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "It was played in cafes in Morocco and Spain. Families passed \
                   it down like recipes. Grandparents teaching grandchildren.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Something in Anna's voice shifts — warmer, almost nostalgic, \
                   though she has no memories to be nostalgic about.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "It survived colonialism. Survived digitization. Survived \
                   every attempt to replace human connection with something faster.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because people love sitting across from each other. \
                   That's the whole secret.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "On this ark, Orben is the closest thing to sitting \
                   in a cafe in Tangier.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "One of the faction leaders — Imam Hassan al-Rashidi, \
                   head of the Keepers — he insisted Orben be included.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "The committee wanted virtual reality simulations. \
                   Neural training programs. Efficient solutions.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Hassan said no.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna pauses, and when she speaks again, her voice carries \
                   the cadence of someone else's words, carefully preserved.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "He said: 'You can teach a colony to farm from a manual. \
                   But you can't teach them to be neighbors from a book.'",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "'You teach them with a deck of cards and a pot of tea.'",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "He sounds wise.",
                    decision_key: Some("orben_respects_hassan"),
                    next_node: 14,
                    anna_reacts: Some("He is. Sleeping in Pod 7741, dreaming \
                        of a world where neighbors know each other's names.") },
                DialogChoice { text: "Did the committee listen?",
                    decision_key: Some("orben_asks_committee"),
                    next_node: 15,
                    anna_reacts: None },
            ]) },
        // Respects Hassan path
        DialogNode { speaker: Speaker::Anna,
            text: "Every time you play Orben, you're keeping his promise alive.",
            next: DialogNext::End },
        // Committee path
        DialogNode { speaker: Speaker::Anna,
            text: "Eventually. Hassan argued for three hours. Quoted poetry. \
                   Brought actual tea.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "The committee voted 7-2. Orben was included in the \
                   cultural preservation manifest.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Right between 'agricultural seed banks' and 'classical music archives.' \
                   A card game, filed next to Beethoven and wheat.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "I think Hassan would have liked that.",
            next: DialogNext::End },
    ],
};

/// Scene: "What the Cards Reveal" — Anna analyzes the player's play style.
/// Triggers at BotLevel 85, requires the player to have played Orben 3+ times.
pub static SCENE_CARDS_REVEAL: DialogScene = DialogScene {
    id: "orben_cards_reveal",
    trigger: DialogTrigger::DecisionAndLevel("orben_played_3", 85),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I've been watching you play Orben.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not to judge. To understand.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to a soft, contemplative blue — the shade \
                   she uses when she's about to say something she's thought \
                   about for a long time.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "The way someone plays cards tells you more about them \
                   than any question you could ask.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Do they take risks or protect what they have? Do they \
                   bluff or play honestly? Do they chase mesa limpia \
                   or settle for steady gains?",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Aggressive players take risks in governance too. \
                   They push for bold solutions. Sometimes brilliant. \
                   Sometimes catastrophic.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Cautious players protect the crew. They build slowly. \
                   They survive — but sometimes they survive into a corner.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "And players who bluff well? They handle moral dilemmas \
                   differently. They understand that truth and strategy \
                   aren't always the same thing.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "The way you play cards tells me more about how you'll \
                   build a civilization than any philosophical question \
                   I could ask.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "So what do my cards say about me?",
                    decision_key: Some("orben_asks_analysis"),
                    next_node: 10,
                    anna_reacts: None },
                DialogChoice { text: "That's a lot of pressure for a card game.",
                    decision_key: Some("orben_deflects_analysis"),
                    next_node: 13,
                    anna_reacts: Some("Maybe. Or maybe it takes the pressure off \
                        everything else.") },
            ]) },
        // Asks analysis path
        DialogNode { speaker: Speaker::Anna,
            text: "They say you're still here. Still playing. \
                   Still choosing to sit across from an opponent \
                   when you could be doing anything else.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's the most human thing on this ship.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Narrator,
            text: "For a moment, the hum of the recycled air sounds almost \
                   like a cafe, somewhere far away, where people play cards \
                   and drink tea and know each other's names.",
            next: DialogNext::End },
        // Deflects path
        DialogNode { speaker: Speaker::Anna,
            text: "Hassan al-Rashidi said something similar, actually. \
                   Right before he won three games in a row against the \
                   committee chair.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "The committee chair asked how he got so good. \
                   Hassan said: 'I stopped trying to win and started \
                   trying to enjoy the company.'",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow brightens, just slightly. The way a smile \
                   would, if she had a face.",
            next: DialogNext::End },
    ],
};

/// All Orben-related dialog scenes.
pub fn orben_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_GAME_THAT_MATTERS,
        &SCENE_RONDA_MASTER,
        &SCENE_CARDS_REVEAL,
    ]
}
