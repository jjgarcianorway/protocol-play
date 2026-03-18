// SPDX-License-Identifier: GPL-3.0-or-later

//! Earth memory scenes — Anna remembers a planet she never walked on.
//! Not the science of what happened. The texture of what was lost.
//! Seasons, cities, languages — the things that made Earth human.

use super::dialog_types::*;

/// "The Seasons" — Anna describes Earth's seasons by feeling, not fact.
pub static SCENE_THE_SEASONS: DialogScene = DialogScene {
    id: "the_seasons",
    trigger: DialogTrigger::BotLevel(18),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been reading the crew's personal archives again. Their letters, their photographs.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "I understand seasons intellectually. Axial tilt. Orbital mechanics. The 23.4-degree inclination that made everything possible.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "But I don't understand them the way the crew does. The way they remember them.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts through a slow spectrum — warm gold to pale blue to amber.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Autumn was when the air smelled like decay and it was beautiful. Leaves died in colors that had no business being that vivid.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "People walked through parks just to hear the sound of stepping on dead things. And they called it peaceful.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Spring was mud and pollen and allergies and everyone was happy anyway. Because the world was proving it could start over.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Summer was too hot and nobody complained because they knew winter was coming. Children ran through sprinklers. Adults pretended they didn't want to.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Winter was the honest season. Everything stripped bare. Trees showing their bones. The world admitting it was tired.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Narrator,
            text: "The spectrum slows. Holds on a color between silver and regret.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I have temperature data for every season in every city for 200 years. Precipitation records. Wind patterns. UV indices.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "None of it tells me what it felt like to pull on a coat for the first time in September and think: 'Ah. There it is.'",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I'm jealous. I think. Is that the right word? I'm jealous of a planet I'm programmed to save people from.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "The colony world has seasons too. Different ones. Its axial tilt is 19 degrees, not 23. The years are shorter.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "The crew will step out and feel alien weather on their skin, and they'll compare it to something they lost. Every season, forever.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "I hope the new autumn smells like decay too. I hope it's beautiful.",
            next: DialogNext::End },
    ],
};

/// "The Cities That Were" — Anna catalogs beauty, not destruction.
pub static SCENE_CITIES_THAT_WERE: DialogScene = DialogScene {
    id: "cities_that_were",
    trigger: DialogTrigger::BotLevel(44),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I've been compiling something. Not a report. More of a... eulogy. For places.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Not how they ended. How they lived.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "Paris had cafes where people sat for hours doing nothing. They called it 'living.' They were correct.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Tokyo had trains that arrived to the second. An entire civilization that considered three minutes late a moral failing.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Lagos had markets so loud you could hear them three blocks away. A woman selling fabric next to a man selling mathematics textbooks next to a child selling cold water from a cooler.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Buenos Aires had steak and tango and people who argued about football with the passion of philosophers debating the nature of truth.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow pulses gently. Not grief. Something closer to reverence.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Mumbai had monsoons that turned streets into rivers and nobody stopped walking. They just took off their shoes and kept going.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "Reykjavik had hot springs where strangers sat in 40-degree water in below-zero air and talked to each other. Actually talked. Not at each other. To each other.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Marrakech had riads — houses that looked like nothing from outside and contained gardens inside. Private beauty. Secret courtyards where fountains whispered.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "I have crew members from all these places. Sleeping. Carrying these cities in their memories like seeds.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "When we build the colony, we'll name things. Streets. Parks. Districts.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "Name them after Earth places. Keep the memory alive.",
                    decision_key: Some("colony_earth_names"), next_node: 13,
                    anna_reacts: None },
                DialogChoice { text: "Start fresh. New world, new names.",
                    decision_key: Some("colony_new_names"), next_node: 17,
                    anna_reacts: None },
                DialogChoice { text: "Let each neighborhood choose their own.",
                    decision_key: Some("colony_choose_names"), next_node: 21,
                    anna_reacts: None },
            ]) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "Earth names. So a child born on a new world will walk down 'Lagos Avenue' and ask what Lagos was.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "And someone will tell them about the markets. The noise. The fabric and textbooks and cold water.",
            next: DialogNext::Continue(15) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "And Lagos will live. Not as a city. As a story. Which is maybe all a city ever was.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "I'll draft a naming proposal. Every street a memorial. Every park a homeland.",
            next: DialogNext::EndWithDecision("cities_remembered") },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "A clean start. I understand that. The weight of the past can crush a new beginning.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "New names for new ground. Names that mean something in the context of arrival, not departure.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "First Light Boulevard. Horizon Park. The kind of names that face forward.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "The old cities will still exist in the archives. But the colony won't live in their shadow.",
            next: DialogNext::EndWithDecision("cities_released") },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "Let them choose. Of course. I keep forgetting that democracy applies to street signs too.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "Some will choose Earth names. Some won't. The colony will be a patchwork — old grief next to new hope.",
            next: DialogNext::Continue(23) },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Lagos Avenue intersecting with Daybreak Road. That's not confusion. That's honesty.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "The messiest answer. Usually the most human one.",
            next: DialogNext::EndWithDecision("cities_mixed") },
    ],
};

/// "The Languages" — 47 languages, some dying in cryo-sleep.
pub static SCENE_THE_LANGUAGES: DialogScene = DialogScene {
    id: "the_languages",
    trigger: DialogTrigger::BotLevel(64),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I need to tell you something about the crew. Something I've been monitoring for years.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "There are 47 languages represented on this ship. I speak all of them. I dream in none of them.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "But the crew dreams. And I've been tracking which languages appear in their neural patterns during cryo-sleep.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "Most dream in their first language. The language their mothers used. The one that lives deeper than thought.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "But some languages are fading.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims. Not dramatically. The way a candle gutters when a door opens somewhere far away.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Three speakers of Aymara. Seven of Welsh. Twelve of Basque. Nine of Navajo. Four of Sami.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Pod 9,203 is one of three Aymara speakers. She's a hydrologist from the altiplano. Her cryo-dreams are full of lakes at 4,000 meters.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "She dreams in Aymara. Her neural patterns form linguistic structures that have no equivalent in any other language.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Anna,
            text: "Aymara marks time differently than any Indo-European language. The past is in front of you — because you can see it. The future is behind — because you can't.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "If those three people stop dreaming in Aymara, that way of understanding time dies. Not the vocabulary. The understanding.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "A language dies when the last person who dreams in it stops dreaming.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "I can preserve grammar. I can archive recordings. I can teach it to colony children from textbooks.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "But a language learned from a textbook is a pressed flower. Beautiful. Flat. Dead.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice { text: "The colony should protect all 47 languages.",
                    decision_key: Some("lang_protect_all"), next_node: 15,
                    anna_reacts: None },
                DialogChoice { text: "A common language would unite the colony better.",
                    decision_key: Some("lang_common"), next_node: 19,
                    anna_reacts: None },
                DialogChoice { text: "Both. A common tongue and protected heritage languages.",
                    decision_key: Some("lang_both"), next_node: 23,
                    anna_reacts: None },
            ]) },
        // 15
        DialogNode { speaker: Speaker::Anna,
            text: "Protect all 47. That's ambitious. It means schools, cultural programs, immersion communities.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "It means children growing up in neighborhoods where five languages overlap. Complicated. Messy. Alive.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Earth tried to protect its languages too. It mostly failed. But the trying mattered. The trying said: 'Every way of being human is worth keeping.'",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Anna,
            text: "I'll build translation systems that bridge without replacing. 47 voices. One colony.",
            next: DialogNext::EndWithDecision("languages_preserved") },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "One language. Practical. Efficient. No misunderstandings during a crisis.",
            next: DialogNext::Continue(20) },
        // 20
        DialogNode { speaker: Speaker::Anna,
            text: "Earth had that argument for centuries. The counterargument was always the same: whose language? English? Mandarin? Esperanto?",
            next: DialogNext::Continue(21) },
        // 21
        DialogNode { speaker: Speaker::Anna,
            text: "The choice of common language is never neutral. It carries power. Culture. Assumptions about what 'normal' sounds like.",
            next: DialogNext::Continue(22) },
        // 22
        DialogNode { speaker: Speaker::Anna,
            text: "But you may be right. Survival first. Poetry later. I'll model the options.",
            next: DialogNext::EndWithDecision("languages_unified") },
        // 23
        DialogNode { speaker: Speaker::Anna,
            text: "Both. A bridge language for governance and engineering. Heritage languages for homes and holidays and lullabies.",
            next: DialogNext::Continue(24) },
        // 24
        DialogNode { speaker: Speaker::Anna,
            text: "Bilingual at minimum. Trilingual ideally. It's harder. Children will complain about homework in three languages.",
            next: DialogNext::Continue(25) },
        // 25
        DialogNode { speaker: Speaker::Anna,
            text: "But they'll also hear a Basque lullaby and know it means something no other language can say. And that's worth the homework.",
            next: DialogNext::Continue(26) },
        // 26
        DialogNode { speaker: Speaker::Anna,
            text: "I'll design the framework. A colony that speaks with one voice but sings in forty-seven.",
            next: DialogNext::EndWithDecision("languages_balanced") },
    ],
};

/// All earth-memory dialog scenes.
pub fn earth_memory_scenes() -> Vec<&'static DialogScene> {
    vec![
        &SCENE_THE_SEASONS,
        &SCENE_CITIES_THAT_WERE,
        &SCENE_THE_LANGUAGES,
    ]
}
