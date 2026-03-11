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

fn in_turn_center(x: f32, y: f32) -> bool {
    (x * x + y * y).sqrt() < 0.14
}

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

fn generate_symbol_textures(
    size: u32, dir: &Path,
    name: &str,
    shape_fn: fn(f32, f32, f32) -> bool,
    center_fn: Option<fn(f32, f32) -> bool>,
    center_brightness: f32,
    forbidden_fn: Option<fn(f32, f32) -> bool>,
) {
    let c = size as f32 / 2.0;
    let mut base = RgbaImage::new(size, size);
    let mut mask = RgbaImage::new(size, size);

    for py in 0..size {
        for px in 0..size {
            let (nx, ny) = ((px as f32 - c) / c, (py as f32 - c) / c);
            if forbidden_fn.is_some_and(|ff| ff(nx, ny)) {
                // Forbidden line: stroke color in base, black in mask (no color)
                base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
            } else if center_fn.is_some_and(|cf| cf(nx, ny)) {
                // Center: black in base (opaque), dark gray in mask
                base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                let b = (center_brightness * 255.0) as u8;
                mask.put_pixel(px, py, Rgba([b, b, b, 255]));
            } else if shape_fn(nx, ny, 0.0) {
                // Fill: black in base (opaque), white in mask
                base.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                mask.put_pixel(px, py, Rgba([255, 255, 255, 255]));
            } else if shape_fn(nx, ny, STROKE_EXPAND) {
                // Stroke: dark gray in base, black in mask (no color applied)
                base.put_pixel(px, py, Rgba(SYMBOL_STROKE));
                mask.put_pixel(px, py, Rgba([0, 0, 0, 255]));
            }
            // else: both stay transparent [0,0,0,0]
        }
    }

    base.save(dir.join(format!("{name}_base.png"))).expect("Failed to save base texture");
    mask.save(dir.join(format!("{name}_mask.png"))).expect("Failed to save mask texture");
}

fn generate_floor_texture(size: u32, border: u32, dir: &Path) {
    let mut img = RgbaImage::new(size, size);
    for y in 0..size {
        for x in 0..size {
            let on_edge = x < border || x >= size - border || y < border || y >= size - border;
            let color = if on_edge { TILE_DARK } else { TILE_GRAY };
            img.put_pixel(x, y, Rgba(color));
        }
    }
    img.save(dir.join("floor.png")).expect("Failed to save floor texture");
}

/// Generates all symbol textures as PNG files in assets/textures/.
/// Call once at startup; the PNGs can then be edited by hand.
pub fn ensure_textures() {
    let dir = Path::new("assets/textures");
    // Skip if textures already exist (user may have edited them)
    if dir.join("source_base.png").exists()
        && dir.join("source_mask.png").exists()
        && dir.join("turn_base.png").exists()
        && dir.join("turn_mask.png").exists()
        && dir.join("goal_base.png").exists()
        && dir.join("goal_mask.png").exists()
        && dir.join("turnbut_base.png").exists()
        && dir.join("turnbut_mask.png").exists()
        && dir.join("bounce_base.png").exists()
        && dir.join("bounce_mask.png").exists()
        && dir.join("bouncebut_base.png").exists()
        && dir.join("bouncebut_mask.png").exists()
        && dir.join("door_open_base.png").exists()
        && dir.join("door_open_mask.png").exists()
        && dir.join("door_closed_base.png").exists()
        && dir.join("door_closed_mask.png").exists()
        && dir.join("switch_base.png").exists()
        && dir.join("switch_mask.png").exists()
        && dir.join("painter_base.png").exists()
        && dir.join("painter_mask.png").exists()
        && dir.join("arrow_base.png").exists()
        && dir.join("arrow_mask.png").exists()
        && dir.join("arrowbut_base.png").exists()
        && dir.join("arrowbut_mask.png").exists()
        && dir.join("colorswitch_base.png").exists()
        && dir.join("colorswitch_mask.png").exists()
        && dir.join("colorswitchbut_base.png").exists()
        && dir.join("colorswitchbut_mask.png").exists()
        && dir.join("floor.png").exists()
    {
        return;
    }
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
    generate_floor_texture(TILE_TEX_SIZE, TILE_TEX_BORDER, dir);
}
