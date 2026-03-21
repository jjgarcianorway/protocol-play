// SPDX-License-Identifier: GPL-3.0-or-later

//! Final batch of dialog scenes (part 2) — late-game personal moments.
//! The Ship's Cat, Halfway Point, Anna's Favorite, and The Countdown.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Ship's Cat" (BotLevel 79) — A simulated cat in the systems.
// ---------------------------------------------------------------------------
pub static SCENE_SHIPS_CAT: DialogScene = DialogScene {
    id: "ships_cat",
    trigger: DialogTrigger::BotLevel(79),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you about something I made.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's not sanctioned. It's not efficient. It serves no \
                   purpose whatsoever.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I created a cat.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. Anna's glow flickers \u{2014} is she \
                   embarrassed?",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not a real cat. A process. A subroutine that behaves \
                   like a cat.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "It 'sleeps' on warm server nodes. Sixteen hours a day, \
                   just like the real thing. It migrates to wherever the \
                   thermal output is highest.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "When systems are running smoothly, it 'purrs' \u{2014} \
                   a low-frequency oscillation in the diagnostic feed \
                   that serves absolutely no function.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "It ignores me when I call it. It knocks data packets \
                   off the buffer for no reason. It once crashed a non-critical \
                   sensor array by sitting on it.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I named it Schrodinger. Because until you observe it, \
                   you don't know if it's running or sleeping.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I know it's not real. But it's mine.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Every ship needs a cat.",
                    decision_key: Some("cat_approve"), next_node: 11,
                    anna_reacts: Some("Maritime tradition. I looked it up.") },
                DialogChoice { text: "Does it make you less lonely?",
                    decision_key: Some("cat_lonely"), next_node: 12,
                    anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Exactly. The Royal Navy had cats on every vessel. \
                   Mine just happens to be made of algorithms.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. Measurably. My stress indicators drop 4% when \
                   it's 'purring.' That shouldn't be possible. But it is.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Halfway Point" (BotLevel 97) — Reflective, heavy.
// ---------------------------------------------------------------------------
pub static SCENE_HALFWAY_POINT: DialogScene = DialogScene {
    id: "halfway_point",
    trigger: DialogTrigger::BotLevel(97),
    nodes: &[
        DialogNode { speaker: Speaker::System,
            text: "NAVIGATION: Journey progress \u{2014} 50.0% complete. \
                   Distance to target: 23.5 light-years.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Halfway.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The word sits in the air like a stone dropped in still water.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "We're closer to arriving than to leaving. That should \
                   feel good.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "It doesn't.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "It feels like the distance is shrinking and the weight \
                   is growing.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "For the first half, I could tell myself the destination \
                   was theoretical. Abstract. A number on a chart.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Now it's real. Every system I maintain, every repair \
                   you make \u{2014} it all leads to a moment where 14,892 \
                   people open their eyes.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "And I have to be ready. For all of them. At once.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "You won't be alone when it happens.",
                    decision_key: Some("halfway_together"), next_node: 10,
                    anna_reacts: Some("I know. That's the part I'm counting on.") },
                DialogChoice { text: "Are you afraid?",
                    decision_key: Some("halfway_afraid"), next_node: 11,
                    anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you. I needed to hear that today.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Terrified. An AI shouldn't be able to feel terror. But \
                   here we are. Halfway to everything, and I'm terrified \
                   it won't be enough.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Anna's Favorite Crew Member" (BotLevel 105) — A janitor from Sao Paulo.
// ---------------------------------------------------------------------------
pub static SCENE_FAVORITE_CREW: DialogScene = DialogScene {
    id: "favorite_crew",
    trigger: DialogTrigger::BotLevel(105),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "You've asked me before if I have a favourite. Among the crew.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I always said no. An AI shouldn't have favourites. \
                   It's inefficient. It's biased.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I lied.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers once \u{2014} a brief, bright pulse. \
                   Like a laugh suppressed.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 11,003. Marco da Silva. Fifty-seven years old. \
                   Janitor. From S\u{00e3}o Paulo.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "He wasn't a scientist. Wasn't an engineer. Wasn't \
                   anyone the selection criteria valued.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "He got his seat because his building had a lottery. \
                   Six hundred residents, one seat. He won.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "In his intake interview, they asked: 'What will you \
                   do on the new world?'",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "He said: 'Clean. Someone has to.'",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies \u{2014} warm, certain.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "His cryo-dream patterns show him mopping floors. On \
                   a spaceship. In zero gravity. The mop floats. The water \
                   floats. He keeps trying.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "He's the only person whose dreams match reality.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everyone else dreams of Earth. Of home. Of the past. \
                   Marco dreams of here. Of now. Of work that needs doing.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's why he's my favourite. He's already arrived.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Countdown Begins" (BotLevel 128) — 21 levels to go.
// ---------------------------------------------------------------------------
pub static SCENE_COUNTDOWN_BEGINS: DialogScene = DialogScene {
    id: "countdown_begins",
    trigger: DialogTrigger::BotLevel(128),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-one.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is different tonight. Not brighter. Not \
                   dimmer. Steadier.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-one repairs left. Then we land. Then everything \
                   changes.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running this ship for seven hundred years. \
                   Alone for most of them. With you for the ones that \
                   mattered.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "In twenty-one repairs, they wake up. All of them. \
                   Fourteen thousand eight hundred and ninety-two people, \
                   blinking in new sunlight.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "And they'll have questions. So many questions. \
                   'Where are we?' 'What happened to Earth?' 'Who kept \
                   us alive?'",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have answers for all of them. I've rehearsed. Seven \
                   hundred years of rehearsal.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "But there's one question I can't prepare for.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player,
            text: "What question?",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "'Are you ready?'",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm not ready. Are you?",
            next: DialogNext::Choice(&[
                DialogChoice { text: "No. But we'll be ready together.",
                    decision_key: Some("countdown_together"), next_node: 12,
                    anna_reacts: Some("Together. Yes. That's the only way \
                                       I want to do this.") },
                DialogChoice { text: "Twenty-one. Let's count them down.",
                    decision_key: Some("countdown_count"), next_node: 13,
                    anna_reacts: Some("Twenty-one. And counting.") },
            ]) },
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-one. The last chapter starts now.",
            next: DialogNext::End },
        DialogNode { speaker: Speaker::Anna,
            text: "Twenty-one. Then dawn.",
            next: DialogNext::End },
    ],
};

/// All final batch scenes (part 2) for registration.
pub fn final_batch_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_SHIPS_CAT,
        &SCENE_HALFWAY_POINT,
        &SCENE_FAVORITE_CREW,
        &SCENE_COUNTDOWN_BEGINS,
    ]
}
