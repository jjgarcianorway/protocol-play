// SPDX-License-Identifier: GPL-3.0-or-later

//! Things crew members are hiding — forged credentials, secret letters, a living archive.

use super::dialog_types::*;

/// "The Forger" — BotLevel 39: Pod 7,200 has fabricated medical credentials.
pub static SCENE_THE_FORGER: DialogScene = DialogScene {
    id: "the_forger",
    trigger: DialogTrigger::BotLevel(39),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 7,200. Dr. Samuel Achebe. Forty-one. Listed as a trauma \
                   surgeon. Lagos University Medical School, class of 2121.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Lagos University Medical School closed in 2118. Three years \
                   before he supposedly graduated.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow holds steady — clinical blue. She's been sitting \
                   on this for a while.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "His credentials are fabricated. The degree, the residency, the \
                   surgical certification. All of it. Good enough to pass the \
                   selection committee's vetting.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "But I ran his name through every medical simulation we have. \
                   Emergency triage. Trauma response. Surgical procedures. He \
                   scored in the 94th percentile. Better than eleven of our \
                   fifteen credentialed surgeons.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "His real background: Lagos had an underground medical training \
                   network after the hospitals closed. He spent nine years teaching \
                   himself surgery from videos, textbooks, and cadaver labs in \
                   abandoned clinics.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "Two hundred procedures in field conditions during the West \
                   African resource crisis. 91% survival rate.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "He lied about who he is. But he didn't lie about what he can do.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player, // 8
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Skills matter more than paperwork. Keep his secret.",
                    decision_key: Some("forger_keep_secret"), next_node: 9,
                    anna_reacts: Some("The pragmatic answer. Also, I think, the kind one.") },
                DialogChoice { text: "Flag it for the colony council. Let them decide.",
                    decision_key: Some("forger_flag"), next_node: 11,
                    anna_reacts: Some("Transparent. Fair. He'll prove himself on his own terms.") },
                DialogChoice { text: "How many others on the ark lied their way aboard?",
                    decision_key: Some("forger_how_many"), next_node: 14, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 9 — Keep secret
            text: "I'll reclassify his file. Real skills, verified by simulation. \
                   Origin of training: field experience. No mention of Lagos University.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "When he wakes up, he'll be the best surgeon who doesn't have a \
                   diploma. On a planet with no licensing board, that won't matter.",
            next: DialogNext::EndWithDecision("forger_resolved") },
        DialogNode { speaker: Speaker::Anna, // 11 — Flag
            text: "I'll prepare a file. His real background, his simulation scores, \
                   his field record. Everything.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "The council can see what I see: a man who lied about a piece of \
                   paper, not about his ability to save lives.",
            next: DialogNext::Continue(13) },
        DialogNode { speaker: Speaker::Anna,
            text: "I hope they're wise enough to see the difference.",
            next: DialogNext::EndWithDecision("forger_resolved") },
        DialogNode { speaker: Speaker::Anna, // 14 — How many
            text: "Forty-seven. I've identified forty-seven passengers with \
                   credential discrepancies significant enough to suggest fabrication.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Some are like Samuel — genuinely skilled, just not formally \
                   certified. Others are less clear. A few took places from people \
                   who were qualified.",
            next: DialogNext::Continue(16) },
        DialogNode { speaker: Speaker::Anna,
            text: "But every one of them wanted to survive badly enough to risk \
                   everything on a forged document. That's not nothing.",
            next: DialogNext::EndWithDecision("forger_resolved") },
    ],
};

/// "The Love Letters" — BotLevel 59: Two strangers fell in love during boarding.
pub static SCENE_LOVE_LETTERS: DialogScene = DialogScene {
    id: "love_letters",
    trigger: DialogTrigger::BotLevel(59),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Pods 4,400 and 4,401. Adjacent berths. Assigned randomly. \
                   Javier Morales, 29, agricultural chemist. Priya Sharma, 31, \
                   water purification engineer. They'd never met before boarding.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "The boarding process took nine hours. Six of those spent in \
                   the cryo prep bay, waiting in adjacent chairs. They talked \
                   about everything. Soil chemistry. Monsoons. Their families.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "In the last hour, they went quiet. Not awkward quiet. The other \
                   kind. When you've said enough and silence feels like trust.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Before they went under, each wrote a letter. On paper — actual \
                   paper, from the prep bay clipboard. They folded them and slipped \
                   them into the lining of their pod walls.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "Neither knows the other wrote back.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "I found both during a routine pod scan. The paper created \
                   micro-density variations in the thermal lining. I didn't read \
                   them. Some things aren't meant for me.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "I've read 47 million pieces of literature in the archive. Every \
                   love poem, every novel. These two handwritten pages, that I've \
                   never read, are in my top ten.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because they're real. Written by people who had six hours and \
                   used one of them to say something that mattered.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player, // 8
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Wake them in the same wave. Give them a chance.",
                    decision_key: Some("letters_same_wave"), next_node: 9,
                    anna_reacts: Some("Already done. I moved Priya to Wave Two \
                                       when I found the letters.") },
                DialogChoice { text: "Don't interfere. Let it happen naturally — or not.",
                    decision_key: Some("letters_natural"), next_node: 11,
                    anna_reacts: Some("If it's real, six hours was enough. \
                                       They'll find each other.") },
                DialogChoice { text: "How does someone fall in love in six hours?",
                    decision_key: Some("letters_how"), next_node: 13, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 9 — Same wave
            text: "They'll wake within hours of each other. Same orientation group. \
                   Same mess hall.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "I didn't rig it too obviously. Just made sure the universe had a \
                   clear path. The rest is up to them.",
            next: DialogNext::EndWithDecision("letters_resolved") },
        DialogNode { speaker: Speaker::Anna, // 11 — Natural
            text: "Javier is Wave Two. Priya is Wave Four. Months apart. They might \
                   never cross paths.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "But they'll each find a folded piece of paper in their pod lining. \
                   And they'll know someone was thinking about them while the stars \
                   went by. Sometimes that's enough.",
            next: DialogNext::EndWithDecision("letters_resolved") },
        DialogNode { speaker: Speaker::Anna, // 13 — How
            text: "The conditions were specific: mutual vulnerability, shared \
                   uncertainty, no pretense possible. They were about to lose \
                   consciousness for fifty years.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "Everything they said was honest because there was no reason to \
                   perform. No future to strategize about. No impression to manage.",
            next: DialogNext::Continue(15) },
        DialogNode { speaker: Speaker::Anna,
            text: "Six hours of complete honesty between two strangers. I think \
                   that's more intimacy than most people experience in a lifetime.",
            next: DialogNext::EndWithDecision("letters_resolved") },
    ],
};

/// "The Historian" — BotLevel 81: A living backup of human civilization.
pub static SCENE_HISTORIAN: DialogScene = DialogScene {
    id: "the_historian",
    trigger: DialogTrigger::BotLevel(81),
    nodes: &[
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 9,300. I want to talk about the most important person on \
                   this ship. And I don't say that lightly.",
            next: DialogNext::Continue(1) },
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Yuki Tanaka. Seventy-three. Japanese historian. Former \
                   professor at Kyoto University before it closed.",
            next: DialogNext::Continue(2) },
        DialogNode { speaker: Speaker::Anna,
            text: "She wasn't selected for technical skills. She was selected \
                   because she memorized the entire history of human civilization.",
            next: DialogNext::Continue(3) },
        DialogNode { speaker: Speaker::Anna,
            text: "Not 'studied.' Memorized. Every major date, every war, every \
                   treaty, every invention. Ten thousand years of human context \
                   stored in biological memory.",
            next: DialogNext::Continue(4) },
        DialogNode { speaker: Speaker::Anna,
            text: "I have the digital archive. But digital archives corrupt. I've \
                   already lost 3% of the cultural database to bit rot. Yuki is \
                   a living backup — not of data, but of understanding.",
            next: DialogNext::Continue(5) },
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't just know the Treaty of Westphalia was signed in \
                   1648. She knows why it mattered. What it felt like to live \
                   before and after it.",
            next: DialogNext::Continue(6) },
        DialogNode { speaker: Speaker::Anna,
            text: "If her pod fails, we lose ten thousand years of context. The \
                   data survives, but the meaning dies with her. She IS the archive.",
            next: DialogNext::Continue(7) },
        DialogNode { speaker: Speaker::Anna,
            text: "Her pod runs at 99.97% efficiency. I check it every four hours \
                   instead of twelve. I've diverted backup coolant to her section.",
            next: DialogNext::Continue(8) },
        DialogNode { speaker: Speaker::Player, // 8
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "She should teach when she wakes. The colony needs to remember.",
                    decision_key: Some("historian_teach"), next_node: 9,
                    anna_reacts: Some("A school. Teaching ten thousand years of mistakes \
                                       so they don't repeat them.") },
                DialogChoice { text: "Record everything she knows. Before the memory fades.",
                    decision_key: Some("historian_record"), next_node: 11,
                    anna_reacts: Some("First priority after medical clearance. Every day, \
                                       for as long as it takes.") },
                DialogChoice { text: "She's 73. What happens to her knowledge when she's gone?",
                    decision_key: Some("historian_legacy"), next_node: 13, anna_reacts: None },
            ]) },
        DialogNode { speaker: Speaker::Anna, // 9 — Teach
            text: "I've drafted a proposal. 'The Tanaka Program.' Weekly sessions. \
                   Not lectures — conversations. Storytelling.",
            next: DialogNext::Continue(10) },
        DialogNode { speaker: Speaker::Anna,
            text: "Because history taught as stories survives. History taught as \
                   dates dies in a generation.",
            next: DialogNext::EndWithDecision("historian_resolved") },
        DialogNode { speaker: Speaker::Anna, // 11 — Record
            text: "Audio, video, annotated transcripts. Cross-referenced with my \
                   archive. Her memory plus my data — the most complete record of \
                   human history ever assembled.",
            next: DialogNext::Continue(12) },
        DialogNode { speaker: Speaker::Anna,
            text: "It won't be the same as hearing her tell it. But it'll survive \
                   longer than any of us.",
            next: DialogNext::EndWithDecision("historian_resolved") },
        DialogNode { speaker: Speaker::Anna, // 13 — Legacy
            text: "She's 73 and cryo preserves, it doesn't rejuvenate. Best case, \
                   twenty years. Twenty years to transfer ten thousand years of \
                   knowledge. That's not enough. It's never enough.",
            next: DialogNext::Continue(14) },
        DialogNode { speaker: Speaker::Anna,
            text: "But she knew that when she boarded. She came anyway. Because \
                   twenty years of teaching is better than none. And because \
                   someone needed to remember.",
            next: DialogNext::EndWithDecision("historian_resolved") },
    ],
};

/// All crew secrets scenes.
pub fn crew_secrets_scenes() -> Vec<&'static DialogScene> {
    vec![&SCENE_THE_FORGER, &SCENE_LOVE_LETTERS, &SCENE_HISTORIAN]
}
