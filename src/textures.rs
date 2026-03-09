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

/// Load a PNG file synchronously and create a Bevy Image asset.
/// `is_srgb` controls whether the texture uses sRGB or linear format.
pub fn load_png_texture(images: &mut Assets<Image>, path: &str, is_srgb: bool) -> Handle<Image> {
    let img = image::open(path).unwrap_or_else(|e| panic!("Failed to load {path}: {e}"));
    let rgba = img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let format = if is_srgb { TextureFormat::Rgba8UnormSrgb } else { TextureFormat::Rgba8Unorm };
    images.add(Image::new(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        TextureDimension::D2, rgba.into_raw(), format, default(),
    ))
}

pub fn create_tile_texture(images: &mut Assets<Image>, size: u32, border: u32) -> Handle<Image> {
    make_image(images, tile_texture_data(size, border), size)
}

pub fn color_to_u8(r: f32, g: f32, b: f32) -> [u8; 4] {
    [(r * 255.0).clamp(0.0, 255.0) as u8, (g * 255.0).clamp(0.0, 255.0) as u8,
     (b * 255.0).clamp(0.0, 255.0) as u8, 255]
}

fn ui_icon(images: &mut Assets<Image>, test: impl Fn(f32, f32) -> bool, color: [u8; 4]) -> Handle<Image> {
    let mut data = vec![0u8; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let (fx, fy) = (x as f32 / ICON_SIZE as f32, y as f32 / ICON_SIZE as f32);
            let c = if test(fx, fy) { color } else { [40, 40, 40, 255] };
            let i = ((y * ICON_SIZE + x) * 4) as usize;
            data[i..i + 4].copy_from_slice(&c);
        }
    }
    make_image(images, data, ICON_SIZE)
}

pub fn create_delete_icon(images: &mut Assets<Image>) -> Handle<Image> {
    ui_icon(images, |fx, fy| {
        ((fx - fy).abs() < 0.09 || (fx - (1.0 - fy)).abs() < 0.09)
        && fx > 0.15 && fx < 0.85 && fy > 0.15 && fy < 0.85
    }, [220, 60, 60, 255])
}

fn create_border_texture(images: &mut Assets<Image>, size: u32, border: u32, edge: [u8; 4]) -> Handle<Image> {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size {
        for x in 0..size {
            let on_edge = x < border || x >= size - border || y < border || y >= size - border;
            if on_edge {
                let i = ((y * size + x) * 4) as usize;
                data[i..i + 4].copy_from_slice(&edge);
            }
        }
    }
    make_image(images, data, size)
}

pub fn create_empty_marker_texture(i: &mut Assets<Image>) -> Handle<Image> { create_border_texture(i, 64, 3, [100, 100, 100, 120]) }
pub fn create_highlight_texture(i: &mut Assets<Image>) -> Handle<Image> { create_border_texture(i, 64, 4, [255, 255, 255, 200]) }

// === Shape tests ===
fn in_star_shape(x: f32, y: f32, expand: f32) -> bool {
    let outer_r = 0.45 + expand;
    let inner_r = 0.20 + expand * 0.5;
    let dist = (x * x + y * y).sqrt();
    if dist > outer_r { return false; }
    let angle = y.atan2(x);
    let n = 5.0;
    let sector = (2.0 * std::f32::consts::PI) / n;
    let half = sector / 2.0;
    // Offset so one point faces up (rotate by 90 degrees)
    let a = (angle + std::f32::consts::FRAC_PI_2).rem_euclid(sector);
    let t = (a - half).abs() / half; // 0 at tip, 1 at valley
    let edge_r = outer_r + t * (inner_r - outer_r);
    dist <= edge_r
}

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

fn in_forbidden_line(x: f32, y: f32) -> bool {
    in_turn_center(x, y) && (x + y).abs() / std::f32::consts::SQRT_2 < 0.035
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

fn in_ring(x: f32, y: f32, e: f32) -> bool {
    let d = (x * x + y * y).sqrt();
    d >= 0.22 - e && d <= 0.44 + e
}

const SEG: [u8; 10] = [0x7E, 0x30, 0x6D, 0x79, 0x33, 0x5B, 0x5F, 0x70, 0x7F, 0x7B];

fn in_7seg(x: f32, y: f32, d: u8, cx: f32) -> bool {
    let (rx, hw, hh, t) = (x - cx, 0.08, 0.14, 0.035);
    let s = SEG[d as usize];
    (s & 0x40 != 0 && rx.abs() < hw && (y - hh).abs() < t)
    || (s & 0x20 != 0 && (rx - hw).abs() < t && y > t / 2.0 && y < hh)
    || (s & 0x10 != 0 && (rx - hw).abs() < t && y > -hh && y < -t / 2.0)
    || (s & 0x08 != 0 && rx.abs() < hw && (y + hh).abs() < t)
    || (s & 0x04 != 0 && (rx + hw).abs() < t && y > -hh && y < -t / 2.0)
    || (s & 0x02 != 0 && (rx + hw).abs() < t && y > t / 2.0 && y < hh)
    || (s & 0x01 != 0 && rx.abs() < hw && y.abs() < t)
}

fn in_tp_num(x: f32, y: f32, num: usize) -> bool {
    let n = num + 1;
    if n < 10 { in_7seg(x, y, n as u8, 0.0) }
    else { in_7seg(x, y, 1, -0.10) || in_7seg(x, y, 0, 0.10) }
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
    forbidden_fn: Option<fn(f32, f32) -> bool>,
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
                if forbidden_fn.is_some_and(|ff| ff(rnx, rny)) { color = SYMBOL_STROKE; }
                else if cf(rnx, rny) { color = center_fill; }
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
    rotated_shape_data(size, border, rotation, fill, in_source_shape, None, [0; 4], None)
}

pub fn goal_texture_colored_data(size: u32, border: u32, fill: [u8; 4]) -> Vec<u8> {
    rotated_shape_data(size, border, 0.0, fill, in_star_shape, None, [0; 4], None)
}

fn turn_center_fill(fill: [u8; 4]) -> [u8; 4] {
    let b = TURN_CENTER_BRIGHTNESS;
    [
        (fill[0] as f32 / 255.0 * b * 255.0) as u8,
        (fill[1] as f32 / 255.0 * b * 255.0) as u8,
        (fill[2] as f32 / 255.0 * b * 255.0) as u8,
        255,
    ]
}

pub fn turn_texture_colored_data(size: u32, border: u32, rotation: f32, fill: [u8; 4]) -> Vec<u8> {
    rotated_shape_data(size, border, rotation, fill, in_turn_shape, Some(in_turn_center), turn_center_fill(fill), None)
}

pub fn turnbut_texture_colored_data(size: u32, border: u32, rotation: f32, fill: [u8; 4]) -> Vec<u8> {
    rotated_shape_data(size, border, rotation, fill, in_turn_shape, Some(in_turn_center), turn_center_fill(fill), Some(in_forbidden_line))
}

fn in_bounce_shape(x: f32, y: f32, e: f32) -> bool { x.abs() + y.abs() < 0.42 + e }

pub fn bounce_texture_colored_data(size: u32, border: u32, fill: [u8; 4], forbid: bool) -> Vec<u8> {
    let ff = if forbid { Some(in_forbidden_line as fn(f32, f32) -> bool) } else { None };
    rotated_shape_data(size, border, 0.0, fill, in_bounce_shape, Some(in_turn_center), turn_center_fill(fill), ff)
}

pub fn teleport_texture_colored_data(size: u32, border: u32, num: usize, fill: [u8; 4]) -> Vec<u8> {
    let c = size as f32 / 2.0;
    let mut data = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let on_edge = px < border || px >= size - border || py < border || py >= size - border;
            let mut color = if on_edge { TILE_DARK } else { TILE_GRAY };
            let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
            if in_ring(nx, ny, 0.0) || in_tp_num(nx, ny, num) { color = fill; }
            else if in_ring(nx, ny, STROKE_EXPAND) { color = SYMBOL_STROKE; }
            let i = ((py * size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    data
}

pub fn create_teleport_tile_textures(
    images: &mut Assets<Image>, size: u32, num: usize,
) -> (Handle<Image>, Handle<Image>) {
    let c = size as f32 / 2.0;
    let mut base = vec![0u8; (size * size * 4) as usize];
    let mut mask = vec![0u8; (size * size * 4) as usize];
    for py in 0..size {
        for px in 0..size {
            let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
            let i = ((py * size + px) * 4) as usize;
            if in_ring(nx, ny, 0.0) || in_tp_num(nx, ny, num) {
                base[i..i + 4].copy_from_slice(&[0, 0, 0, 255]);
                mask[i..i + 4].copy_from_slice(&[255, 255, 255, 255]);
            } else if in_ring(nx, ny, STROKE_EXPAND) {
                base[i..i + 4].copy_from_slice(&SYMBOL_STROKE);
                mask[i..i + 4].copy_from_slice(&[0, 0, 0, 255]);
            }
        }
    }
    let mut mk = |data: Vec<u8>, srgb: bool| {
        let fmt = if srgb { TextureFormat::Rgba8UnormSrgb } else { TextureFormat::Rgba8Unorm };
        images.add(Image::new(
            Extent3d { width: size, height: size, depth_or_array_layers: 1 },
            TextureDimension::D2, data, fmt, default(),
        ))
    };
    (mk(base, true), mk(mask, false))
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
    ui_icon(images, |fx, fy| fx >= 0.3 && fx <= 0.8 && (fy - 0.5).abs() <= 0.3 * (1.0 - (fx - 0.3) / 0.5),
        [80, 200, 80, 255])
}

pub fn create_stop_icon(images: &mut Assets<Image>) -> Handle<Image> {
    ui_icon(images, |fx, fy| fx >= 0.28 && fx <= 0.72 && fy >= 0.28 && fy <= 0.72,
        [220, 60, 60, 255])
}
