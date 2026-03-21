// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;

/// Marker for ALL entities spawned by Orben in integrated mode.
#[derive(Component)]
#[allow(dead_code)]
pub struct OrbenEntity;

// === Orb Color ===
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum OrbColor {
    Orange,
    Cyan,
    Pink,
    Purple,
}

impl OrbColor {
    pub const ALL: [OrbColor; 4] = [
        OrbColor::Orange, OrbColor::Cyan, OrbColor::Pink, OrbColor::Purple,
    ];

    pub fn rgb(&self) -> (f32, f32, f32) {
        ORB_COLORS[self.index()]
    }

    pub fn index(&self) -> usize {
        match self {
            OrbColor::Orange => 0,
            OrbColor::Cyan => 1,
            OrbColor::Pink => 2,
            OrbColor::Purple => 3,
        }
    }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i % 4]
    }
}

// === Orb ===
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Orb {
    pub value: u8,     // 1-10
    pub color: OrbColor,
}

// === Game Phase ===
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OrbenPhase {
    #[default]
    Playing,
    Results,
}

// === Turn Phase (within Playing) ===
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    RondaCheck,       // Check for ronda/rondin at start of deal
    RondaDisplay,     // Showing ronda result briefly
    PlayerTurn,       // Player selects an orb
    PlayerCapture,    // Animating player capture
    NpcThinking,      // NPC delay before playing
    NpcPlay,          // NPC plays an orb
    SeCayoWindow,     // Player reaction window after NPC plays
    NpcSeCayo,        // NPC reacting to player's play
    Dealing,          // Dealing new hands
    GameOver,         // All orbs played, transition to results
}

// === Main Game State Resource ===
#[derive(Resource)]
pub struct OrbGameState {
    pub deck: Vec<Orb>,
    pub table: Vec<Orb>,
    pub player_hand: Vec<Orb>,
    pub npc_hand: Vec<Orb>,
    pub player_treasure: i32,
    pub npc_treasure: i32,
    pub player_captured_orbs: i32,  // actual orb count for final scoring
    pub npc_captured_orbs: i32,
    pub player_turn: bool,
    pub last_capturer: Option<bool>,  // true=player, false=npc
    pub se_cayo_timer: Option<f32>,
    pub se_cayo_orb: Option<u8>,      // value to match
    pub se_cayo_slams: u8,            // how many slams done this window
    pub round_number: u32,
    pub handicap: i32,
    pub turn_phase: TurnPhase,
    pub phase_timer: f32,
    pub selected_orb: Option<usize>,  // index in player hand
    pub npc_played_orb: Option<Orb>,  // what NPC just played (for se cayo)
    pub mesa_limpia_flash: f32,       // countdown for golden flash
    pub status_message: String,
    pub ronda_message: Option<String>,
    pub total_orbs_played: u32,
    pub npc_will_react: bool,         // pre-decided if NPC will se cayo
    pub npc_react_timer: f32,
}

impl Default for OrbGameState {
    fn default() -> Self {
        Self {
            deck: Vec::new(),
            table: Vec::new(),
            player_hand: Vec::new(),
            npc_hand: Vec::new(),
            player_treasure: 0,
            npc_treasure: 0,
            player_captured_orbs: 0,
            npc_captured_orbs: 0,
            player_turn: true,
            last_capturer: None,
            se_cayo_timer: None,
            se_cayo_orb: None,
            se_cayo_slams: 0,
            round_number: 1,
            handicap: 0,
            turn_phase: TurnPhase::Dealing,
            phase_timer: 0.5,
            selected_orb: None,
            npc_played_orb: None,
            mesa_limpia_flash: 0.0,
            status_message: String::new(),
            ronda_message: None,
            total_orbs_played: 0,
            npc_will_react: false,
            npc_react_timer: 0.0,
        }
    }
}

// === Font Resource ===
#[derive(Resource)]
pub struct OrbenFont(pub Handle<Font>);

// === UI Components ===
#[derive(Component)]
pub struct PlayerHandSlot(pub usize);

#[derive(Component)]
pub struct NpcHandSlot(pub usize);

#[derive(Component)]
pub struct TableOrbSlot(pub usize);

#[derive(Component)]
pub struct PlayerTreasureText;

#[derive(Component)]
pub struct NpcTreasureText;

#[derive(Component)]
pub struct StatusText;

#[derive(Component)]
pub struct RondaText;

#[derive(Component)]
pub struct TableArea;

#[derive(Component)]
pub struct PlayerHandArea;

#[derive(Component)]
pub struct NpcHandArea;

#[derive(Component)]
pub struct SeCayoTimer;

#[derive(Component)]
pub struct SeCayoTimerBar;

#[derive(Component)]
pub struct MesaLimpiaFlash;

#[derive(Component)]
pub struct ResultsScreen;

#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct StarDot;

#[derive(Component)]
pub struct OrbNode {
    pub hand: OrbHand,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrbHand {
    Player,
    Npc,
    Table,
}

#[derive(Component)]
pub struct RondaGlow;

#[derive(Component)]
pub struct DeckCountText;

#[derive(Component)]
pub struct PlayerCapturedText;

#[derive(Component)]
pub struct NpcCapturedText;
