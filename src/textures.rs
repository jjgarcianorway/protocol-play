// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::constants::*;

fn make_image(images: &mut Assets<Image>, data: Vec<u8>, size: u32) -> Handle<Image> {
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

pub fn color_to_u8(r: f32, g: f32, b: f32) -> [u8; 4] {
    [(r * 255.0).clamp(0.0, 255.0) as u8, (g * 255.0).clamp(0.0, 255.0) as u8,
     (b * 255.0).clamp(0.0, 255.0) as u8, 255]
}

pub fn create_tile_texture(images: &mut Assets<Image>, size: u32, border: u32) -> Handle<Image> {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            let on_edge = x < border || x >= size - border || y < border || y >= size - border;
            let color = if on_edge { TILE_DARK } else { TILE_GRAY };
            let i = ((y * size + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, size)
}

pub fn create_delete_icon(images: &mut Assets<Image>) -> Handle<Image> {
    let mut data = vec![0u8; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let fx = x as f32 / ICON_SIZE as f32;
            let fy = y as f32 / ICON_SIZE as f32;
            let on_diag1 = (fx - fy).abs() < 0.09;
            let on_diag2 = (fx - (1.0 - fy)).abs() < 0.09;
            let in_margin = fx > 0.15 && fx < 0.85 && fy > 0.15 && fy < 0.85;
            let color: [u8; 4] = if (on_diag1 || on_diag2) && in_margin {
                [220, 60, 60, 255]
            } else { [40, 40, 40, 255] };
            let i = ((y * ICON_SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, ICON_SIZE)
}

pub fn create_empty_marker_texture(images: &mut Assets<Image>) -> Handle<Image> {
    const SIZE: u32 = 64;
    const BORDER: u32 = 3;
    let mut data = vec![0u8; (SIZE * SIZE * 4) as usize];
    for y in 0..SIZE {
        for x in 0..SIZE {
            let on_edge = x < BORDER || x >= SIZE - BORDER || y < BORDER || y >= SIZE - BORDER;
            let color: [u8; 4] = if on_edge { [100, 100, 100, 120] } else { [0, 0, 0, 0] };
            let i = ((y * SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, SIZE)
}

pub fn create_highlight_texture(images: &mut Assets<Image>) -> Handle<Image> {
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
    make_image(images, data, SIZE)
}

// === Shape tests ===
fn in_turn_shape(x: f32, y: f32, expand: f32) -> bool {
    let half_width = 0.06 + expand;
    if x > -expand && x < 0.75 + expand && y.abs() < half_width { return true; }
    if y > -(0.75 + expand) && y < expand && x.abs() < half_width { return true; }
    let end_r = half_width;
    let dx = x - 0.75;
    if (dx * dx + y * y).sqrt() < end_r { return true; }
    let dy = y + 0.75;
    if (x * x + dy * dy).sqrt() < end_r { return true; }
    false
}

fn in_turn_center(x: f32, y: f32) -> bool {
    (x * x + y * y).sqrt() < 0.14
}

fn in_source_shape(x: f32, y: f32, expand: f32) -> bool {
    let dist = (x * x + y * y).sqrt();
    if dist < 0.35 + expand { return true; }
    if x.abs() < 0.05 + expand && y < -(0.35 - expand) && y > -(0.62 + expand) { return true; }
    let tip_y = -0.82;
    let base_y = -0.58;
    if y > tip_y - expand && y < base_y + expand {
        let t = ((y - tip_y) / (base_y - tip_y)).clamp(0.0, 1.0);
        if x.abs() < 0.14 * t + expand { return true; }
    }
    false
}

// === Symbol textures ===
pub fn create_source_symbol_texture(images: &mut Assets<Image>, size: u32) -> Handle<Image> {
    let fill: [u8; 4] = [255, 255, 255, 255];
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let color = if in_source_shape(nx, ny, 0.0) { fill }
                else if in_source_shape(nx, ny, STROKE_EXPAND) { SYMBOL_STROKE }
                else { [0, 0, 0, 0] };
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, size)
}

pub fn create_turn_symbol_texture(images: &mut Assets<Image>, size: u32) -> Handle<Image> {
    let fill: [u8; 4] = [255, 255, 255, 255];
    let b = (TURN_CENTER_BRIGHTNESS * 255.0) as u8;
    let center_dot: [u8; 4] = [b, b, b, 255];
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let color = if in_turn_center(nx, ny) { center_dot }
                else if in_turn_shape(nx, ny, 0.0) { fill }
                else if in_turn_shape(nx, ny, STROKE_EXPAND) { SYMBOL_STROKE }
                else { [0, 0, 0, 0] };
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, size)
}

// === Icon texture data ===
pub fn tile_texture_data(size: u32, border: u32) -> Vec<u8> {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            let on_edge = x < border || x >= size - border || y < border || y >= size - border;
            let color = if on_edge { TILE_DARK } else { TILE_GRAY };
            let i = ((y * size + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    data
}

fn rotated_shape_data(
    size: u32, border: u32, rotation: f32, fill: [u8; 4],
    shape_fn: fn(f32, f32, f32) -> bool,
    center_fn: Option<fn(f32, f32) -> bool>, center_fill: [u8; 4],
) -> Vec<u8> {
    let center = size as f32 / 2.0;
    let scale = size as f32 / 2.0;
    let cos_r = rotation.cos();
    let sin_r = rotation.sin();
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let on_edge = px < border || px >= size - border || py < border || py >= size - border;
            let mut color = if on_edge { TILE_DARK } else { TILE_GRAY };
            let nx = (px as f32 - center) / scale;
            let ny = (py as f32 - center) / scale;
            let rnx = nx * cos_r + ny * sin_r;
            let rny = -nx * sin_r + ny * cos_r;
            if let Some(cf) = center_fn {
                if cf(rnx, rny) { color = center_fill; }
                else if shape_fn(rnx, rny, 0.0) { color = fill; }
                else if shape_fn(rnx, rny, STROKE_EXPAND) { color = SYMBOL_STROKE; }
            } else {
                if shape_fn(rnx, rny, 0.0) { color = fill; }
                else if shape_fn(rnx, rny, STROKE_EXPAND) { color = SYMBOL_STROKE; }
            }
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    data
}

pub fn source_texture_colored_data(size: u32, border: u32, rotation: f32, fill: [u8; 4]) -> Vec<u8> {
    rotated_shape_data(size, border, rotation, fill, in_source_shape, None, [0; 4])
}

pub fn turn_texture_colored_data(size: u32, border: u32, rotation: f32, fill: [u8; 4]) -> Vec<u8> {
    let b = (TURN_CENTER_BRIGHTNESS * 255.0) as u8;
    let center_fill = [
        ((fill[0] as f32 / 255.0 * b as f32 / 255.0) * 255.0) as u8,
        ((fill[1] as f32 / 255.0 * b as f32 / 255.0) * 255.0) as u8,
        ((fill[2] as f32 / 255.0 * b as f32 / 255.0) * 255.0) as u8,
        255,
    ];
    rotated_shape_data(size, border, rotation, fill, in_turn_shape, Some(in_turn_center), center_fill)
}

// === Isometric icon rendering ===
fn point_in_quad_uv(
    px: f32, py: f32,
    tl: (f32, f32), tr: (f32, f32), br: (f32, f32), bl: (f32, f32),
) -> Option<(f32, f32)> {
    // Triangle 1: tl(0,0), tr(1,0), br(1,1)
    {
        let (d00x, d00y) = (tr.0 - tl.0, tr.1 - tl.1);
        let (d01x, d01y) = (br.0 - tl.0, br.1 - tl.1);
        let (d02x, d02y) = (px - tl.0, py - tl.1);
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
                return Some((s + t, t));
            }
        }
    }
    // Triangle 2: tl(0,0), br(1,1), bl(0,1)
    {
        let (d00x, d00y) = (br.0 - tl.0, br.1 - tl.1);
        let (d01x, d01y) = (bl.0 - tl.0, bl.1 - tl.1);
        let (d02x, d02y) = (px - tl.0, py - tl.1);
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
                return Some((s, s + t));
            }
        }
    }
    None
}

pub fn create_isometric_icon(
    images: &mut Assets<Image>, top_data: &[u8], tex_size: u32, icon_size: u32,
) -> Handle<Image> {
    let elev = CAMERA_ELEVATION.to_radians();
    let azim = CAMERA_AZIMUTH.to_radians();
    let (sin_e, cos_e) = (elev.sin(), elev.cos());
    let (sin_a, cos_a) = (azim.sin(), azim.cos());

    let project = |x: f32, y: f32, z: f32| -> (f32, f32) {
        (x * cos_a - z * sin_a, -(x * sin_a * sin_e) - (z * cos_a * sin_e) + y * cos_e)
    };

    let (hx, hy, hz) = (0.5f32, TILE_HEIGHT / 2.0, 0.5f32);
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

    let to_px = |sx: f32, sy: f32| (cx + (sx - off_x) * scale, cy - (sy - off_y) * scale);
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
            let (x, y) = (px as f32 + 0.5, py as f32 + 0.5);
            let i = ((py * icon_size + px) * 4) as usize;
            if let Some((u, v)) = point_in_quad_uv(x, y, top_tl_px, top_tr_px, top_br_px, top_bl_px) {
                let tx = ((u * tex_size as f32) as u32).min(tex_size - 1);
                let ty = ((v * tex_size as f32) as u32).min(tex_size - 1);
                let ti = ((ty * tex_size + tx) * 4) as usize;
                data[i..i + 3].copy_from_slice(&top_data[ti..ti + 3]);
                data[i + 3] = 255;
            } else if point_in_quad_uv(x, y, top_tr_px, bot_tr_px, bot_br_px, top_br_px).is_some() {
                data[i..i + 4].copy_from_slice(&[50, 50, 50, 255]);
            } else if point_in_quad_uv(x, y, top_bl_px, top_br_px, bot_br_px, bot_bl_px).is_some() {
                data[i..i + 4].copy_from_slice(&[35, 35, 35, 255]);
            }
        }
    }
    make_image(images, data, icon_size)
}

pub fn create_play_icon(images: &mut Assets<Image>) -> Handle<Image> {
    let mut data = vec![0u8; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let fx = x as f32 / ICON_SIZE as f32;
            let fy = y as f32 / ICON_SIZE as f32;
            let in_triangle = fx >= 0.3 && fx <= 0.8 && {
                let t = (fx - 0.3) / 0.5;
                (fy - 0.5).abs() <= 0.3 * (1.0 - t)
            };
            let color: [u8; 4] = if in_triangle { [80, 200, 80, 255] } else { [40, 40, 40, 255] };
            let i = ((y * ICON_SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, ICON_SIZE)
}

pub fn create_stop_icon(images: &mut Assets<Image>) -> Handle<Image> {
    let mut data = vec![0u8; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let fx = x as f32 / ICON_SIZE as f32;
            let fy = y as f32 / ICON_SIZE as f32;
            let in_square = fx >= 0.28 && fx <= 0.72 && fy >= 0.28 && fy <= 0.72;
            let color: [u8; 4] = if in_square { [220, 60, 60, 255] } else { [40, 40, 40, 255] };
            let i = ((y * ICON_SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    make_image(images, data, ICON_SIZE)
}
