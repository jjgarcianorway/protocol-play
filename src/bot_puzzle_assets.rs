// SPDX-License-Identifier: GPL-3.0-or-later
//! Bot puzzle asset creation — extracted from setup_scene for reuse in integrated mode.

use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::textures::*;
use crate::mat_helpers::*;
use crate::ui_helpers::rgba;

/// Create all GameAssets (materials, meshes) needed by the bot puzzle.
/// This is the same logic as setup_scene but returns assets without spawning entities.
pub fn create_bot_puzzle_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> GameAssets {
    let floor_texture = create_tile_texture(images, TILE_TEX_SIZE, TILE_TEX_BORDER);
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture.clone()),
        base_color: Color::srgb(FLOOR_TINT.0, FLOOR_TINT.1, FLOOR_TINT.2),
        perceptual_roughness: 0.6, ..default()
    });
    let floor_mesh = meshes.add(Cuboid::new(1.0, TILE_HEIGHT, 1.0));
    let ghost_floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture),
        base_color: Color::srgba(1.0, 1.0, 1.0, GHOST_ALPHA),
        alpha_mode: AlphaMode::Blend, ..default()
    });
    let ghost_delete_material = materials.add(StandardMaterial {
        base_color: rgba(DELETE_OVERLAY_COLOR),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let ghost_delete_mesh = meshes.add(Cuboid::new(1.02, OVERLAY_MESH_THICKNESS, 1.02));
    let overlay_tex_mat = |mats: &mut Assets<StandardMaterial>, tex| mats.add(StandardMaterial {
        base_color_texture: Some(tex),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });
    let empty_material = overlay_tex_mat(materials, create_empty_marker_texture(images));
    let empty_mesh = meshes.add(Cuboid::new(0.95, OVERLAY_MESH_THICKNESS, 0.95));
    let highlight_material = overlay_tex_mat(materials, create_highlight_texture(images));
    let highlight_mesh = meshes.add(Cuboid::new(1.05, OVERLAY_MESH_THICKNESS, 1.05));
    let marker_material = overlay_tex_mat(materials, create_inv_marker_texture(images));
    let marker_mesh = meshes.add(Cuboid::new(1.03, OVERLAY_MESH_THICKNESS, 1.03));
    let sym_mesh = meshes.add(Cuboid::new(0.99, OVERLAY_MESH_THICKNESS, 0.99));

    let (source_symbol_materials, ghost_symbol_materials, _, _) =
        load_tile_mats(materials, images, "source");
    let (goal_symbol_materials, ghost_goal_materials, _, _) =
        load_tile_mats(materials, images, "goal");
    let (mut turn_symbol_materials, mut ghost_turn_materials, tb, tm) =
        load_tile_mats(materials, images, "turn");
    add_grey_mat(materials, &mut turn_symbol_materials, &mut ghost_turn_materials, &tb, &tm);
    let (turnbut_symbol_materials, ghost_turnbut_materials, _, _) =
        load_but_tile_mats(materials, images, "turnbut");
    let (mut bounce_symbol_materials, mut ghost_bounce_materials, bb, bm) =
        load_tile_mats(materials, images, "bounce");
    add_grey_mat(materials, &mut bounce_symbol_materials, &mut ghost_bounce_materials, &bb, &bm);
    let (bouncebot_symbol_materials, ghost_bouncebot_materials, _, _) =
        load_but_tile_mats(materials, images, "bouncebut");

    let load_grey = |mats: &mut Assets<StandardMaterial>, imgs: &mut Assets<Image>, name: &str| {
        let b = load_png_texture(imgs, &format!("assets/textures/{name}_base.png"), true);
        let m = load_png_texture(imgs, &format!("assets/textures/{name}_mask.png"), false);
        make_grey_mat(mats, b, m)
    };

    let (mut teleport_symbol_materials, mut ghost_teleport_materials) = (Vec::new(), Vec::new());
    let (mut teleportbut_symbol_materials, mut ghost_teleportbut_materials) = (Vec::new(), Vec::new());
    for num in 0..NUM_TELEPORTS {
        let (mut ms, mut gs, b, m) = load_tile_mats(materials, images, &format!("teleport_{num}"));
        add_grey_mat(materials, &mut ms, &mut gs, &b, &m);
        teleport_symbol_materials.extend(ms);
        ghost_teleport_materials.extend(gs);
        let (ms2, gs2, _, _) = load_but_tile_mats(materials, images, &format!("teleportbut_{num}"));
        teleportbut_symbol_materials.extend(ms2);
        ghost_teleportbut_materials.extend(gs2);
    }

    let (door_open_material, ghost_door_open_material) = load_grey(materials, images, "door_open");
    let (door_closed_material, ghost_door_closed_material) = load_grey(materials, images, "door_closed");
    let (switch_material, ghost_switch_material) = load_grey(materials, images, "switch");

    let (colorswitch_symbol_materials, ghost_colorswitch_materials, _, _) =
        load_tile_mats(materials, images, "colorswitch");
    let (colorswitchbut_symbol_materials, ghost_colorswitchbut_materials, _, _) =
        load_but_tile_mats(materials, images, "colorswitchbut");
    let (painter_symbol_materials, ghost_painter_materials, _, _) =
        load_tile_mats(materials, images, "painter");
    let (mut arrow_symbol_materials, mut ghost_arrow_materials, ab, am) =
        load_tile_mats(materials, images, "arrow");
    add_grey_mat(materials, &mut arrow_symbol_materials, &mut ghost_arrow_materials, &ab, &am);
    let (arrowbut_symbol_materials, ghost_arrowbut_materials, _, _) =
        load_but_tile_mats(materials, images, "arrowbut");

    let bot_mesh = meshes.add(Cuboid::new(BOT_SIZE, BOT_SIZE, BOT_SIZE));
    let eye_mesh = meshes.add(Cuboid::new(BOT_EYE_W, BOT_EYE_H, BOT_EYE_D));
    let eye_material = materials.add(StandardMaterial {
        base_color: Color::WHITE, unlit: true, ..default()
    });
    let bot_materials: Vec<_> = SOURCE_COLORS.iter().map(|&(r, g, b)|
        materials.add(StandardMaterial { base_color: Color::srgb(r, g, b), ..default() })
    ).collect();
    let flash_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.6),
        alpha_mode: AlphaMode::Blend, unlit: true, ..default()
    });

    GameAssets {
        floor_mesh, floor_material,
        empty_mesh, empty_material,
        ghost_floor_material,
        ghost_delete_mesh, ghost_delete_material,
        highlight_mesh, highlight_material,
        source_symbol_mesh: sym_mesh.clone(), source_symbol_materials,
        ghost_symbol_materials,
        goal_symbol_mesh: sym_mesh.clone(), goal_symbol_materials,
        ghost_goal_materials,
        turn_symbol_mesh: sym_mesh.clone(), turn_symbol_materials,
        ghost_turn_materials,
        turnbut_symbol_mesh: sym_mesh.clone(), turnbut_symbol_materials,
        ghost_turnbut_materials,
        teleport_symbol_materials, ghost_teleport_materials,
        teleportbut_symbol_materials, ghost_teleportbut_materials,
        bounce_symbol_materials, ghost_bounce_materials,
        bouncebot_symbol_materials, ghost_bouncebot_materials,
        door_open_material, door_closed_material,
        ghost_door_open_material, ghost_door_closed_material,
        switch_material, ghost_switch_material,
        colorswitch_symbol_materials, ghost_colorswitch_materials,
        colorswitchbut_symbol_materials, ghost_colorswitchbut_materials,
        painter_symbol_materials, ghost_painter_materials,
        arrow_symbol_mesh: sym_mesh.clone(), arrow_symbol_materials,
        ghost_arrow_materials,
        arrowbut_symbol_mesh: sym_mesh, arrowbut_symbol_materials,
        ghost_arrowbut_materials,
        marker_mesh, marker_material,
        bot_mesh, eye_mesh, bot_materials, eye_material, flash_material,
    }
}
