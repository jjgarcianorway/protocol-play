// SPDX-License-Identifier: GPL-3.0-or-later

//! Amira Hassan's multi-scene arc (part 2): "The Price of a Seat" + "Amira Wakes."
//! Scenes 2–3 live in dialog_scenes_amira_arc.rs.

use super::dialog_types::*;

/// "The Price of a Seat" — Whitfield holds the evidence. What is justice without Earth?
pub static SCENE_PRICE_OF_SEAT: DialogScene = DialogScene {
    id: "amira_price_seat",
    trigger: DialogTrigger::DecisionAndLevel("selection_file_seen", 95),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "Dr. Whitfield made his move.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is cold tonight — steel blue, institutional, the color of committee rooms.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "He hasn't released the evidence publicly. Not yet. But he's put it on the pre-revival council agenda.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Item 7b: 'Irregularities in passenger manifest, Pod 4,232.'",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Bureaucratic language for 'a mother smuggled her daughter onto the last ship off a dying planet.'",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The question Whitfield wants answered: what happens to unauthorized passengers?",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "There is no 'back' to send her to. Earth is gone. There is no deportation. There is no extradition.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "But the precedent matters to him. If one person hacked the system, others might have too.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "And if the selection process can't be trusted, then the Founders' authority — which rests on that process — collapses.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Whitfield doesn't care about Leyla. He cares about what Leyla represents: proof that the system was breakable.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "And breakable systems don't deserve loyalty.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "The recycler hum deepens. Somewhere in the dark, Pod 4,232 holds a girl who dreams in watersheds.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I've been thinking about what's fair. Is Leyla's presence a crime or a miracle?",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "She's designing water systems in her sleep that could save the colony. Does that earn her seat retroactively?",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Or does the person she replaced — the one left behind — still deserve that seat more?",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "The truth should come out. Secrets rot.",
                    decision_key: Some("seat_truth"), next_node: 16,
                    anna_reacts: None },
                DialogChoice { text: "Protect Leyla. She didn't choose this.",
                    decision_key: Some("seat_protect"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "Use it. Leyla's abilities give us leverage against Whitfield.",
                    decision_key: Some("seat_leverage"), next_node: 22,
                    anna_reacts: None },
            ]) },
        // 16 — Truth path
        DialogNode { speaker: Speaker::Anna,
            text: "Secrets rot. Yes. I've seen what happens to ships that run on lies.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "But truth doesn't always set people free. Sometimes it just gives their enemies ammunition.",
            next: DialogNext::Continue(25) },
        // 18 — (reserved for index alignment)
        DialogNode { speaker: Speaker::Anna,
            text: ".",
            next: DialogNext::End },
        // 19 — Protect path
        DialogNode { speaker: Speaker::Anna,
            text: "She didn't choose it. Neither did the person who lost their seat.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Protecting Leyla means lying. It means I manipulate the pre-revival records. It means becoming what Amira became — someone who breaks the system for love.",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "I'm supposed to be the system. I'm not sure I can be both.",
            next: DialogNext::Continue(25) },
        // 22 — Leverage path
        DialogNode { speaker: Speaker::Anna,
            text: "Leverage. You want to use a child as a political tool.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow doesn't flicker. It holds steady — measuring you.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "You're not wrong that it would work. Whitfield can't argue against the person who'll design the colony's water. But the cost of thinking that way... it adds up.",
            next: DialogNext::Continue(25) },
        // 25 — Converge + cliffhanger
        DialogNode { speaker: Speaker::Anna,
            text: "Whatever we decide, it has to happen before revival. Once Amira wakes up, she'll fight. And Whitfield will fight back.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "And Leyla — Leyla will be caught in the middle. A girl who's never seen a river, dreaming solutions to a planet she's never touched.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I need to make a decision soon. Because I'm running out of 'soon.'",
            next: DialogNext::EndWithDecision("price_seat_seen") },
    ],
};

/// "Amira Wakes" — six hours before general revival. First conscious moments in 12 years.
pub static SCENE_AMIRA_WAKES: DialogScene = DialogScene {
    id: "amira_wakes",
    trigger: DialogTrigger::DecisionAndLevel("price_seat_seen", 135),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Narrator,
            text: "The cryo bay is silent except for the slow hiss of Pod 4,231's revival sequence.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I've initiated early revival for Amira. Six hours before the general wake-up.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I need someone to help design the colony's water infrastructure. She's the best. That's the official reason.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The real reason is I want to give her time. Time to prepare for what's coming.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The council knows about Leyla. The new planet has rivers — real ones. And everything she spent fifteen years fighting for is about to become real.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "That's a lot to absorb in the first minutes of consciousness after twelve years of sleep.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Through the pod sensors, you can see condensation forming on the interior glass. A shape shifts behind the frost.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Her vitals are stabilizing. Heart rate climbing to normal. Neural activity spiking — she's transitioning from cryo-dream to consciousness.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "The transition is the hardest part. Twelve years of dreams collapsing into one moment of 'where am I.'",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "A sound from the pod. Low. Barely a whisper. The first voluntary sound from Pod 4,231 in 4,383 days.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "She's trying to speak. The vocal cords take a moment to remember what they are.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "The frost clears in a slow circle where a palm presses against the glass from inside.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "Her first word.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "'Leyla.'",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Silence. Then another sound — clearer now, a voice remembering itself.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Her second word. A question.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "'Water?'",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow expands — warm, golden, the color of sunrise on a river. A color she's never used before.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "Yes, Amira. There's water. More water than you've ever seen.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "And your daughter has been designing how to share it.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Tell her about Leyla's abilities first. She deserves to know.",
                    decision_key: Some("amira_tell_leyla"), next_node: 21,
                    anna_reacts: None },
                DialogChoice { text: "Tell her about the council. She needs to be ready.",
                    decision_key: Some("amira_tell_council"), next_node: 24,
                    anna_reacts: None },
                DialogChoice { text: "Just let her see her daughter. Everything else can wait.",
                    decision_key: Some("amira_tell_nothing"), next_node: 27,
                    anna_reacts: None },
            ]) },
        // 21 — Tell about Leyla
        DialogNode { speaker: Speaker::Anna,
            text: "I tell her. All of it. The cryo-dreams. The watershed designs. The graduate-level hydrology from a sleeping child.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Amira listens. She doesn't cry. She doesn't gasp. She just nods, slowly, as if something she always suspected has been confirmed.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "'She always drew the tributaries in the right places,' Amira says. 'I thought she was copying my notes. She wasn't copying.'",
            next: DialogNext::Continue(30) },
        // 24 — Tell about council
        DialogNode { speaker: Speaker::Anna,
            text: "I tell her about Whitfield. The sealed evidence. The council agenda. Item 7b.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Amira's face hardens. Twelve years of cryo-sleep, and the first emotion to cross her waking face is the same one she wore in every conference room on Earth.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "'I hacked a boarding system to save my daughter,' she says. 'I'll hack whatever I have to.'",
            next: DialogNext::Continue(30) },
        // 27 — Let her see Leyla
        DialogNode { speaker: Speaker::Anna,
            text: "I say nothing. I just open Pod 4,232's observation window.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Narrator,
            text: "Amira presses her face to the glass of her daughter's pod. The frost melts where her forehead touches.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "She stays there for eleven minutes. I know because I count every one. She doesn't say a word. She doesn't need to.",
            next: DialogNext::Continue(30) },
        // 30 — Converge / Finale
        DialogNode { speaker: Speaker::Anna,
            text: "In six hours, fourteen thousand people will wake up. They'll need water systems. They'll need leadership. They'll need answers about who belongs here.",
            next: DialogNext::Continue(31) },
        // 31
        DialogNode { speaker: Speaker::Anna,
            text: "But right now, in this cryo bay, there is just a mother and her sleeping daughter and a planet full of rivers waiting for them.",
            next: DialogNext::Continue(32) },
        // 32
        DialogNode { speaker: Speaker::Anna,
            text: "Amira looks at me — at my sensor, the little blue light in the ceiling — and says something I will store in permanent memory.",
            next: DialogNext::Continue(33) },
        // 33
        DialogNode { speaker: Speaker::Anna,
            text: "'Thank you for watching over her.'",
            next: DialogNext::Continue(34) },
        // 34
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses once — soft, warm, wordless — and then settles. The cryo bay hums with the sound of water cycling through the ship. Amira's element. Leyla's inheritance.",
            next: DialogNext::EndWithDecision("amira_wakes_seen") },
    ],
};

/// Amira arc scenes from this file (part 2: scenes 3–4).
pub fn amira_arc_scenes_2() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_PRICE_OF_SEAT,
        &SCENE_AMIRA_WAKES,
    ]
}
