// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;

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
    /// Get resource value by index (0-4).
    pub fn resource(&self, index: usize) -> f32 {
        match index {
            0 => self.power,
            1 => self.life_support,
            2 => self.cryo,
            3 => self.shields,
            4 => self.repair,
            _ => 0.0,
        }
    }

    /// Lowest resource level — determines urgency.
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
pub struct MissionFont(pub Handle<Font>);

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
pub struct CardStatusText(pub GameCard);

/// Marker for game card recommended badge.
#[derive(Component)]
pub struct CardRecommended(pub GameCard);

/// Anna's message text component.
#[derive(Component)]
pub struct AnnaMessageText;

/// Anna's message cycling state.
#[derive(Resource)]
pub struct AnnaState {
    pub current_msg: String,
    pub timer: f32,
    pub fade_alpha: f32,
    pub fading_out: bool,
}

impl Default for AnnaState {
    fn default() -> Self {
        Self {
            current_msg: String::new(),
            timer: 0.5, // short delay before first message
            fade_alpha: 0.0,
            fading_out: false,
        }
    }
}

/// Star twinkle component.
#[derive(Component)]
pub struct StarTwinkle {
    pub phase: f32,
    pub speed: f32,
}
