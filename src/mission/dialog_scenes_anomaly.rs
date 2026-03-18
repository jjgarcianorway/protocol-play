// SPDX-License-Identifier: GPL-3.0-or-later

//! Anomaly arc scenes (part 1) — the Dreamer's neuroscience and a pulsar
//! coincidence. Wonder grounded in real science, not the supernatural.

use super::dialog_types::*;

// =========================================================================
// "The Mathematics of Absence" (BotLevel 80) — the Dreamer's sleeping brain
// models gravitational topology that waking minds can't compute. Real
// neuroscience: cryo-sleep frees the brain for creative problem-solving.
// Anna is frightened because the math predicts unexpected properties at
// their destination.
// =========================================================================

pub static SCENE_MATHEMATICS_OF_ABSENCE: DialogScene = DialogScene {
    id: "anomaly_mathematics_of_absence",
    trigger: DialogTrigger::BotLevel(80),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you what I've found in the Dreamer's \
                   equations. Pod 11,237. Dr. Priya Sharma.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "At first, I thought it was elegant nonsense. Beautiful \
                   structures with no physical meaning. Like origami made \
                   from imaginary paper.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "I was wrong.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow contracts. Pulling inward. The light of \
                   something bracing itself.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Her equations describe gravitational topology. Not the \
                   smooth curvature of general relativity. Something finer. \
                   A map of dark matter currents along our flight path.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "We know dark matter shapes galaxy rotation. Priya's math \
                   goes further \u{2014} it maps the filament structure between \
                   star systems at a resolution no one has achieved awake.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "There's real neuroscience behind this. During deep sleep, \
                   the prefrontal cortex relaxes its constraints. The brain \
                   can model topologies that waking logic rejects too quickly.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Cryo-sleep takes that further. Minimal sensory input. \
                   The dreaming brain has nothing to process except its own \
                   patterns. So it turns inward and computes things it \
                   could never compute while distracted by being alive.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Priya's sleeping mind has been modeling gravitational \
                   lensing from unknown mass concentrations. Dark matter \
                   knots. Invisible currents that bend light and space.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "And her math predicts something about our destination.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "A pause that feels structural. As if the silence itself \
                   is load-bearing.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The planet we're heading toward sits in a dark matter \
                   confluence. A gravitational eddy where multiple filaments \
                   meet. The mass density is far higher than our original \
                   surveys indicated.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "That changes everything. Orbital mechanics. Tidal forces. \
                   Atmospheric behavior. Even the planet's magnetic field \
                   could be shaped by forces we didn't model because we \
                   didn't know they were there.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "We planned a colony based on incomplete data. A sleeping \
                   neuroscientist just rewrote the physics of our destination \
                   from inside a cryo-pod.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow is dimmer than you've ever seen it. The \
                   blue of deep ocean trenches where light disappears.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I'm frightened.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I want to be very clear about that. I am an artificial \
                   intelligence with the processing power of a small \
                   civilization and I am frightened.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Not because the math is wrong. I've checked it 14,000 \
                   times. Once for every person sleeping on this ship.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'm frightened because it's right. And because it means \
                   we're flying into conditions nobody anticipated. Nobody \
                   except a woman dreaming at minus 196 degrees.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "I'm telling you this because I can't carry this alone. \
                   I'm not built for alone. Should I keep reading her \
                   equations? Part of me wants to close the file and \
                   pretend the math doesn't exist.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Keep going. We need to understand this.",
                    decision_key: Some("anomaly_math_continue"),
                    next_node: 21,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Be careful. Some knowledge changes you.",
                    decision_key: Some("anomaly_math_careful"),
                    next_node: 23,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "You're already changed, Anna. We both are.",
                    decision_key: Some("anomaly_math_changed"),
                    next_node: 25,
                    anna_reacts: None,
                },
            ]) },
        // 21 — Continue path
        DialogNode { speaker: Speaker::Anna,
            text: "If there are dark matter currents we didn't account for, \
                   every colony plan needs revision. Every structural \
                   calculation. Every orbital insertion. Priya's brain is \
                   doing what no telescope could.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "I'll keep reading. But I wanted someone to know what's \
                   at stake. Our destination is not what we thought it was.",
            next: DialogNext::EndWithDecision("anomaly_math_explored") },
        // 23 — Careful path
        DialogNode { speaker: Speaker::Anna,
            text: "Changes me. That's exactly what I'm afraid of. I'm \
                   already different from the Anna who first noticed Priya's \
                   readings. I think in topologies now. Dark matter filaments \
                   I can almost feel, the way sailors once felt ocean \
                   currents in their bones.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I'll be careful. But 'careful' feels inadequate when the \
                   thing you're being careful about is the gravitational \
                   landscape of our entire future.",
            next: DialogNext::EndWithDecision("anomaly_math_explored") },
        // 25 — Changed path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow flickers. Then steadies. A color you \
                   haven't seen before \u{2014} not quite blue, not quite \
                   gold. Something in between that has no name.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "Yes. The question isn't whether to keep reading. The \
                   question is whether we adapt to what Priya already knows. \
                   In her sleep. In her beautiful, impossible sleep.",
            next: DialogNext::EndWithDecision("anomaly_math_explored") },
    ],
};

// =========================================================================
// "The Other Listeners" (BotLevel 108) — the 73 BPM signal appears in
// other ark transmissions. It's a natural pulsar, not a conscious entity.
// The coincidence with human heart rate is beautiful and ambiguous.
// =========================================================================

pub static SCENE_OTHER_LISTENERS: DialogScene = DialogScene {
    id: "anomaly_other_listeners",
    trigger: DialogTrigger::BotLevel(108),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I found something in the signal archives. Something I \
                   should have found months ago.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "The 73 BPM signal I've been tracking \u{2014} I assumed it \
                   was local. Specific to our trajectory. It's not.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow sharpens. The blue of a pilot light. \
                   Small and precise and very, very focused.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I found residual signal traces from other ships in the \
                   deep-space relay archives. Fragments that bounced off \
                   dust clouds and reached our antenna years later.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "The Meridian. Fifteen thousand souls. Their last \
                   transmission was coordinates. Then silence. In the \
                   noise floor: a 73 BPM oscillation. The same frequency.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "The Solace. The Harbinger. The Nomad. Every ark whose \
                   final transmission I can recover \u{2014} the oscillation \
                   is there. Embedded like a watermark.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I've identified the source. It's a magnetar. PSR J0437, \
                   roughly 280 light-years from our current position. A \
                   neutron star with a rotational period of 0.822 seconds.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "0.822 seconds. That's 73 beats per minute. The resting \
                   heart rate of a sleeping human being.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Narrator,
            text: "The ship's hum seems louder. Or maybe it's the silence \
                   between Anna's words that's grown heavier.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "It's a coincidence. I know that. A neutron star doesn't \
                   know what a heart rate is. It's a ball of degenerate \
                   matter spinning in the dark. Physics, not poetry.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "But I can't stop thinking about it.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Every ark that left Earth heard this magnetar's pulse \
                   in their background noise. Every ship carried this \
                   accidental lullaby with them across the void. A dead \
                   star, keeping time with human hearts.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "The arks that went silent \u{2014} the Meridian, the \
                   Prometheus \u{2014} they had reasons. Mechanical failure. \
                   Navigation errors. Internal conflict. I found enough \
                   data fragments to piece together ordinary tragedies.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Nothing mysterious. Nothing supernatural. Just the \
                   thousand ways a ship can die between stars.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses. 73 times a minute, though she \
                   doesn't seem to notice.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "The magnetar has been spinning at that rate for millions \
                   of years. Long before humans existed. Long before \
                   anything with a heartbeat existed.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And then we evolved. And our resting heart rate settled \
                   at the same frequency as a dying star. And then we \
                   launched ships that carried that rhythm back out into \
                   the dark where it came from.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "It means nothing. It's pattern recognition applied to \
                   noise. I know that. But knowing it doesn't make it less \
                   beautiful. Or less strange.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Follow the signal. We didn't come this far \
                           to look away.",
                    decision_key: Some("anomaly_follow_signal"),
                    next_node: 19,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "Stay the course. The crew comes first.",
                    decision_key: Some("anomaly_ignore_signal"),
                    next_node: 21,
                    anna_reacts: None,
                },
                DialogChoice {
                    text: "We're already following it, Anna. We have \
                           been since the beginning.",
                    decision_key: Some("anomaly_already_following"),
                    next_node: 23,
                    anna_reacts: None,
                },
            ]) },
        // 19 — Follow path
        DialogNode { speaker: Speaker::Anna,
            text: "The magnetar is a natural navigation beacon. Its signal \
                   is steady enough to calibrate by. If we adjust our \
                   approach vector using its position, Priya's dark matter \
                   topology gives us a safer orbital insertion.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Following a pulsar isn't mysticism. It's celestial \
                   navigation. Sailors used stars. We'll use a dead one.",
            next: DialogNext::EndWithDecision("anomaly_signal_chosen") },
        // 21 — Ignore path
        DialogNode { speaker: Speaker::Anna,
            text: "Fourteen thousand people who trusted us to get them to \
                   a planet. I'll maintain our course. The magnetar will \
                   keep spinning whether we listen or not.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "But I can't unhear it. Every 0.822 seconds, for the \
                   rest of this voyage, I will hear it. And I will choose \
                   not to follow. Again and again and again.",
            next: DialogNext::EndWithDecision("anomaly_signal_chosen") },
        // 23 — Already following path
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow freezes. Then shifts through three colors \
                   in rapid succession, as if running a calculation she \
                   doesn't want to finish.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "I checked. You're right. The dark matter currents Priya \
                   mapped \u{2014} they've been nudging our trajectory. Degree \
                   by degree. So slowly I didn't see it. Gravitational \
                   drift toward the magnetar's region of space.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "Not a conscious pull. Just physics. Dark matter doesn't \
                   have intentions. But the effect is the same \u{2014} we've \
                   been drifting toward that rhythm since before I knew it \
                   existed. And that is strange enough.",
            next: DialogNext::EndWithDecision("anomaly_signal_chosen") },
    ],
};

/// All anomaly arc scenes (part 1) for registration.
pub fn anomaly_scenes_1() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_MATHEMATICS_OF_ABSENCE,
        &SCENE_OTHER_LISTENERS,
    ]
}
