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

/// Load a PNG file and return its raw RGBA pixel data + width.
pub fn load_png_raw(path: &str) -> (Vec<u8>, u32) {
    let img = image::open(path).unwrap_or_else(|e| panic!("Failed to load {path}: {e}"));
    let rgba = img.to_rgba8();
    let w = rgba.width();
    (rgba.into_raw(), w)
}

/// Load base+mask PNG pair as raw data.
pub fn load_png_pair(name: &str) -> (Vec<u8>, Vec<u8>, u32) {
    let (base, w) = load_png_raw(&format!("assets/textures/{name}_base.png"));
    let (mask, _) = load_png_raw(&format!("assets/textures/{name}_mask.png"));
    (base, mask, w)
}

/// Composite an icon texture from base+mask PNG data.
/// Point-samples the source PNGs at exact coordinates matching procedural generation.
pub fn composite_icon_from_png(
    base_data: &[u8], mask_data: &[u8], src_size: u32,
    out_size: u32, border: u32, rotation: f32, fill: [u8; 4],
) -> Vec<u8> {
    let center = out_size as f32 / 2.0;
    let src_c = src_size as f32 / 2.0;
    let (cos_r, sin_r) = (rotation.cos(), rotation.sin());
    let mut data = vec![0u8; (out_size * out_size * 4) as usize];
    for py in 0..out_size {
        for px in 0..out_size {
            let on_edge = px < border || px >= out_size - border || py < border || py >= out_size - border;
            let mut color = if on_edge { TILE_DARK } else { TILE_GRAY };
            let nx = (px as f32 - center) / center;
            let ny = (py as f32 - center) / center;
            let rnx = nx * cos_r + ny * sin_r;
            let rny = -nx * sin_r + ny * cos_r;
            let spx = (rnx * src_c + src_c) as i32;
            let spy = (rny * src_c + src_c) as i32;
            if spx >= 0 && spy >= 0 && (spx as u32) < src_size && (spy as u32) < src_size {
                let si = ((spy as u32 * src_size + spx as u32) * 4) as usize;
                if base_data[si + 3] > 0 {
                    if base_data[si] == SYMBOL_STROKE[0] && base_data[si + 1] == SYMBOL_STROKE[1]
                        && base_data[si + 2] == SYMBOL_STROKE[2] {
                        color = SYMBOL_STROKE;
                    } else {
                        let b = mask_data[si] as f32 / 255.0;
                        color = [(fill[0] as f32 * b) as u8, (fill[1] as f32 * b) as u8,
                                 (fill[2] as f32 * b) as u8, 255];
                    }
                }
            }
            let i = ((py * out_size + px) * 4) as usize;
            data[i..i + 4].copy_from_slice(&color);
        }
    }
    data
}

pub fn create_tile_texture(images: &mut Assets<Image>, size: u32, border: u32) -> Handle<Image> {
    make_image(images, tile_texture_data(size, border), size)
}

pub fn color_to_u8(r: f32, g: f32, b: f32) -> [u8; 4] {
    [(r * 255.0).clamp(0.0, 255.0) as u8, (g * 255.0).clamp(0.0, 255.0) as u8,
     (b * 255.0).clamp(0.0, 255.0) as u8, 255]
}

fn ui_icon(images: &mut Assets<Image>, test: impl Fn(f32, f32) -> bool, color: [u8; 4]) -> Handle<Image> {
    ui_icon_bg(images, test, color, ICON_DARK_BG)
}

fn ui_icon_bg(images: &mut Assets<Image>, test: impl Fn(f32, f32) -> bool, color: [u8; 4], bg: [u8; 4]) -> Handle<Image> {
    let mut data = vec![0u8; (ICON_SIZE * ICON_SIZE * 4) as usize];
    for y in 0..ICON_SIZE { for x in 0..ICON_SIZE {
        let (fx, fy) = (x as f32 / ICON_SIZE as f32, y as f32 / ICON_SIZE as f32);
        let c = if test(fx, fy) { color } else { bg };
        let i = ((y * ICON_SIZE + x) * 4) as usize; data[i..i + 4].copy_from_slice(&c);
    }}
    make_image(images, data, ICON_SIZE)
}

pub fn create_delete_icon(images: &mut Assets<Image>) -> Handle<Image> {
    ui_icon_bg(images, |fx, fy| {
        ((fx - fy).abs() < 0.09 || (fx - (1.0 - fy)).abs() < 0.09)
        && fx > 0.15 && fx < 0.85 && fy > 0.15 && fy < 0.85
    }, DELETE_ICON_COLOR, [0, 0, 0, 0])
}

fn create_border_texture(images: &mut Assets<Image>, size: u32, border: u32, edge: [u8; 4]) -> Handle<Image> {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size { for x in 0..size {
        let on_edge = x < border || x >= size - border || y < border || y >= size - border;
        if on_edge { let i = ((y * size + x) * 4) as usize; data[i..i + 4].copy_from_slice(&edge); }
    }}
    make_image(images, data, size)
}

pub fn create_empty_marker_texture(i: &mut Assets<Image>) -> Handle<Image> { create_border_texture(i, MARKER_TEX_SIZE, EMPTY_MARKER_BORDER, EMPTY_MARKER_COLOR) }
pub fn create_highlight_texture(i: &mut Assets<Image>) -> Handle<Image> { create_border_texture(i, MARKER_TEX_SIZE, HIGHLIGHT_TEX_BORDER, HIGHLIGHT_TEX_COLOR) }
pub fn create_inv_marker_texture(i: &mut Assets<Image>) -> Handle<Image> { create_border_texture(i, MARKER_TEX_SIZE, INV_MARKER_BORDER, INV_MARKER_COLOR) }

pub fn tile_texture_data(size: u32, border: u32) -> Vec<u8> {
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size { for x in 0..size {
        let color = if x < border || x >= size - border || y < border || y >= size - border { TILE_DARK } else { TILE_GRAY };
        let i = ((y * size + x) * 4) as usize; data[i..i + 4].copy_from_slice(&color);
    }}
    data
}

// === Isometric icon rendering ===
fn bary(px: f32, py: f32, a: (f32,f32), b: (f32,f32), c: (f32,f32)) -> Option<(f32, f32)> {
    let (d0x, d0y) = (b.0 - a.0, b.1 - a.1);
    let (d1x, d1y) = (c.0 - a.0, c.1 - a.1);
    let (d2x, d2y) = (px - a.0, py - a.1);
    let det = d0x * d1y - d0y * d1x;
    if det.abs() < 1e-10 { return None; }
    let inv = 1.0 / det;
    let s = (d2x * d1y - d2y * d1x) * inv;
    let t = (d0x * d2y - d0y * d2x) * inv;
    if s >= 0.0 && t >= 0.0 && s + t <= 1.0 { Some((s, t)) } else { None }
}

fn point_in_quad_uv(
    px: f32, py: f32, tl: (f32, f32), tr: (f32, f32), br: (f32, f32), bl: (f32, f32),
) -> Option<(f32, f32)> {
    if let Some((s, t)) = bary(px, py, tl, tr, br) { return Some((s + t, t)); }
    if let Some((s, t)) = bary(px, py, tl, br, bl) { return Some((s, s + t)); }
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
    let margin = ISO_MARGIN;
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
                data[i..i + 4].copy_from_slice(&ISO_SIDE_COLOR);
            } else if point_in_quad_uv(x, y, top_bl_px, top_br_px, bot_br_px, bot_bl_px).is_some() {
                data[i..i + 4].copy_from_slice(&ISO_BOTTOM_COLOR);
            }
        }
    }
    make_image(images, data, icon_size)
}

pub fn create_play_icon(images: &mut Assets<Image>) -> Handle<Image> {
    ui_icon(images, |fx, fy| (0.3..=0.8).contains(&fx) && (fy - 0.5).abs() <= 0.3 * (1.0 - (fx - 0.3) / 0.5),
        PLAY_ICON_COLOR)
}

pub fn create_stop_icon(images: &mut Assets<Image>) -> Handle<Image> {
    ui_icon(images, |fx, fy| (0.28..=0.72).contains(&fx) && (0.28..=0.72).contains(&fy),
        STOP_ICON_COLOR)
}

pub fn create_vignette_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let s = VIGNETTE_SIZE;
    let mut data = vec![0u8; (s * s * 4) as usize];
    let c = s as f32 / 2.0;
    for y in 0..s { for x in 0..s {
        let d = (((x as f32 - c) / c).powi(2) + ((y as f32 - c) / c).powi(2)).sqrt();
        let a = ((d - 0.5) / 0.8).clamp(0.0, 1.0);
        let i = ((y * s + x) * 4) as usize;
        data[i + 3] = (a * a * VIGNETTE_ALPHA * 255.0) as u8;
    }}
    make_image(images, data, s)
}
