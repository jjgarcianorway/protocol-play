// SPDX-License-Identifier: GPL-3.0-or-later
//! Render-to-texture system for inventory icons.
//! Spawns each tile variant on an offscreen camera (render layer 1),
//! captures the rendered image, and stores it in InventoryIcons.

use bevy::prelude::*;
use bevy::camera::{RenderTarget, ImageRenderTarget};
use bevy::camera::visibility::RenderLayers;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use crate::constants::*;
use crate::types::*;
use crate::board::camera_direction;

#[derive(Component)] pub struct IconLight;
#[derive(Component)] pub struct IconTile;

/// State machine for the icon render pipeline.
#[derive(Resource)]
pub struct IconRenderState {
    pub queue: Vec<(Handle<Image>, TileKind)>,
    pub index: usize,
    pub current_entity: Option<Entity>,
    pub wait_frames: u8,
    pub render_target: Handle<Image>,
}

/// Create a placeholder transparent icon image.
pub fn create_placeholder(images: &mut Assets<Image>) -> Handle<Image> {
    let size = Extent3d { width: ICON_SIZE, height: ICON_SIZE, depth_or_array_layers: 1 };
    images.add(Image::new_fill(size, TextureDimension::D2, &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb, default()))
}

/// Compute camera position to frame a single 1x1 tile.
fn icon_camera_pos() -> Vec3 {
    camera_direction() * 1.8
}

/// Setup offscreen camera + light on render layer 1, build the render queue.
pub fn setup_icon_render(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    icons: Res<InventoryIcons>,
) {
    let size = Extent3d { width: ICON_SIZE, height: ICON_SIZE, depth_or_array_layers: 1 };
    let mut rt_image = Image::new_fill(size, TextureDimension::D2, &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb, default());
    rt_image.texture_descriptor.usage =
        TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC;
    let render_target = images.add(rt_image);

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderTarget::Image(ImageRenderTarget {
            handle: render_target.clone(), scale_factor: 1.0,
        }),
        Transform::from_translation(icon_camera_pos()).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(1),
        IconCamera,
    ));
    commands.spawn((
        DirectionalLight { illuminance: LIGHT_ILLUMINANCE, shadows_enabled: false, ..default() },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ, LIGHT_ELEVATION, LIGHT_AZIMUTH, 0.0)),
        RenderLayers::layer(1),
        IconLight,
    ));

    let queue = build_render_queue(&icons);
    info!("Icon render queue: {} icons to render", queue.len());
    commands.insert_resource(IconRenderState {
        queue, index: 0, current_entity: None, wait_frames: 0, render_target,
    });
}

/// Build the full list of (target_handle, TileKind) for all icons.
fn build_render_queue(icons: &InventoryIcons) -> Vec<(Handle<Image>, TileKind)> {
    let mut q: Vec<(Handle<Image>, TileKind)> = Vec::with_capacity(320);

    q.push((icons.floor.clone(), TileKind::Floor));
    q.push((icons.source.clone(), TileKind::Source(0, Direction::North)));
    q.push((icons.goal.clone(), TileKind::Goal(0)));
    q.push((icons.turn.clone(), TileKind::Turn(0, Direction::North)));
    q.push((icons.turnbut.clone(), TileKind::TurnBut(0, Direction::North)));

    for d in Direction::all() {
        q.push((icons.source_dir_icons[d.index()].clone(), TileKind::Source(0, d)));
    }
    for ci in 0..NUM_COLORS {
        for d in Direction::all() {
            q.push((icons.source_color_icons[ci * 4 + d.index()].clone(),
                TileKind::Source(ci, d)));
        }
    }
    for ci in 0..NUM_COLORS {
        q.push((icons.goal_color_icons[ci].clone(), TileKind::Goal(ci)));
    }
    for d in Direction::all() {
        q.push((icons.turn_dir_icons[d.index()].clone(), TileKind::Turn(0, d)));
    }
    for ci in 0..NUM_COLORS {
        for d in Direction::all() {
            q.push((icons.turn_color_icons[ci * 4 + d.index()].clone(),
                TileKind::Turn(ci, d)));
        }
    }
    for d in Direction::all() {
        q.push((icons.turn_color_icons[NUM_COLORS * 4 + d.index()].clone(),
            TileKind::Turn(NUM_COLORS, d)));
    }
    for d in Direction::all() {
        q.push((icons.turnbut_dir_icons[d.index()].clone(), TileKind::TurnBut(0, d)));
    }
    for ci in 0..NUM_COLORS {
        for d in Direction::all() {
            q.push((icons.turnbut_color_icons[ci * 4 + d.index()].clone(),
                TileKind::TurnBut(ci, d)));
        }
    }
    q.push((icons.teleport.clone(), TileKind::Teleport(0, 0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.teleport_color_icons[ci].clone(), TileKind::Teleport(ci, 0)));
    }
    q.push((icons.teleport_color_icons[NUM_COLORS].clone(), TileKind::Teleport(NUM_COLORS, 0)));
    q.push((icons.teleportbut.clone(), TileKind::TeleportBut(0, 0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.teleportbut_color_icons[ci].clone(), TileKind::TeleportBut(ci, 0)));
    }
    q.push((icons.bounce.clone(), TileKind::Bounce(0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.bounce_color_icons[ci].clone(), TileKind::Bounce(ci)));
    }
    q.push((icons.bounce_color_icons[NUM_COLORS].clone(), TileKind::Bounce(NUM_COLORS)));
    q.push((icons.bouncebot.clone(), TileKind::BounceBut(0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.bouncebot_color_icons[ci].clone(), TileKind::BounceBut(ci)));
    }
    q.push((icons.door.clone(), TileKind::Door(false)));
    q.push((icons.door_open.clone(), TileKind::Door(true)));
    q.push((icons.door_closed.clone(), TileKind::Door(false)));
    q.push((icons.switch.clone(), TileKind::Switch));
    for ci in 0..NUM_COLORS {
        q.push((icons.switch_color_icons[ci].clone(), TileKind::ColorSwitch(ci)));
    }
    q.push((icons.switch_color_icons[NUM_COLORS].clone(), TileKind::Switch));
    q.push((icons.switchbut.clone(), TileKind::ColorSwitchBut(0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.switchbut_color_icons[ci].clone(), TileKind::ColorSwitchBut(ci)));
    }
    q.push((icons.painter.clone(), TileKind::Painter(0)));
    for ci in 0..NUM_COLORS {
        q.push((icons.painter_color_icons[ci].clone(), TileKind::Painter(ci)));
    }
    q.push((icons.arrow.clone(), TileKind::Arrow(0, Direction::North)));
    for d in Direction::all() {
        q.push((icons.arrow_dir_icons[d.index()].clone(), TileKind::Arrow(0, d)));
    }
    for ci in 0..NUM_COLORS {
        for d in Direction::all() {
            q.push((icons.arrow_color_icons[ci * 4 + d.index()].clone(),
                TileKind::Arrow(ci, d)));
        }
    }
    for d in Direction::all() {
        q.push((icons.arrow_color_icons[NUM_COLORS * 4 + d.index()].clone(),
            TileKind::Arrow(NUM_COLORS, d)));
    }
    q.push((icons.arrowbut.clone(), TileKind::ArrowBut(0, Direction::North)));
    for d in Direction::all() {
        q.push((icons.arrowbut_dir_icons[d.index()].clone(), TileKind::ArrowBut(0, d)));
    }
    for ci in 0..NUM_COLORS {
        for d in Direction::all() {
            q.push((icons.arrowbut_color_icons[ci * 4 + d.index()].clone(),
                TileKind::ArrowBut(ci, d)));
        }
    }
    q
}

/// Spawn a tile for icon rendering on layer 1 (no TileCoord/animation).
fn spawn_icon_tile(commands: &mut Commands, kind: TileKind, assets: &GameAssets) -> Entity {
    let layer = RenderLayers::layer(1);
    let floor_transform = match kind {
        TileKind::Source(_, dir) | TileKind::Turn(_, dir) | TileKind::TurnBut(_, dir)
        | TileKind::Arrow(_, dir) | TileKind::ArrowBut(_, dir) =>
            Transform::from_rotation(Quat::from_rotation_y(dir.rotation())),
        _ => Transform::default(),
    };

    let entity = commands.spawn((
        Mesh3d(assets.floor_mesh.clone()),
        MeshMaterial3d(assets.floor_material.clone()),
        floor_transform, layer.clone(), IconTile,
    )).id();

    if let Some(mat) = symbol_material(&kind, assets) {
        let child = commands.spawn((
            Mesh3d(symbol_mesh(&kind, assets)),
            MeshMaterial3d(mat),
            Transform::from_translation(Vec3::new(0.0, FLOOR_TOP_Y + SYMBOL_OVERLAY_OFFSET, 0.0)),
            layer,
        )).id();
        commands.entity(entity).add_child(child);
    }
    entity
}

fn symbol_material(kind: &TileKind, a: &GameAssets) -> Option<Handle<StandardMaterial>> {
    match kind {
        TileKind::Floor | TileKind::Empty => None,
        TileKind::Source(ci, _) => a.source_symbol_materials.get(*ci).cloned(),
        TileKind::Goal(ci) => a.goal_symbol_materials.get(*ci).cloned(),
        TileKind::Turn(ci, _) => a.turn_symbol_materials.get(*ci).cloned(),
        TileKind::TurnBut(ci, _) => a.turnbut_symbol_materials.get(*ci).cloned(),
        TileKind::Teleport(ci, num) => {
            let idx = num * NUM_TELEPORT_COLORS + (*ci).min(NUM_TELEPORT_COLORS - 1);
            a.teleport_symbol_materials.get(idx).cloned()
        }
        TileKind::TeleportBut(ci, num) => {
            let idx = num * NUM_COLORS + (*ci).min(NUM_COLORS - 1);
            a.teleportbut_symbol_materials.get(idx).cloned()
        }
        TileKind::Bounce(ci) => a.bounce_symbol_materials.get(*ci).cloned(),
        TileKind::BounceBut(ci) => a.bouncebot_symbol_materials.get(*ci).cloned(),
        TileKind::Door(open) => Some(if *open { a.door_open_material.clone() }
            else { a.door_closed_material.clone() }),
        TileKind::Switch => Some(a.switch_material.clone()),
        TileKind::ColorSwitch(ci) => a.colorswitch_symbol_materials.get(*ci).cloned(),
        TileKind::ColorSwitchBut(ci) => a.colorswitchbut_symbol_materials.get(*ci).cloned(),
        TileKind::Painter(ci) => a.painter_symbol_materials.get(*ci).cloned(),
        TileKind::Arrow(ci, _) => a.arrow_symbol_materials.get(*ci).cloned(),
        TileKind::ArrowBut(ci, _) => a.arrowbut_symbol_materials.get(*ci).cloned(),
    }
}

fn symbol_mesh(kind: &TileKind, a: &GameAssets) -> Handle<Mesh> {
    match kind {
        TileKind::Source(_, _) => a.source_symbol_mesh.clone(),
        TileKind::Turn(_, _) => a.turn_symbol_mesh.clone(),
        TileKind::TurnBut(_, _) => a.turnbut_symbol_mesh.clone(),
        TileKind::Arrow(_, _) => a.arrow_symbol_mesh.clone(),
        TileKind::ArrowBut(_, _) => a.arrowbut_symbol_mesh.clone(),
        _ => a.goal_symbol_mesh.clone(),
    }
}

/// Per-frame system: spawn one tile, wait for render, capture, despawn, repeat.
pub fn update_icon_render(
    mut commands: Commands,
    mut state: ResMut<IconRenderState>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<GameAssets>,
    camera_q: Query<Entity, With<IconCamera>>,
    light_q: Query<Entity, With<IconLight>>,
) {
    // Done?
    if state.index >= state.queue.len() && state.current_entity.is_none() {
        for e in &camera_q { commands.entity(e).despawn(); }
        for e in &light_q { commands.entity(e).despawn(); }
        commands.remove_resource::<IconRenderState>();
        info!("Icon rendering complete ({} icons)", state.queue.len());
        return;
    }

    // Waiting for current tile to render?
    if let Some(entity) = state.current_entity {
        if state.wait_frames > 0 {
            state.wait_frames -= 1;
            return;
        }
        // Capture: copy render target data to the icon handle
        let rt = state.render_target.clone();
        if let Some(rendered) = images.get(&rt) {
            let data = rendered.data.clone();
            let idx = state.index - 1;
            let target = &state.queue[idx].0;
            if let Some(img) = images.get_mut(target) {
                img.data = data;
            }
        }
        commands.entity(entity).despawn();
        state.current_entity = None;
    }

    // Spawn next tile
    if state.index < state.queue.len() {
        let kind = state.queue[state.index].1;
        let entity = spawn_icon_tile(&mut commands, kind, &assets);
        state.current_entity = Some(entity);
        state.index += 1;
        state.wait_frames = 2; // wait 2 frames for render pipeline
    }
}
