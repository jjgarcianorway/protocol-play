// SPDX-License-Identifier: GPL-3.0-or-later

//! Credits text content — all sections for the scrolling credits screen.

/// A single section in the credits scroll.
pub struct CreditsSection {
    pub heading: &'static str,
    pub lines: &'static [&'static str],
}

/// Build the full credits content.
pub fn credits_sections() -> Vec<CreditsSection> {
    vec![
        CreditsSection {
            heading: "",
            lines: &[
                "protocol: play",
                "",
                "A game about carrying each other",
            ],
        },
        CreditsSection {
            heading: "Created by",
            lines: &["jjgarcianorway"],
        },
        CreditsSection {
            heading: "Story & Dialog",
            lines: &["Written with Claude (Anthropic)"],
        },
        CreditsSection {
            heading: "Game Engine",
            lines: &["Bevy 0.18 \u{2014} bevyengine.org"],
        },
        CreditsSection {
            heading: "Programming Language",
            lines: &["Rust \u{2014} rust-lang.org"],
        },
        CreditsSection {
            heading: "Libraries & Dependencies",
            lines: &[
                "bevy 0.18 \u{2014} Game engine",
                "image 0.25 \u{2014} Image processing",
                "rand 0.8 \u{2014} Random number generation",
                "serde 1.0 \u{2014} Serialization framework",
                "serde_json 1.0 \u{2014} JSON serialization",
            ],
        },
        CreditsSection {
            heading: "Font",
            lines: &["Fira Sans \u{2014} Mozilla (SIL Open Font License)"],
        },
        CreditsSection {
            heading: "AI Assistance",
            lines: &[
                "Claude by Anthropic",
                "Story writing, code generation, dialog design",
            ],
        },
        CreditsSection {
            heading: "Inspirations",
            lines: &[
                "Baldur's Gate 3 (Larian Studios)",
                "Detroit: Become Human (Quantic Dream)",
                "The Walking Dead (Telltale Games)",
                "The Wolf Among Us (Telltale Games)",
                "Life is Strange (Dontnod Entertainment)",
                "Battlestar Galactica (Ronald D. Moore)",
                "The Expanse (James S.A. Corey)",
            ],
        },
        CreditsSection {
            heading: "Special Thanks",
            lines: &[
                "Paula \u{2014} First playtester",
                "Barbara \u{2014} Playtester",
            ],
        },
        CreditsSection {
            heading: "Open Source",
            lines: &[
                "Licensed under GPL-3.0-or-later",
                "github.com/jjgarcianorway/protocol-play",
            ],
        },
        CreditsSection {
            heading: "The Characters",
            lines: &[],
        },
    ]
}

/// Named story characters and their one-line epitaphs.
pub fn character_credits() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Anna", "who remembered every name so no one was ever truly gone"),
        ("Dr. Amira Hassan", "who never stopped believing in rivers"),
        ("Viktor Petrov", "who woke at 4:17 every night"),
        ("Mei-Lin Chen", "who smuggled jasmine into the stars"),
        ("Kwame Asante", "who designed bridges for a world that didn't exist yet"),
        ("Kofi Asante", "who gave his seat to a stranger"),
        ("Dr. Elena Vasquez", "who made the choices no one else could bear"),
        ("Youssef Karam", "who watched everyone and trusted no one"),
        ("Yuki Tanabe", "who built paradise and then opened the gates"),
        ("Dr. Aisha Okonkwo", "who decided which imperfections to save"),
        ("Tom\u{00e1}s Herrera", "who composed silence into something we could carry"),
    ]
}

/// The final emotional lines after all characters.
pub fn closing_lines() -> Vec<&'static str> {
    vec![
        "",
        "And the 14,892 who trusted us to carry them",
        "",
        "",
        "\u{201c}The beauty isn't in perfection.",
        "It's in continuing after the mistake.\u{201d}",
        "",
        "\u{2014} Anna",
        "",
        "",
        "",
        "",
        "",
    ]
}
