// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::f32::consts::{FRAC_PI_2, PI};

// === Enums ===
#[derive(Default, PartialEq, Clone, Copy)]
pub enum Tool {
    #[default] Floor, Source, Goal, Turn, TurnBut, Teleport, TeleportBut, Bounce, BounceBut,
    Door, Switch, ColorSwitch, ColorSwitchBut, Painter, Arrow, ArrowBut, Delete,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
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
pub struct GhostCell {
    pub current: Option<(u32, u32)>,
    pub last_placed: Option<(u32, u32)>,
}

#[derive(Resource, Default, Clone)]
pub struct InventoryState {
    pub level: u8,
    pub direction: Option<Direction>,
    pub color_index: Option<usize>,
    pub last_placed_color: Option<usize>,
}

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub enum PlayMode {
    #[default]
    Editing,
    Marking,
    Playing,
    TestEditing,
    TestPlaying,
}

#[derive(Resource)]
pub struct PlayIcons {
    pub play: Handle<Image>,
    pub stop: Handle<Image>,
}

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

#[derive(Resource)]
#[allow(dead_code)]
pub struct InventoryIcons {
    pub floor: Handle<Image>,
    pub source: Handle<Image>,
    pub goal: Handle<Image>,
    pub turn: Handle<Image>,
    pub delete: Handle<Image>,
    pub source_dir_icons: [Handle<Image>; 4],
    pub source_color_icons: Vec<Handle<Image>>,
    pub goal_color_icons: Vec<Handle<Image>>,
    pub turn_dir_icons: [Handle<Image>; 4],
    pub turn_color_icons: Vec<Handle<Image>>,
    pub turnbut: Handle<Image>,
    pub turnbut_dir_icons: [Handle<Image>; 4],
    pub turnbut_color_icons: Vec<Handle<Image>>,
    pub teleport: Handle<Image>,
    pub teleport_color_icons: Vec<Handle<Image>>,
    pub teleportbut: Handle<Image>,
    pub teleportbut_color_icons: Vec<Handle<Image>>,
    pub bounce: Handle<Image>,
    pub bounce_color_icons: Vec<Handle<Image>>,
    pub bouncebot: Handle<Image>,
    pub bouncebot_color_icons: Vec<Handle<Image>>,
    pub door: Handle<Image>, pub door_open: Handle<Image>, pub door_closed: Handle<Image>,
    pub switch: Handle<Image>,
    pub switch_color_icons: Vec<Handle<Image>>,
    pub switchbut: Handle<Image>, pub switchbut_color_icons: Vec<Handle<Image>>,
    pub painter: Handle<Image>, pub painter_color_icons: Vec<Handle<Image>>,
    pub arrow: Handle<Image>, pub arrow_dir_icons: [Handle<Image>; 4],
    pub arrow_color_icons: Vec<Handle<Image>>,
    pub arrowbut: Handle<Image>, pub arrowbut_dir_icons: [Handle<Image>; 4],
    pub arrowbut_color_icons: Vec<Handle<Image>>,
}

impl InventoryIcons {
    pub fn source_dir(&self, dir: Direction) -> Handle<Image> { self.source_dir_icons[dir.index()].clone() }
    pub fn source_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> { self.source_color_icons[ci * 4 + dir.index()].clone() }
    pub fn goal_color(&self, ci: usize) -> Handle<Image> { self.goal_color_icons[ci].clone() }
    pub fn turn_dir(&self, dir: Direction) -> Handle<Image> { self.turn_dir_icons[dir.index()].clone() }
    pub fn turn_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> { self.turn_color_icons[ci * 4 + dir.index()].clone() }
    pub fn turnbut_dir(&self, dir: Direction) -> Handle<Image> { self.turnbut_dir_icons[dir.index()].clone() }
    pub fn turnbut_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> { self.turnbut_color_icons[ci * 4 + dir.index()].clone() }
    pub fn teleport_color(&self, ci: usize) -> Handle<Image> { self.teleport_color_icons[ci].clone() }
    pub fn teleportbut_color(&self, ci: usize) -> Handle<Image> { self.teleportbut_color_icons[ci].clone() }
    pub fn bounce_color(&self, ci: usize) -> Handle<Image> { self.bounce_color_icons[ci].clone() }
    pub fn bouncebot_color(&self, ci: usize) -> Handle<Image> { self.bouncebot_color_icons[ci].clone() }
    pub fn switch_color(&self, ci: usize) -> Handle<Image> { self.switch_color_icons[ci].clone() }
    pub fn switchbut_color(&self, ci: usize) -> Handle<Image> { self.switchbut_color_icons[ci].clone() }
    pub fn painter_color(&self, ci: usize) -> Handle<Image> { self.painter_color_icons[ci].clone() }
    pub fn arrow_dir(&self, dir: Direction) -> Handle<Image> { self.arrow_dir_icons[dir.index()].clone() }
    pub fn arrow_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> { self.arrow_color_icons[ci * 4 + dir.index()].clone() }
    pub fn arrowbut_dir(&self, dir: Direction) -> Handle<Image> { self.arrowbut_dir_icons[dir.index()].clone() }
    pub fn arrowbut_color_dir(&self, ci: usize, dir: Direction) -> Handle<Image> { self.arrowbut_color_icons[ci * 4 + dir.index()].clone() }
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
    pub teleport_symbol_materials: Vec<Handle<StandardMaterial>>,  // [num * NUM_TELEPORT_COLORS + color]
    pub ghost_teleport_materials: Vec<Handle<StandardMaterial>>,
    pub teleportbut_symbol_materials: Vec<Handle<StandardMaterial>>, // [num * NUM_COLORS + color]
    pub ghost_teleportbut_materials: Vec<Handle<StandardMaterial>>,
    pub bounce_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_bounce_materials: Vec<Handle<StandardMaterial>>,
    pub bouncebot_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_bouncebot_materials: Vec<Handle<StandardMaterial>>,
    pub door_open_material: Handle<StandardMaterial>,
    pub door_closed_material: Handle<StandardMaterial>,
    pub ghost_door_open_material: Handle<StandardMaterial>,
    pub ghost_door_closed_material: Handle<StandardMaterial>,
    pub switch_material: Handle<StandardMaterial>,
    pub ghost_switch_material: Handle<StandardMaterial>,
    pub colorswitch_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_colorswitch_materials: Vec<Handle<StandardMaterial>>,
    pub colorswitchbut_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_colorswitchbut_materials: Vec<Handle<StandardMaterial>>,
    pub painter_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_painter_materials: Vec<Handle<StandardMaterial>>,
    pub arrow_symbol_mesh: Handle<Mesh>,
    pub arrow_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_arrow_materials: Vec<Handle<StandardMaterial>>,
    pub arrowbut_symbol_mesh: Handle<Mesh>,
    pub arrowbut_symbol_materials: Vec<Handle<StandardMaterial>>,
    pub ghost_arrowbut_materials: Vec<Handle<StandardMaterial>>,
    pub marker_mesh: Handle<Mesh>,
    pub marker_material: Handle<StandardMaterial>,
    pub bot_mesh: Handle<Mesh>,
    pub eye_mesh: Handle<Mesh>,
    pub bot_materials: Vec<Handle<StandardMaterial>>,
    pub eye_material: Handle<StandardMaterial>,
    pub flash_material: Handle<StandardMaterial>,
}

// === Components ===
#[derive(Component)]
pub struct Tile;

#[derive(Component, Clone, Copy)]
pub struct TileCoord {
    pub col: u32,
    pub row: u32,
}

#[derive(Component, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum TileKind {
    Empty,
    Floor,
    Source(usize, Direction),
    Goal(usize),
    Turn(usize, Direction),
    TurnBut(usize, Direction),
    Teleport(usize, usize),
    TeleportBut(usize, usize),
    Bounce(usize),
    BounceBut(usize),
    Door(bool),  // true = open, false = closed
    Switch,
    ColorSwitch(usize),
    ColorSwitchBut(usize),
    Painter(usize),
    Arrow(usize, Direction),
    ArrowBut(usize, Direction),
}

#[derive(Component)] pub struct TargetScale(pub Vec3);
#[derive(Component)] pub struct DespawnAtZeroScale;
#[derive(Component)] pub struct GhostPreview;
#[derive(Component)] pub struct GhostTrail;
#[derive(Component)] pub struct TileHighlight;
#[derive(Component)] pub struct BoardSizeText;

#[derive(Component)]
pub enum BoardButton {
    Increase,
    Decrease,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum InventorySlot {
    Floor, Source, Goal, Turn, TurnBut, Delete,
    SourceDir(Direction), SourceColor(usize),
    GoalColor(usize),
    TurnDir(Direction), TurnColor(usize),
    TurnButDir(Direction), TurnButColor(usize),
    Teleport, TeleportBut, TeleportColor(usize), TeleportButColor(usize),
    Bounce, BounceBut, BounceColor(usize), BounceButColor(usize),
    Door, Switch, SwitchBut, SwitchColor(usize), SwitchButColor(usize),
    Painter, PainterColor(usize),
    DoorState(bool),
    Arrow, ArrowBut, ArrowDir(Direction), ArrowButDir(Direction),
    ArrowColor(usize), ArrowButColor(usize),
}

#[derive(Component)] pub struct InventoryContainer;
#[derive(Component)] pub struct NodeWidthAnim { pub target: f32, pub despawn_at_zero: bool }

#[derive(Component)] pub struct Level2Slot;
#[derive(Component)] pub struct Level3Slot;
#[derive(Component)] pub struct L2L3Divider;
#[derive(Component)] pub struct ExpansionContainer;
#[derive(Component)] pub struct ExpHeightAnim { pub target: f32 }
#[derive(Component)] pub struct Bot;
#[derive(Component)] pub struct PlayStopButton;
#[derive(Component)] pub struct PlayButtonImage;
#[derive(Component)] pub struct GhostSymbolOverlay;
#[derive(Component)] pub struct StatusBarText;

// === Test mode types ===
#[derive(Component)] pub struct InventoryMarker;
#[derive(Component)] pub struct InventoryMarkerVisual;
#[derive(Component)] pub struct MarkButton;
#[derive(Component)] pub struct MarkButtonImage;
#[derive(Component)] pub struct TestButton;
#[derive(Component)] pub struct StopTestButton;
#[derive(Component)] pub struct ResetTestButton;
#[derive(Component)] pub struct TestInventorySlot(pub usize);
#[derive(Component)] pub struct TestInventoryContainer;
#[derive(Component)] pub struct TestTopButtons;
#[derive(Component)] pub struct TopControlsBar;

#[derive(Resource, Default)]
pub struct SavedBoardState {
    pub tiles: Vec<(u32, u32, TileKind, bool)>,
    pub inv_state: InventoryState,
    pub selected_tool: Tool,
}

#[derive(Resource, Default)]
pub struct SavedTestState {
    pub tiles: Vec<(u32, u32, TileKind)>,
    pub inventory: Vec<(TileKind, u8)>,
}

#[derive(Resource, Default)]
pub struct TestInventory {
    pub items: Vec<(TileKind, u8)>,
    pub selected: Option<usize>,
    pub remove_mode: bool,
}

#[derive(Component)] pub struct TestModeBanner;
#[derive(Component)] pub struct BorderFade { pub target: [f32; 4], pub speed: f32 }
#[derive(Component)] pub struct UiBottomAnim { pub target: f32, pub despawn_at_target: bool }
#[derive(Component)] pub struct UiTopAnim { pub target: f32, pub despawn_at_target: bool }
#[derive(Component)] pub struct UiBgFade { pub target: f32, pub despawn_at_zero: bool }
#[derive(Serialize, Deserialize)] pub struct LevelData {
    pub name: String, pub board_size: u32, pub tiles: Vec<(u32, u32, TileKind, bool)>,
    #[serde(default)] pub solution: Vec<(u32, u32, TileKind)>,
}
#[derive(Component)] pub struct SaveButton;
#[derive(Component)] pub struct LoadButton;
#[derive(Component)] pub struct SaveDialog;
#[derive(Component)] pub struct SaveDialogInput;
#[derive(Component)] pub struct SaveDialogConfirm;
#[derive(Component)] pub struct SaveDialogCancel;
#[derive(Component)] pub struct LoadDialog;
#[derive(Component)] pub struct LoadDialogCancel;
#[derive(Component)] pub struct LoadDialogEntry(pub String);
#[derive(Component)] pub struct ValidationErrorDialog;
#[derive(Component)] pub struct ValidationErrorOk;
#[derive(Resource, Default)] pub struct LevelValidated(pub bool);
#[derive(Component)] pub struct BotColorTransition {
    pub from_color: usize, pub to_color: usize, pub progress: f32,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct BotFormation {
    pub offset: Vec2, pub target_offset: Vec2,
    pub visual_scale: f32, pub target_scale: f32,
}
impl Default for BotFormation {
    fn default() -> Self {
        Self { offset: Vec2::ZERO, target_offset: Vec2::ZERO, visual_scale: 1.0, target_scale: 1.0 }
    }
}

#[derive(Component)]
pub struct MergeFlash { pub progress: f32 }
#[derive(Component)] pub struct SaveDialogCursor;
#[derive(Component)] pub struct OverwriteDialog;
#[derive(Component)] pub struct OverwriteConfirm;
#[derive(Component)] pub struct OverwriteCancel;
#[derive(Component)] pub struct LoadDialogList;
#[derive(Component)] pub struct ScrollbarThumb;
#[derive(Resource, Default)] pub struct CursorBlinkTimer(pub f32);
#[derive(Resource, Default)] pub struct LoadedLevelName(pub Option<String>);
#[derive(Resource, Default)] pub struct PendingSave(pub Option<(String, LevelData)>);
#[derive(Resource, Default)] pub struct ScrollbarDrag(pub Option<f32>);
#[derive(Component)] pub struct DeleteLevelButton(pub String);
#[derive(Component)] pub struct DeleteLevelDialog;
#[derive(Component)] pub struct DeleteLevelConfirm(pub String);
#[derive(Component)] pub struct DeleteLevelCancel;
