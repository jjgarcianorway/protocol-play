// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;

/// Marker for ALL entities spawned by Delivery in integrated mode.
#[derive(Component)]
#[allow(dead_code)]
pub struct DeliveryEntity;

/// Resource color for pods.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PodColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl PodColor {
    pub const ALL: [PodColor; 5] = [
        PodColor::Red, PodColor::Green, PodColor::Blue,
        PodColor::Yellow, PodColor::Purple,
    ];

    pub fn index(&self) -> usize {
        match self {
            PodColor::Red => 0,
            PodColor::Green => 1,
            PodColor::Blue => 2,
            PodColor::Yellow => 3,
            PodColor::Purple => 4,
        }
    }

    pub fn rgb(&self) -> (f32, f32, f32) {
        POD_COLORS[self.index()]
    }

    pub fn icon(&self) -> &str { RESOURCE_ICONS[self.index()] }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i % 5]
    }
}

/// Game phase state machine.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DeliveryPhase {
    #[default]
    Playing,
    Results,
}

/// Tracks the full game state.
#[derive(Resource)]
pub struct DeliveryState {
    pub score: [u32; 5],
    pub wasted: u32,
    pub missed: u32,
    pub streak: u32,
    pub best_streak: u32,
    pub total_pods: u32,
    pub pods_spawned: u32,
    pub pods_resolved: u32,
    pub difficulty: f32,
    pub selected_slot: Option<usize>,
    pub spawn_timer: f32,
    pub game_started: bool,
}

impl Default for DeliveryState {
    fn default() -> Self {
        Self {
            score: [0; 5],
            wasted: 0,
            missed: 0,
            streak: 0,
            best_streak: 0,
            total_pods: TOTAL_PODS,
            pods_spawned: 0,
            pods_resolved: 0,
            difficulty: 0.0,
            selected_slot: None,
            spawn_timer: 1.0,
            game_started: false,
        }
    }
}

impl DeliveryState {
    /// Current streak multiplier.
    pub fn streak_mult(&self) -> f32 {
        if self.streak >= STREAK_TIER_3 { STREAK_MULT_3 }
        else if self.streak >= STREAK_TIER_2 { STREAK_MULT_2 }
        else if self.streak >= STREAK_TIER_1 { STREAK_MULT_1 }
        else { 1.0 }
    }

    /// Delivery efficiency (0..100).
    pub fn efficiency(&self) -> f32 {
        let total_delivered: u32 = self.score.iter().sum();
        let total_resolved = total_delivered + self.wasted + self.missed;
        if total_resolved == 0 { return 0.0; }
        total_delivered as f32 / total_resolved as f32 * 100.0
    }

    /// Current fall duration based on difficulty.
    pub fn fall_duration(&self) -> f32 {
        let t = self.difficulty.clamp(0.0, 1.0);
        INITIAL_FALL_DURATION + (MIN_FALL_DURATION - INITIAL_FALL_DURATION) * t
    }

    /// Current spawn interval based on difficulty.
    pub fn spawn_interval(&self) -> f32 {
        let t = self.difficulty.clamp(0.0, 1.0);
        INITIAL_SPAWN_INTERVAL + (MIN_SPAWN_INTERVAL - INITIAL_SPAWN_INTERVAL) * t
    }
}

/// Font resource for the delivery game.
#[derive(Resource)]
pub struct DeliveryFont(pub Handle<Font>);

// === Components ===

/// A falling resource pod.
#[derive(Component)]
pub struct Pod {
    pub color: PodColor,
    pub progress: f32,
    pub fall_duration: f32,
    pub routed: Option<usize>,
    pub route_progress: f32,
}

/// Marker for deposit slot buttons.
#[derive(Component)]
pub struct DepositSlot(pub usize);

/// Flash effect on a slot.
#[derive(Component)]
pub struct SlotFlash {
    pub timer: f32,
    pub correct: bool,
}

/// HUD text markers.
#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct StreakText;

#[derive(Component)]
pub struct PodsRemainingText;

/// Multiplier display (top-right, pulses when active).
#[derive(Component)]
pub struct MultiplierText;

/// Speed/difficulty meter.
#[derive(Component)]
pub struct SpeedMeterText;

/// Results screen marker.
#[derive(Component)]
pub struct ResultsScreen;

/// Return button on results.
#[derive(Component)]
pub struct ReturnButton;

/// Root UI container for cleanup.
#[derive(Component)]
pub struct DeliveryRoot;

/// The pod visual node (for updating position).
#[derive(Component)]
pub struct PodVisual;

/// A trail ghost behind a falling pod.
#[derive(Component)]
pub struct PodTrail {
    pub owner: Entity,
    pub delay_index: usize,
}

/// Star background dot.
#[derive(Component)]
pub struct StarDotD;

/// Streak milestone popup text.
#[derive(Component)]
pub struct StreakPopup {
    pub lifetime: f32,
}
