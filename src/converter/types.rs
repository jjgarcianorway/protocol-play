// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;

// === Crystal Color (local to converter) ===
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CrystalColor {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl CrystalColor {
    pub const ALL: [CrystalColor; 5] = [
        CrystalColor::Red, CrystalColor::Green, CrystalColor::Blue,
        CrystalColor::Yellow, CrystalColor::Purple,
    ];

    pub fn rgb(&self) -> (f32, f32, f32) {
        CRYSTAL_COLORS[self.index()]
    }

    pub fn color(&self) -> Color {
        let (r, g, b) = self.rgb();
        Color::srgb(r, g, b)
    }

    pub fn index(&self) -> usize {
        match self {
            CrystalColor::Red => 0,
            CrystalColor::Green => 1,
            CrystalColor::Blue => 2,
            CrystalColor::Yellow => 3,
            CrystalColor::Purple => 4,
        }
    }

    pub fn name(&self) -> &str { RESOURCE_NAMES[self.index()] }
    pub fn icon(&self) -> &str { RESOURCE_ICONS[self.index()] }

    pub fn from_index(i: usize) -> Self {
        Self::ALL[i % 5]
    }
}

// === Game States ===
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConverterPhase {
    #[default]
    Processing,
    Results,
}

// === Grid processing phases ===
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridPhase {
    Idle,          // Player can click
    Bursting,      // Chain is bursting (animated)
    Gravity,       // Crystals falling
    CascadeCheck,  // Checking for auto-trigger groups
    Refilling,     // New crystals sliding in from top
}

// === Resources ===
#[derive(Resource)]
pub struct GridState {
    pub cells: Vec<Vec<Option<CrystalColor>>>, // [row][col], row 0 = top
    pub width: usize,
    pub height: usize,
    pub phase: GridPhase,
    pub phase_timer: f32,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            cells: vec![vec![None; GRID_COLS]; GRID_ROWS],
            width: GRID_COLS,
            height: GRID_ROWS,
            phase: GridPhase::Idle,
            phase_timer: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct CrystalPile {
    pub total: u64,
    pub remaining: u64,
}

impl Default for CrystalPile {
    fn default() -> Self {
        Self { total: INITIAL_PILE_SIZE, remaining: INITIAL_PILE_SIZE }
    }
}

#[derive(Resource, Default)]
pub struct ResourceTanks {
    pub levels: [f32; 5], // accumulated resource units
}

#[derive(Resource, Default)]
pub struct ConversionStats {
    pub total_converted: u64,
    pub chains_triggered: u32,
    pub best_chain: u32,
    pub cascades: u32,
}

#[derive(Resource)]
pub struct ConverterFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct HoveredGroup {
    pub cells: Vec<(usize, usize)>, // (row, col)
    pub color: Option<CrystalColor>,
}

// === Components ===
#[derive(Component)]
pub struct GridCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct CellCrystal;

#[derive(Component)]
pub struct ChainSizeLabel;

#[derive(Component)]
pub struct TankFill(pub usize); // color index

#[derive(Component)]
pub struct TankLabel(pub usize);

#[derive(Component)]
pub struct PileCountText;

#[derive(Component)]
pub struct PileFill;

#[derive(Component)]
pub struct BurstParticle {
    pub target: Vec2,  // screen position of target tank
    pub start: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color_index: usize,
}

#[derive(Component)]
pub struct ResultsScreen;

#[derive(Component)]
pub struct ReturnButton;

#[derive(Component)]
pub struct ConverterRoot; // root UI node, despawn on cleanup

// === Helper: efficiency multiplier ===
pub fn efficiency_mult(chain_size: u32) -> f32 {
    let mut mult = EFFICIENCY_TABLE[0].1;
    for &(min_size, m) in &EFFICIENCY_TABLE {
        if chain_size >= min_size { mult = m; }
    }
    mult
}
