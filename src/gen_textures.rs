// SPDX-License-Identifier: GPL-3.0-or-later
//
// Generates symbol textures and saves them as PNG files in assets/textures/.
// The game loads these files at runtime, so you can edit the PNGs to customize appearance.

use image::{RgbaImage, Rgba};
use std::path::Path;
use crate::constants::*;

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

fn in_turn_center(x: f32, y: f32) -> bool { (x * x + y * y).sqrt() < 0.14 }

fn in_star_shape(x: f32, y: f32, expand: f32) -> bool {
    let outer_r = 0.45 + expand;
    let inner_r = 0.20 + expand * 0.5;
    let dist = (x * x + y * y).sqrt();
    if dist > outer_r { return false; }
    let angle = y.atan2(x);
    let n = 5.0;
    let sector = (2.0 * std::f32::consts::PI) / n;
    let half = sector / 2.0;
    let a = (angle + std::f32::consts::FRAC_PI_2).rem_euclid(sector);
    let t = (a - half).abs() / half;
    let edge_r = outer_r + t * (inner_r - outer_r);
    dist <= edge_r
}

fn in_forbidden_line(x: f32, y: f32) -> bool {
    in_turn_center(x, y) && (x + y).abs() / std::f32::consts::SQRT_2 < 0.035
}

fn in_bounce_shape(x: f32, y: f32, e: f32) -> bool { x.abs() + y.abs() < 0.42 + e }

fn in_door_sq(x: f32, y: f32, e: f32) -> bool {
    let (s, o) = (0.12 + e, 0.30);
    ((x-o).abs() < s && (y-o).abs() < s) || ((x+o).abs() < s && (y-o).abs() < s)
    || ((x-o).abs() < s && (y+o).abs() < s) || ((x+o).abs() < s && (y+o).abs() < s)
}
fn in_door_closed_s(x: f32, y: f32, e: f32) -> bool {
    in_door_sq(x, y, e) || (((x-y).abs() < 0.06+e || (x+y).abs() < 0.06+e) && x.abs() < 0.38+e && y.abs() < 0.38+e)
}
fn in_switch_s(x: f32, y: f32, e: f32) -> bool { (x*x + y*y).sqrt() < 0.25 + e }

fn in_brush_shape(x: f32, y: f32, e: f32) -> bool {
    (x.abs() < 0.07 + e && y > -0.05 && y < 0.55 + e)
    || (x.abs() < 0.12 + e && y > -0.15 - e && y < 0.0 + e)
    || (x.abs() < 0.22 + e && y > -0.50 - e && y < -0.10 + e)
}

fn in_arrow_shape(x: f32, y: f32, e: f32) -> bool {
    (x.abs() < 0.07 + e && y > -0.15 - e && y < 0.50 + e)
    || (y >= -0.60 - e && y < -0.10 + e && x.abs() < 0.30 * (1.0 - ((y + 0.10) / -0.50).clamp(0.0, 1.0)) + e)
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

// === Teleport shapes (ring + 7-segment numbers) ===
fn in_ring(x: f32, y: f32, e: f32) -> bool {
    let d = (x * x + y * y).sqrt();
    d >= 0.22 - e && d <= 0.44 + e
}

const SEG: [u8; 10] = [0x7E, 0x30, 0x6D, 0x79, 0x33, 0x5B, 0x5F, 0x70, 0x7F, 0x7B];

fn in_7seg(x: f32, y: f32, d: u8, cx: f32) -> bool {
    let y = -y;
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
    if n < 10 { in_7seg(x, y, n as u8, 0.0) } else { in_7seg(x, y, 1, -0.10) || in_7seg(x, y, 0, 0.10) }
}

fn in_teleport_shape(x: f32, y: f32, e: f32, num: usize) -> bool {
    in_ring(x, y, e) || (e == 0.0 && in_tp_num(x, y, num))
}

fn generate_symbol_textures(
    size: u32, dir: &Path, name: &str,
    shape_fn: fn(f32, f32, f32) -> bool,
    center_fn: Option<fn(f32, f32) -> bool>, center_brightness: f32,
    forbidden_fn: Option<fn(f32, f32) -> bool>,
) {
    let c = size as f32 / 2.0;
    let mut base = RgbaImage::new(size, size);
    let mut mask = RgbaImage::new(size, size);
    for py in 0..size {
        for px in 0..size {
            let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
            if forbidden_fn.is_some_and(|ff| ff(nx, ny)) {
                base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
            } else if center_fn.is_some_and(|cf| cf(nx, ny)) {
                base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                let b = (center_brightness * 255.0) as u8;
                mask.put_pixel(px, py, Rgba([b, b, b, 255]));
            } else if shape_fn(nx, ny, 0.0) {
                base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                mask.put_pixel(px, py, Rgba([255, 255, 255, 255]));
            } else if shape_fn(nx, ny, STROKE_EXPAND) {
                base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
            }
        }
    }
    base.save(dir.join(format!("{name}_base.png"))).expect("Failed to save base texture");
    mask.save(dir.join(format!("{name}_mask.png"))).expect("Failed to save mask texture");
}

fn generate_teleport_textures(size: u32, dir: &Path) {
    let c = size as f32 / 2.0;
    for num in 0..NUM_TELEPORTS {
        let mut base = RgbaImage::new(size, size);
        let mut mask = RgbaImage::new(size, size);
        for py in 0..size {
            for px in 0..size {
                let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
                if in_teleport_shape(nx, ny, 0.0, num) {
                    base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                    mask.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                } else if in_ring(nx, ny, STROKE_EXPAND) {
                    base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                    mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                }
            }
        }
        base.save(dir.join(format!("teleport_{num}_base.png"))).expect("Failed to save teleport base");
        mask.save(dir.join(format!("teleport_{num}_mask.png"))).expect("Failed to save teleport mask");
    }
}

fn generate_teleportbut_textures(size: u32, dir: &Path) {
    let c = size as f32 / 2.0;
    for num in 0..NUM_TELEPORTS {
        let mut base = RgbaImage::new(size, size);
        let mut mask = RgbaImage::new(size, size);
        for py in 0..size {
            for px in 0..size {
                let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
                if in_forbidden_line(nx, ny) {
                    base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                    mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                } else if in_turn_center(nx, ny) {
                    base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                    let b = (TURN_CENTER_BRIGHTNESS * 255.0) as u8;
                    mask.put_pixel(px, py, Rgba([b, b, b, 255]));
                } else if in_teleport_shape(nx, ny, 0.0, num) {
                    base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                    mask.put_pixel(px, py, Rgba([255, 255, 255, 255]));
                } else if in_ring(nx, ny, STROKE_EXPAND) {
                    base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                    mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                }
            }
        }
        base.save(dir.join(format!("teleportbut_{num}_base.png"))).expect("save teleportbut base");
        mask.save(dir.join(format!("teleportbut_{num}_mask.png"))).expect("save teleportbut mask");
    }
}

fn generate_floor_texture(size: u32, border: u32, dir: &Path) {
    let mut img = RgbaImage::new(size, size);
    for y in 0..size { for x in 0..size {
        let on_edge = x < border || x >= size - border || y < border || y >= size - border;
        img.put_pixel(x, y, Rgba(if on_edge { TILE_DARK } else { TILE_GRAY }));
    }}
    img.save(dir.join("floor.png")).expect("Failed to save floor texture");
}

/// Generates all symbol textures as PNG files in assets/textures/.
/// Call once at startup; the PNGs can then be edited by hand.
pub fn ensure_textures() {
    let dir = Path::new("assets/textures");
    let names = ["source", "turn", "goal", "turnbut", "bounce", "bouncebut",
        "door_open", "door_closed", "switch", "painter", "arrow", "arrowbut",
        "colorswitch", "colorswitchbut"];
    let all_exist = names.iter().all(|n| dir.join(format!("{n}_base.png")).exists() && dir.join(format!("{n}_mask.png")).exists())
        && (0..NUM_TELEPORTS).all(|n| dir.join(format!("teleport_{n}_base.png")).exists() && dir.join(format!("teleport_{n}_mask.png")).exists())
        && (0..NUM_TELEPORTS).all(|n| dir.join(format!("teleportbut_{n}_base.png")).exists() && dir.join(format!("teleportbut_{n}_mask.png")).exists())
        && dir.join("floor.png").exists();
    if all_exist { return; }
    std::fs::create_dir_all(dir).expect("Failed to create textures directory");
    generate_symbol_textures(TILE_TEX_SIZE, dir, "source", in_source_shape, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "turn", in_turn_shape, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "goal", in_star_shape, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "turnbut", in_turn_shape, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, Some(in_forbidden_line));
    generate_symbol_textures(TILE_TEX_SIZE, dir, "bounce", in_bounce_shape, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "bouncebut", in_bounce_shape, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, Some(in_forbidden_line));
    generate_symbol_textures(TILE_TEX_SIZE, dir, "door_open", in_door_sq, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "door_closed", in_door_closed_s, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "switch", in_switch_s, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "painter", in_brush_shape, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "arrow", in_arrow_shape, None, 0.0, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "arrowbut", in_arrow_shape, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, Some(in_forbidden_line));
    generate_symbol_textures(TILE_TEX_SIZE, dir, "colorswitch", in_switch_s, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, None);
    generate_symbol_textures(TILE_TEX_SIZE, dir, "colorswitchbut", in_switch_s, Some(in_turn_center), TURN_CENTER_BRIGHTNESS, Some(in_forbidden_line));
    generate_teleport_textures(TILE_TEX_SIZE, dir);
    generate_teleportbut_textures(TILE_TEX_SIZE, dir);
    generate_floor_texture(TILE_TEX_SIZE, TILE_TEX_BORDER, dir);
}
