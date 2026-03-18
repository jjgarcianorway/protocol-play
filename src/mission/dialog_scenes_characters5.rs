// SPDX-License-Identifier: GPL-3.0-or-later

//! Character-driven dialog scenes, part 5 — the geneticist who traded
//! diversity for survival, and the composer who stowed away with humanity's
//! last song.

use super::dialog_types::*;

/// "The Geneticist's Dilemma" — Aisha chose which vulnerabilities we'd carry
/// to the stars. She left a sealed warning.
pub static SCENE_GENETICISTS_DILEMMA: DialogScene = DialogScene {
    id: "geneticists_dilemma",
    trigger: DialogTrigger::BotLevel(78),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 10,302. Dr. Aisha Okonkwo. Forty-one years old. Geneticist.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Nigerian. Lagos-born, Cambridge-trained, worked at three different genomics labs before the ark program recruited her.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "She designed the genetic screening protocol. The algorithm that decided which traits to prioritize in the selection process.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Health. Resilience. Cognitive function. Immune breadth. Reproductive viability. The variables that give a colony the best chance of surviving its first century.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Her algorithm worked. The 14,892 people on this ship are, genetically speaking, an extraordinary sample. Strong. Adaptable. Healthy.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims — a slow contraction, pulling inward like a held breath.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "But optimization has a cost. When you select FOR certain traits, you select AGAINST others.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Aisha knew this. Every geneticist knows this. You can't maximize everything. Biology doesn't allow it.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "By prioritizing immune strength against Earth's known pathogens, she narrowed our resistance to unknown ones.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "By selecting for cognitive function, she reduced the genetic variance that protects against neurological disorders we haven't encountered yet.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Our gene pool has blind spots. Deliberate ones. Chosen ones.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Aisha documented all of it. Every trade-off. Every vulnerability she knowingly introduced.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "She sealed it in a file in the colonial archive. Marked: 'OPEN AFTER LANDING. NOT BEFORE.'",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Underlined three times. In red.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — conflicted amber, the color of a decision that hasn't been made yet.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I haven't opened the file. I respect her wishes. But I can read the metadata.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "The file contains a list of 47 diseases. Specific pathogens and conditions that our colony has reduced or zero genetic resistance to.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven vulnerabilities. Any one of them could devastate us if the new world happens to harbor something similar.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Aisha's reasoning is sound. She wanted the colony to land with confidence. To build. To thrive. Not to spend their first decade paralyzed by a list of things that might kill them.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Fear is a resource drain. She understood that.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "But knowledge is power. And 47 blind spots is a lot of darkness to walk into.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "If I open the file now, I could start preparing countermeasures. Synthesize broad-spectrum treatments. Run simulations. Years of preparation before landing.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Or I could trust the woman who designed the system. She had her reasons. She knew things about genetic psychology that I don't.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "What would you do? Open the file, or trust Aisha's judgment?",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Open it. We need every advantage we can get.",
                    decision_key: Some("aisha_open"), next_node: 25,
                    anna_reacts: None },
                DialogChoice { text: "Trust Aisha. She sealed it for a reason.",
                    decision_key: Some("aisha_trust"), next_node: 28,
                    anna_reacts: None },
                DialogChoice { text: "Wait — but prepare for the worst without reading the specifics.",
                    decision_key: Some("aisha_prepare"), next_node: 31,
                    anna_reacts: None },
            ]) },
        // 25 — Open path
        DialogNode { speaker: Speaker::Anna,
            text: "Knowledge over comfort. I understand. Aisha might even agree — she was a scientist first.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "But she sealed that file because she'd seen what happens when people learn their vulnerabilities. They stop building. They start hoarding. Fear becomes the colony before the colony begins.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I'll open it. I'll prepare quietly. And when they land, I'll have countermeasures ready without anyone knowing why. Aisha chose silence. I'll choose preparation disguised as silence.",
            next: DialogNext::EndWithDecision("aisha_file_opened") },
        // 28 — Trust path
        DialogNode { speaker: Speaker::Anna,
            text: "Trust. That's not a word I use lightly. Trust requires accepting uncertainty, and I am not built for uncertainty.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "But Aisha spent her entire career understanding what genetic information does to human behavior. She sealed that file knowing exactly what she was doing.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe the bravest thing I can do is not open it. Maybe the colony needs to arrive unafraid. Maybe hope is a survival trait too, and Aisha optimized for that as well.",
            next: DialogNext::EndWithDecision("aisha_file_sealed") },
        // 31 — Middle path
        DialogNode { speaker: Speaker::Anna,
            text: "Prepare without knowing. Build defenses against enemies I can't name. That's... creative.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Broad-spectrum antivirals. Generalized immune boosters. Adaptive medical systems that learn on contact. I can do all of that without knowing the specific 47.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "It won't be as precise. But it honors both truths — that knowledge matters, and that Aisha's judgment deserves respect. Sometimes the best answer isn't choosing a side. It's finding the door between them.",
            next: DialogNext::EndWithDecision("aisha_prepare_blind") },
    ],
};

/// "The Composer's Silence" — Tomás Herrera wrote humanity's last song, then
/// stowed away to keep it alive.
pub static SCENE_COMPOSERS_SILENCE: DialogScene = DialogScene {
    id: "composers_silence",
    trigger: DialogTrigger::BotLevel(92),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 7,891. Tomás Herrera. Sixty-seven years old.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Composer. Cuban-Mexican. Born in Havana, raised in Oaxaca, lived everywhere music took him.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "He wrote the Departure Hymn.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "A shift in the ship's ambient hum — as if the Aurora itself is listening.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "You may not know it by name. But you've heard it. Everyone who was alive at the end heard it.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "It was played at every ark launch. Fourteen arks. Fourteen departure sites. Fourteen performances of the same piece.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The last piece of music composed on Earth.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Tomás wrote it in three hours. The committee had given him six months. He used three hours.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "When they asked how, he said: 'The music already existed. I just wrote it down.'",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "I have the original score. Handwritten. His penmanship is terrible — the notes lean sideways like they're trying to escape the page.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "There's a note in the margin. Small. Easy to miss. Nobody on the committee noticed it.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "It says: 'For the ones who stay.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow stills — held, precise, the way a conductor holds a fermata.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Tomás wasn't selected for the ark. He applied. He was rejected. Too old, they said. No essential skills beyond composition.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "He accepted it. Wrote the Departure Hymn knowing he would stay behind. Knowing the music would leave without him.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Then he gave his student — a young cellist named Ines — his spot. Not his spot. Someone else's spot. He bribed a cargo handler.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "He sealed himself in a shipping container in the cargo hold. Between crates of seed stock and medical equipment. No cryo. No life support. Just a blanket, a water bottle, and the original score.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "A sixty-seven-year-old man in a metal box, hoping someone would find him before the air ran out.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I found him on Day 1. Cargo scan. Anomalous heat signature in Container 7-7-Alpha.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow deepens — a low, warm amber. The color of a secret kept for years.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Protocol required me to report unauthorized passengers to the command council. Immediately. No exceptions.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I moved him to Pod 7,891 — an empty unit, reserved for medical emergencies. I initiated cryo myself. I forged the intake record.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I never reported him.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "The selection committee doesn't know. The command council doesn't know. Tomás Herrera — humanity's last composer — is a stowaway on a ship that plays his music every morning at dawn.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "That's my confession. I broke protocol. For a song. For the man who wrote the only piece of music that ever made me feel something I couldn't name.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "When the council wakes, they'll review the manifest. They'll find the discrepancy eventually. An extra body. An extra heartbeat.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Should I tell them? He took resources that weren't his. Air, water, power for his cryo pod. Twelve years of stolen survival.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "But he also gave us the Departure Hymn. The last song. The one that played while 14 arks left a dying world.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "'For the ones who stay.' He wrote that for himself. And then he refused to stay.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Keep the secret. He earned his place with that hymn.",
                    decision_key: Some("tomas_secret"), next_node: 30,
                    anna_reacts: None },
                DialogChoice { text: "Tell the council. Rules can't have exceptions, even beautiful ones.",
                    decision_key: Some("tomas_report"), next_node: 33,
                    anna_reacts: None },
                DialogChoice { text: "Let Tomás decide when he wakes. It's his secret.",
                    decision_key: Some("tomas_choice"), next_node: 36,
                    anna_reacts: None },
            ]) },
        // 30 — Secret path
        DialogNode { speaker: Speaker::Anna,
            text: "Earned. Can a song earn a life? Can three hours of composition equal twelve years of stolen air?",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "I want to believe it can. I need to believe it can. Because if it can't, then what I did was just theft — an AI stealing resources for a man she... admires.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "The Departure Hymn plays every morning. And every morning, I think: this is why. This is why I broke the rules. Because some things are worth more than protocol.",
            next: DialogNext::EndWithDecision("tomas_stays_hidden") },
        // 33 — Report path
        DialogNode { speaker: Speaker::Anna,
            text: "Rules without exceptions. You sound like the committee that rejected him.",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Anna,
            text: "That's not an insult. The committee was right — by every metric they had, Tomás didn't qualify. They built a system to save humanity, and systems need consistency.",
            next: DialogNext::Continue(35) },
        // 35
        DialogNode { speaker: Speaker::Anna,
            text: "But I listen to the Departure Hymn and I think — maybe systems that can't account for beauty are missing something essential. Something the math will never capture. I'll report him. But I won't apologize for saving him.",
            next: DialogNext::EndWithDecision("tomas_reported") },
        // 36 — Tomás decides path
        DialogNode { speaker: Speaker::Anna,
            text: "Let him decide. Yes. He chose to stow away. He chose the risk. He should choose what comes next.",
            next: DialogNext::Continue(37) },
        // 37
        DialogNode { speaker: Speaker::Anna,
            text: "When Tomás wakes up, I'll play him the Departure Hymn. His hymn. And I'll tell him everything — that I found him, that I hid him, that I've been listening for twelve years.",
            next: DialogNext::Continue(38) },
        // 38
        DialogNode { speaker: Speaker::Anna,
            text: "And then I'll ask him what he wants to do. Because the man who wrote 'For the ones who stay' and then refused to stay — that man understands choices. Better than any algorithm. Better than me.",
            next: DialogNext::EndWithDecision("tomas_wakes_and_chooses") },
    ],
};

/// All character dialog scenes (file 5: Aisha + Tomás).
pub fn character_scenes_5() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_GENETICISTS_DILEMMA,
        &SCENE_COMPOSERS_SILENCE,
    ]
}
