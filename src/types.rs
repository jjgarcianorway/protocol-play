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
    Goal,
    Turn,
    TurnBut,
    Teleport,
    Bounce,
    BounceBut,
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

    pub fn grid_delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    /// Returns exit direction for a bot with travel direction `self` entering a Turn with `turn_dir`.
    pub fn turn_exit(self, turn_dir: Direction) -> Option<Direction> {
        // The L-shape arms at each turn orientation
        let (arm1, arm2) = match turn_dir {
            Direction::North => (Direction::East, Direction::North),
            Direction::East => (Direction::South, Direction::East),
            Direction::South => (Direction::West, Direction::South),
            Direction::West => (Direction::North, Direction::West),
        };
        let entry_side = self.opposite();
        if entry_side == arm1 { Some(arm2) }
        else if entry_side == arm2 { Some(arm1) }
        else { None }
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
    pub last_placed_color: Option<usize>,
}

#[derive(Resource, Default)]
pub struct PlacedSources(pub HashSet<usize>);

#[derive(Resource, Default)]
pub struct PlacedGoals(pub HashSet<usize>);

#[derive(Resource, Default)]
pub struct PlacedTeleports(pub [u8; 10]);

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
    pub goal: Handle<Image>,
    pub turn: Handle<Image>,
    pub delete: Handle<Image>,
    pub source_north: Handle<Image>,
    pub source_east: Handle<Image>,
    pub source_south: Handle<Image>,
    pub source_west: Handle<Image>,
    pub source_color_icons: Vec<Handle<Image>>,
    pub goal_color_icons: Vec<Handle<Image>>,
    pub turn_north: Handle<Image>,
    pub turn_east: Handle<Image>,
    pub turn_south: Handle<Image>,
    pub turn_west: Handle<Image>,
    pub turn_color_icons: Vec<Handle<Image>>,
    pub turnbut: Handle<Image>,
    pub turnbut_dir_icons: [Handle<Image>; 4],
    pub turnbut_color_icons: Vec<Handle<Image>>,
    pub teleport: Handle<Image>,
    pub teleport_num_icons: Vec<Handle<Image>>,
    pub bounce: Handle<Image>,
    pub bounce_color_icons: Vec<Handle<Image>>,
    pub bouncebot: Handle<Image>,
    pub bouncebot_color_icons: Vec<Handle<Image>>,
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

    pub fn goal_color(&self, ci: usize) -> Handle<Image> {
        self.goal_color_icons[ci].clone()
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

    pub fn turnbut_dir(&self, dir: Direction) -> Handle<Image> {
        self.turnbut_dir_icons[dir.index()].clone()
    }

    pub fn turnbut_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> {
        self.turnbut_color_icons[ci * 4 + dir.index()].clone()
    }

    pub fn teleport_num(&self, num: usize) -> Handle<Image> {
        self.teleport_num_icons[num].clone()
    }

    pub fn bounce_color(&self, ci: usize) -> Handle<Image> {
        self.bounce_color_icons[ci].clone()
    }

    pub fn bouncebot_color(&self, ci: usize) -> Handle<Image> {
        self.bouncebot_color_icons[ci].clone()
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
    pub goal_symbol_mesh: Handle<Mesh>,
    pub goal_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_goal_materials: Vec<Handle<StandardMaterial>>,
    pub turn_symbol_mesh: Handle<Mesh>,
    pub turn_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_turn_materials: Vec<Handle<StandardMaterial>>,
    pub turnbut_symbol_mesh: Handle<Mesh>,
    pub turnbut_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_turnbut_materials: Vec<Handle<StandardMaterial>>,
    pub teleport_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_teleport_materials: Vec<Handle<StandardMaterial>>,
    pub bounce_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_bounce_materials: Vec<Handle<StandardMaterial>>,
    pub bouncebot_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_bouncebot_materials: Vec<Handle<StandardMaterial>>,
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
    Goal(usize),
    Turn(usize, Direction),
    TurnBut(usize, Direction),
    Teleport(usize),
    Bounce(usize),
    BounceBut(usize),
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
    Goal,
    Turn,
    TurnBut,
    Delete,
    SourceDir(Direction),
    SourceColor(usize),
    GoalColor(usize),
    TurnDir(Direction),
    TurnColor(usize),
    TurnButDir(Direction),
    TurnButColor(usize),
    Teleport,
    TeleportNum(usize),
    Bounce,
    BounceBut,
    BounceColor(usize),
    BounceButColor(usize),
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

#[derive(Clone, Copy, PartialEq)]
pub enum BotPhase {
    Accelerating,
    Cruising,
    Decelerating(Option<Direction>), // Some(exit_dir) = turn, None = goal
    Rotating { entry_dir: Direction, exit_dir: Direction, progress: f32 },
    Spinning, // at goal center, spinning forever
    TeleportShrink { target_col: i32, target_row: i32 },
    TeleportGrow,
    Stopped,
}

#[derive(Component)]
pub struct BotMovement {
    pub direction: Direction,
    pub color_index: usize,
    pub col: i32,
    pub row: i32,
    pub progress: f32,  // 0.0 = entry edge, 0.5 = center, 1.0 = exit edge
    pub speed: f32,
    pub phase: BotPhase,
}

#[derive(Resource)]
pub struct PlayTimer(pub Timer);
