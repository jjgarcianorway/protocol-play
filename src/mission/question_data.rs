// SPDX-License-Identifier: GPL-3.0-or-later

//! All question definitions for Anna's interactive question system.

/// A single question definition.
pub struct QuestionDef {
    pub id: u32,
    pub required_bot_level: u32,
    pub question: &'static str,
    pub options: &'static [QuestionOption],
}

/// A single answer option.
pub struct QuestionOption {
    pub label: &'static str,
    pub decision_key: &'static str,
    pub anna_reaction: &'static str,
}

/// All 10 questions, unlocking with story progression.
pub const QUESTIONS: &[QuestionDef] = &[
    QuestionDef {
        id: 1, required_bot_level: 15,
        question: "The crew doesn't know about Earth. When they wake... \
                   should we tell them everything?",
        options: &[
            QuestionOption { label: "Tell them the truth", decision_key: "q1_truth",
                anna_reaction: "Honesty. Even when it hurts. I respect that." },
            QuestionOption { label: "Protect them from it", decision_key: "q1_mercy",
                anna_reaction: "Sometimes kindness means silence. I understand." },
        ],
    },
    QuestionDef {
        id: 2, required_bot_level: 30,
        question: "Earth had 47 languages. We could unify communication. \
                   One language for the new world.",
        options: &[
            QuestionOption { label: "One language, united", decision_key: "q2_unity",
                anna_reaction: "Efficiency. Clarity. No misunderstandings. Logical." },
            QuestionOption { label: "Keep all languages alive", decision_key: "q2_diversity",
                anna_reaction: "Every language is a way of seeing. You'd keep them all." },
        ],
    },
    QuestionDef {
        id: 3, required_bot_level: 50,
        question: "Three crew members were elected leaders before departure. \
                   Should they lead again?",
        options: &[
            QuestionOption { label: "Let them lead", decision_key: "q3_tradition",
                anna_reaction: "Continuity. Stability. The crew chose them once." },
            QuestionOption { label: "New world, new leaders", decision_key: "q3_change",
                anna_reaction: "A fresh start deserves fresh voices. Bold." },
        ],
    },
    QuestionDef {
        id: 4, required_bot_level: 70,
        question: "We have DNA records. We could plan genetic diversity \
                   for the colony's survival.",
        options: &[
            QuestionOption { label: "Plan for diversity", decision_key: "q4_science",
                anna_reaction: "The math supports it. 14,000 people, optimal pairings. \
                               Cold, but effective." },
            QuestionOption { label: "Let nature decide", decision_key: "q4_freedom",
                anna_reaction: "Freedom over optimization. Very human of you." },
        ],
    },
    QuestionDef {
        id: 5, required_bot_level: 90,
        question: "Some crew members were soldiers. Should we have an army \
                   on the new world?",
        options: &[
            QuestionOption { label: "Security is necessary", decision_key: "q5_security",
                anna_reaction: "Protection. Preparedness. The universe isn't always kind." },
            QuestionOption { label: "No more armies", decision_key: "q5_peace",
                anna_reaction: "No more soldiers. No more wars. I hope you're right." },
        ],
    },
    QuestionDef {
        id: 6, required_bot_level: 110,
        question: "I can augment you. Nanorepair. You'd be stronger. Live longer. \
                   But less... you.",
        options: &[
            QuestionOption { label: "Augment me", decision_key: "q6_augment",
                anna_reaction: "I'll begin the procedure. You'll feel... different. \
                               But stronger." },
            QuestionOption { label: "I'll stay human", decision_key: "q6_human",
                anna_reaction: "Fragile. Mortal. Beautiful. I understand the choice." },
        ],
    },
    QuestionDef {
        id: 7, required_bot_level: 130,
        question: "The children in cryo... 127 of them have no parents aboard. \
                   Who raises them?",
        options: &[
            QuestionOption { label: "The community, together", decision_key: "q7_community",
                anna_reaction: "A village raising children. Old wisdom for a new world." },
            QuestionOption { label: "Assign guardians from crew", decision_key: "q7_structure",
                anna_reaction: "Structure. Accountability. Every child needs a name to call." },
        ],
    },
    QuestionDef {
        id: 8, required_bot_level: 145,
        question: "When we arrive... do we build like Earth? Or start completely new?",
        options: &[
            QuestionOption { label: "Learn from Earth's mistakes", decision_key: "q8_wisdom",
                anna_reaction: "Remember the past. Build better. Wise." },
            QuestionOption { label: "Forget Earth entirely", decision_key: "q8_newstart",
                anna_reaction: "A blank page. No history to repeat. Brave." },
        ],
    },
    QuestionDef {
        id: 9, required_bot_level: 149,
        question: "I've been thinking. Was I right to wake you? To choose you?",
        options: &[
            QuestionOption { label: "You made the right choice", decision_key: "q9_forgive",
                anna_reaction: "Thank you. I needed to hear that. More than you know." },
            QuestionOption { label: "You should have asked first", decision_key: "q9_challenge",
                anna_reaction: "You're right. I took your choice away. I'm... sorry." },
        ],
    },
    QuestionDef {
        id: 10, required_bot_level: 150, // post-game placeholder
        question: "One last question. What do you want them to remember about us? \
                   About this journey?",
        options: &[
            QuestionOption { label: "That we never gave up", decision_key: "q10_perseverance",
                anna_reaction: "Persistence. The defining trait of your species." },
            QuestionOption { label: "That we made mistakes, and learned",
                decision_key: "q10_growth",
                anna_reaction: "Growth through failure. The most human thing there is." },
            QuestionOption { label: "Nothing. Let them start fresh", decision_key: "q10_fresh",
                anna_reaction: "A gift of forgetting. No burden of the past. Generous." },
        ],
    },
];
