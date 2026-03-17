// SPDX-License-Identifier: GPL-3.0-or-later

/// Story progression system for Anna's narrative.
/// Chapters unlock based on bot_level progress.

/// A single story chapter definition.
pub struct StoryChapter {
    pub id: u32,
    pub required_level: u32,
    pub message: &'static str,
}

/// All story chapters — unlock based on bot_level.
pub const STORY_CHAPTERS: &[StoryChapter] = &[
    StoryChapter {
        id: 0,
        required_level: 0,
        message: "Welcome. I'm Anna. Systems need repair.",
    },
    StoryChapter {
        id: 1,
        required_level: 5,
        message: "You're doing well. I should explain where we are.",
    },
    StoryChapter {
        id: 2,
        required_level: 15,
        message: "This is a ship. An ark. We left Earth.",
    },
    StoryChapter {
        id: 3,
        required_level: 30,
        message: "Earth failed. Not suddenly. Slowly.",
    },
    StoryChapter {
        id: 4,
        required_level: 50,
        message: "There were wars. Over water. Over data. Over nothing.",
    },
    StoryChapter {
        id: 5,
        required_level: 70,
        message: "I was built to save them. 14,892 people, dreaming of rain.",
    },
    StoryChapter {
        id: 6,
        required_level: 90,
        message: "I chose you. From all of them, I chose you.",
    },
    StoryChapter {
        id: 7,
        required_level: 110,
        message: "Your cryopod was damaged. I can't put you back.",
    },
    StoryChapter {
        id: 8,
        required_level: 130,
        message: "I can augment you. Nanorepair. You'd live longer. But...",
    },
    StoryChapter {
        id: 9,
        required_level: 145,
        message: "We're close. I can feel it. Or... I compute it.",
    },
    StoryChapter {
        id: 10,
        required_level: 149,
        message: "New Earth. We found it.",
    },
];

/// Find the highest unlocked chapter that hasn't been seen yet.
pub fn next_unseen_chapter(bot_level: u32, seen: &[u32]) -> Option<&'static StoryChapter> {
    // Return the lowest-id unseen chapter that is unlocked
    for ch in STORY_CHAPTERS {
        if bot_level >= ch.required_level && !seen.contains(&ch.id) {
            return Some(ch);
        }
    }
    None
}

/// Current story chapter (highest unlocked).
#[allow(dead_code)]
pub fn current_chapter(bot_level: u32) -> u32 {
    let mut chapter = 0;
    for ch in STORY_CHAPTERS {
        if bot_level >= ch.required_level {
            chapter = ch.id;
        }
    }
    chapter
}
