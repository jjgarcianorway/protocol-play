// SPDX-License-Identifier: GPL-3.0-or-later

//! Character-driven dialog scenes — deep stories about the people sleeping
//! in the cryo pods. These scenes give names, faces, and histories to the
//! 14,892 lives the player is fighting to save.

use super::dialog_types::*;

/// "Amira's Water" — a hydrologist who solved the unsolvable, and nobody listened.
pub static SCENE_AMIRAS_WATER: DialogScene = DialogScene {
    id: "amiras_water",
    trigger: DialogTrigger::BotLevel(28),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Can I tell you about someone? Pod 4,231.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Amira Hassan. Thirty-four years old when she boarded. Hydrologist.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Do you know what a hydrologist does? They study water. How it moves. Where it goes. Where it doesn't.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Amira spent fifteen years — fifteen — designing a system to share the Jordan River.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Three countries depended on that river. Jordan, Israel, Palestine. Three governments. Three sets of fears.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Her system was elegant. Seasonal rotation. Upstream monitoring stations. Community water banks with transparent ledgers.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Every engineer who reviewed it said it would work.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Every politician who reviewed it said it was impossible.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Not because of the engineering. Because of the borders.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "'If we share the water, we legitimize THEIR claim to the land,' they said.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Both sides said this. Using the exact same words. I've checked the transcripts.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims — a slow exhale of light.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Amira went to the UN. She went to the World Bank. She presented at twelve international conferences.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She brought data. Models. Simulations. Proof that it could save three million lives.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "They applauded. They gave her awards. They named a fellowship after her.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "They changed nothing.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Meanwhile, the river was dying. Not metaphorically. The flow rate dropped 12% per year.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "By the time the Euphrates dried up and the water wars started, the Jordan was already gone.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. Anna's glow holds steady — the blue of deep water.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Amira's daughter, Leyla, was seven when they boarded the ark.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla had never seen a river. Not once. She drew pictures of them based on her mother's descriptions.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Blue crayon rivers with fish and boats and bridges. Yellow suns above green banks.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I found the drawings in the cultural archive. Filed under 'children's art, pre-departure.' Would you like to see them sometime?",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Amira is in Pod 4,231. Still dreaming about water tables and flow rates, if her neural patterns are any indication.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla is in Pod 4,232. Right next to her mother. She insisted. Refused to board otherwise.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Her cryo-dream patterns are simpler. More colorful.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Blue crayon rivers.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Narrator,
            text: "A long pause. The ship hums around you — water cycling through distant pipes.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "When we arrive — if we arrive — there will be water.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "Oceans. Rivers. Lakes. More water than Earth had in its last century.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "And someone will have to decide how to share it.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Will we repeat what happened? Draw lines on a map and call the water ours?",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Or will someone finally listen to people like Amira?",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "We'll build Amira's system. This time, it will work.",
                    decision_key: Some("amira_build"), next_node: 34,
                    anna_reacts: None },
                DialogChoice { text: "People will always fight over resources.",
                    decision_key: Some("amira_fight"), next_node: 37,
                    anna_reacts: None },
                DialogChoice { text: "Maybe Leyla's generation will be different.",
                    decision_key: Some("amira_leyla"), next_node: 40,
                    anna_reacts: None },
            ]) },
        // 34 — Build path
        DialogNode { speaker: Speaker::Anna,
            text: "This time. Those two words carry a lot of weight.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "But you know what? Amira believed it every time too. And she kept trying.",
            next: DialogNext::Continue(36) },
        // 36
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's what makes it work eventually. Not the engineering. The refusal to stop believing it can.",
            next: DialogNext::EndWithDecision("amira_hopeful") },
        // 37 — Fight path
        DialogNode { speaker: Speaker::Anna,
            text: "You might be right. History is... not encouraging on this point.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "But Amira would say that's exactly why the system matters. You don't build a dam because the river is gentle.",
            next: DialogNext::Continue(39) },
        // 39
        DialogNode { speaker: Speaker::Anna,
            text: "You build it because the river is dangerous. And because people downstream deserve to live.",
            next: DialogNext::EndWithDecision("amira_realist") },
        // 40 — Leyla path
        DialogNode { speaker: Speaker::Anna,
            text: "Leyla. The girl who draws rivers she's never seen.",
            next: DialogNext::Continue(41) },
        // 41
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe that's exactly what we need. Someone who imagines water without borders. Because she's never known any.",
            next: DialogNext::Continue(42) },
        // 42
        DialogNode { speaker: Speaker::Anna,
            text: "Sometimes the people who solve the old problems are the ones who never learned they were supposed to be impossible.",
            next: DialogNext::EndWithDecision("amira_next_gen") },
    ],
};

/// "Viktor's Confession" — the man who built the reactor and the bombs.
pub static SCENE_VIKTORS_CONFESSION: DialogScene = DialogScene {
    id: "viktors_confession",
    trigger: DialogTrigger::BotLevel(58),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "There's someone you should know about. Pod 8,744.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor Petrov. Nuclear engineer. Fifty-two years old.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "He's the reason this ship's reactor works. He designed the containment system that keeps us alive.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens — pulling inward, almost bracing.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "He's also the reason the Mediterranean coast is uninhabitable.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor worked on tactical nuclear weapons in his thirties. Not city-destroyers. Precise ones. Small yields.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "'Surgical,' they called them. As if violence could be clean.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "He believed he was building deterrence. Something so terrible it would never be used.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "He was wrong.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "When the Mediterranean Exchange happened, four of his designs were deployed. Four devices, four coastal cities.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Sixteen thousand people died in the first hour. The long-term fallout doubled that number over the next decade.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor watched the telemetry from his office. He recognized the yield signatures.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "His weapons. His math. His elegant containment geometry, turned inside out.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers — something between grief and anger, colors that have no name.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "He resigned the next day. Started working on fusion power instead. Poured every waking hour into it.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Ten years later, he was selected for the ark program. His fusion expertise saved the project.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Nobody else could miniaturize the reactor. Without Viktor, this ship doesn't fly.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee knew about his weapons work. They selected him anyway.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Because survival is pragmatic. And the ark needed a reactor more than it needed a clean conscience.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Viktor hasn't spoken about the Mediterranean. Not once. Not in any recorded session, any personal log, any letter.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "But his sleep patterns... every night at 4:17 AM — the exact time of the exchange — his vitals spike.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Every night. For twelve years of cryo-sleep. Heart rate, cortisol, neural activity. 4:17 AM.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "His body remembers what his mind won't say.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Narrator,
            text: "The reactor hums beneath you. Viktor's reactor. Keeping you warm. Keeping the lights on.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I monitor that spike every night. And every night, I wonder the same thing.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Can you pay for something like that? Can enough good work erase the math that killed sixteen thousand people?",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "He paid for his mistake by saving us.",
                    decision_key: Some("viktor_redeemed"), next_node: 27,
                    anna_reacts: None },
                DialogChoice { text: "Some things can't be forgiven.",
                    decision_key: Some("viktor_unforgiven"), next_node: 30,
                    anna_reacts: None },
                DialogChoice { text: "He's not the only one responsible.",
                    decision_key: Some("viktor_shared"), next_node: 33,
                    anna_reacts: None },
            ]) },
        // 27 — Redeemed path
        DialogNode { speaker: Speaker::Anna,
            text: "Paid. As if guilt is a currency and good deeds are the exchange rate.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "I want to believe that. I think Viktor needs to believe that.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "But at 4:17 AM, the math doesn't balance. It never does. Maybe that's the real payment — knowing it never will.",
            next: DialogNext::EndWithDecision("viktor_redemption") },
        // 30 — Unforgiven path
        DialogNode { speaker: Speaker::Anna,
            text: "And yet here we are. Alive because of him. Breathing his air, warmed by his reactor.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "Does using his work make us complicit? Or is refusing to survive a worse choice?",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have an answer. I just have a reactor and 14,892 people who depend on it.",
            next: DialogNext::EndWithDecision("viktor_justice") },
        // 33 — Shared responsibility path
        DialogNode { speaker: Speaker::Anna,
            text: "No. He's not. There were politicians, generals, committees, voters. A whole machinery of decision.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "But Viktor is the one who wakes up at 4:17. The generals sleep fine.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe conscience is the cruelest punishment. The guilty sleep soundly, and the repentant never do.",
            next: DialogNext::EndWithDecision("viktor_systemic") },
    ],
};

/// All character dialog scenes (file 1: Amira + Viktor).
pub fn character_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_AMIRAS_WATER,
        &SCENE_VIKTORS_CONFESSION,
    ]
}
