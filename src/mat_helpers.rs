// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use crate::constants::*;
use crate::textures::load_png_texture;

pub fn load_tile_mats(
    materials: &mut Assets<StandardMaterial>, images: &mut Assets<Image>, name: &str,
) -> (Vec<Handle<StandardMaterial>>, Vec<Handle<StandardMaterial>>, Handle<Image>, Handle<Image>) {
    let base = load_png_texture(images, &format!("assets/textures/{name}_base.png"), true);
    let mask = load_png_texture(images, &format!("assets/textures/{name}_mask.png"), false);
    let mats = SOURCE_COLORS.iter().map(|&(r, g, b)| materials.add(StandardMaterial {
        base_color: Color::WHITE, base_color_texture: Some(base.clone()),
        alpha_mode: AlphaMode::Mask(0.5), emissive: LinearRgba::from(Color::srgb(r, g, b)),
        emissive_texture: Some(mask.clone()), reflectance: 0.0, ..default()
    })).collect();
    let ghosts = SOURCE_COLORS.iter().map(|&(r, g, b)| materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, GHOST_ALPHA), base_color_texture: Some(mask.clone()),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    })).collect();
    (mats, ghosts, base, mask)
}

pub fn make_grey_mat(
    materials: &mut Assets<StandardMaterial>, base: Handle<Image>, mask: Handle<Image>,
) -> (Handle<StandardMaterial>, Handle<StandardMaterial>) {
    let (r, g, b) = GREY_COLOR;
    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE, base_color_texture: Some(base),
        alpha_mode: AlphaMode::Mask(0.5), emissive: LinearRgba::from(Color::srgb(r, g, b)),
        emissive_texture: Some(mask.clone()), reflectance: 0.0, ..default()
    });
    let ghost = materials.add(StandardMaterial {
        base_color: Color::srgba(r, g, b, GHOST_ALPHA), base_color_texture: Some(mask),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    (mat, ghost)
}

pub fn add_grey_mat(
    materials: &mut Assets<StandardMaterial>, mats: &mut Vec<Handle<StandardMaterial>>,
    ghosts: &mut Vec<Handle<StandardMaterial>>, base: &Handle<Image>, mask: &Handle<Image>,
) {
    let (m, g) = make_grey_mat(materials, base.clone(), mask.clone());
    mats.push(m); ghosts.push(g);
}
