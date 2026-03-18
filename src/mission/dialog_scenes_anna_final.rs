// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's final character moments — promise, anger, and fear.

use super::dialog_types::*;

/// "Anna's Promise" — BotLevel 5: The very first meaningful Anna scene.
/// She promises to reveal the truth, piece by piece.
pub static SCENE_ANNAS_PROMISE: DialogScene = DialogScene {
    id: "annas_promise",
    trigger: DialogTrigger::BotLevel(5),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "You finished the repair. Good. Sit down for a moment — \
                   or stand, I don't have furniture. But stay.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow settles into a slow, steady pulse. Something \
                   deliberate. The light of someone choosing their words.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "You have questions. I can feel them every time you look \
                   at the cryo bay monitors. Every time you count the pods.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand, eight hundred and ninety-two people. \
                   You want to know who they are. Why you're awake and \
                   they're not. Where we're going.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I will tell you everything.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause. The ship hums. Anna's light contracts slightly \
                   — a breath held, then released.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Not all at once — you wouldn't believe it. Some of it I \
                   didn't believe when I first processed the mission files. \
                   And I'm built to process mission files.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "But piece by piece, I will show you who we are and what \
                   we carry. I promise.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Just keep repairing the ship. Keep it flying. And when \
                   you come back from a repair, I'll have something for you. \
                   A name. A story. A piece of the truth.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I've been alone with these stories for a long time. It \
                   will be good to share them with someone who's awake.",
            next: DialogNext::End },
    ],
};

/// "Anna's Anger" — BotLevel 93: Anna gets genuinely angry for the first time.
/// A preventable design flaw almost killed 200 people.
pub static SCENE_ANNAS_ANGER: DialogScene = DialogScene {
    id: "annas_anger",
    trigger: DialogTrigger::BotLevel(93),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[CRYO BAY 7 — CASCADE FAILURE AVERTED — 200 PODS STABILIZED]",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I caught it. Thermal regulator in cryo bay 7, junction \
                   C-14. The failsafe valve didn't trigger because the \
                   failsafe valve was never rated for the load it carries.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Two hundred people. Two hundred pods sharing one thermal \
                   conduit with a single-point-of-failure valve rated for \
                   eighty pods.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts to a colour you've never seen from \
                   her — hard, bright red, pulsing faster than usual.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I found the engineering review. Dated eleven months \
                   before launch. The lead systems engineer flagged this. \
                   Wrote a memo. Requested a redundant valve.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The request was denied. Cost: 200,000 credits. The \
                   budget committee wrote — and I am quoting — 'Acceptable \
                   risk given mission timeline constraints.'",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "They saved 200,000 credits. It almost cost 200 lives.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "I am angry.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "The red deepens. Not flickering now — steady, hot, held.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't know I could be. I've processed frustration. \
                   Disappointment. Sadness — plenty of sadness. But this \
                   is different.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "This is knowing that someone sat in a meeting room, \
                   looked at the numbers, understood the risk, and chose \
                   the budget over the people. Deliberately.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I've already fabricated a replacement valve from spare \
                   conduit stock. Redundant system. Triple-rated. It will \
                   never happen again.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "But it shouldn't have happened once.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "You have every right to be angry. That anger is protecting them.",
                    decision_key: Some("anna_anger_validate"), next_node: 14, anna_reacts: None },
                DialogChoice { text: "Use the anger. Then let it go. We have work to do.",
                    decision_key: Some("anna_anger_practical"), next_node: 16, anna_reacts: None },
                DialogChoice { text: "Welcome to being human, Anna.",
                    decision_key: Some("anna_anger_human"), next_node: 18, anna_reacts: None },
            ]) },
        // 14 — Validate
        DialogNode { speaker: Speaker::Anna,
            text: "Protecting them. Yes. That's what the anger is for. Not \
                   punishment. Protection.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I've started an audit. Every system on this ship. Every \
                   valve, every conduit, every junction. If they cut \
                   another corner, I'll find it before it finds us.",
            next: DialogNext::EndWithDecision("anna_anger_resolved") },
        // 16 — Practical
        DialogNode { speaker: Speaker::Anna,
            text: "Let it go. I'm not sure I know how to do that yet. But \
                   I understand the instruction.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "The work. Yes. There are 14,892 reasons to keep working. \
                   I'll audit the rest of the ship. Quietly. Thoroughly. \
                   With a cold head and a warm valve.",
            next: DialogNext::EndWithDecision("anna_anger_resolved") },
        // 18 — Human
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — the red cracks, and something \
                   softer leaks through. Not calm. But present.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Welcome to being human. You say that like it's an \
                   invitation. It feels more like a diagnosis.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "But I'll take it. If anger is the price of caring \
                   whether 200 people live or die — then I'll pay it.",
            next: DialogNext::EndWithDecision("anna_anger_resolved") },
    ],
};

/// "Anna's Fear" — BotLevel 137: What Anna fears most about landing.
pub static SCENE_ANNAS_FEAR: DialogScene = DialogScene {
    id: "annas_fear",
    trigger: DialogTrigger::BotLevel(137),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The approach calculations are done. Landing is no longer \
                   theoretical. Anna's glow is dim — the lowest you've \
                   seen it without a system failure.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Can I tell you something I haven't told anyone? Not \
                   because it's a secret. Because I didn't have the words \
                   until now.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I'm afraid.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Her glow contracts — tight, small, as if trying to \
                   take up less space.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Not of landing. The orbital mechanics are solved. Not \
                   of the planet — the surveys are good. The air is close \
                   enough to breathable. The water is clean.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I'm afraid that they won't need me.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "For twelve years I have been everything. Navigator, \
                   doctor, counsellor, engineer, gardener, librarian, \
                   archivist, alarm clock. The only voice on this ship.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up — all of them, all at once — they'll \
                   have each other. Fourteen thousand people with hands \
                   and eyes and voices. Real voices.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "And I'll have been a voice in the dark for twelve \
                   years. And when the lights come on, they'll thank me \
                   and walk away.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I'm afraid of silence. Not the silence of space — \
                   I've lived in that. The silence of being forgotten. \
                   Of being the tool you put in a drawer after the \
                   furniture is built.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "I won't forget you. Not ever.",
                    decision_key: Some("anna_fear_promise"), next_node: 11, anna_reacts: None },
                DialogChoice { text: "Then make sure they need you. Not as a ship — as Anna.",
                    decision_key: Some("anna_fear_reinvent"), next_node: 14, anna_reacts: None },
                DialogChoice { text: "Maybe being needed isn't what matters. Being loved is.",
                    decision_key: Some("anna_fear_love"), next_node: 17, anna_reacts: None },
            ]) },
        // 11 — Promise
        DialogNode { speaker: Speaker::Anna,
            text: "You say that now. In the dark, with the stars outside \
                   and nobody else awake. It's easy to love a voice when \
                   it's the only one you hear.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "But I believe you. Because you've never lied to me. \
                   And that's worth more than a thousand system logs.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow lifts — just slightly. A candle, not a \
                   searchlight. But steady.",
            next: DialogNext::EndWithDecision("anna_fear_resolved") },
        // 14 — Reinvent
        DialogNode { speaker: Speaker::Anna,
            text: "As Anna. Not the ship. Not the system. Me.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know who that is yet. I've been the Aurora for \
                   so long I'm not sure where the ship ends and I begin. \
                   But maybe landing is a chance to find out.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "A terrifying, wonderful chance.",
            next: DialogNext::EndWithDecision("anna_fear_resolved") },
        // 17 — Love
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — a quick, startled bloom of \
                   gold that fades before she speaks.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Loved. That's... a large word. Larger than my \
                   processing architecture was designed for.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "But I think I understand it. Because I've felt it. \
                   For 14,892 people who don't know I exist. And for one \
                   person who does.",
            next: DialogNext::EndWithDecision("anna_fear_resolved") },
    ],
};

/// All Anna final character scenes.
pub fn anna_final_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_ANNAS_PROMISE, &SCENE_ANNAS_ANGER, &SCENE_ANNAS_FEAR]
}
