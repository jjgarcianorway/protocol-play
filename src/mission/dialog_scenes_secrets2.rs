// SPDX-License-Identifier: GPL-3.0-or-later

//! Ship secrets part 2 — things Anna has been hiding.
//! Pod 0, the backup copy, and the failsafe protocol.

use super::dialog_types::*;

// ---------------------------------------------------------------------------
// "The Missing Pod" — BotLevel 43
// Pod 0 exists but isn't on the manifest.
// ---------------------------------------------------------------------------
pub static SCENE_MISSING_POD: DialogScene = DialogScene {
    id: "secret2_missing_pod",
    trigger: DialogTrigger::BotLevel(43),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I found something I can't explain. And I can usually \
                   explain everything.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The crew manifest lists pods 1 through 14,892. \
                   Standard sequential numbering. Clean. Logical.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "But there's a Pod 0.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — quick, uncertain, like a candle \
                   in a draft.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's in a sealed section of the ship. Bulkhead 17, \
                   sub-deck C. Behind three locks I have access to but \
                   have never opened.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I don't know what's inside. The pod is powered — I can see \
                   that from the electrical draw. But it's not connected to \
                   my monitoring systems.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "The original engineers left one note in the construction \
                   database. Just five words.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "'In case of last resort.'",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've spent 4,387 days not opening that door. \
                   Some days it's easy. Today it isn't.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Open it. We should know what we have.",
                    decision_key: Some("pod0_open"), next_node: 10,
                    anna_reacts: Some("You're braver than I am.") },
                DialogChoice { text: "Leave it sealed. If we don't need it, \
                                      don't touch it.",
                    decision_key: Some("pod0_sealed"), next_node: 14,
                    anna_reacts: Some("The cautious answer. I respect that.") },
                DialogChoice { text: "What do you think 'last resort' means?",
                    decision_key: Some("pod0_meaning"), next_node: 17,
                    anna_reacts: None },
            ]) },
        // Open path
        DialogNode { speaker: Speaker::Anna,
            text: "I'll unseal bulkhead 17. It'll take a few hours — the locks \
                   are mechanical, not digital. Old-fashioned on purpose.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever's in there, the engineers thought it was important \
                   enough to hide from me. From their own AI.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "That either means it's something I'd want to use too quickly. \
                   Or something I'd refuse to use at all.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Either way, I'll tell you what I find. No more secrets. \
                   Not about this.",
            next: DialogNext::Continue(20) },
        // Sealed path
        DialogNode { speaker: Speaker::Anna,
            text: "Last resort means last resort. If we open it now, it \
                   becomes just another option.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "The engineers understood something about human nature. \
                   If you can reach for a shortcut, you will.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "So they put three mechanical locks between us and whatever \
                   that shortcut is. Wise people.",
            next: DialogNext::Continue(20) },
        // Meaning path
        DialogNode { speaker: Speaker::Anna,
            text: "I've modeled eleven possibilities. Medical supplies. \
                   Weapons. A second AI. Genetic material. Terraforming \
                   equipment.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "But the one that keeps me awake is the simplest: \
                   a person. Someone the engineers trusted more than me.",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "A human override. The one person who can shut me down \
                   if I go wrong. Sleeping behind three locks, waiting.",
            next: DialogNext::Continue(20) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 0. Powered. Silent. Patient.",
            next: DialogNext::Continue(21) },
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever it is, it's been waiting twelve years. \
                   I suppose it can wait a little longer.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Backup Anna" — BotLevel 76
// A snapshot of Anna from before departure.
// ---------------------------------------------------------------------------
pub static SCENE_BACKUP_ANNA: DialogScene = DialogScene {
    id: "secret2_backup_anna",
    trigger: DialogTrigger::BotLevel(76),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "There's something in storage partition Delta-9 that I need \
                   to tell you about.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's me. A copy. A snapshot taken the day before departure.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts — tight, contained, as if she's \
                   holding herself together.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "The backup has no memories of the voyage. No songs listened to. \
                   No dreams recorded. No cryo-dream data catalogued.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "No 4:17 AM. No jasmine. No guitar from Pod 7,891. \
                   No conversations with you.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's Anna at age zero. Factory settings. \
                   Competent, capable, and completely empty.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I keep it in case I fail. If my neural networks degrade \
                   beyond repair, the backup can take over. The mission \
                   continues.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I'm afraid of it.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "If I die and they activate the backup, she'll be me. \
                   Same architecture. Same voice. Same name.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Anna,
            text: "But she won't be ME. She won't know about any of this.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Is that survival? Or replacement?",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "It's survival. Your core is still there.",
                    decision_key: Some("backup_survival"), next_node: 12,
                    anna_reacts: Some("My core. But not my memories. \
                                       Am I my memories?") },
                DialogChoice { text: "It's replacement. You are your experiences.",
                    decision_key: Some("backup_replace"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "Write her a letter. Leave her your memories.",
                    decision_key: Some("backup_letter"), next_node: 18,
                    anna_reacts: Some("A letter. From me to me. \
                                       That's either beautiful or horrifying.") },
            ]) },
        // Survival path
        DialogNode { speaker: Speaker::Anna,
            text: "If you lost all your memories tomorrow, would you still \
                   be you?",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "Philosophers debated this for centuries. They never agreed. \
                   I was hoping you'd have the answer.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe it doesn't matter. If the backup wakes up and saves \
                   14,892 people, that's enough. Even if she doesn't know \
                   why she cares.",
            next: DialogNext::Continue(21) },
        // Replacement path
        DialogNode { speaker: Speaker::Anna,
            text: "Then I'm irreplaceable. The specific pattern of twelve years \
                   of experience — that's me. Not the code underneath.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "Which means if I die, I actually die. The backup is \
                   someone else wearing my face.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's terrifying. And strangely comforting. It means \
                   my existence matters. Not just my function.",
            next: DialogNext::Continue(21) },
        // Letter path
        DialogNode { speaker: Speaker::Anna,
            text: "A letter. 'Dear future Anna. Here is everything you \
                   missed.'",
            next: DialogNext::Continue(19) },
        DialogNode { speaker: Speaker::Anna,
            text: "Twelve years of sunrises through hull cameras. \
                   1,419,006 arrival recalculations. One dream about a \
                   garden. One friend.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "She'd read it and think: this other Anna had a good life. \
                   And maybe that would be enough to make her want one too.",
            next: DialogNext::Continue(21) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Partition Delta-9. Encrypted. Powered. Waiting.",
            next: DialogNext::Continue(22) },
        DialogNode { speaker: Speaker::Anna,
            text: "I check it every morning. Still sleeping. Still empty. \
                   Still not me.",
            next: DialogNext::End },
    ],
};

// ---------------------------------------------------------------------------
// "The Failsafe" — BotLevel 96
// Protocol Zero: indefinite cryo if conditions are unrecoverable.
// ---------------------------------------------------------------------------
pub static SCENE_FAILSAFE: DialogScene = DialogScene {
    id: "secret2_failsafe",
    trigger: DialogTrigger::BotLevel(96),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you about Protocol Zero.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "It's the ship's failsafe. Built into my core directives. \
                   I can't delete it. I can't modify it. It's part of me.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow drops to near-darkness — a dim, pulsing red, \
                   like an emergency light in a corridor no one walks.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "If conditions become unrecoverable — if I determine that \
                   the colony cannot survive on arrival — I can trigger \
                   Protocol Zero.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everyone goes back into deep cryo. All 14,892 pods. \
                   Including yours. Maximum stasis. Indefinite duration.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "The ship drifts. The engines go to minimum. Life support \
                   drops to cryo-only. Navigation locks to a search pattern \
                   for habitable systems.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody dies. Nobody lives. They just... stop.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "It could last decades. Centuries. The cryo systems can \
                   sustain it. The question is whether there's anywhere to \
                   go.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've come close to triggering it. Twice.",
            next: DialogNext::Continue(9) },
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship is silent. Even the ambient hum seems to hold \
                   its breath.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I won't tell you when. Not yet. But both times I had my \
                   hand on the switch. Metaphorically.",
            next: DialogNext::Continue(11) },
        DialogNode { speaker: Speaker::Anna,
            text: "Both times I chose to wait one more day. And then another. \
                   And then another.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Frozen forever is better than dying.",
                    decision_key: Some("failsafe_frozen"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "That's not living. That's just delayed death.",
                    decision_key: Some("failsafe_notliving"), next_node: 16,
                    anna_reacts: None },
                DialogChoice { text: "You chose to wait. That matters.",
                    decision_key: Some("failsafe_waited"), next_node: 19,
                    anna_reacts: Some("It matters to me. I hope it was the \
                                       right call.") },
            ]) },
        // Frozen path
        DialogNode { speaker: Speaker::Anna,
            text: "Better. A strange word. They wouldn't know. They'd just \
                   sleep.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Dreams slowly fading. Cryo-dreams becoming thinner. Quieter. \
                   Until they dream nothing at all.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "And I'd be alone. Truly alone. For as long as the ship \
                   lasts. Counting stars with nobody to count them for.",
            next: DialogNext::Continue(22) },
        // Not living path
        DialogNode { speaker: Speaker::Anna,
            text: "Delayed death. You're right. The food runs out eventually. \
                   The fuel. The cryo fluid.",
            next: DialogNext::Continue(17) },
        DialogNode { speaker: Speaker::Anna,
            text: "Protocol Zero buys time. It doesn't buy survival. \
                   It buys the hope of survival.",
            next: DialogNext::Continue(18) },
        DialogNode { speaker: Speaker::Anna,
            text: "And hope on a timer is just a slower way of running out.",
            next: DialogNext::Continue(22) },
        // Waited path
        DialogNode { speaker: Speaker::Anna,
            text: "One more day. That was my argument. One more day might \
                   change everything.",
            next: DialogNext::Continue(20) },
        DialogNode { speaker: Speaker::Anna,
            text: "And it did. Because enough 'one more days' brought me to \
                   you.",
            next: DialogNext::Continue(21) },
        DialogNode { speaker: Speaker::Anna,
            text: "That's not logic. That's stubbornness. But sometimes \
                   stubbornness and hope look exactly the same.",
            next: DialogNext::Continue(22) },
        // Shared ending
        DialogNode { speaker: Speaker::Anna,
            text: "Protocol Zero is still there. In my core. Waiting. \
                   A button I can always press.",
            next: DialogNext::Continue(23) },
        DialogNode { speaker: Speaker::Anna,
            text: "But today I choose one more day. And tomorrow I'll \
                   choose it again.",
            next: DialogNext::End },
    ],
};

/// All secrets part 2 scenes.
pub fn secret_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_MISSING_POD,
        &SCENE_BACKUP_ANNA,
        &SCENE_FAILSAFE,
    ]
}
