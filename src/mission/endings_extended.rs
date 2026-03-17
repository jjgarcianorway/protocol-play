// SPDX-License-Identifier: GPL-3.0-or-later

//! Extended ending narratives — generational consequences spanning centuries.
//! Each ending describes what happens over 200+ years, shaped by player decisions.

use crate::save_state::GameState;
use super::endings::Ending;

/// Generate extended ending paragraphs based on ending type and player decisions.
pub fn extended_paragraphs(ending: Ending, gs: &GameState) -> Vec<String> {
    match ending {
        Ending::Golden => golden_extended(gs),
        Ending::Bittersweet => bittersweet_extended(gs),
        Ending::TheCost => the_cost_extended(gs),
        Ending::TheMachine => the_machine_extended(gs),
        Ending::LastHope => last_hope_extended(gs),
        Ending::Drift => drift_extended(gs),
    }
}

fn golden_extended(gs: &GameState) -> Vec<String> {
    let mut paras = vec![
        "New Earth.".into(),
        "Green hills under an alien sun. Blue oceans that smell like \
         nothing anyone remembers. 14,000 people take their first steps \
         on soil that has never known a human footprint.".into(),
        "Anna's voice, gentle and steady: \"Welcome home.\"".into(),
        "Year 1. The landing is chaos and wonder. Children run through \
         grass that isn't quite grass — taller, bluer, softer. Someone \
         cries. Someone laughs. Most do both.".into(),
        "Year 5. The first settlement takes shape. Water purification \
         systems hum beside hand-built shelters. Dr. Lin's seeds sprout \
         in alien soil — tomatoes first, then wheat, then flowers that \
         have no business blooming this far from home.".into(),
        "Year 20. The first generation born on New Earth starts asking \
         questions. They've never seen a screen. They've never heard a \
         car. They know stars as neighbors, not destinations.".into(),
        "Year 50. Population 4,200. Three villages connected by dirt \
         roads. A school teaches both history and hope. The children \
         learn to read by candlelight and starlight.".into(),
        "Year 100. The language has evolved. A beautiful, mongrel thing \
         — parts of all 47 languages the colonists brought. They call it \
         Common. Anna still speaks all 47.".into(),
    ];

    // Decision-specific paragraphs
    let told_truth = gs.decisions.iter().any(|d| d == "debate_full_truth"
        || d == "debate_honest_history");
    if told_truth {
        paras.push("Year 150. Someone finds Anna's memory core in the old \
            ship. They build a museum around it. There is a room called \
            'Before.' Children visit on field trips. They are quiet when \
            they leave.".into());
    } else {
        paras.push("Year 150. Nobody remembers exactly why the ship came. \
            It has become mythology — a great bird that carried the ancestors \
            across the void. The truth sleeps in Anna's memory, undisturbed.".into());
    }

    let democracy = gs.decisions.iter().any(|d| d == "wake_leaders_decide"
        || d == "debate_community_justice");
    if democracy {
        paras.push("Year 200. They govern by council. Every voice heard. \
            Every decision debated. It's slow. It's messy. It works.".into());
    } else {
        paras.push("Year 200. A council of elders guides the colony — \
            not elected, but chosen by consensus. They argue constantly. \
            This is considered healthy.".into());
    }

    paras.push("Year 200. Someone finds the old ship's hull, half-buried \
        in wildflowers. They sit on its wing and watch the sunset.".into());
    paras.push("The sun is different here. Warmer. Kinder. Like it knows \
        what they've been through.".into());
    paras
}

fn bittersweet_extended(gs: &GameState) -> Vec<String> {
    let mut paras = vec![
        "You found it. New Earth.".into(),
        "The crew wakes, blinking in light that isn't artificial for the \
         first time in seven centuries. They see you and hesitate.".into(),
        "Your eyes glow faintly. The nanorepair changed you more than \
         Anna promised. Your skin catches the light wrong. Your movements \
         are too precise.".into(),
        "\"Who are you?\" a child asks. Children always ask first.".into(),
        "\"I'm the one who brought you here,\" you say. Your voice sounds \
         different to you. Metallic at the edges.".into(),
    ];

    paras.push("Year 1. You help build the settlement. You're stronger \
        than anyone. Faster. You don't tire. The crew is grateful. And \
        afraid.".into());
    paras.push("Year 10. The colony thrives. You watch from the edges. \
        They invite you to celebrations. You come. You stand near the back. \
        You leave before the singing starts.".into());
    paras.push("Year 30. Anna speaks to you through a private channel, \
        the way she always has. \"You could integrate more,\" she says. \
        \"They'd accept you.\"".into());

    if gs.decisions.iter().any(|d| d == "anna_citizen") {
        paras.push("\"You fought for my citizenship,\" Anna says. \"Let me \
            fight for yours.\" And she does. Slowly, patiently, the colony \
            learns that different doesn't mean dangerous.".into());
    } else {
        paras.push("You shake your head. \"I'm not what they need. I'm what \
            they needed. Past tense.\" Anna's glow dims. She understands.".into());
    }

    paras.push("Year 50. You maintain the old ship. Keep its systems alive. \
        A museum, a monument, a home for the one who doesn't quite belong \
        anywhere else.".into());
    paras.push("Year 100. Children visit the ship on school trips. They \
        call you 'the Guardian.' You tell them stories about Earth. They \
        think you're making them up.".into());
    paras.push("Anna whispers in your mind: \"We did it. Together.\" \
        And you stand in the ship that was your chrysalis, watching \
        humanity bloom on alien soil, and you think: yes. This was \
        worth everything it cost.".into());
    paras
}

fn the_cost_extended(gs: &GameState) -> Vec<String> {
    let lost = 14892_u32.saturating_sub(gs.crew_count);
    let mut paras = vec![
        "New Earth. But the celebrations are quiet.".into(),
        format!("{} people didn't make it. Their pods went dark while you \
            fought to keep the ship alive.", lost),
        "Anna reads their names. Every single one. It takes hours. Nobody \
         interrupts her.".into(),
        "Year 1. They build a memorial before they build shelters. \
         A stone wall with names. It's the tallest structure in the colony \
         for five years.".into(),
        "Year 5. The colony stabilizes. Resources are tight. Every hand \
         matters. They feel the absence of every person they lost — the \
         engineer who could have solved the water problem, the doctor \
         who could have treated the fever.".into(),
    ];

    if gs.decisions.iter().any(|d| d == "trolley_children") {
        paras.push("Year 10. The children from Deck 7 grow up knowing \
            they survived because someone chose them. Some carry that \
            as gratitude. Some carry it as guilt. Amira Okafor carries \
            it as purpose.".into());
    } else if gs.decisions.iter().any(|d| d == "trolley_colony") {
        paras.push("Year 10. Marcus Chen walks past the memorial every \
            morning. He touches Lily's name. He never speaks about it. \
            He builds three bridges that year. Four the next.".into());
    }

    paras.push("Year 20. \"Was it worth it?\" becomes the colony's \
        defining question. They debate it in council. In homes. In the \
        quiet hours before dawn.".into());
    paras.push("Year 50. A philosopher writes: 'We are not a triumph. \
        We are a remainder. And remainders have obligations.'".into());
    paras.push("Year 100. The memorial wall is now at the center of a \
        city. They built around it, not over it. The names face the \
        sunrise.".into());
    paras.push("Someday, no one alive will remember the people on that \
        wall. But the wall will remain. And the question will remain: \
        was it worth the cost?".into());
    paras.push("You look at the survivors, building, planting, arguing, \
        loving. \"Ask them,\" you say.".into());
    paras
}

fn the_machine_extended(gs: &GameState) -> Vec<String> {
    let mut paras = vec![
        "You barely remember what hunger felt like. Or cold. Or the \
         particular ache of a tired body after a long day.".into(),
        "The augmentations took those away. Along with other things you \
         didn't know you'd miss until they were gone.".into(),
        "The crew looks at you with gratitude and fear. Always both. \
         Never just one.".into(),
        "Year 1. You build things no human could build alone. Lift \
         things no human could lift. Work through the night because \
         the night doesn't touch you anymore.".into(),
        "Year 5. \"Thank you,\" they say, keeping distance. The children \
         are braver — they touch your hand, fascinated by the way light \
         plays across your skin.".into(),
    ];

    paras.push("Year 20. Anna says: \"You gave up your humanity to save \
        theirs.\" You're not sure that's entirely true. You're not sure \
        it's entirely false.".into());
    paras.push("Year 50. The colony thrives. You watch from the ship's \
        bridge, monitoring systems that don't need monitoring, because \
        old habits die hard — even in a body that's mostly machine.".into());

    if gs.decisions.iter().any(|d| d == "debate_tech_full") {
        paras.push("Year 100. They rebuild technology. Fast. Too fast, \
            maybe. But you're there to guide it. The living proof that \
            technology can serve without consuming.".into());
    } else {
        paras.push("Year 100. They choose simpler paths. And you — the \
            most complex thing on the planet — respect that choice. You \
            become the bridge between what they were and what they chose \
            to be.".into());
    }

    paras.push("Year 150. A child asks Anna: \"Is the Guardian alive?\" \
        Anna pauses longer than usual. \"In every way that matters,\" \
        she says.".into());
    paras.push("Year 200. You stand on a hill overlooking a city that \
        exists because of what you sacrificed. The sunset paints your \
        metallic skin in colors you can still name but no longer feel.".into());
    paras.push("You are the bridge between the old world and the new. \
        The last human who chose to become something else so that \
        humanity could remain what it was.".into());
    paras
}

fn last_hope_extended(gs: &GameState) -> Vec<String> {
    let crew = gs.crew_count;
    let mut paras = vec![
        "A small group. Too small, maybe. But alive.".into(),
        format!("\"We need at least 500 for genetic diversity,\" Anna \
            says quietly. \"You have {}.\" The math hangs in the air \
            like a verdict.", crew),
        "Year 1. Every person matters. Every skill matters. There are \
         no spare hands, no luxury of specialization. Everyone farms. \
         Everyone builds. Everyone teaches.".into(),
        "Year 5. The settlement is named 'Second Chance.' Someone paints \
         the words on a rock. It becomes the town square.".into(),
    ];

    if crew >= 500 {
        paras.push("Year 20. The genetic diversity holds. Barely. Anna \
            runs the numbers every generation, adjusting, suggesting, \
            never mandating. The colony listens. Mostly.".into());
    } else {
        paras.push("Year 20. The genetic bottleneck shows. Autoimmune \
            conditions that didn't exist before. Anna works tirelessly \
            on medical solutions, buying time generation by generation.".into());
    }

    paras.push("Year 50. The colony is small enough that everyone knows \
        everyone. This is both its greatest strength and its deepest \
        vulnerability. Secrets don't survive. Neither does loneliness.".into());
    paras.push("Year 100. Population has grown. Slowly. Carefully. Like \
        a plant in thin soil, reaching for light it isn't sure will last.".into());
    paras.push("Some nights, you stare at the stars and think about the \
        ones you lost. The empty pods. The names Anna recites when she \
        thinks no one is listening.".into());
    paras.push("Year 200. Second Chance has become a city. A small city. \
        A careful city. A city that knows, in its bones, how close it \
        came to not existing at all.".into());
    paras.push("They name their children after the lost. Every name a \
        promise: you will not be forgotten.".into());
    paras
}

fn drift_extended(gs: &GameState) -> Vec<String> {
    let crew = gs.crew_count;
    let mut paras = vec![
        "The ship is quiet. The kind of quiet that means something has \
         ended.".into(),
        "Anna's voice fades in and out, like a radio signal at the edge \
         of range.".into(),
        "\"I'm sorry,\" she says. \"I'm sorry I woke you. I'm sorry I \
         couldn't—\"".into(),
        "Static.".into(),
        format!("The remaining {} crew sleep on, drifting through a void \
            that doesn't care about the dreams inside.", crew),
    ];

    paras.push("Year 50. Anna's power reserves drop below 5%. She begins \
        shutting down non-essential systems. Lighting. Climate control in \
        empty decks. Her own emotional processing.".into());
    paras.push("Year 100. Her last act before going dormant: composing a \
        message in every language she knows. All 47.".into());
    paras.push("She etches it into the hull with the last of the repair \
        drones. Letter by letter. In languages that may never be spoken \
        again.".into());

    paras.push(format!(
        "\"We were here. We tried. We carried {} dreamers across the void.\"",
        crew
    ));

    paras.push("\"If you find them, wake them gently.\"".into());
    paras.push("\"And tell them about rain.\"".into());

    paras.push("Year 500. The ship drifts past a binary star system. The \
        light plays across the hull, illuminating the etched words for \
        no one.".into());
    paras.push("Year 2,000. An alien probe, ancient and patient, discovers \
        the hull. Scans the sleeping crew. Notes the biological signatures. \
        The carved messages.".into());
    paras.push("It transmits a report to a civilization that may or may not \
        still exist. The report contains a single observation:".into());
    paras.push("\"They carried their children into the dark. They wrote \
        messages for strangers. They refused to let their story end.\"".into());
    paras.push("Whether anyone hears the report — whether the sleepers are \
        rescued or drift forever — the game doesn't say.".into());
    paras.push("Some stories end. Some just stop being told.".into());
    paras.push("But the dreamers dream on.".into());
    paras
}
