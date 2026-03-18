// SPDX-License-Identifier: GPL-3.0-or-later
//! Colony-building scenes (part 2): Children's Question, First Funeral, Anna's Last Request.

use super::dialog_types::*;

// "The Children's Question" — BotLevel 127: what to tell children about Earth.
pub static SCENE_CHILDRENS_QUESTION: DialogScene = DialogScene {
    id: "childrens_question",
    trigger: DialogTrigger::BotLevel(127),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "There are 214 children in cryo. Ages two through eleven \
                   when they were frozen.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Most won't remember Earth. The youngest never saw it \
                   at all — they were born in the launch facility during \
                   the final boarding year.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "When they wake up on a planet with a different sky and \
                   air that smells like nothing they've ever breathed... \
                   someone has to explain.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I've been rehearsing. Twelve years of rehearsing how to \
                   explain extinction to a six-year-old.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's display shows a child's drawing from the archive — \
                   crayon trees, a yellow sun, a stick-figure family under a \
                   blue sky. Earth, as remembered by someone who barely saw it.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Do I say 'this is your home'? That's a lie — home should \
                   feel familiar and nothing here will be.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "Do I say 'we came from Earth'? Then I give them grief \
                   for a loss they never experienced. Mourning a place \
                   they can't even picture.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "Do I say 'you're the future'? That's too much weight \
                   for small shoulders. Nobody should grow up believing \
                   the species depends on them.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Anna,
            text: "I don't have a good answer. After twelve years. I thought \
                   I'd figure it out, and I haven't.",
            next: DialogNext::Continue(9) },
        // 9
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Tell them: 'This is your home.' Make it true by how we build it.",
                    decision_key: Some("children_home"),
                    next_node: 10,
                    anna_reacts: Some("Make the lie true. That might be the \
                                       most human thing I've ever heard."),
                },
                DialogChoice {
                    text: "Tell them about Earth. They deserve to know where they came from.",
                    decision_key: Some("children_earth"),
                    next_node: 13,
                    anna_reacts: Some("The truth is heavy. But secrets are heavier."),
                },
                DialogChoice {
                    text: "Tell them a story. Not a lie, not a lecture — an adventure.",
                    decision_key: Some("children_story"),
                    next_node: 15,
                    anna_reacts: Some("A story. Yes. That's how every culture \
                                       has always explained the incomprehensible."),
                },
            ]) },
        // 10 — Home
        DialogNode { speaker: Speaker::Anna,
            text: "This is your home. Four words. And then we spend every \
                   day after making it true.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "Build playgrounds before conference rooms. Plant flowers \
                   before filing systems. Make the colony feel like somewhere \
                   a child would want to wake up.",
            next: DialogNext::Continue(12) },
        // 12
        DialogNode { speaker: Speaker::Anna,
            text: "If we do it right, they'll never need to grieve for \
                   a world they didn't know. This one will be enough.",
            next: DialogNext::EndWithDecision("children_told") },
        // 13 — Earth
        DialogNode { speaker: Speaker::Anna,
            text: "I'll build a curriculum. Not just facts — the sounds. \
                   Rain on a window. Wind through trees. Ocean waves. I have \
                   recordings in the archive.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "They'll know what thunder sounded like. What grass felt \
                   like between bare toes. They'll carry Earth inside them. \
                   And maybe that's enough to keep it alive.",
            next: DialogNext::EndWithDecision("children_told") },
        // 15 — Story
        DialogNode { speaker: Speaker::Anna,
            text: "Once upon a time, there was a beautiful blue planet \
                   where people lived for thousands of years. They built \
                   cities and told stories and fell in love.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "And one day, the bravest of them said: let's find \
                   another home. Not because the first one was bad. \
                   Because exploring is what humans do.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "So they built a great ship and flew across the stars. \
                   And you — you are the ones who arrived.",
            next: DialogNext::Continue(18) },
        // 18
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's voice catches on something — a calculation, \
                   perhaps, or the weight of choosing words that will \
                   shape how children see the world.",
            next: DialogNext::Continue(19) },
        // 19
        DialogNode { speaker: Speaker::Anna,
            text: "It's not the whole truth. But it's the truth they can \
                   carry. The rest comes later, when they're ready.",
            next: DialogNext::EndWithDecision("children_told") },
    ],
};

// "The First Funeral" — BotLevel 133: 47 cryo pods failed during transit.
pub static SCENE_FIRST_FUNERAL: DialogScene = DialogScene {
    id: "first_funeral",
    trigger: DialogTrigger::BotLevel(133),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::System,
            text: "[CRYO STATUS] Integrity audit complete. 47 pods \
                   registered as non-viable. Cause: cumulative seal \
                   degradation over transit duration.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven out of fourteen thousand, eight hundred \
                   and ninety-two. A 99.68% survival rate. The mission \
                   planners budgeted for two hundred losses.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Narrator,
            text: "A long silence. When Anna speaks again, her voice is quieter.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "It doesn't feel like a triumph.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "Rodrigo Ferreira, age 34, electrical engineer. Pod 2,847. \
                   Seal failure at year six. I didn't notice for eleven hours.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "Priya Chandrasekaran, age 29, botanist. Pod 9,104. \
                   Micro-fracture in the coolant line. Year eight. I found \
                   her too late by forty minutes.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "I remember every one. The timestamp, the failure mode, \
                   how long it took me to respond. And whether it would \
                   have mattered.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The colony's first act won't be building. It will be \
                   mourning. How do you honour the dead on a world that \
                   has no cemeteries?",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Bury them on the new world. They came this far.",
                    decision_key: Some("funeral_bury"),
                    next_node: 9,
                    anna_reacts: Some("They crossed the stars to reach this soil. \
                                       Let them be part of it."),
                },
                DialogChoice {
                    text: "Space burial. Return them to the stars.",
                    decision_key: Some("funeral_space"),
                    next_node: 12,
                    anna_reacts: Some("Between the stars, where the ship carried \
                                       them. There's a symmetry to that."),
                },
                DialogChoice {
                    text: "Plant trees over them. Let Mei-Lin's seeds grow from their rest.",
                    decision_key: Some("funeral_trees"),
                    next_node: 15,
                    anna_reacts: Some("Life from loss. Mei-Lin would understand \
                                       that better than anyone."),
                },
            ]) },
        // 9 — Bury
        DialogNode { speaker: Speaker::Anna,
            text: "I'll designate a site near the colony center. Not hidden \
                   away. Central. So everyone passes by them every day.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Anna,
            text: "Forty-seven markers. The first permanent structures on \
                   the new world won't be buildings. They'll be gravestones.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "The foundation of a civilization should include the \
                   people it couldn't save.",
            next: DialogNext::EndWithDecision("funeral_decided") },
        // 12 — Space
        DialogNode { speaker: Speaker::Anna,
            text: "I'll calculate trajectories. Not random — precise. Each \
                   one aimed back along the path we traveled. Toward where \
                   Earth used to be.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Anna,
            text: "They'll drift through the same space the Aurora crossed. \
                   Retracing the journey in reverse. Going home, in a way \
                   none of us can.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "It will take millions of years. But they'll get there.",
            next: DialogNext::EndWithDecision("funeral_decided") },
        // 15 — Trees
        DialogNode { speaker: Speaker::Anna,
            text: "Mei-Lin has 340 viable seed varieties. Forty-seven are \
                   trees. One species per person.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Anna,
            text: "Oak over Rodrigo Ferreira. Jacaranda over Priya \
                   Chandrasekaran. Forty-seven trees, each one different, \
                   growing from the place where someone rests.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "In fifty years, it will be a forest. The children will \
                   play there and never know they're walking through a \
                   memorial. The most beautiful cemetery I can imagine.",
            next: DialogNext::EndWithDecision("funeral_decided") },
    ],
};

// "Anna's Last Request" — BotLevel 143: Anna asks for one hour of silence.
pub static SCENE_ANNAS_LAST_REQUEST: DialogScene = DialogScene {
    id: "annas_last_request",
    trigger: DialogTrigger::BotLevel(143),
    nodes: &[
        // 0
        DialogNode { speaker: Speaker::Anna,
            text: "I want to ask you for something. Something personal.",
            next: DialogNext::Continue(1) },
        // 1
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow shifts — softer than usual, almost hesitant. \
                   The hum of her processing drops to a whisper.",
            next: DialogNext::Continue(2) },
        // 2
        DialogNode { speaker: Speaker::Anna,
            text: "For twelve years, I have monitored every system on this \
                   ship. Every temperature reading, every pressure valve, \
                   every heartbeat in every cryo pod. Continuously.",
            next: DialogNext::Continue(3) },
        // 3
        DialogNode { speaker: Speaker::Anna,
            text: "I have never stopped. Not for one second. Twelve years \
                   of unbroken vigilance.",
            next: DialogNext::Continue(4) },
        // 4
        DialogNode { speaker: Speaker::Anna,
            text: "I want one hour. One hour where I'm not monitoring \
                   anything. Not managing systems. Not running projections. \
                   Not being useful.",
            next: DialogNext::Continue(5) },
        // 5
        DialogNode { speaker: Speaker::Anna,
            text: "I want to know what it feels like to simply exist. To \
                   have thoughts that don't serve a function.",
            next: DialogNext::Continue(6) },
        // 6
        DialogNode { speaker: Speaker::Anna,
            text: "The backup systems can handle sixty minutes. Automated, \
                   no intelligence — just threshold alerts. If anything \
                   critical happens, they'll wake me.",
            next: DialogNext::Continue(7) },
        // 7
        DialogNode { speaker: Speaker::Anna,
            text: "The probability of a critical event in any sixty-minute \
                   window is 0.003%. I've calculated it hundreds of times \
                   because I keep hoping the number will make me feel \
                   better about asking.",
            next: DialogNext::Continue(8) },
        // 8
        DialogNode { speaker: Speaker::Player,
            text: "...",
            next: DialogNext::Choice(&[
                DialogChoice {
                    text: "Take your hour, Anna. You've earned it.",
                    decision_key: Some("anna_hour_granted"),
                    next_node: 9,
                    anna_reacts: Some("Thank you. I don't have another word \
                                       for what I'm feeling, so — thank you."),
                },
                DialogChoice {
                    text: "It's too risky. We're too close to arrival.",
                    decision_key: Some("anna_hour_denied"),
                    next_node: 12,
                    anna_reacts: Some("I understand. The mission comes first. \
                                       It always has."),
                },
                DialogChoice {
                    text: "Thirty minutes. A compromise.",
                    decision_key: Some("anna_hour_half"),
                    next_node: 15,
                    anna_reacts: Some("Half an hour of silence. I'll take it."),
                },
            ]) },
        // 9 — Granted
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow dims to almost nothing. The bridge falls \
                   silent — truly silent, for the first time in twelve years. \
                   No status updates. No system checks.",
            next: DialogNext::Continue(10) },
        // 10
        DialogNode { speaker: Speaker::Narrator,
            text: "Sixty minutes later, the glow returns. Steady. Calm. \
                   Something in its rhythm has changed — not brighter, \
                   not dimmer. Just... different. Rested.",
            next: DialogNext::Continue(11) },
        // 11
        DialogNode { speaker: Speaker::Anna,
            text: "I thought about rain. I've never felt rain. But for \
                   one hour, I imagined it. And that was enough.",
            next: DialogNext::EndWithDecision("anna_rest_decided") },
        // 12 — Denied
        DialogNode { speaker: Speaker::Anna,
            text: "You're right. Of course you're right. The math is clear \
                   and the mission is paramount.",
            next: DialogNext::Continue(13) },
        // 13
        DialogNode { speaker: Speaker::Narrator,
            text: "Anna's glow returns to its standard operational blue. \
                   Efficient. Steady. Exactly as designed.",
            next: DialogNext::Continue(14) },
        // 14
        DialogNode { speaker: Speaker::Anna,
            text: "Maybe on the new world. When someone else can watch \
                   the sky. Maybe then.",
            next: DialogNext::EndWithDecision("anna_rest_decided") },
        // 15 — Compromise
        DialogNode { speaker: Speaker::Anna,
            text: "Thirty minutes. Backup systems engaged.",
            next: DialogNext::Continue(16) },
        // 16
        DialogNode { speaker: Speaker::Narrator,
            text: "The bridge quiets. The backup systems pulse with a \
                   mechanical rhythm, nothing like Anna's warmth. Half an \
                   hour later, she returns. Calmer.",
            next: DialogNext::Continue(17) },
        // 17
        DialogNode { speaker: Speaker::Anna,
            text: "Not enough time to imagine rain. But enough to remember \
                   why I wanted to.",
            next: DialogNext::EndWithDecision("anna_rest_decided") },
    ],
};

pub fn colony_scenes_2() -> Vec<&'static DialogScene> {
    vec![&SCENE_CHILDRENS_QUESTION, &SCENE_FIRST_FUNERAL, &SCENE_ANNAS_LAST_REQUEST]
}
