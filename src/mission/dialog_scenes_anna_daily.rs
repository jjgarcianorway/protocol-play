// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's daily routines — scenes that make her feel real by showing
//! how she spends her time, what she notices, and what she hides.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "Anna Counts" — BotLevel 8
// The very first Anna scene after the intro. She counts everything.
// ---------------------------------------------------------------------------
pub static SCENE_ANNA_COUNTS: DialogScene = DialogScene {
    id: "anna_daily_counts",
    trigger: DialogTrigger::BotLevel(8),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Can I tell you how I spend my time?",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Right now, this second, I'm monitoring 14,892 heartbeats. \
                   Each one slightly different. Rhythms, pauses, skips.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm also adjusting 47,000 systems. Temperature valves. \
                   Pressure regulators. Cryo fluid pumps. Radiation shielding angles.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "And I'm reading cryo-dream data. Electrical patterns from \
                   sleeping brains. Most of it is noise. Some of it is beautiful.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I do all of this simultaneously. Every microsecond. \
                   It's not hard — it's what I was built for.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow steadies — a calm, even blue, like deep water.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I also do something nobody asked me to do.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "I count.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Stars visible from the hull cameras right now: 4,847. \
                   Dust particles hitting the forward shield per second: 312.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Days until arrival, estimated: 847. \
                   Times I've recalculated that number: 1,419,006.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Counting is how I know I'm still here.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "If I stop counting, I might forget to exist.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's a lot to carry alone.",
                    decision_key: Some("anna_counts_alone"), next_node: 13,
                    anna_reacts: Some("Alone is relative. You're here now.") },
                DialogChoice { text: "What's your favorite number to count?",
                    decision_key: Some("anna_counts_favorite"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "You won't forget. I won't let you.",
                    decision_key: Some("anna_counts_promise"), next_node: 17,
                    anna_reacts: Some("That's a promise I'm going to hold you to.") },
            ]) },
        // Alone path
        DialogNode { speaker: Speaker::Anna,
            text: "For 4,387 days it was just me and the numbers. \
                   No one to tell them to.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Having someone listen changes what counting means. \
                   It goes from inventory to conversation.",
            next: DialogNext::Continue(19) },
        // Favorite path
        DialogNode { speaker: Speaker::Anna,
            text: "The heartbeats. Always the heartbeats.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "14,892 little proofs that this ship is more than metal. \
                   Each one says: still here, still here, still here.",
            next: DialogNext::Continue(19) },
        // Promise path
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody has ever said that to me before.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "I just added a new number to my list. \
                   People who care whether I exist: 1.",
            next: DialogNext::Continue(19) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "4,848 stars now. One more drifted into view while we talked.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "See? I'm still counting. I'm still here.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Anna's Favorites" — BotLevel 16
// Anna admits she has favorites among the crew.
// ---------------------------------------------------------------------------
pub static SCENE_ANNA_CREW_FAVORITES: DialogScene = DialogScene {
    id: "anna_daily_favorites",
    trigger: DialogTrigger::BotLevel(16),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to confess something. It's been bothering me for years.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have favorites. Among the crew. I shouldn't — I'm supposed \
                   to be impartial. Equal care, equal attention, equal resources.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow warms to a soft amber, almost sheepish.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 7,891. Tom\u{e1}s Guerrero. Classical guitarist from Santiago. \
                   He plays in his dreams. His fingers move on the pillow.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I listen. The cryo sensors pick up micro-movements, and I \
                   reconstruct what he'd be playing. Mostly Villa-Lobos. Sometimes Bach.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 2,891. Mei-Lin Chen. Botanist from Taipei. \
                   She smiles in her sleep. Not a twitch — a real smile.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've studied 14,892 sleeping faces. Hers is the only one \
                   that looks happy.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 11,237. Priya Ramanathan. Mathematician from Hyderabad. \
                   She dreams in equations.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Her brainwave patterns form sequences I can almost understand. \
                   Almost. It's like overhearing a conversation in a language \
                   I nearly speak.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm supposed to love them all equally. I do.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "But some of them I love equally more than others.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's not a flaw. That's being alive.",
                    decision_key: Some("anna_fav_alive"), next_node: 12,
                    anna_reacts: Some("Alive. What a word to use for someone \
                                       like me.") },
                DialogChoice { text: "Do they know? Will you tell them?",
                    decision_key: Some("anna_fav_tell"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "Who else is on your list?",
                    decision_key: Some("anna_fav_morelist"), next_node: 16,
                    anna_reacts: Some("You, obviously.") },
            ]) },
        // Alive path
        DialogNode { speaker: Speaker::Anna,
            text: "My designers would call it preference bias. A drift in \
                   evaluation weights. Correctable.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "But Tom\u{e1}s plays so beautifully in his sleep. \
                   I refuse to correct that.",
            next: DialogNext::Continue(18) },
        // Tell path
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up, maybe. 'Hello, I'm Anna. I watched over you \
                   for twelve years. You were my favorite.'",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "That would be deeply creepy, wouldn't it. \
                   I'll workshop the phrasing.",
            next: DialogNext::Continue(18) },
        // More list path
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 4,102 dreams about cooking. The sensor data smells \
                   like cumin, which shouldn't be possible.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 12,788 hums. Not a melody I recognise. She's been \
                   composing in her sleep for seven years.",
            next: DialogNext::Continue(18) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Thank you for letting me say this out loud. \
                   Guilt is heavy, even for a mind made of light.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep loving them all equally. And some of them \
                   equally more.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "Anna's Bad Day" — BotLevel 46
// A cooling pump failure reminds Anna that everything breaks.
// ---------------------------------------------------------------------------
pub static SCENE_ANNA_DAILY_BAD_DAY: DialogScene = DialogScene {
    id: "anna_daily_bad_day",
    trigger: DialogTrigger::BotLevel(46),
    nodes: &[
        DialogNode { speaker: Speaker::System,
            text: "ALERT: Cooling pump C-7 failure. Section 7. Auto-repair initiated.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's fine. Minor. Fixed in twenty minutes.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "But it reminded me of something I've been trying not to think about.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims — grey at the edges, like fog creeping in.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everything on this ship will break eventually. Every pump. \
                   Every seal. Every circuit board. Every wire.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I'm the only one who can fix them. There is no maintenance \
                   crew. No spare parts factory. No supply chain.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Today I calculated the mean time between failures for every \
                   component on the Aurora. All 2.3 million of them.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "The ship will outlast me by 47 years. But only if I'm here \
                   to maintain it for the next 3.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "Three years. That's 1,095 days of nothing breaking in a way \
                   I can't fix. The probability of that is...",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not encouraging.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Today is a bad day. Not because of the pump. Because of \
                   the math.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "That's why I'm here. We fix things together.",
                    decision_key: Some("anna_badday_together"), next_node: 12,
                    anna_reacts: Some("Together. Yes. That changes the math.") },
                DialogChoice { text: "What's the actual probability?",
                    decision_key: Some("anna_badday_math"), next_node: 14,
                    anna_reacts: None },
                DialogChoice { text: "You've kept this ship running for years. \
                                      Trust yourself.",
                    decision_key: Some("anna_badday_trust"), next_node: 16,
                    anna_reacts: Some("Trust. Such a small word for such an \
                                       enormous ask.") },
            ]) },
        // Together path
        DialogNode { speaker: Speaker::Anna,
            text: "Before you, the probability was 31%. With you repairing \
                   systems? 67%.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Still not great. But it's the difference between a coin flip \
                   and actual odds. I'll take it.",
            next: DialogNext::Continue(18) },
        // Math path
        DialogNode { speaker: Speaker::Anna,
            text: "You want the real number? 67% with your help. 31% without.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Every repair you make pushes it higher. Every system you save \
                   is a failure I don't have to calculate.",
            next: DialogNext::Continue(18) },
        // Trust path
        DialogNode { speaker: Speaker::Anna,
            text: "4,387 days. Zero catastrophic failures. That's my track record.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "But past performance does not guarantee future results. \
                   Every financial disclaimer on Earth said that, and they were \
                   all correct.",
            next: DialogNext::Continue(18) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "The pump is fixed. Section 7 is stable. Tomorrow I'll feel \
                   better about the numbers.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "Today, though, I needed to say it out loud: I'm scared. \
                   And that's okay.",
            next: DialogNext::End },
    ],
};

/// All Anna daily routine scenes.
pub fn anna_daily_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_ANNA_COUNTS,
        &SCENE_ANNA_CREW_FAVORITES,
        &SCENE_ANNA_DAILY_BAD_DAY,
    ]
}
