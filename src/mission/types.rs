// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;

/// Top-level application phase — controls main menu vs gameplay.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppPhase {
    ProfileSelect,
    #[default]
    MainMenu,
    Loading,
    Playing,
}

/// Ship status — central resource tracking all ship systems.
#[derive(Resource)]
pub struct ShipStatus {
    pub power: f32,
    pub life_support: f32,
    pub cryo: f32,
    pub shields: f32,
    pub repair: f32,
    pub crystals: u64,
    pub crew_count: u32,
    pub day: u32,
    pub distance_au: f32,
    pub bot_level: u32,
}

impl Default for ShipStatus {
    fn default() -> Self {
        Self {
            power: 78.0,
            life_support: 62.0,
            cryo: 91.0,
            shields: 41.0,
            repair: 33.0,
            crystals: 47,
            crew_count: 14_892,
            day: 847,
            distance_au: 23.4,
            bot_level: 12,
        }
    }
}

impl ShipStatus {
    /// Lowest resource level — determines urgency.
    #[allow(dead_code)]
    pub fn lowest_resource(&self) -> (usize, f32) {
        let vals = [self.power, self.life_support, self.cryo, self.shields, self.repair];
        let mut min_idx = 0;
        let mut min_val = vals[0];
        for (i, &v) in vals.iter().enumerate().skip(1) {
            if v < min_val { min_val = v; min_idx = i; }
        }
        (min_idx, min_val)
    }
}

/// Font resource for Mission Control.
#[derive(Resource)]
pub struct MissionFont(#[allow(dead_code)] pub Handle<Font>);

/// Displayed bar values (lerped toward targets for smooth animation).
#[derive(Resource)]
pub struct BarDisplayValues {
    pub values: [f32; 5],
}

impl Default for BarDisplayValues {
    fn default() -> Self {
        Self { values: [0.0; 5] }
    }
}

// === Components ===

/// Marker for the resource bar fill node (index 0-4).
#[derive(Component)]
pub struct ResourceBarFill(pub usize);

/// Marker for the resource percentage text (index 0-4).
#[derive(Component)]
pub struct ResourcePctText(pub usize);

/// Marker for crew count text.
#[derive(Component)]
pub struct CrewText;

/// Marker for journey progress text.
#[derive(Component)]
pub struct JourneyText;

/// Marker for crystal reserves text.
#[derive(Component)]
pub struct CrystalText;

/// Marker for a game selection card.
#[derive(Component, Clone, Copy)]
pub enum GameCard {
    BotGame,
    Gathering,
    Converter,
    Delivery,
}

/// Marker for game card status text.
#[derive(Component)]
pub struct CardStatusText(#[allow(dead_code)] pub GameCard);

/// Marker for game card recommended badge.
#[derive(Component)]
pub struct CardRecommended(#[allow(dead_code)] pub GameCard);

/// Anna's message text component.
#[derive(Component)]
pub struct AnnaMessageText;

/// Marker for Anna's glowing circle (portrait).
#[derive(Component)]
pub struct AnnaCircle;

/// Marker for the Anna panel (click-to-dismiss area).
#[derive(Component)]
pub struct AnnaPanelArea;

/// Anna's glow visual mood.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnnaMood {
    Normal,
    Warning,
    Story,
    Glitching,
}

/// Anna's message cycling state with full dialog support.
#[derive(Resource)]
pub struct AnnaState {
    pub current_msg: String,
    pub timer: f32,
    pub fade_alpha: f32,
    pub fading_out: bool,
    pub is_story_msg: bool,
    pub mood: AnnaMood,
    pub last_personality_idx: Option<usize>,
    /// Whether the player clicked to dismiss the current message.
    pub dismissed: bool,
    /// Queue of pending messages (text, is_story).
    pub queue: Vec<(String, bool)>,
    /// Whether we've shown the first message yet.
    pub initialized: bool,
}

impl Default for AnnaState {
    fn default() -> Self {
        Self {
            current_msg: String::new(),
            timer: 0.5,
            fade_alpha: 0.0,
            fading_out: false,
            is_story_msg: false,
            mood: AnnaMood::Normal,
            last_personality_idx: None,
            dismissed: false,
            queue: Vec::new(),
            initialized: false,
        }
    }
}

/// Timer resource for resource drain and periodic saving.
#[derive(Resource)]
pub struct DrainTimer {
    pub day_timer: f32,
    pub save_timer: f32,
}

impl Default for DrainTimer {
    fn default() -> Self {
        Self { day_timer: 0.0, save_timer: 0.0 }
    }
}

/// Running child game process.
#[derive(Resource)]
pub struct RunningGame(pub Option<std::process::Child>);

impl Default for RunningGame {
    fn default() -> Self {
        Self(None)
    }
}

/// Marker for the "Game in progress..." overlay.
#[derive(Component)]
pub struct GameRunningOverlay;

/// Marker for drain rate text next to resource bars.
#[derive(Component)]
pub struct DrainRateText(#[allow(dead_code)] pub usize);

/// Star twinkle component.
#[derive(Component)]
pub struct StarTwinkle {
    pub phase: f32,
    pub speed: f32,
}
