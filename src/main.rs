// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashSet;
use std::f32::consts::{FRAC_PI_2, PI};

// === Constants ===
const CAMERA_ELEVATION: f32 = 30.0;
const CAMERA_AZIMUTH: f32 = 45.0;
const CAMERA_MARGIN: f32 = 1.05;
const MIN_BOARD_SIZE: u32 = 1;
const MAX_BOARD_SIZE: u32 = 12;
const FLOOR_TOP_Y: f32 = 0.125;
const EMPTY_CENTER_Y: f32 = 0.375;
const ANIM_SPEED: f32 = 10.0;
const UI_ANIM_SPEED: f32 = 12.0;
const SLOT_WIDTH: f32 = 64.0;
const SLOT_HEIGHT: f32 = 80.0;
const NUM_COLORS: usize = 10;

const SOURCE_COLORS: [(f32, f32, f32); NUM_COLORS] = [
    (1.0, 0.2, 0.2),   // Red
    (0.2, 0.6, 1.0),   // Blue
    (0.2, 0.9, 0.2),   // Green
    (1.0, 0.85, 0.0),  // Yellow
    (0.8, 0.3, 0.9),   // Purple
    (1.0, 0.5, 0.0),   // Orange
    (0.0, 0.9, 0.9),   // Cyan
    (1.0, 0.4, 0.6),   // Pink
    (1.0, 1.0, 1.0),   // White
    (0.6, 0.4, 0.2),   // Brown
];

// === Enums ===
#[derive(Default, PartialEq, Clone, Copy)]
enum Tool {
    #[default]
    Floor,
    Source,
    Delete,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotation(self) -> f32 {
        match self {
            Direction::North => 0.0,
            Direction::East => FRAC_PI_2,
            Direction::South => PI,
            Direction::West => 3.0 * FRAC_PI_2,
        }
    }

    fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    fn index(self) -> usize {
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
struct BoardSize(u32);

#[derive(Resource, Default)]
struct SelectedTool(Tool);

#[derive(Resource, Default)]
struct HoveredCell(Option<(u32, u32)>);

#[derive(Resource, Default)]
struct HiddenTileEntity(Option<Entity>);

#[derive(Resource, Default)]
struct InventoryState {
    level: u8,
    direction: Option<Direction>,
    color_index: Option<usize>,
}

#[derive(Resource, Default)]
struct PlacedSources(HashSet<usize>);

#[derive(Resource)]
struct InventoryIcons {
    floor: Handle<Image>,
    source: Handle<Image>,
    delete: Handle<Image>,
    source_north: Handle<Image>,
    source_east: Handle<Image>,
    source_south: Handle<Image>,
    source_west: Handle<Image>,
    source_color_icons: Vec<Handle<Image>>, // NUM_COLORS * 4 directions
}

impl InventoryIcons {
    fn source_dir(&self, dir: Direction) -> Handle<Image> {
        match dir {
            Direction::North => self.source_north.clone(),
            Direction::East => self.source_east.clone(),
            Direction::South => self.source_south.clone(),
            Direction::West => self.source_west.clone(),
        }
    }

    fn source_color_dir(&self, color_index: usize, dir: Direction) -> Handle<Image> {
        self.source_color_icons[color_index * 4 + dir.index()].clone()
    }
}

#[derive(Resource, Clone)]
struct GameAssets {
    floor_mesh: Handle<Mesh>,
    floor_material: Handle<StandardMaterial>,
    empty_mesh: Handle<Mesh>,
    empty_material: Handle<StandardMaterial>,
    ghost_floor_material: Handle<StandardMaterial>,
    ghost_delete_mesh: Handle<Mesh>,
    ghost_delete_material: Handle<StandardMaterial>,
    highlight_mesh: Handle<Mesh>,
    highlight_material: Handle<StandardMaterial>,
    source_materials: Vec<Handle<StandardMaterial>>,
    ghost_source_materials: Vec<Handle<StandardMaterial>>,
}

// === Components ===
#[derive(Component)]
struct Tile;

#[derive(Component, Clone, Copy)]
struct TileCoord { col: u32, row: u32 }

#[derive(Component, PartialEq, Clone, Copy)]
enum TileKind {
    Empty,
    Floor,
    Source(usize, Direction),
}

#[derive(Component)]
struct TargetScale(Vec3);

#[derive(Component)]
struct DespawnAtZeroScale;

#[derive(Component)]
struct GhostPreview;

#[derive(Component)]
struct TileHighlight;

#[derive(Component)]
struct BoardSizeText;

#[derive(Component)]
enum BoardButton { Increase, Decrease }

#[derive(Component, Clone, Copy, PartialEq)]
enum InventorySlot {
    Floor,
    Source,
    Delete,
    SourceDir(Direction),
    SourceColor(usize),
}

#[derive(Component)]
struct InventoryContainer;

#[derive(Component)]
struct NodeWidthAnim {
    target: f32,
    despawn_at_zero: bool,
}

#[derive(Component)]
struct Level3Slot;

// === Main ===
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "protocol: play".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(BoardSize(1))
        .insert_resource(SelectedTool::default())
        .insert_resource(HoveredCell::default())
        .insert_resource(HiddenTileEntity::default())
        .insert_resource(InventoryState { level: 1, direction: None, color_index: None })
        .insert_resource(PlacedSources::default())
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, (
            animate_scale,
            animate_node_width,
            cleanup_despawned.after(animate_scale),
            button_interaction,
            inventory_interaction,
            update_inventory_visuals.after(inventory_interaction),
            update_l3_availability.after(inventory_interaction),
            update_hovered_cell,
            update_ghost_and_highlight.after(update_hovered_cell),
            handle_tile_click.after(update_hovered_cell),
            adapt_camera,
        ))
        .run();
}

// === Texture Creation ===
fn create_tile_texture(images: &mut Assets<Image>, size: u32, border: u32) -> Handle<Image> {
    let gray: [u8; 4] = [180, 180, 180, 255];
    let dark: [u8; 4] = [60, 60, 60, 255];
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            let on_edge = x < border || x >= size - border || y < border || y >= size - border;
            let color = if on_edge { dark } else { gray };
            let i = ((y * size + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn create_delete_icon(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 128;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let fx = x as f32 / SIZE as f32;
            let fy = y as f32 / SIZE as f32;
            let on_diag1 = (fx - fy).abs() < 0.09;
            let on_diag2 = (fx - (1.0 - fy)).abs() < 0.09;
            let in_margin = fx > 0.15 && fx < 0.85 && fy > 0.15 && fy < 0.85;
            let color: [u8; 4] = if (on_diag1 || on_diag2) && in_margin {
                [220, 60, 60, 255]
            } else {
                [40, 40, 40, 255]
            };
            let i = ((y * SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    images.add(Image::new(
        Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn create_highlight_texture(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 64;
    const BORDER: u32 = 4;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let on_edge = x < BORDER || x >= SIZE - BORDER || y < BORDER || y >= SIZE - BORDER;
            let color: [u8; 4] = if on_edge { [255, 255, 255, 200] } else { [0, 0, 0, 0] };
            let i = ((y * SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    images.add(Image::new(
        Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn in_source_shape(x: f32, y: f32, expand: f32) -> bool {
    let dist = (x * x + y * y).sqrt();
    if dist < 0.35 + expand { return true; }
    if x.abs() < 0.05 + expand && y < -(0.35 - expand) && y > -(0.62 + expand) {
        return true;
    }
    let tip_y = -0.82;
    let base_y = -0.58;
    if y > tip_y - expand && y < base_y + expand {
        let t = ((y - tip_y) / (base_y - tip_y)).clamp(0.0, 1.0);
        if x.abs() < 0.14 * t + expand { return true; }
    }
    false
}

fn create_source_texture(images: &mut Assets<Image>, size: u32, border: u32, rotation: f32) -> Handle<Image> {
    create_source_texture_colored(images, size, border, rotation, [240, 240, 240, 255])
}

fn create_source_texture_colored(
    images: &mut Assets<Image>, size: u32, border: u32, rotation: f32, fill: [u8; 4],
) -> Handle<Image> {
    let base_gray: [u8; 4] = [180, 180, 180, 255];
    let dark: [u8; 4] = [60, 60, 60, 255];
    let stroke: [u8; 4] = [80, 80, 80, 255];
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let stroke_expand = 0.025;
    let cos_r = rotation.cos();
    let sin_r = rotation.sin();

    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let on_edge = px < border || px >= size - border || py < border || py >= size - border;
            let mut color = if on_edge { dark } else { base_gray };
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let rnx = nx * cos_r + ny * sin_r;
            let rny = -nx * sin_r + ny * cos_r;
            if in_source_shape(rnx, rny, 0.0) {
                color = fill;
            } else if in_source_shape(rnx, rny, stroke_expand) {
                color = stroke;
            }
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn create_source_emission_mask(images: &mut Assets<Image>, size: u32, rotation: f32) -> Handle<Image> {
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let cos_r = rotation.cos();
    let sin_r = rotation.sin();
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let rnx = nx * cos_r + ny * sin_r;
            let rny = -nx * sin_r + ny * cos_r;
            let color: [u8; 4] = if in_source_shape(rnx, rny, 0.0) {
                [255, 255, 255, 255]
            } else {
                [0, 0, 0, 255]
            };
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

// === Helpers ===
fn camera_direction() -> Vec3 {
    let elev = CAMERA_ELEVATION.to_radians();
    let azim = CAMERA_AZIMUTH.to_radians();
    Vec3::new(elev.cos() * azim.sin(), elev.sin(), elev.cos() * azim.cos())
}

fn board_bounding_radius(size: u32) -> f32 {
    let half = size as f32 / 2.0;
    (half * half + 0.35 * 0.35 + half * half).sqrt()
}

fn tile_world_pos(col: u32, row: u32, board_size: u32, kind: &TileKind) -> Vec3 {
    let offset = (board_size as f32 - 1.0) / 2.0;
    let y = match kind {
        TileKind::Empty => EMPTY_CENTER_Y,
        TileKind::Floor | TileKind::Source(_, _) => 0.0,
    };
    Vec3::new(col as f32 - offset, y, row as f32 - offset)
}

fn color_to_u8(r: f32, g: f32, b: f32) -> [u8; 4] {
    [
        (r * 255.0).clamp(0.0, 255.0) as u8,
        (g * 255.0).clamp(0.0, 255.0) as u8,
        (b * 255.0).clamp(0.0, 255.0) as u8,
        255,
    ]
}

// === Spawning ===
fn spawn_tile(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets,
) {
    let pos = tile_world_pos(col, row, board_size, &kind);
    let (mesh, material, rotation) = match kind {
        TileKind::Empty => (assets.empty_mesh.clone(), assets.empty_material.clone(), Quat::IDENTITY),
        TileKind::Floor => (assets.floor_mesh.clone(), assets.floor_material.clone(), Quat::IDENTITY),
        TileKind::Source(ci, dir) => (
            assets.floor_mesh.clone(),
            assets.source_materials[ci].clone(),
            Quat::from_rotation_y(dir.rotation()),
        ),
    };
    commands.spawn((
        Mesh3d(mesh), MeshMaterial3d(material),
        Transform::from_translation(pos)
            .with_rotation(rotation)
            .with_scale(Vec3::ZERO),
        TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
    ));
}

fn spawn_board(commands: &mut Commands, size: u32, assets: &GameAssets) {
    for row in 0..size {
        for col in 0..size {
            spawn_tile(commands, col, row, size, TileKind::Empty, assets);
        }
    }
}

// === UI Slot helpers ===
fn slot_node() -> Node {
    Node {
        width: Val::Px(SLOT_WIDTH),
        height: Val::Px(SLOT_HEIGHT),
        border: UiRect::all(Val::Px(3.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        ..default()
    }
}

fn spawn_slot(
    commands: &mut Commands, parent: Entity,
    slot: InventorySlot, icon: Handle<Image>,
    selected: bool, animated: bool,
) -> Entity {
    let slot_bg = Color::srgb(0.15, 0.15, 0.15);
    let border = if selected {
        Color::srgba(1.0, 1.0, 0.0, 0.8)
    } else {
        Color::srgba(1.0, 1.0, 1.0, 0.2)
    };

    let mut node = slot_node();
    if animated {
        node.width = Val::Px(0.0);
    }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg),
        BorderColor(border), slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Px(48.0), height: Val::Px(48.0), ..default() },
            ImageNode::new(icon),
        ));
        // Spacer matching color slot count text height
        parent.spawn((
            Text::new(" "),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ));
    }).id();

    if animated {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_WIDTH, despawn_at_zero: false });
    }

    commands.entity(parent).add_child(child);
    child
}

fn spawn_color_slot(
    commands: &mut Commands, parent: Entity,
    color_index: usize, icon: Handle<Image>,
    selected: bool, animated: bool, available: bool,
) -> Entity {
    let slot_bg = Color::srgb(0.15, 0.15, 0.15);
    let border = if selected {
        Color::srgba(1.0, 1.0, 0.0, 0.8)
    } else {
        Color::srgba(1.0, 1.0, 1.0, 0.2)
    };

    let mut node = slot_node();
    if !available {
        node.width = Val::Px(0.0);
    }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg),
        BorderColor(border),
        InventorySlot::SourceColor(color_index),
        Level3Slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Px(48.0), height: Val::Px(48.0), ..default() },
            ImageNode::new(icon),
        ));
        parent.spawn((
            Text::new("1"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        ));
    }).id();

    if animated && available {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_WIDTH, despawn_at_zero: false });
    } else if !available {
        // Already at 0 width, no animation needed
    }

    commands.entity(parent).add_child(child);
    child
}

fn rebuild_inventory_l1(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
) {
    spawn_slot(commands, container, InventorySlot::Floor, icons.floor.clone(), true, true);
    spawn_slot(commands, container, InventorySlot::Source, icons.source.clone(), false, true);
    spawn_slot(commands, container, InventorySlot::Delete, icons.delete.clone(), false, true);
}

fn rebuild_inventory_l2(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    selected_dir: Option<Direction>,
) {
    spawn_slot(commands, container, InventorySlot::Source, icons.source.clone(), true, false);
    for dir in Direction::all() {
        let selected = selected_dir == Some(dir);
        spawn_slot(commands, container, InventorySlot::SourceDir(dir),
            icons.source_dir(dir), selected, true);
    }
}

fn rebuild_l3_colors(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    dir: Direction, selected_color: Option<usize>, placed: &PlacedSources,
) {
    for ci in 0..NUM_COLORS {
        let available = !placed.0.contains(&ci);
        let selected = selected_color == Some(ci);
        let icon = icons.source_color_dir(ci, dir);
        spawn_color_slot(commands, container, ci, icon, selected, true, available);
    }
}

// === Setup ===
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    board_size: Res<BoardSize>,
) {
    let floor_texture = create_tile_texture(&mut images, 1024, 12);
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture.clone()),
        ..default()
    });
    let floor_mesh = meshes.add(Cuboid::new(1.0, 0.25, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture),
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.6),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.9, 0.2, 0.2, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, 0.27, 1.02));
    let empty_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 0.2),
        ..default()
    });
    let empty_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let highlight_texture = create_highlight_texture(&mut images);
    let highlight_material = materials.add(StandardMaterial {
        base_color_texture: Some(highlight_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let highlight_mesh = meshes.add(Cuboid::new(1.05, 0.001, 1.05));

    let source_texture = create_source_texture(&mut images, 1024, 12, 0.0);
    let source_emission = create_source_emission_mask(&mut images, 1024, 0.0);

    let source_materials: Vec<Handle<StandardMaterial>> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color_texture: Some(source_texture.clone()),
            emissive_texture: Some(source_emission.clone()),
            emissive: LinearRgba::new(r * 2.0, g * 2.0, b * 2.0, 1.0),
            ..default()
        })
    }).collect();

    let ghost_source_materials: Vec<Handle<StandardMaterial>> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color_texture: Some(source_texture.clone()),
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.6),
            emissive_texture: Some(source_emission.clone()),
            emissive: LinearRgba::new(r * 1.2, g * 1.2, b * 1.2, 1.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })
    }).collect();

    let assets = GameAssets {
        floor_mesh: floor_mesh.clone(), floor_material,
        empty_mesh, empty_material,
        ghost_floor_material: ghost_floor_material.clone(),
        ghost_delete_mesh: ghost_delete_mesh.clone(),
        ghost_delete_material: ghost_delete_material.clone(),
        highlight_mesh: highlight_mesh.clone(),
        highlight_material: highlight_material.clone(),
        source_materials, ghost_source_materials,
    };

    spawn_board(&mut commands, board_size.0, &assets);
    commands.insert_resource(assets);

    commands.spawn((
        Mesh3d(floor_mesh), MeshMaterial3d(ghost_floor_material),
        Transform::from_xyz(0.0, 0.0, 0.0), Visibility::Hidden, GhostPreview,
    ));
    commands.spawn((
        Mesh3d(highlight_mesh), MeshMaterial3d(highlight_material),
        Transform::from_xyz(0.0, FLOOR_TOP_Y + 0.01, 0.0), Visibility::Hidden, TileHighlight,
    ));
    commands.spawn((
        DirectionalLight { illuminance: 10000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let floor_icon = create_tile_texture(&mut images, 128, 6);
    let source_icon = create_source_texture(&mut images, 128, 4, 0.0);
    let delete_icon = create_delete_icon(&mut images);
    let source_north = create_source_texture(&mut images, 128, 4, Direction::North.rotation());
    let source_east = create_source_texture(&mut images, 128, 4, Direction::East.rotation());
    let source_south = create_source_texture(&mut images, 128, 4, Direction::South.rotation());
    let source_west = create_source_texture(&mut images, 128, 4, Direction::West.rotation());

    // Generate color icons: 10 colors x 4 directions
    let mut source_color_icons = Vec::with_capacity(NUM_COLORS * 4);
    for ci in 0..NUM_COLORS {
        let (r, g, b) = SOURCE_COLORS[ci];
        let fill = color_to_u8(r, g, b);
        for dir in Direction::all() {
            let icon = create_source_texture_colored(&mut images, 128, 4, dir.rotation(), fill);
            source_color_icons.push(icon);
        }
    }

    let icons = InventoryIcons {
        floor: floor_icon.clone(), source: source_icon.clone(), delete: delete_icon.clone(),
        source_north, source_east, source_south, source_west,
        source_color_icons,
    };
    commands.insert_resource(icons);

    let btn_node = Node {
        width: Val::Px(40.0), height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(2.0)),
        ..default()
    };
    let btn_color = Color::srgb(0.25, 0.25, 0.25);
    let text_style = TextFont { font_size: 24.0, ..default() };

    // Board size controls
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(10.0), top: Val::Px(10.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(4.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn((Button, btn_node.clone(), BackgroundColor(btn_color), BoardButton::Decrease))
            .with_child((Text::new("-"), text_style.clone(), TextColor(Color::WHITE)));
        parent.spawn(Node {
            width: Val::Px(70.0), justify_content: JustifyContent::Center, ..default()
        }).with_child((Text::new("1x1"), text_style.clone(), TextColor(Color::WHITE), BoardSizeText));
        parent.spawn((Button, btn_node, BackgroundColor(btn_color), BoardButton::Increase))
            .with_child((Text::new("+"), text_style, TextColor(Color::WHITE)));
    });

    // Inventory bar
    let slot_bg = Color::srgb(0.15, 0.15, 0.15);
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(20.0), width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.08, 0.85)),
            InventoryContainer,
        )).with_children(|container| {
            let sn = slot_node();
            let spacer_font = TextFont { font_size: 14.0, ..default() };
            let spacer_color = TextColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
            container.spawn((
                Button, sn.clone(), BackgroundColor(slot_bg),
                BorderColor(Color::srgba(1.0, 1.0, 0.0, 0.8)),
                InventorySlot::Floor,
            )).with_children(|slot| {
                slot.spawn((
                    Node { width: Val::Px(48.0), height: Val::Px(48.0), ..default() },
                    ImageNode::new(floor_icon),
                ));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            container.spawn((
                Button, sn.clone(), BackgroundColor(slot_bg),
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                InventorySlot::Source,
            )).with_children(|slot| {
                slot.spawn((
                    Node { width: Val::Px(48.0), height: Val::Px(48.0), ..default() },
                    ImageNode::new(source_icon),
                ));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            container.spawn((
                Button, sn, BackgroundColor(slot_bg),
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                InventorySlot::Delete,
            )).with_children(|slot| {
                slot.spawn((
                    Node { width: Val::Px(48.0), height: Val::Px(48.0), ..default() },
                    ImageNode::new(delete_icon),
                ));
                slot.spawn((Text::new(" "), spacer_font, spacer_color));
            });
        });
    });
}

// === Animation ===
fn animate_scale(mut query: Query<(&mut Transform, &TargetScale)>, time: Res<Time>) {
    for (mut transform, target) in &mut query {
        transform.scale = transform.scale.lerp(target.0, ANIM_SPEED * time.delta_secs());
        if transform.scale.distance(target.0) < 0.01 {
            transform.scale = target.0;
        }
    }
}

fn animate_node_width(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Node, &NodeWidthAnim)>,
    time: Res<Time>,
) {
    for (entity, mut node, anim) in &mut query {
        let current = match node.width {
            Val::Px(w) => w,
            _ => anim.target,
        };
        let new_w = current + (anim.target - current) * UI_ANIM_SPEED * time.delta_secs();
        if (new_w - anim.target).abs() < 0.5 {
            if anim.despawn_at_zero && anim.target < 1.0 {
                commands.entity(entity).despawn_recursive();
            } else {
                node.width = Val::Px(anim.target);
                commands.entity(entity).remove::<NodeWidthAnim>();
            }
        } else {
            node.width = Val::Px(new_w);
        }
    }
}

fn cleanup_despawned(mut commands: Commands, query: Query<(Entity, &Transform), With<DespawnAtZeroScale>>) {
    for (entity, transform) in &query {
        if transform.scale.length() < 0.02 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// === Board Size Buttons ===
fn button_interaction(
    mut commands: Commands,
    mut board_size: ResMut<BoardSize>,
    interaction_query: Query<(&Interaction, &BoardButton), Changed<Interaction>>,
    tiles: Query<Entity, With<Tile>>,
    assets: Res<GameAssets>,
    mut size_text: Query<&mut Text, With<BoardSizeText>>,
    mut placed_sources: ResMut<PlacedSources>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }
        let new_size = match button {
            BoardButton::Increase => (board_size.0 + 1).min(MAX_BOARD_SIZE),
            BoardButton::Decrease => board_size.0.saturating_sub(1).max(MIN_BOARD_SIZE),
        };
        if new_size == board_size.0 { continue; }
        board_size.0 = new_size;
        placed_sources.0.clear();
        for entity in &tiles { commands.entity(entity).despawn(); }
        spawn_board(&mut commands, new_size, &assets);
        let mut text = size_text.single_mut();
        **text = format!("{}x{}", new_size, new_size);
    }
}

// === Inventory ===
fn inventory_interaction(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    slots: Query<(&Interaction, &InventorySlot), Changed<Interaction>>,
    all_slots: Query<Entity, With<InventorySlot>>,
    l3_slots: Query<Entity, With<Level3Slot>>,
    container_q: Query<Entity, With<InventoryContainer>>,
    icons: Res<InventoryIcons>,
    placed_sources: Res<PlacedSources>,
) {
    let mut clicked = None;
    for (interaction, slot) in &slots {
        if *interaction == Interaction::Pressed {
            clicked = Some(*slot);
        }
    }
    let Some(clicked) = clicked else { return };

    match clicked {
        InventorySlot::Floor => {
            selected_tool.0 = Tool::Floor;
        }
        InventorySlot::Delete => {
            selected_tool.0 = Tool::Delete;
        }
        InventorySlot::Source => {
            let container = container_q.single();
            for entity in &all_slots {
                commands.entity(entity).despawn_recursive();
            }

            if inv_state.level == 1 {
                inv_state.level = 2;
                selected_tool.0 = Tool::Source;
                rebuild_inventory_l2(&mut commands, container, &icons, inv_state.direction);
            } else {
                // From L2 or L3, go back to L1
                inv_state.level = 1;
                selected_tool.0 = Tool::Floor;
                rebuild_inventory_l1(&mut commands, container, &icons);
            }
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Source;

            if inv_state.level == 2 {
                // Transition to L3: add color slots
                inv_state.level = 3;
                let container = container_q.single();
                rebuild_l3_colors(&mut commands, container, &icons, dir, inv_state.color_index, &placed_sources);
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                // Direction changed in L3: rebuild color slots with new direction icons
                for entity in &l3_slots {
                    commands.entity(entity).despawn_recursive();
                }
                let container = container_q.single();
                rebuild_l3_colors(&mut commands, container, &icons, dir, inv_state.color_index, &placed_sources);
            }
        }
        InventorySlot::SourceColor(ci) => {
            if !placed_sources.0.contains(&ci) {
                inv_state.color_index = Some(ci);
                selected_tool.0 = Tool::Source;
            }
        }
    }
}

fn update_inventory_visuals(
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    mut slots: Query<(&Interaction, &InventorySlot, &mut BorderColor)>,
) {
    for (interaction, slot, mut border) in &mut slots {
        let selected = match slot {
            InventorySlot::Floor => selected_tool.0 == Tool::Floor && inv_state.level == 1,
            InventorySlot::Source => inv_state.level >= 2,
            InventorySlot::Delete => selected_tool.0 == Tool::Delete,
            InventorySlot::SourceDir(dir) => inv_state.direction == Some(*dir),
            InventorySlot::SourceColor(ci) => inv_state.color_index == Some(*ci),
        };
        border.0 = match (*interaction, selected) {
            (Interaction::Hovered | Interaction::Pressed, _) => Color::srgba(1.0, 1.0, 1.0, 0.8),
            (_, true) => Color::srgba(1.0, 1.0, 0.0, 0.8),
            (_, false) => Color::srgba(1.0, 1.0, 1.0, 0.2),
        };
    }
}

fn update_l3_availability(
    mut commands: Commands,
    placed: Res<PlacedSources>,
    inv_state: Res<InventoryState>,
    l3_slots: Query<(Entity, &InventorySlot, &Node), With<Level3Slot>>,
) {
    if !placed.is_changed() || inv_state.level != 3 { return; }

    for (entity, slot, node) in &l3_slots {
        if let InventorySlot::SourceColor(ci) = slot {
            let should_show = !placed.0.contains(ci);
            let target = if should_show { SLOT_WIDTH } else { 0.0 };
            let current = match node.width { Val::Px(w) => w, _ => target };
            if (current - target).abs() > 0.5 {
                commands.entity(entity).insert(NodeWidthAnim { target, despawn_at_zero: false });
            }
        }
    }
}

// === Mouse Hover ===
fn update_hovered_cell(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    board_size: Res<BoardSize>,
    mut hovered: ResMut<HoveredCell>,
    ui_interactions: Query<&Interaction, With<Button>>,
) {
    for interaction in &ui_interactions {
        if *interaction != Interaction::None {
            hovered.0 = None;
            return;
        }
    }
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    let Some(cursor) = window.cursor_position() else { hovered.0 = None; return; };
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor) else { hovered.0 = None; return; };
    let dir = ray.direction.as_vec3();
    if dir.y.abs() < 1e-6 { hovered.0 = None; return; }
    let t = -ray.origin.y / dir.y;
    if t < 0.0 { hovered.0 = None; return; }
    let hit = ray.origin + dir * t;
    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let col = (hit.x + offset + 0.5).floor() as i32;
    let row = (hit.z + offset + 0.5).floor() as i32;
    if col >= 0 && col < board_size.0 as i32 && row >= 0 && row < board_size.0 as i32 {
        hovered.0 = Some((col as u32, row as u32));
    } else {
        hovered.0 = None;
    }
}

fn update_ghost_and_highlight(
    hovered: Res<HoveredCell>,
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    mut ghost_q: Query<
        (&mut Transform, &mut Visibility, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>),
        (With<GhostPreview>, Without<TileHighlight>),
    >,
    mut highlight_q: Query<
        (&mut Transform, &mut Visibility),
        (With<TileHighlight>, Without<GhostPreview>, Without<Tile>),
    >,
    mut hidden_tile: ResMut<HiddenTileEntity>,
    mut vis_q: Query<&mut Visibility, (With<Tile>, Without<GhostPreview>, Without<TileHighlight>)>,
) {
    if let Some(entity) = hidden_tile.0.take() {
        if let Ok(mut vis) = vis_q.get_mut(entity) {
            *vis = Visibility::Inherited;
        }
    }
    let (mut ghost_tf, mut ghost_vis, mut ghost_mesh, mut ghost_mat) = ghost_q.single_mut();
    let (mut hl_tf, mut hl_vis) = highlight_q.single_mut();
    let Some((col, row)) = hovered.0 else {
        *ghost_vis = Visibility::Hidden;
        *hl_vis = Visibility::Hidden;
        return;
    };
    let tile_info = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile_info else {
        *ghost_vis = Visibility::Hidden;
        *hl_vis = Visibility::Hidden;
        return;
    };
    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let world_x = col as f32 - offset;
    let world_z = row as f32 - offset;

    let hl_y = match (selected_tool.0, kind) {
        (Tool::Floor, TileKind::Empty) => FLOOR_TOP_Y + 0.01,
        (Tool::Source, TileKind::Empty) => FLOOR_TOP_Y + 0.01,
        (Tool::Delete, TileKind::Floor) => FLOOR_TOP_Y + 0.01,
        (Tool::Delete, TileKind::Source(_, _)) => FLOOR_TOP_Y + 0.01,
        (_, TileKind::Empty) => EMPTY_CENTER_Y + 0.26,
        (_, _) => FLOOR_TOP_Y + 0.01,
    };
    hl_tf.translation = Vec3::new(world_x, hl_y, world_z);
    *hl_vis = Visibility::Inherited;

    match (selected_tool.0, kind) {
        (Tool::Floor, TileKind::Empty) => {
            ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
            ghost_tf.scale = Vec3::ONE;
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
            *ghost_vis = Visibility::Inherited;
            if let Ok(mut vis) = vis_q.get_mut(entity) {
                *vis = Visibility::Hidden;
                hidden_tile.0 = Some(entity);
            }
        }
        (Tool::Source, TileKind::Empty) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
                ghost_tf.scale = Vec3::ONE;
                ghost_tf.rotation = Quat::from_rotation_y(dir.rotation());
                *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
                *ghost_mat = MeshMaterial3d(assets.ghost_source_materials[ci].clone());
                *ghost_vis = Visibility::Inherited;
                if let Ok(mut vis) = vis_q.get_mut(entity) {
                    *vis = Visibility::Hidden;
                    hidden_tile.0 = Some(entity);
                }
            } else {
                *ghost_vis = Visibility::Hidden;
            }
        }
        (Tool::Delete, TileKind::Floor) | (Tool::Delete, TileKind::Source(_, _)) => {
            ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
            ghost_tf.scale = Vec3::ONE;
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.ghost_delete_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_delete_material.clone());
            *ghost_vis = Visibility::Inherited;
        }
        _ => { *ghost_vis = Visibility::Hidden; }
    }
}

// === Tile Placement ===
fn handle_tile_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
    mut placed_sources: ResMut<PlacedSources>,
) {
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else { return; };
    let tile = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile else { return; };
    match (selected_tool.0, *kind) {
        (Tool::Floor, TileKind::Empty) => {
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Floor, &assets);
        }
        (Tool::Source, TileKind::Empty) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    placed_sources.0.insert(ci);
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                    spawn_tile(&mut commands, col, row, board_size.0, TileKind::Source(ci, dir), &assets);
                }
            }
        }
        (Tool::Delete, TileKind::Floor) => {
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
        (Tool::Delete, TileKind::Source(ci, _)) => {
            placed_sources.0.remove(&ci);
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
        _ => {}
    }
}

// === Camera ===
fn adapt_camera(
    windows: Query<&Window>,
    mut cameras: Query<(&mut Transform, &Projection), With<Camera3d>>,
    board_size: Res<BoardSize>,
) {
    let window = windows.single();
    let (mut transform, projection) = cameras.single_mut();
    let aspect = window.width() / window.height();
    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => return,
    };
    let radius = board_bounding_radius(board_size.0);
    let half_fov_v = fov / 2.0;
    let half_fov_h = (half_fov_v.tan() * aspect).atan();
    let dist_v = radius / half_fov_v.sin();
    let dist_h = radius / half_fov_h.sin();
    let distance = dist_v.max(dist_h) * CAMERA_MARGIN;
    let dir = camera_direction();
    *transform = Transform::from_translation(dir * distance).looking_at(Vec3::ZERO, Vec3::Y);
}
