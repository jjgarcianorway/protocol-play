// SPDX-License-Identifier: GPL-3.0-or-later

//! Core types for the branching dialog system.
//! This is a visual-novel-style conversation engine with typewriter text,
//! branching choices, and persistent decisions.

use bevy::prelude::*;

/// When a dialog scene should trigger.
#[derive(Debug, Clone)]
pub enum DialogTrigger {
    /// After reaching this bot repair level.
    BotLevel(u32),
    /// After returning from a gathering run.
    GatheringReturn,
    /// When a resource index (0-4) drops below critical (< 20%).
    ResourceCritical(usize),
    /// First time a named event occurs (e.g., "first_gathering").
    FirstTime(&'static str),
    /// Only on the Nth playthrough (0-indexed).
    PlaythroughN(u32),
    /// After the player made a specific decision key.
    Decision(&'static str),
    /// When crew drops below this threshold.
    CrewLoss(u32),
}

/// Who is speaking a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    /// "Anna:" prefix, warm blue color.
    Anna,
    /// "[SYSTEM]" prefix, orange/warning color.
    System,
    /// No prefix, italic style, muted grey.
    Narrator,
    /// "You:" prefix, white.
    Player,
}

/// What happens after a dialog node.
#[derive(Debug, Clone)]
pub enum DialogNext {
    /// Continue to the node at this index.
    Continue(usize),
    /// Present choices to the player.
    Choice(&'static [DialogChoice]),
    /// Scene ends normally.
    End,
    /// Scene ends and saves a decision key.
    EndWithDecision(&'static str),
}

/// A single choice the player can make.
#[derive(Debug, Clone)]
pub struct DialogChoice {
    /// Button label text.
    pub text: &'static str,
    /// If present, saved to GameState.decisions when chosen.
    pub decision_key: Option<&'static str>,
    /// Index of the next dialog node after this choice.
    pub next_node: usize,
    /// Optional brief Anna reaction shown before continuing.
    pub anna_reacts: Option<&'static str>,
}

/// A single node in a conversation tree.
#[derive(Debug, Clone)]
pub struct DialogNode {
    /// Who is speaking.
    pub speaker: Speaker,
    /// The text content.
    pub text: &'static str,
    /// What happens after this node is finished.
    pub next: DialogNext,
}

/// A complete dialog scene with branching conversation.
#[derive(Debug, Clone)]
pub struct DialogScene {
    /// Unique identifier for this scene.
    pub id: &'static str,
    /// When this scene should trigger.
    pub trigger: DialogTrigger,
    /// The conversation tree (indexed by DialogNext).
    pub nodes: &'static [DialogNode],
}

/// Runtime state of the dialog engine.
#[derive(Resource)]
pub struct DialogState {
    /// Queue of scenes waiting to play.
    pub queue: Vec<&'static DialogScene>,
    /// Currently playing scene (if any).
    pub active_scene: Option<ActiveDialog>,
    /// Whether we've checked triggers this session return.
    pub checked_triggers: bool,
    /// Delay before showing dialog after returning from a game.
    pub delay_timer: f32,
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            queue: Vec::new(),
            active_scene: None,
            checked_triggers: false,
            delay_timer: 2.0,
        }
    }
}

/// State of the currently active dialog scene.
pub struct ActiveDialog {
    /// Reference to the scene being played.
    pub scene: &'static DialogScene,
    /// Current node index.
    pub node_index: usize,
    /// Typewriter state: how many characters are revealed.
    pub chars_revealed: usize,
    /// Total characters in current node text.
    pub total_chars: usize,
    /// Time accumulator for typewriter effect.
    pub char_timer: f32,
    /// Whether the full text is shown (typewriter complete or skipped).
    pub text_complete: bool,
    /// Whether choices are visible (delayed after text completes).
    pub choices_visible: bool,
    /// Choice delay timer (brief pause before showing options).
    pub choice_delay: f32,
    /// Anna reaction text being shown (from a choice).
    pub reaction_text: Option<&'static str>,
    /// Reaction display timer.
    pub reaction_timer: f32,
}

// === UI marker components ===

/// The full-screen dialog overlay.
#[derive(Component)]
pub struct DialogOverlay;

/// The speaker name label.
#[derive(Component)]
pub struct DialogSpeakerText;

/// The main dialog text area.
#[derive(Component)]
pub struct DialogBodyText;

/// A choice button with its index.
#[derive(Component)]
pub struct DialogChoiceBtn(pub usize);

/// Container for choice buttons (hidden until ready).
#[derive(Component)]
pub struct DialogChoiceContainer;

/// Anna's portrait circle in the dialog.
#[derive(Component)]
pub struct DialogAnnaCircle;

/// The subtle background glow behind the text panel.
#[derive(Component)]
pub struct DialogPanelGlow;

/// Skip indicator ("Click to continue" / "Click to skip").
#[derive(Component)]
pub struct DialogSkipHint;

// === Constants ===

/// Characters revealed per second (typewriter speed).
pub const TYPEWRITER_SPEED: f32 = 35.0;
/// Delay before showing choices after text completes (seconds).
pub const CHOICE_APPEAR_DELAY: f32 = 0.8;
/// How long Anna's reaction text shows before continuing.
pub const REACTION_DURATION: f32 = 2.5;
/// Delay before starting dialog after returning from game.
pub const DIALOG_START_DELAY: f32 = 2.0;

// === Dialog UI colors ===
pub const DIALOG_OVERLAY_BG: (f32, f32, f32, f32) = (0.02, 0.03, 0.06, 0.92);
pub const DIALOG_PANEL_BG: (f32, f32, f32, f32) = (0.06, 0.07, 0.12, 0.95);
pub const DIALOG_PANEL_CORNER: f32 = 14.0;
pub const DIALOG_MAX_WIDTH: f32 = 700.0;
pub const DIALOG_PADDING: f32 = 36.0;

pub const DIALOG_ANNA_COLOR: (f32, f32, f32) = (0.4, 0.7, 1.0);
pub const DIALOG_SYSTEM_COLOR: (f32, f32, f32) = (0.95, 0.6, 0.2);
pub const DIALOG_NARRATOR_COLOR: (f32, f32, f32) = (0.55, 0.58, 0.65);
pub const DIALOG_PLAYER_COLOR: (f32, f32, f32) = (0.92, 0.92, 0.95);
pub const DIALOG_BODY_COLOR: (f32, f32, f32) = (0.88, 0.86, 0.82);

pub const DIALOG_SPEAKER_FONT: f32 = 13.0;
pub const DIALOG_BODY_FONT: f32 = 19.0;
pub const DIALOG_CHOICE_FONT: f32 = 16.0;
pub const DIALOG_HINT_FONT: f32 = 12.0;

pub const DIALOG_CHOICE_BG: (f32, f32, f32, f32) = (0.10, 0.12, 0.18, 0.9);
pub const DIALOG_CHOICE_HOVER: (f32, f32, f32, f32) = (0.18, 0.22, 0.32, 0.95);
pub const DIALOG_CHOICE_BORDER: (f32, f32, f32, f32) = (0.28, 0.32, 0.48, 0.6);
pub const DIALOG_CHOICE_HOVER_BORDER: (f32, f32, f32, f32) = (0.48, 0.58, 0.82, 0.9);
pub const DIALOG_CHOICE_PAD: f32 = 14.0;
pub const DIALOG_CHOICE_CORNER: f32 = 8.0;

pub const DIALOG_GLOW_COLOR: (f32, f32, f32, f32) = (0.3, 0.5, 0.8, 0.15);
pub const DIALOG_GLOW_BLUR: f32 = 30.0;
pub const DIALOG_GLOW_SPREAD: f32 = 10.0;

pub const DIALOG_CIRCLE_SIZE: f32 = 50.0;
