// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use std::collections::HashSet;
use std::f32::consts::{FRAC_PI_2, PI};


// === Enums ===
#[derive(Default, PartialEq, Clone, Copy)]
pub enum Tool {
    #[default]
    Floor,
    Source,
    Turn,
    Delete,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotation(self) -> f32 {
        match self {
            Direction::North => 0.0,
            Direction::East => -FRAC_PI_2,
            Direction::South => PI,
            Direction::West => FRAC_PI_2,
        }
    }

    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    pub fn index(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

// === Resources ===
#[derive(Resource)]
pub struct BoardSize(pub u32);

#[derive(Resource, Default)]
pub struct SelectedTool(pub Tool);

#[derive(Resource, Default)]
pub struct HoveredCell(pub Option<(u32, u32)>);

#[derive(Resource, Default)]
pub struct HiddenTileEntity(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct GhostCell(pub Option<(u32, u32)>);

#[derive(Resource, Default)]
pub struct InventoryState {
    pub level: u8,
    pub direction: Option<Direction>,
    pub color_index: Option<usize>,
}

#[derive(Resource, Default)]
pub struct PlacedSources(pub HashSet<usize>);

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub enum PlayMode {
    #[default]
    Editing,
    Playing,
}

#[derive(Resource)]
pub struct PlayIcons {
    pub play: Handle<Image>,
    pub stop: Handle<Image>,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct InventoryIcons {
    pub floor: Handle<Image>,
    pub source: Handle<Image>,
    pub turn: Handle<Image>,
    pub delete: Handle<Image>,
    pub source_north: Handle<Image>,
    pub source_east: Handle<Image>,
    pub source_south: Handle<Image>,
    pub source_west: Handle<Image>,
    pub source_color_icons: Vec<Handle<Image>>,
    pub turn_north: Handle<Image>,
    pub turn_east: Handle<Image>,
    pub turn_south: Handle<Image>,
    pub turn_west: Handle<Image>,
    pub turn_color_icons: Vec<Handle<Image>>,
}

impl InventoryIcons {
    pub fn source_dir(&self, dir: Direction) -> Handle<Image> {
        match dir {
            Direction::North => self.source_north.clone(),
            Direction::East => self.source_east.clone(),
            Direction::South => self.source_south.clone(),
            Direction::West => self.source_west.clone(),
        }
    }

    pub fn source_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> {
        self.source_color_icons[ci * 4 + dir.index()].clone()
    }

    pub fn turn_dir(&self, dir: Direction) -> Handle<Image> {
        match dir {
            Direction::North => self.turn_north.clone(),
            Direction::East => self.turn_east.clone(),
            Direction::South => self.turn_south.clone(),
            Direction::West => self.turn_west.clone(),
        }
    }

    pub fn turn_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> {
        self.turn_color_icons[ci * 4 + dir.index()].clone()
    }
}

#[derive(Resource, Clone)]
#[allow(dead_code)]
pub struct GameAssets {
    pub floor_mesh: Handle<Mesh>,
    pub floor_material: Handle<StandardMaterial>,
    pub empty_mesh: Handle<Mesh>,
    pub empty_material: Handle<StandardMaterial>,
    pub ghost_floor_material: Handle<StandardMaterial>,
    pub ghost_delete_mesh: Handle<Mesh>,
    pub ghost_delete_material: Handle<StandardMaterial>,
    pub highlight_mesh: Handle<Mesh>,
    pub highlight_material: Handle<StandardMaterial>,
    pub source_symbol_mesh: Handle<Mesh>,
    pub source_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub turn_symbol_mesh: Handle<Mesh>,
    pub turn_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_turn_materials: Vec<Handle<StandardMaterial>>,
    pub bot_mesh: Handle<Mesh>,
    pub eye_mesh: Handle<Mesh>,
    pub bot_materials: Vec<Handle<StandardMaterial>>,
    pub eye_material: Handle<StandardMaterial>,
}

// === Components ===
#[derive(Component)]
pub struct Tile;

#[derive(Component, Clone, Copy)]
pub struct TileCoord {
    pub col: u32,
    pub row: u32,
}

#[derive(Component, PartialEq, Clone, Copy)]
pub enum TileKind {
    Empty,
    Floor,
    Source(usize, Direction),
    Turn(usize, Direction),
}

#[derive(Component)]
pub struct TargetScale(pub Vec3);

#[derive(Component)]
pub struct DespawnAtZeroScale;

#[derive(Component)]
pub struct GhostPreview;

#[derive(Component)]
pub struct TileHighlight;

#[derive(Component)]
pub struct BoardSizeText;

#[derive(Component)]
pub enum BoardButton {
    Increase,
    Decrease,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum InventorySlot {
    Floor,
    Source,
    Turn,
    Delete,
    SourceDir(Direction),
    SourceColor(usize),
    TurnDir(Direction),
    TurnColor(usize),
}

#[derive(Component)]
pub struct InventoryContainer;

#[derive(Component)]
pub struct NodeWidthAnim {
    pub target: f32,
    pub despawn_at_zero: bool,
}

#[derive(Component)]
pub struct Level2Slot;

#[derive(Component)]
pub struct Level3Slot;

#[derive(Component)]
pub struct ExpansionContainer;

#[derive(Component)]
pub struct Bot;

#[derive(Component)]
pub struct PlayStopButton;

#[derive(Component)]
pub struct PlayButtonImage;

#[derive(Component)]
pub struct GhostSymbolOverlay;
