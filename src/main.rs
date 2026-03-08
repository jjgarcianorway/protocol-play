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
const MIN_BOARD_SIZE: u32 = 3;
const MAX_BOARD_SIZE: u32 = 12;
const FLOOR_TOP_Y: f32 = 0.125;
const EMPTY_MARKER_Y: f32 = -0.124;
const ANIM_SPEED: f32 = 12.0;
const HOVER_ANIM_SPEED: f32 = 8.0;
const UI_ANIM_SPEED: f32 = 12.0;
const SLOT_VW: f32 = 4.5;
const SLOT_HEIGHT_VW: f32 = 5.6;
const ICON_VW: f32 = 3.4;
const NUM_COLORS: usize = 10;

const SOURCE_COLORS: [(f32, f32, f32); NUM_COLORS] = [
    (0.95, 0.1, 0.1),   // Red
    (1.0, 0.5, 0.0),    // Orange
    (1.0, 0.88, 0.0),   // Yellow
    (0.35, 0.85, 0.2),  // Light Green
    (0.0, 0.45, 0.12),  // Dark Green
    (0.3, 0.7, 1.0),    // Light Blue
    (0.1, 0.15, 0.75),  // Dark Blue
    (1.0, 0.35, 0.55),  // Pink
    (0.6, 0.15, 0.85),  // Purple
    (0.55, 0.3, 0.08),  // Brown
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
            Direction::East => -FRAC_PI_2,
            Direction::South => PI,
            Direction::West => FRAC_PI_2,
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
struct GhostCell(Option<(u32, u32)>);

#[derive(Resource, Default)]
struct InventoryState {
    level: u8,
    direction: Option<Direction>,
    color_index: Option<usize>,
}

#[derive(Resource, Default)]
struct PlacedSources(HashSet<usize>);

#[derive(Resource, Default, PartialEq, Clone, Copy)]
enum PlayMode { #[default] Editing, Playing }

#[derive(Resource)]
struct PlayIcons {
    play: Handle<Image>,
    stop: Handle<Image>,
}

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
    source_symbol_mesh: Handle<Mesh>,
    source_symbol_materials: Vec<Handle<StandardMaterial>>,
    ghost_symbol_materials: Vec<Handle<StandardMaterial>>,
    bot_mesh: Handle<Mesh>,
    eye_mesh: Handle<Mesh>,
    bot_materials: Vec<Handle<StandardMaterial>>,
    eye_material: Handle<StandardMaterial>,
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
struct Level2Slot;

#[derive(Component)]
struct Level3Slot;

#[derive(Component)]
struct ExpansionContainer;

#[derive(Component)]
struct Bot;

#[derive(Component)]
struct PlayStopButton;

#[derive(Component)]
struct PlayButtonImage;

#[derive(Component)]
struct GhostSymbolOverlay;

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
        .insert_resource(BoardSize(3))
        .insert_resource(SelectedTool::default())
        .insert_resource(HoveredCell::default())
        .insert_resource(HiddenTileEntity::default())
        .insert_resource(GhostCell::default())
        .insert_resource(InventoryState { level: 1, direction: None, color_index: None })
        .insert_resource(PlacedSources::default())
        .insert_resource(PlayMode::default())
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, (
            animate_node_width,
            button_interaction,
            inventory_interaction,
            update_inventory_visuals.after(inventory_interaction),
            update_l3_availability.after(inventory_interaction),
            update_hovered_cell,
            update_ghost_and_highlight.after(update_hovered_cell),
            handle_tile_click.after(update_hovered_cell),
            animate_scale.after(update_ghost_and_highlight),
            cleanup_despawned.after(animate_scale),
            play_stop_interaction,
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

fn create_empty_marker_texture(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 64;
    const BORDER: u32 = 3;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let on_edge = x < BORDER || x >= SIZE - BORDER || y < BORDER || y >= SIZE - BORDER;
            let color: [u8; 4] = if on_edge {
                [100, 100, 100, 120]
            } else {
                [0, 0, 0, 0]
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

fn create_source_symbol_texture(images: &mut Assets<Image>, size: u32) -> Handle<Image> {
    let fill: [u8; 4] = [255, 255, 255, 255];
    let stroke: [u8; 4] = [80, 80, 80, 255];
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let stroke_expand = 0.025;
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let color: [u8; 4] = if in_source_shape(nx, ny, 0.0) {
                fill
            } else if in_source_shape(nx, ny, stroke_expand) {
                stroke
            } else {
                [0, 0, 0, 0]
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

// === Isometric Icon Rendering ===
// Software renderer for 3D-looking inventory icons.
// Projects a unit cuboid (1×0.25×1) from the game camera angle (30° elev, 45° azim).

fn tile_texture_data(size: u32, border: u32) -> Vec<u8> {
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
    data
}

fn source_texture_colored_data(
    size: u32, border: u32, rotation: f32, fill: [u8; 4],
) -> Vec<u8> {
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
    data
}

fn point_in_quad_uv(
    px: f32, py: f32,
    tl: (f32, f32), tr: (f32, f32), br: (f32, f32), bl: (f32, f32),
) -> Option<(f32, f32)> {
    // Triangle 1: tl, tr, br => u = s + t, v = t  (since tl=0,0 tr=1,0 br=1,1)
    // Wait, let me think again: tl(0,0) tr(1,0) br(1,1)
    // P = tl + s*(tr-tl) + t*(br-tl) = tl + s*right + t*diag
    // At v1=tr: s=1,t=0 => u=1,v=0
    // At v2=br: s=0,t=1 => u=1,v=1
    // So u = s+t, v = t? No... that gives u=1 for both. Hmm.
    // Actually for proper bilinear: use the two-triangle approach with correct UV mapping.

    // Triangle 1: tl(u=0,v=0), tr(u=1,v=0), br(u=1,v=1)
    {
        let d00x = tr.0 - tl.0;
        let d00y = tr.1 - tl.1;
        let d01x = br.0 - tl.0;
        let d01y = br.1 - tl.1;
        let d02x = px - tl.0;
        let d02y = py - tl.1;

        let dot00 = d00x * d00x + d00y * d00y;
        let dot01 = d00x * d01x + d00y * d01y;
        let dot02 = d00x * d02x + d00y * d02y;
        let dot11 = d01x * d01x + d01y * d01y;
        let dot12 = d01x * d02x + d01y * d02y;

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom.abs() > 1e-10 {
            let inv = 1.0 / denom;
            let s = (dot11 * dot02 - dot01 * dot12) * inv;
            let t = (dot00 * dot12 - dot01 * dot02) * inv;
            if s >= 0.0 && t >= 0.0 && s + t <= 1.0 {
                // tl + s*(tr-tl) + t*(br-tl)
                // UV: tl=(0,0), tr=(1,0), br=(1,1) => u = s + t, v = t
                // Wait no. s is weight for (tr-tl), t for (br-tl).
                // Point = (1-s-t)*tl_uv + s*tr_uv + t*br_uv
                //       = s*(1,0) + t*(1,1) = (s+t, t)
                return Some((s + t, t));
            }
        }
    }

    // Triangle 2: tl(u=0,v=0), br(u=1,v=1), bl(u=0,v=1)
    {
        let d00x = br.0 - tl.0;
        let d00y = br.1 - tl.1;
        let d01x = bl.0 - tl.0;
        let d01y = bl.1 - tl.1;
        let d02x = px - tl.0;
        let d02y = py - tl.1;

        let dot00 = d00x * d00x + d00y * d00y;
        let dot01 = d00x * d01x + d00y * d01y;
        let dot02 = d00x * d02x + d00y * d02y;
        let dot11 = d01x * d01x + d01y * d01y;
        let dot12 = d01x * d02x + d01y * d02y;

        let denom = dot00 * dot11 - dot01 * dot01;
        if denom.abs() > 1e-10 {
            let inv = 1.0 / denom;
            let s = (dot11 * dot02 - dot01 * dot12) * inv;
            let t = (dot00 * dot12 - dot01 * dot02) * inv;
            if s >= 0.0 && t >= 0.0 && s + t <= 1.0 {
                // Point = (1-s-t)*tl_uv + s*br_uv + t*bl_uv
                //       = s*(1,1) + t*(0,1) = (s, s+t)
                return Some((s, s + t));
            }
        }
    }

    None
}

fn create_isometric_icon(
    images: &mut Assets<Image>,
    top_data: &[u8], tex_size: u32, icon_size: u32,
) -> Handle<Image> {
    let elev = CAMERA_ELEVATION.to_radians();
    let azim = CAMERA_AZIMUTH.to_radians();
    let sin_e = elev.sin();
    let cos_a = azim.cos();
    let sin_a = azim.sin();
    let cos_e = elev.cos();

    let project = |x: f32, y: f32, z: f32| -> (f32, f32) {
        let sx = x * cos_a - z * sin_a;
        let sy = -(x * sin_a * sin_e) - (z * cos_a * sin_e) + y * cos_e;
        (sx, sy)
    };

    let hx = 0.5f32;
    let hy = 0.125f32;
    let hz = 0.5f32;

    let top_tl = project(-hx, hy, -hz);
    let top_tr = project(hx, hy, -hz);
    let top_br = project(hx, hy, hz);
    let top_bl = project(-hx, hy, hz);
    let bot_tr = project(hx, -hy, -hz);
    let bot_br = project(hx, -hy, hz);
    let bot_bl = project(-hx, -hy, hz);

    let all_pts = [top_tl, top_tr, top_br, top_bl, bot_tr, bot_br, bot_bl];
    let min_sx = all_pts.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
    let max_sx = all_pts.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
    let min_sy = all_pts.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
    let max_sy = all_pts.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);

    let range = (max_sx - min_sx).max(max_sy - min_sy);
    let margin = 0.08;
    let scale = (icon_size as f32) * (1.0 - 2.0 * margin) / range;
    let cx = icon_size as f32 / 2.0;
    let cy = icon_size as f32 / 2.0;
    let off_x = (min_sx + max_sx) / 2.0;
    let off_y = (min_sy + max_sy) / 2.0;

    let to_px = |sx: f32, sy: f32| -> (f32, f32) {
        (cx + (sx - off_x) * scale, cy - (sy - off_y) * scale)
    };

    let top_tl_px = to_px(top_tl.0, top_tl.1);
    let top_tr_px = to_px(top_tr.0, top_tr.1);
    let top_br_px = to_px(top_br.0, top_br.1);
    let top_bl_px = to_px(top_bl.0, top_bl.1);
    let bot_tr_px = to_px(bot_tr.0, bot_tr.1);
    let bot_br_px = to_px(bot_br.0, bot_br.1);
    let bot_bl_px = to_px(bot_bl.0, bot_bl.1);

    let mut data = vec![0u8; (icon_size * icon_size * 4) as usize];

    for py in 0..icon_size {
        for px in 0..icon_size {
            let x = px as f32 + 0.5;
            let y = py as f32 + 0.5;
            let i = ((py * icon_size + px) * 4) as usize;

            // Top face — sample texture with UV mapping
            if let Some((u, v)) = point_in_quad_uv(x, y, top_tl_px, top_tr_px, top_br_px, top_bl_px) {
                let tx = ((u * tex_size as f32) as u32).min(tex_size - 1);
                let ty = ((v * tex_size as f32) as u32).min(tex_size - 1);
                let ti = ((ty * tex_size + tx) * 4) as usize;
                data[i] = top_data[ti];
                data[i + 1] = top_data[ti + 1];
                data[i + 2] = top_data[ti + 2];
                data[i + 3] = 255;
            }
            // Right side face — dark
            else if point_in_quad_uv(x, y, top_tr_px, bot_tr_px, bot_br_px, top_br_px).is_some() {
                data[i] = 50;
                data[i + 1] = 50;
                data[i + 2] = 50;
                data[i + 3] = 255;
            }
            // Left side face — darker
            else if point_in_quad_uv(x, y, top_bl_px, top_br_px, bot_br_px, bot_bl_px).is_some() {
                data[i] = 35;
                data[i + 1] = 35;
                data[i + 2] = 35;
                data[i + 3] = 255;
            }
        }
    }

    images.add(Image::new(
        Extent3d { width: icon_size, height: icon_size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn create_play_icon(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 128;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let fx = x as f32 / SIZE as f32;
            let fy = y as f32 / SIZE as f32;
            let in_triangle = fx >= 0.3 && fx <= 0.8 && {
                let t = (fx - 0.3) / 0.5;
                let half_h = 0.3 * (1.0 - t);
                (fy - 0.5).abs() <= half_h
            };
            let color: [u8; 4] = if in_triangle {
                [80, 200, 80, 255]
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

fn create_stop_icon(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 128;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let fx = x as f32 / SIZE as f32;
            let fy = y as f32 / SIZE as f32;
            let in_square = fx >= 0.28 && fx <= 0.72 && fy >= 0.28 && fy <= 0.72;
            let color: [u8; 4] = if in_square {
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
        TileKind::Empty => EMPTY_MARKER_Y,
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
    spawn_tile_at_scale(commands, col, row, board_size, kind, assets, Vec3::ZERO);
}

fn spawn_tile_at_scale(
    commands: &mut Commands, col: u32, row: u32,
    board_size: u32, kind: TileKind, assets: &GameAssets, initial_scale: Vec3,
) {
    let pos = tile_world_pos(col, row, board_size, &kind);
    match kind {
        TileKind::Empty => {
            commands.spawn((
                Mesh3d(assets.empty_mesh.clone()), MeshMaterial3d(assets.empty_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
        }
        TileKind::Floor => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos).with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            ));
        }
        TileKind::Source(ci, dir) => {
            commands.spawn((
                Mesh3d(assets.floor_mesh.clone()), MeshMaterial3d(assets.floor_material.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_y(dir.rotation()))
                    .with_scale(initial_scale),
                TargetScale(Vec3::ONE), Tile, TileCoord { col, row }, kind,
            )).with_children(|parent| {
                parent.spawn((
                    Mesh3d(assets.source_symbol_mesh.clone()),
                    MeshMaterial3d(assets.source_symbol_materials[ci].clone()),
                    Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + 0.002, 0.0)),
                ));
            });
        }
    }
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
        width: Val::Vw(SLOT_VW),
        height: Val::Vw(SLOT_HEIGHT_VW),
        border: UiRect::all(Val::Px(2.0)),
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
        node.width = Val::Vw(0.0);
    }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg),
        BorderColor(border), slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
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
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false });
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
    if !available || animated {
        node.width = Val::Vw(0.0);
    }

    let child = commands.spawn((
        Button, node, BackgroundColor(slot_bg),
        BorderColor(border),
        InventorySlot::SourceColor(color_index),
        Level3Slot,
    )).with_children(|parent| {
        parent.spawn((
            Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
            ImageNode::new(icon),
        ));
        parent.spawn((
            Text::new("1"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        ));
    }).id();

    if animated && available {
        commands.entity(child).insert(NodeWidthAnim { target: SLOT_VW, despawn_at_zero: false });
    } else if !available {
        // Already at 0 width, no animation needed
    }

    commands.entity(parent).add_child(child);
    child
}

fn spawn_l2_directions(
    commands: &mut Commands, container: Entity, icons: &InventoryIcons,
    selected_dir: Option<Direction>,
) {
    for dir in Direction::all() {
        let selected = selected_dir == Some(dir);
        let child = spawn_slot(commands, container, InventorySlot::SourceDir(dir),
            icons.source_dir(dir), selected, true);
        commands.entity(child).insert(Level2Slot);
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
        base_color: Color::srgba(0.9, 0.2, 0.2, 0.6),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, 0.001, 1.02));
    let empty_marker_texture = create_empty_marker_texture(&mut images);
    let empty_material = materials.add(StandardMaterial {
        base_color_texture: Some(empty_marker_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let empty_mesh = meshes.add(Cuboid::new(0.95, 0.001, 0.95));
    let highlight_texture = create_highlight_texture(&mut images);
    let highlight_material = materials.add(StandardMaterial {
        base_color_texture: Some(highlight_texture),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let highlight_mesh = meshes.add(Cuboid::new(1.05, 0.001, 1.05));

    // Source symbol overlay (top face only)
    let source_symbol_texture = create_source_symbol_texture(&mut images, 1024);
    let source_symbol_mesh = meshes.add(Cuboid::new(0.99, 0.001, 0.99));
    let source_symbol_materials: Vec<Handle<StandardMaterial>> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
            base_color_texture: Some(source_symbol_texture.clone()),
            alpha_mode: AlphaMode::Mask(0.5),
            unlit: true,
            ..default()
        })
    }).collect();

    // Ghost symbol overlay (transparent, unlit, for hover preview)
    let ghost_symbol_materials: Vec<Handle<StandardMaterial>> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgba(r, g, b, 0.6),
            base_color_texture: Some(source_symbol_texture.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    }).collect();

    let bot_mesh = meshes.add(Cuboid::new(0.35, 0.35, 0.35));
    let eye_mesh = meshes.add(Cuboid::new(0.06, 0.065, 0.015));
    let eye_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });
    let bot_materials: Vec<Handle<StandardMaterial>> = SOURCE_COLORS.iter().map(|&(r, g, b)| {
        materials.add(StandardMaterial {
            base_color: Color::srgb(r, g, b),
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
        source_symbol_mesh: source_symbol_mesh.clone(), source_symbol_materials,
        ghost_symbol_materials: ghost_symbol_materials.clone(),
        bot_mesh, eye_mesh, bot_materials, eye_material,
    };

    spawn_board(&mut commands, board_size.0, &assets);
    commands.insert_resource(assets);

    commands.spawn((
        Mesh3d(floor_mesh), MeshMaterial3d(ghost_floor_material),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), GhostPreview,
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(source_symbol_mesh.clone()),
            MeshMaterial3d(ghost_symbol_materials[0].clone()),
            Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + 0.002, 0.0))
                .with_scale(Vec3::ZERO),
            GhostSymbolOverlay,
        ));
    });
    commands.spawn((
        Mesh3d(highlight_mesh), MeshMaterial3d(highlight_material),
        Transform::from_xyz(0.0, FLOOR_TOP_Y + 0.01, 0.0).with_scale(Vec3::ZERO),
        TargetScale(Vec3::ZERO), TileHighlight,
    ));
    commands.spawn((
        DirectionalLight { illuminance: 3000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_direction() * 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    const ICON: u32 = 128;
    const TEX: u32 = 128;
    const BRD: u32 = 6;

    // Floor icon: isometric cuboid with floor texture
    let floor_tex_data = tile_texture_data(TEX, BRD);
    let floor_icon = create_isometric_icon(&mut images, &floor_tex_data, TEX, ICON);

    // Source icon (L1, white, north-facing)
    let source_tex_data = source_texture_colored_data(TEX, BRD, 0.0, [240, 240, 240, 255]);
    let source_icon = create_isometric_icon(&mut images, &source_tex_data, TEX, ICON);

    let delete_icon = create_delete_icon(&mut images);

    // Source direction icons (L2): 4 rotated isometric cubes
    // Icon textures need negated rotation: mesh rotation and texture rotation have opposite handedness
    let source_north_data = source_texture_colored_data(TEX, BRD, -Direction::North.rotation(), [240, 240, 240, 255]);
    let source_north = create_isometric_icon(&mut images, &source_north_data, TEX, ICON);
    let source_east_data = source_texture_colored_data(TEX, BRD, -Direction::East.rotation(), [240, 240, 240, 255]);
    let source_east = create_isometric_icon(&mut images, &source_east_data, TEX, ICON);
    let source_south_data = source_texture_colored_data(TEX, BRD, -Direction::South.rotation(), [240, 240, 240, 255]);
    let source_south = create_isometric_icon(&mut images, &source_south_data, TEX, ICON);
    let source_west_data = source_texture_colored_data(TEX, BRD, -Direction::West.rotation(), [240, 240, 240, 255]);
    let source_west = create_isometric_icon(&mut images, &source_west_data, TEX, ICON);

    // Generate color icons: 10 colors x 4 directions
    let mut source_color_icons = Vec::with_capacity(NUM_COLORS * 4);
    for ci in 0..NUM_COLORS {
        let (r, g, b) = SOURCE_COLORS[ci];
        let fill = color_to_u8(r, g, b);
        for dir in Direction::all() {
            let tex_data = source_texture_colored_data(TEX, BRD, -dir.rotation(), fill);
            let icon = create_isometric_icon(&mut images, &tex_data, TEX, ICON);
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
        }).with_child((Text::new("3x3"), text_style.clone(), TextColor(Color::WHITE), BoardSizeText));
        parent.spawn((Button, btn_node, BackgroundColor(btn_color), BoardButton::Increase))
            .with_child((Text::new("+"), text_style, TextColor(Color::WHITE)));
    });

    // Play/Stop button
    let play_icon = create_play_icon(&mut images);
    let stop_icon = create_stop_icon(&mut images);
    commands.insert_resource(PlayIcons { play: play_icon.clone(), stop: stop_icon });

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(10.0), top: Val::Px(10.0),
        ..default()
    }).with_child((
        Button,
        Node {
            width: Val::Px(48.0), height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        PlayStopButton,
        ImageNode::new(play_icon),
        PlayButtonImage,
    ));

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
                padding: UiRect::all(Val::Vw(0.6)),
                column_gap: Val::Vw(0.5),
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
                    Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
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
                    Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
                    ImageNode::new(source_icon),
                ));
                slot.spawn((Text::new(" "), spacer_font.clone(), spacer_color));
            });
            // Expansion container for L2/L3 slots (between Source and Delete)
            container.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vw(0.5),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ExpansionContainer,
            ));
            container.spawn((
                Button, sn, BackgroundColor(slot_bg),
                BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                InventorySlot::Delete,
            )).with_children(|slot| {
                slot.spawn((
                    Node { width: Val::Vw(ICON_VW), height: Val::Vw(ICON_VW), ..default() },
                    ImageNode::new(delete_icon),
                ));
                slot.spawn((Text::new(" "), spacer_font, spacer_color));
            });
        });
    });
}

// === Animation ===
fn animate_scale(
    mut query: Query<(&mut Transform, &TargetScale, Option<&GhostPreview>, Option<&TileHighlight>)>,
    time: Res<Time>,
) {
    for (mut transform, target, ghost, highlight) in &mut query {
        let speed = if ghost.is_some() || highlight.is_some() {
            HOVER_ANIM_SPEED
        } else {
            ANIM_SPEED
        };
        transform.scale = transform.scale.lerp(target.0, speed * time.delta_secs());
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
            Val::Vw(w) | Val::Px(w) => w,
            _ => anim.target,
        };
        let new_w = current + (anim.target - current) * UI_ANIM_SPEED * time.delta_secs();
        if (new_w - anim.target).abs() < 0.1 {
            if anim.despawn_at_zero && anim.target < 0.1 {
                commands.entity(entity).despawn_recursive();
            } else {
                node.width = Val::Vw(anim.target);
                commands.entity(entity).remove::<NodeWidthAnim>();
            }
        } else {
            node.width = Val::Vw(new_w);
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
fn collapse_expansion(
    commands: &mut Commands,
    l2_slots: &Query<Entity, With<Level2Slot>>,
    l3_slots: &Query<(Entity, &InventorySlot), With<Level3Slot>>,
) {
    for entity in l2_slots.iter() {
        commands.entity(entity).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true });
    }
    for (entity, _) in l3_slots.iter() {
        commands.entity(entity).insert(NodeWidthAnim { target: 0.0, despawn_at_zero: true });
    }
}

fn inventory_interaction(
    mut commands: Commands,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    slots: Query<(&Interaction, &InventorySlot), Changed<Interaction>>,
    l2_slots: Query<Entity, With<Level2Slot>>,
    l3_slots: Query<(Entity, &InventorySlot), With<Level3Slot>>,
    expansion_q: Query<Entity, With<ExpansionContainer>>,
    icons: Res<InventoryIcons>,
    placed_sources: Res<PlacedSources>,
    children_q: Query<&Children>,
    mut image_q: Query<&mut ImageNode>,
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
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::Delete => {
            selected_tool.0 = Tool::Delete;
            if inv_state.level > 1 {
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::Source => {
            if inv_state.level == 1 {
                // Expand: show L2 + L3 with auto-selected direction and color
                let dir = inv_state.direction.unwrap_or(Direction::North);
                inv_state.direction = Some(dir);
                if inv_state.color_index.is_none() || placed_sources.0.contains(&inv_state.color_index.unwrap_or(0)) {
                    inv_state.color_index = (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci));
                }
                inv_state.level = 3;
                selected_tool.0 = Tool::Source;
                let expansion = expansion_q.single();
                spawn_l2_directions(&mut commands, expansion, &icons, Some(dir));
                rebuild_l3_colors(&mut commands, expansion, &icons, dir, inv_state.color_index, &placed_sources);
            } else {
                // Collapse back to L1
                collapse_expansion(&mut commands, &l2_slots, &l3_slots);
                inv_state.level = 1;
                selected_tool.0 = Tool::Floor;
                inv_state.direction = None;
                inv_state.color_index = None;
            }
        }
        InventorySlot::SourceDir(dir) => {
            let old_dir = inv_state.direction;
            inv_state.direction = Some(dir);
            selected_tool.0 = Tool::Source;
            let expansion = expansion_q.single();

            if inv_state.level == 2 {
                // Transition to L3: add color slots, auto-select first available color
                inv_state.level = 3;
                if inv_state.color_index.is_none() || placed_sources.0.contains(&inv_state.color_index.unwrap_or(0)) {
                    inv_state.color_index = (0..NUM_COLORS).find(|ci| !placed_sources.0.contains(ci));
                }
                rebuild_l3_colors(&mut commands, expansion, &icons, dir, inv_state.color_index, &placed_sources);
            } else if inv_state.level == 3 && old_dir != Some(dir) {
                // Direction changed in L3: update icons in-place (no animation needed)
                for (entity, slot) in &l3_slots {
                    if let InventorySlot::SourceColor(ci) = slot {
                        let new_icon = icons.source_color_dir(*ci, dir);
                        if let Ok(children) = children_q.get(entity) {
                            for &child in children.iter() {
                                if let Ok(mut img) = image_q.get_mut(child) {
                                    img.image = new_icon.clone();
                                }
                            }
                        }
                    }
                }
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
            InventorySlot::Floor => selected_tool.0 == Tool::Floor,
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
            let target = if should_show { SLOT_VW } else { 0.0 };
            let current = match node.width { Val::Vw(w) => w, _ => target };
            if (current - target).abs() > 0.1 {
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
    mut commands: Commands,
    hovered: Res<HoveredCell>,
    selected_tool: Res<SelectedTool>,
    inv_state: Res<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    mut ghost_q: Query<
        (&mut Transform, &mut TargetScale, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>),
        (With<GhostPreview>, Without<TileHighlight>),
    >,
    mut ghost_overlay_q: Query<
        (&mut Transform, &mut MeshMaterial3d<StandardMaterial>),
        (With<GhostSymbolOverlay>, Without<GhostPreview>, Without<TileHighlight>, Without<Tile>),
    >,
    mut highlight_q: Query<
        (&mut Transform, &mut TargetScale),
        (With<TileHighlight>, Without<GhostPreview>, Without<Tile>),
    >,
    mut hidden_tile: ResMut<HiddenTileEntity>,
    mut ghost_cell: ResMut<GhostCell>,
    mut tile_scale_q: Query<
        &mut TargetScale,
        (With<Tile>, Without<GhostPreview>, Without<TileHighlight>, Without<DespawnAtZeroScale>),
    >,
    placed_sources: Res<PlacedSources>,
) {
    // Restore previous suppressed tile
    if let Some(old_entity) = hidden_tile.0.take() {
        if let Ok(mut target) = tile_scale_q.get_mut(old_entity) {
            target.0 = Vec3::ONE;
        }
    }

    let (mut ghost_tf, mut ghost_target, mut ghost_mesh, mut ghost_mat) = ghost_q.single_mut();
    let (mut overlay_tf, mut overlay_mat) = ghost_overlay_q.single_mut();
    let (mut hl_tf, mut hl_target) = highlight_q.single_mut();

    // Helper: hide the ghost symbol overlay
    let mut show_overlay = false;

    let Some((col, row)) = hovered.0 else {
        // Spawn trail when leaving the board with delete tool
        if ghost_target.0.length() > 0.5 && selected_tool.0 == Tool::Delete {
            commands.spawn((
                Mesh3d(assets.ghost_delete_mesh.clone()),
                MeshMaterial3d(assets.ghost_delete_material.clone()),
                Transform::from_translation(ghost_tf.translation)
                    .with_scale(ghost_tf.scale),
                TargetScale(Vec3::ZERO),
                DespawnAtZeroScale,
            ));
        }
        ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO;
        hl_target.0 = Vec3::ZERO;
        ghost_cell.0 = None;
        return;
    };

    let tile_info = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile_info else {
        ghost_target.0 = Vec3::ZERO;
        overlay_tf.scale = Vec3::ZERO;
        hl_target.0 = Vec3::ZERO;
        ghost_cell.0 = None;
        return;
    };

    let offset = (board_size.0 as f32 - 1.0) / 2.0;
    let world_x = col as f32 - offset;
    let world_z = row as f32 - offset;

    // Cell changed
    let cell_changed = ghost_cell.0 != Some((col, row));
    if cell_changed {
        if selected_tool.0 == Tool::Delete {
            // Delete tool: spawn fade trail at old position
            if ghost_target.0.length() > 0.5 {
                commands.spawn((
                    Mesh3d(assets.ghost_delete_mesh.clone()),
                    MeshMaterial3d(assets.ghost_delete_material.clone()),
                    Transform::from_translation(ghost_tf.translation)
                        .with_scale(ghost_tf.scale),
                    TargetScale(Vec3::ZERO),
                    DespawnAtZeroScale,
                ));
            }
        }
        // All tools: snap ghost scale to zero so it fades in fresh at new position
        ghost_tf.scale = Vec3::ZERO;
        // Reset highlight scale for fade-in at new position
        hl_tf.scale = Vec3::ZERO;
        ghost_cell.0 = Some((col, row));
    }

    let hl_y = FLOOR_TOP_Y + 0.01;
    hl_tf.translation = Vec3::new(world_x, hl_y, world_z);
    hl_target.0 = Vec3::ONE;

    let can_place_floor = matches!(kind, TileKind::Empty | TileKind::Source(_, _));
    let can_place_source = !matches!(kind, TileKind::Source(_, _));
    let can_delete = matches!(kind, TileKind::Floor | TileKind::Source(_, _));

    match selected_tool.0 {
        Tool::Floor if can_place_floor => {
            ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
            ghost_target.0 = Vec3::ONE;
            if let Ok(mut target) = tile_scale_q.get_mut(entity) {
                target.0 = Vec3::ZERO;
                hidden_tile.0 = Some(entity);
            }
        }
        Tool::Source if can_place_source => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    ghost_tf.translation = Vec3::new(world_x, 0.0, world_z);
                    ghost_tf.rotation = Quat::from_rotation_y(dir.rotation());
                    *ghost_mesh = Mesh3d(assets.floor_mesh.clone());
                    *ghost_mat = MeshMaterial3d(assets.ghost_floor_material.clone());
                    *overlay_mat = MeshMaterial3d(assets.ghost_symbol_materials[ci].clone());
                    show_overlay = true;
                    ghost_target.0 = Vec3::ONE;
                    if let Ok(mut target) = tile_scale_q.get_mut(entity) {
                        target.0 = Vec3::ZERO;
                        hidden_tile.0 = Some(entity);
                    }
                } else {
                    ghost_target.0 = Vec3::ZERO;
                }
            } else {
                ghost_target.0 = Vec3::ZERO;
            }
        }
        Tool::Delete if can_delete => {
            ghost_tf.translation = Vec3::new(world_x, FLOOR_TOP_Y + 0.005, world_z);
            ghost_tf.rotation = Quat::IDENTITY;
            *ghost_mesh = Mesh3d(assets.ghost_delete_mesh.clone());
            *ghost_mat = MeshMaterial3d(assets.ghost_delete_material.clone());
            ghost_target.0 = Vec3::ONE;
        }
        _ => { ghost_target.0 = Vec3::ZERO; }
    }

    // Show or hide the ghost symbol overlay
    overlay_tf.scale = if show_overlay { Vec3::ONE } else { Vec3::ZERO };
}

// === Tile Placement ===
fn handle_tile_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    hovered: Res<HoveredCell>,
    mut selected_tool: ResMut<SelectedTool>,
    mut inv_state: ResMut<InventoryState>,
    board_size: Res<BoardSize>,
    tiles: Query<(Entity, &TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    assets: Res<GameAssets>,
    ui_interactions: Query<&Interaction, With<Button>>,
    mut placed_sources: ResMut<PlacedSources>,
    play_mode: Res<PlayMode>,
    ghost_q: Query<&Transform, With<GhostPreview>>,
) {
    if *play_mode == PlayMode::Playing { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    for interaction in &ui_interactions {
        if *interaction != Interaction::None { return; }
    }
    let Some((col, row)) = hovered.0 else { return; };
    let tile = tiles.iter().find(|(_, c, _)| c.col == col && c.row == row);
    let Some((entity, _, kind)) = tile else { return; };
    // Get ghost's current scale for smooth transition
    let ghost_scale = ghost_q.single().scale;
    // Free the color if overwriting a source tile
    if let TileKind::Source(ci, _) = kind {
        if selected_tool.0 != Tool::Source || inv_state.color_index != Some(*ci) {
            placed_sources.0.remove(ci);
        }
    }

    match (selected_tool.0, *kind) {
        (Tool::Floor, TileKind::Empty | TileKind::Source(_, _)) => {
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Floor, &assets, ghost_scale);
        }
        (Tool::Source, TileKind::Empty | TileKind::Floor) => {
            if let (Some(dir), Some(ci)) = (inv_state.direction, inv_state.color_index) {
                if !placed_sources.0.contains(&ci) {
                    placed_sources.0.insert(ci);
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                    spawn_tile_at_scale(&mut commands, col, row, board_size.0, TileKind::Source(ci, dir), &assets, ghost_scale);
                    // Auto-select next available color
                    let next = (1..NUM_COLORS)
                        .map(|offset| (ci + offset) % NUM_COLORS)
                        .find(|c| !placed_sources.0.contains(c));
                    inv_state.color_index = next;
                    if next.is_none() {
                        selected_tool.0 = Tool::Floor;
                    }
                }
            }
        }
        (Tool::Delete, TileKind::Floor | TileKind::Source(_, _)) => {
            // Pre-select the freed color so it's ready if user switches to Source
            if let TileKind::Source(ci, _) = kind {
                if inv_state.level >= 2 {
                    inv_state.color_index = Some(*ci);
                }
            }
            commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
            spawn_tile(&mut commands, col, row, board_size.0, TileKind::Empty, &assets);
        }
        _ => {}
    }
}

// === Play/Stop ===
fn play_stop_interaction(
    mut commands: Commands,
    mut play_mode: ResMut<PlayMode>,
    interaction_query: Query<&Interaction, (With<PlayStopButton>, Changed<Interaction>)>,
    tiles: Query<(&TileCoord, &TileKind), (With<Tile>, Without<DespawnAtZeroScale>)>,
    bots: Query<Entity, With<Bot>>,
    assets: Res<GameAssets>,
    board_size: Res<BoardSize>,
    mut button_image: Query<&mut ImageNode, With<PlayButtonImage>>,
    play_icons: Res<PlayIcons>,
) {
    for interaction in &interaction_query {
        if *interaction != Interaction::Pressed { continue; }

        match *play_mode {
            PlayMode::Editing => {
                *play_mode = PlayMode::Playing;
                // Switch to stop icon
                let mut img = button_image.single_mut();
                img.image = play_icons.stop.clone();
                // Spawn bots on all source tiles
                for (coord, kind) in &tiles {
                    if let TileKind::Source(ci, dir) = kind {
                        let pos = tile_world_pos(coord.col, coord.row, board_size.0, kind);
                        let bot_y = FLOOR_TOP_Y + 0.175;
                        commands.spawn((
                            Mesh3d(assets.bot_mesh.clone()),
                            MeshMaterial3d(assets.bot_materials[*ci].clone()),
                            Transform::from_translation(Vec3::new(pos.x, bot_y, pos.z))
                                .with_rotation(Quat::from_rotation_y(dir.rotation()))
                                .with_scale(Vec3::ZERO),
                            TargetScale(Vec3::ONE),
                            Bot,
                        )).with_children(|parent| {
                            // Left eye
                            parent.spawn((
                                Mesh3d(assets.eye_mesh.clone()),
                                MeshMaterial3d(assets.eye_material.clone()),
                                Transform::from_translation(Vec3::new(-0.07, 0.04, -0.176)),
                            ));
                            // Right eye
                            parent.spawn((
                                Mesh3d(assets.eye_mesh.clone()),
                                MeshMaterial3d(assets.eye_material.clone()),
                                Transform::from_translation(Vec3::new(0.07, 0.04, -0.176)),
                            ));
                        });
                    }
                }
            }
            PlayMode::Playing => {
                *play_mode = PlayMode::Editing;
                // Switch to play icon
                let mut img = button_image.single_mut();
                img.image = play_icons.play.clone();
                // Despawn all bots smoothly
                for entity in &bots {
                    commands.entity(entity).insert((TargetScale(Vec3::ZERO), DespawnAtZeroScale));
                }
            }
        }
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
