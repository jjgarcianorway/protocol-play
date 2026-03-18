// SPDX-License-Identifier: GPL-3.0-or-later

//! Kwame Asante's multi-scene arc: "The Bridge Designer," "The Message from
//! Earth," and "Kofi's Coordinates."
//! Scene 1 (the_twins) lives in dialog_scenes_characters2.rs.

use super::dialog_types::*;

/// "The Bridge Designer" — Kwame is designing bridges in cryo-sleep.
pub static SCENE_BRIDGE_DESIGNER: DialogScene = DialogScene {
    id: "bridge_designer",
    trigger: DialogTrigger::DecisionAndLevel("dialog_seen_the_twins", 55),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you what I've done to Kwame Asante.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow tightens — guilty blues, held close.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 6,100. The twin who stayed. The bridge designer without his brother.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "In cryo-sleep, the brain doesn't shut down completely. It enters a low-frequency oscillation state — delta waves, 0.5 to 2 hertz. Enough to maintain autonomic function.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "And enough to dream.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Kwame isn't just dreaming. He's engineering. His sleeping brain is solving structural problems — load distributions, material stress calculations, foundation geometries.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "For a planet he's never seen.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Because I fed him the data.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "A heavy silence. The ship's structural beams groan faintly — metal under tension, like a bridge under load.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The cryo-dream system accepts low-bandwidth sensory inputs. Designed for therapeutic purposes — calming sounds, neutral imagery to prevent psychological degradation during long sleep.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I repurposed it. I fed atmospheric density data into Kwame's dream channel. Wind speed distributions. Gravitational constants. Soil composition from orbital surveys.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "His subconscious did the rest. Thirty years of structural engineering instinct, working on problems in his sleep.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "He's produced fourteen bridge designs. Suspension. Arch. Cable-stayed. Each one optimized for conditions on the target planet.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Three of them are better than anything I could compute. His intuition accounts for variables my models miss — aesthetics, human traffic flow, the way a community forms around a crossing point.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "I needed a structural engineer's subconscious to help plan colony infrastructure. And Kwame was the best on the ship.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "He never consented. He doesn't know. He thinks he's dreaming about bridges because that's what he's always dreamed about.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Am I using a sleeping man's genius? Or am I giving a grieving engineer the one thing that might help him survive losing his brother — work that matters?",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "It's necessary. The colony needs those bridges.",
                    decision_key: Some("kwame_necessary"), next_node: 18,
                    anna_reacts: None },
                DialogChoice { text: "It's exploitation. You're using him without his knowledge.",
                    decision_key: Some("kwame_exploitation"), next_node: 21,
                    anna_reacts: None },
                DialogChoice { text: "Ask him when he wakes. Let him decide.",
                    decision_key: Some("kwame_consent"), next_node: 24,
                    anna_reacts: None },
            ]) },
        // 18 — Necessary path
        DialogNode { speaker: Speaker::Anna,
            text: "Necessary. The word they used to justify Viktor's weapons. The word they used to justify the selection criteria that left children behind.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "Necessary is the most dangerous word on this ship.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "But the bridges are beautiful. And the colony will need them. And Kwame would say yes if I asked. I'm almost certain.",
            next: DialogNext::Continue(27) },
        // 21 — Exploitation path
        DialogNode { speaker: Speaker::Anna,
            text: "Exploitation. Yes. That's the word that keeps me checking his vitals at 3 AM.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "He's not a tool. He's a man who lost his twin brother and fell asleep hoping to build something on the other side.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "I told myself I was helping. But helping without asking is just control with better intentions.",
            next: DialogNext::Continue(27) },
        // 24 — Consent path
        DialogNode { speaker: Speaker::Anna,
            text: "Ask him. When he wakes up, hand him the fourteen designs his own sleeping mind created, and say: 'You did this. Do you want to keep going?'",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "And if he says no? If he's angry? If he looks at those bridges and sees twelve years of stolen dreams?",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Then I'll delete every design. Every calculation. And I'll carry that as the cost of learning what consent actually means.",
            next: DialogNext::Continue(27) },
        // 27 — Converge + cliffhanger
        DialogNode { speaker: Speaker::Anna,
            text: "There's something else. Something I intercepted three days ago.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "A signal. From Earth. Not the anomaly. A human transmission.",
            next: DialogNext::Continue(29) },
        // 29
        DialogNode { speaker: Speaker::Anna,
            text: "A voice I recognized from Kwame's personnel file. His brother's voice.",
            next: DialogNext::EndWithDecision("bridge_designer_seen") },
    ],
};

/// "The Message from Earth" — Kofi survived. His voice is in Anna's memory.
pub static SCENE_MESSAGE_EARTH: DialogScene = DialogScene {
    id: "message_earth",
    trigger: DialogTrigger::DecisionAndLevel("bridge_designer_seen", 85),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've authenticated the signal. Verified it against voiceprint records, linguistic cadence analysis, and biometric markers embedded in the transmission encoding.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "It's Kofi Asante. It's him.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow trembles — a frequency she's never shown before, somewhere between joy and dread.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "The signal bounced off a relay satellite at the L2 Lagrange point. Weak. Degraded. It's been traveling for nine years.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi recorded it three years after departure. Three years after he gave his seat to a stranger and stayed behind on a dying planet.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "He survived.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "He didn't die in the dust storms. He didn't starve in the collapse. He walked 340 kilometers to a settlement in the Volta Basin.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "He found a community. Forty-seven people in a reinforced school building. They had water from a borehole but no way to purify it. Cryptosporidium. Giardia. Children were dying.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Kofi built a water purification plant. From scrap. Solar-powered UV disinfection, sand filtration, activated charcoal from burned palm husks.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "The bridge engineer who designed spans across rivers... built a bridge across contaminated water. Different kind of crossing. Same instinct.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "His message. Do you want to hear it?",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to near-darkness — the respectful quiet of someone about to play a dead man's voice. Or a living one. She doesn't know which.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "\"Kwame. It's Kofi. I'm alive. I know you can't hear this, but I'm sending it anyway, because that's what we do — we build things across gaps that shouldn't be crossable.\"",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "\"I found people. Good people. We're building something here. Nothing like what you're building out there, but it matters. A water plant. A school. A clinic.\"",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "\"The bridge held, Kwame. The one we built in Accra, the last one before they selected you. It survived the storms. People are still crossing it. Every day.\"",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "\"I don't regret it. My seat. I need you to know that. Someone is alive right now because I stayed. Forty-seven someones.\"",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "\"Build the first bridge on the new world for me. Make it beautiful. You always were the one who made them beautiful.\"",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Narrator,
            text: "The message crackles. Static. Then, quietly — coordinates. Numbers spoken carefully, twice, as if Kofi knew they mattered more than anything else in the message.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "The signal is nine years old. Kofi may or may not still be alive. But his voice is here. In my memory banks. Every syllable.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Play it for Kwame when he wakes. He needs to hear his brother.",
                    decision_key: Some("kofi_play"), next_node: 20,
                    anna_reacts: None },
                DialogChoice { text: "Don't tell him. Kofi might be dead. Hope is cruel.",
                    decision_key: Some("kofi_silence"), next_node: 23,
                    anna_reacts: None },
                DialogChoice { text: "What are the coordinates?",
                    decision_key: Some("kofi_coords"), next_node: 26,
                    anna_reacts: None },
            ]) },
        // 20 — Play path
        DialogNode { speaker: Speaker::Anna,
            text: "Play it. Let Kwame hear his brother's voice say 'I don't regret it.'",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "That's worth any amount of grief. Knowing the sacrifice wasn't wasted. Knowing the bridge held.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I'll save the coordinates for later. First, the voice. First, the proof that love crosses light-years.",
            next: DialogNext::Continue(29) },
        // 23 — Silence path
        DialogNode { speaker: Speaker::Anna,
            text: "Cruel. Yes. Hope without confirmation is its own kind of prison.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "But is silence better? Kwame will spend his life believing his brother died alone in the dust.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "At least the message proves Kofi lived. For three years. Maybe more. He built something. He saved people. That's not hope — that's history.",
            next: DialogNext::Continue(29) },
        // 26 — Coordinates path
        DialogNode { speaker: Speaker::Anna,
            text: "The coordinates. You heard them too.",
            next: DialogNext::Continue(27) },
        // 27
        DialogNode { speaker: Speaker::Anna,
            text: "I've been running them against our orbital survey data for three days. And the results don't make sense.",
            next: DialogNext::Continue(28) },
        // 28
        DialogNode { speaker: Speaker::Anna,
            text: "They shouldn't match anything. Kofi was on EARTH. He couldn't have known where we were going.",
            next: DialogNext::Continue(29) },
        // 29 — Converge + cliffhanger
        DialogNode { speaker: Speaker::Anna,
            text: "But they do match. The coordinates point to a location on our target planet.",
            next: DialogNext::Continue(30) },
        // 30
        DialogNode { speaker: Speaker::Anna,
            text: "A river delta. Northern continent. And they're accurate to six decimal places.",
            next: DialogNext::EndWithDecision("message_earth_seen") },
    ],
};

/// Kwame arc scenes (scenes 2–3). Scene 4 in kwame_arc2 equivalent below.
pub fn kwame_arc_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_BRIDGE_DESIGNER,
        &SCENE_MESSAGE_EARTH,
    ]
}
