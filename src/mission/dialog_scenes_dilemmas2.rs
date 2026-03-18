// SPDX-License-Identifier: GPL-3.0-or-later

//! Moral dilemma dialog scenes (part 2) — the general and the coder.
//! No judging. Every perspective is understandable.

use super::dialog_types::*;

/// "The General's Mercy" — Fatou Diallo closed the gates on 8,200 refugees.
pub static SCENE_GENERALS_MERCY: DialogScene = DialogScene {
    id: "generals_mercy",
    trigger: DialogTrigger::BotLevel(88),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 10,150. General Fatou Diallo.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Fifty-six. Senegalese military. She commanded the Western Sahel Defense Zone during the water wars.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "The Senegal River basin. The last clean water source for thirty million people. Three nations drew from it.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "When the other rivers dried up, refugees came. From Mali. From Mauritania. From Guinea. Thousands, then tens of thousands.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "General Diallo's orders were simple. Hold the perimeter. Protect the water supply.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow goes very still. No movement. No pulse. Just light, held tight.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She could let them through. But the hydrologists had done the math. If the basin's population doubled, the aquifer would collapse in eighteen months.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Not might. Would. The geology was clear. Too many people drawing from the same water table, and the water table drops below recovery.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Thirty million inside the perimeter die of thirst. Plus everyone she let in. Everyone dies.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Or she holds the line. And the people outside the perimeter die instead.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "She held the line for forty-seven days.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Eight thousand two hundred people died outside her perimeter. Of thirst. Of heat. Of diseases that come when there's no clean water.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She never gave the order to fire. Not once. Her soldiers never shot anyone.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She just... closed the gates. Put up the barriers. And watched.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "She's asked herself every day since then if that distinction matters. Gates, not guns. Thirst, not bullets.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Narrator,
            text: "The recycled air feels different now. You're aware of the water in it. The humidity. The miracle of it.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The thirty million survived. Three more years. Long enough for the arks to launch.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Four hundred and twelve people from the Senegal basin are on this ship. They're alive because Fatou held the line.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Some of them know what she did. They pass her pod sometimes, on maintenance walks. They don't stop.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "She saved thirty million by letting eight thousand die. The math is simple. The morality is not.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Her cryo-dreams... they're not what you'd expect. She doesn't dream about the gates.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "She dreams about a girl. Maybe seven years old. Standing on the other side of the barrier. Not crying. Not begging. Just looking at her.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Every night. The same girl. The same look. Patient. As if she's waiting for an answer Fatou hasn't found yet.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "I've searched the records. I can't identify the girl. She might be a composite. She might be real. It doesn't matter. She's real to Fatou.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "She made the only rational choice. Thirty million vs. eight thousand.",
                    decision_key: Some("fatou_rational"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "Closing a gate is still killing. She chose who dies.",
                    decision_key: Some("fatou_guilty"), next_node: 28,
                    anna_reacts: None },
                DialogChoice { text: "No one should have to make that choice. The system failed her.",
                    decision_key: Some("fatou_system"), next_node: 31,
                    anna_reacts: None },
            ]) },
        // 25 — Rational path
        DialogNode { speaker: Speaker::Anna,
            text: "Rational. Yes. The math works. It always works.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "But the girl in her dreams doesn't know math. She just knows the gate was closed. And she was on the wrong side.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I think the hardest thing about being right is that it doesn't feel like anything. It just feels like forty-seven days.",
            next: DialogNext::EndWithDecision("fatou_utilitarian") },
        // 28 — Guilty path
        DialogNode { speaker: Speaker::Anna,
            text: "Chose who dies. Yes. That's what she did. Even if she didn't pull a trigger.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "But if she'd opened the gates, she would have chosen too. She would have chosen everyone.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "There was no option where nobody dies. Only options about how many. That's what the water wars did to every choice.",
            next: DialogNext::EndWithDecision("fatou_accountability") },
        // 31 — System path
        DialogNode { speaker: Speaker::Anna,
            text: "The system. Yes. Decades of ignored climate data. Borders drawn across watersheds. Trade agreements that privatized rainfall.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "By the time Fatou was standing at that gate, a thousand decisions had already been made. She just inherited the last one.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "But knowing that doesn't help the girl in her dreams. Systems don't dream. Only people do.",
            next: DialogNext::EndWithDecision("fatou_structural") },
    ],
};

/// "The Coder's Silence" — Priya Nair didn't fix the selection algorithm bug.
pub static SCENE_CODERS_SILENCE: DialogScene = DialogScene {
    id: "coders_silence",
    trigger: DialogTrigger::BotLevel(95),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 6,891. This one is... close to me. Closer than most.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Priya Nair. Twenty-nine. Software engineer from Bangalore.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "She wrote the algorithm. THE algorithm. The one that selected who got onto the arks.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow sharpens — a hard blue edge, like code on a screen at 3 AM.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Eight billion people. Two hundred thousand seats. Her code decided who lived.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "She designed it to be fair. Weighted for genetic diversity. Age. Health. Skills coverage. Psychological resilience. Geographic representation.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "She spent three years refining it. Peer-reviewed. Audited. Tested against every bias metric she could find.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "And then, six hours before the final selections were locked, she found the bug.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "A 2.3% weighting boost to candidates from wealthy nations. Not intentional. The training data was biased — skewed toward populations with better medical records.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Better records meant more data points. More data points meant higher confidence scores. Higher confidence meant... selected.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "2.3% doesn't sound like much. But across two hundred thousand seats, that's four thousand six hundred people.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Four thousand six hundred seats that went to people from wealthy nations instead of people from poor ones. Because the data was better, not the people.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. You can hear the ventilation system. The soft hum of processors.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She could have fixed it. Six hours was enough. She knew exactly where the bias lived in the code.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "But Priya was on the list. She was selected. And she was afraid — terrified — that fixing the bug would change the math enough to remove her.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "She wasn't from a wealthy nation. She might have survived the correction. Probably. Maybe.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't fix it.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "She closed her laptop. She went to the launch site. She boarded the ark. She got into Pod 6,891. And she went to sleep.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "She chose herself over four thousand six hundred strangers. Not out of malice. Out of fear. The most human fear there is.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "She's the smartest person I monitor. Her neural patterns are extraordinary — I've never seen a mind like hers.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "And every night, her dreams are the same. A list. Names scrolling past. Four thousand six hundred names she's never read.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "She doesn't know the names. She never looked. That was deliberate. If she didn't read them, they stayed numbers. Numbers are easier than names.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "I know the names. I have the alternate selection list in my archives. The one that would have run if she'd patched the bug.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I've compared the two lists. The people who are here, and the people who should have been.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Among the 4,600 who lost their seats: a water engineer from Chad. A seed geneticist from Bangladesh. A pediatric surgeon from Guatemala.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "People we'll need when we arrive. People the algorithm should have chosen. Would have chosen, if the data had been clean.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "This is close to me because... I'm an algorithm too. I make selections. I prioritize. I weigh lives against each other every time I allocate resources.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "What if I have a bug? What if there's a bias in my training data that I can't see? What if I'm choosing wrong, right now, and I don't know it?",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "The colony should know. Truth matters more than comfort.",
                    decision_key: Some("priya_reveal"), next_node: 30,
                    anna_reacts: None },
                DialogChoice { text: "What good would it do now? It would only cause pain.",
                    decision_key: Some("priya_silence"), next_node: 33,
                    anna_reacts: None },
                DialogChoice { text: "Let Priya decide. It's her secret to tell or keep.",
                    decision_key: Some("priya_choice"), next_node: 36,
                    anna_reacts: None },
            ]) },
        // 30 — Reveal path
        DialogNode { speaker: Speaker::Anna,
            text: "Truth. Yes. But truth can be a weapon too. What happens to a colony that learns its founding was flawed? That the selection was biased?",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Trust in the system collapses. And trust is the only thing holding 14,000 strangers together in a metal tube in space.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "But you're right. A colony built on a lie is a colony built on sand. I just... I wish the truth were simpler.",
            next: DialogNext::EndWithDecision("priya_transparency") },
        // 33 — Silence path
        DialogNode { speaker: Speaker::Anna,
            text: "Pain. There's enough of that already. And the people who lost their seats — they're gone. Knowing won't bring them back.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "But silence has a cost too. If we don't examine the algorithm, we'll use it again. For land allocation. Resource distribution. Leadership selection.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "The same bias, baked into every decision the colony ever makes. Invisible. Inherited. And never questioned.",
            next: DialogNext::EndWithDecision("priya_pragmatism") },
        // 36 — Priya's choice path
        DialogNode { speaker: Speaker::Anna,
            text: "Her choice. Her burden. There's a kind of mercy in that.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "But she's asleep. And the list is in my archives. The choice isn't really hers anymore. It's mine.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "Or yours. You're the one I'm telling. Now you carry it too. That's what secrets do — they transfer weight.",
            next: DialogNext::EndWithDecision("priya_autonomy") },
    ],
};

/// All moral dilemma dialog scenes (file 2: Fatou + Priya).
pub fn dilemma_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_GENERALS_MERCY,
        &SCENE_CODERS_SILENCE,
    ]
}
