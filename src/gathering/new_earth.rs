// SPDX-License-Identifier: GPL-3.0-or-later

//! New Earth cutscene — plays when the player has completed all 149 Bot Game levels.
//! A peaceful, emotional ending: no asteroids, just stars, and a planet appearing.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::game_over::FadeOverlay;

// === Resources ===

/// Marker resource: present only when the New Earth cutscene is active.
#[derive(Resource)]
pub struct NewEarthMode;

/// Timeline timer for the cutscene sequence.
#[derive(Resource)]
pub struct NewEarthTimer(pub f32);

// === Components ===

#[derive(Component)]
pub struct EarthSphere;

#[derive(Component)]
pub struct AtmosphereSphere;

#[derive(Component)]
pub struct NewEarthText;

#[derive(Component)]
pub struct WhiteFadeOverlay;

// === Constants ===

const EARTH_APPEAR_TIME: f32 = 5.0;
const EARTH_VISIBLE_TIME: f32 = 10.0;
const EARTH_LARGE_TIME: f32 = 20.0;
const FADE_WHITE_START: f32 = 25.0;
const FADE_WHITE_END: f32 = 28.0;
const TEXT_APPEAR_TIME: f32 = 28.0;
const TEXT_HOLD_END: f32 = 35.0;
const EXIT_TIME: f32 = 35.5;

const EARTH_COLOR: (f32, f32, f32) = (0.15, 0.45, 0.7);
const EARTH_EMISSIVE_MULT: f32 = 1.5;
const EARTH_START_Y: f32 = 120.0;
const EARTH_START_SCALE: f32 = 0.3;
const EARTH_END_SCALE: f32 = 12.0;
const EARTH_ROT_SPEED: f32 = 0.08;

const ATMO_COLOR: (f32, f32, f32, f32) = (0.4, 0.65, 1.0, 0.12);
const ATMO_SCALE_MULT: f32 = 1.15;

const STAR_SLOW_FACTOR: f32 = 0.3;

const TEXT_FONT_SIZE: f32 = 36.0;
const TEXT_COLOR: (f32, f32, f32) = (1.0, 0.98, 0.92);

// === Systems ===

/// Check save state on startup and insert NewEarthMode if qualified.
pub fn check_new_earth_mode(mut commands: Commands) {
    let gs = crate::save_state::load_game_state();
    if gs.bot_level >= 149 {
        commands.insert_resource(NewEarthMode);
        commands.insert_resource(NewEarthTimer(0.0));
    }
}

/// Advance the cutscene timer.
pub fn tick_new_earth_timer(
    mut timer: ResMut<NewEarthTimer>,
    time: Res<Time>,
) {
    timer.0 += time.delta_secs();
}

/// Spawn the planet and atmosphere when the time comes.
pub fn spawn_new_earth(
    mut commands: Commands,
    timer: Res<NewEarthTimer>,
    existing: Query<Entity, With<EarthSphere>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0 < EARTH_APPEAR_TIME { return; }
    if !existing.is_empty() { return; }

    // Planet sphere
    let earth_mesh = meshes.add(Sphere::new(1.0).mesh().ico(4).unwrap());
    let earth_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(EARTH_COLOR.0, EARTH_COLOR.1, EARTH_COLOR.2),
        emissive: LinearRgba::new(
            EARTH_COLOR.0, EARTH_COLOR.1, EARTH_COLOR.2, 1.0,
        ) * EARTH_EMISSIVE_MULT,
        perceptual_roughness: 0.8,
        metallic: 0.1,
        ..default()
    });
    commands.spawn((
        EarthSphere,
        Mesh3d(earth_mesh.clone()),
        MeshMaterial3d(earth_mat),
        Transform::from_xyz(0.0, EARTH_START_Y, -30.0)
            .with_scale(Vec3::splat(EARTH_START_SCALE)),
    ));

    // Atmosphere — slightly larger transparent sphere
    let atmo_mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());
    let atmo_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(
            ATMO_COLOR.0, ATMO_COLOR.1, ATMO_COLOR.2, ATMO_COLOR.3,
        ),
        emissive: LinearRgba::new(
            ATMO_COLOR.0, ATMO_COLOR.1, ATMO_COLOR.2, 1.0,
        ) * 0.8,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((
        AtmosphereSphere,
        Mesh3d(atmo_mesh),
        MeshMaterial3d(atmo_mat),
        Transform::from_xyz(0.0, EARTH_START_Y, -30.0)
            .with_scale(Vec3::splat(EARTH_START_SCALE * ATMO_SCALE_MULT)),
    ));

    // Point light on the planet for warmth
    commands.spawn((
        PointLight {
            color: Color::srgb(0.6, 0.8, 1.0),
            intensity: 5000.0,
            range: 80.0,
            ..default()
        },
        Transform::from_xyz(10.0, EARTH_START_Y + 20.0, -10.0),
        EarthSphere, // tag for cleanup
    ));
}

/// Grow and approach the planet over time.
pub fn grow_earth(
    timer: Res<NewEarthTimer>,
    time: Res<Time>,
    mut earth_q: Query<&mut Transform, (With<EarthSphere>, Without<AtmosphereSphere>)>,
    mut atmo_q: Query<&mut Transform, (With<AtmosphereSphere>, Without<EarthSphere>)>,
) {
    if timer.0 < EARTH_APPEAR_TIME { return; }
    let dt = time.delta_secs();

    // Progress from appear to large (5s to 25s)
    let t = ((timer.0 - EARTH_APPEAR_TIME) / (FADE_WHITE_START - EARTH_APPEAR_TIME))
        .clamp(0.0, 1.0);
    // Ease-in curve for dramatic reveal
    let eased = t * t * (3.0 - 2.0 * t); // smoothstep

    let scale = EARTH_START_SCALE + (EARTH_END_SCALE - EARTH_START_SCALE) * eased;
    // Planet moves toward camera (y decreases from 120 toward 15)
    let y = EARTH_START_Y + (15.0 - EARTH_START_Y) * eased;

    for mut tf in earth_q.iter_mut() {
        tf.scale = Vec3::splat(scale);
        tf.translation.y = y;
        // Gentle rotation
        tf.rotate(Quat::from_rotation_y(EARTH_ROT_SPEED * dt));
    }
    for mut tf in atmo_q.iter_mut() {
        tf.scale = Vec3::splat(scale * ATMO_SCALE_MULT);
        tf.translation.y = y;
        tf.rotate(Quat::from_rotation_y(EARTH_ROT_SPEED * 0.7 * dt));
    }
}

/// Manage the overall sequence: white fade, text, exit.
pub fn new_earth_sequence(
    mut commands: Commands,
    timer: Res<NewEarthTimer>,
    font: Res<GatheringFont>,
    text_q: Query<Entity, With<NewEarthText>>,
    white_q: Query<Entity, With<WhiteFadeOverlay>>,
) {
    let t = timer.0;

    // Spawn white fade overlay (once)
    if t >= FADE_WHITE_START && white_q.is_empty() {
        commands.spawn((
            WhiteFadeOverlay,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            ZIndex(30),
        ));
    }

    // Spawn text (once)
    if t >= TEXT_APPEAR_TIME && text_q.is_empty() {
        commands.spawn((
            NewEarthText,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ZIndex(40),
        )).with_child((
            Text::new("We made it."),
            TextFont {
                font: font.0.clone(),
                font_size: TEXT_FONT_SIZE,
                ..default()
            },
            TextColor(Color::srgba(
                TEXT_COLOR.0, TEXT_COLOR.1, TEXT_COLOR.2, 0.0,
            )),
        ));
    }

    // Exit and save
    if t >= EXIT_TIME {
        let mut gs = crate::save_state::load_game_state();
        gs.reached_new_earth = true;
        crate::save_state::save_game_state(&gs);
        std::process::exit(0);
    }
}

/// Update the white fade overlay alpha.
pub fn update_white_fade(
    timer: Res<NewEarthTimer>,
    mut fade_q: Query<&mut BackgroundColor, With<WhiteFadeOverlay>>,
) {
    let t = timer.0;
    if t < FADE_WHITE_START { return; }
    let alpha = ((t - FADE_WHITE_START) / (FADE_WHITE_END - FADE_WHITE_START))
        .clamp(0.0, 1.0);
    for mut bg in fade_q.iter_mut() {
        bg.0 = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

/// Fade in the "We made it." text.
pub fn update_new_earth_text(
    timer: Res<NewEarthTimer>,
    mut text_q: Query<&Children, With<NewEarthText>>,
    mut text_color_q: Query<&mut TextColor>,
) {
    let t = timer.0;
    if t < TEXT_APPEAR_TIME { return; }
    let alpha = ((t - TEXT_APPEAR_TIME) / 1.5).clamp(0.0, 1.0);
    for children in text_q.iter_mut() {
        for child in children.iter() {
            if let Ok(mut tc) = text_color_q.get_mut(child) {
                tc.0 = Color::srgba(TEXT_COLOR.0, TEXT_COLOR.1, TEXT_COLOR.2, alpha);
            }
        }
    }
}

/// In NewEarthMode, move the ship gently forward automatically.
pub fn auto_fly_ship(
    mut ship_q: Query<&mut Transform, With<Ship>>,
    time: Res<Time>,
) -> Result {
    let mut tf = ship_q.single_mut()?;
    let dt = time.delta_secs();
    // Gentle forward drift and very slight sway
    let sway = (time.elapsed_secs() * 0.3).sin() * 0.3;
    tf.translation.x += sway * dt;
    // Subtle up-drift
    tf.translation.y += 0.5 * dt;
    Ok(())
}

/// Slow down star scrolling in NewEarthMode.
pub fn scroll_stars_slow(
    mut star_q: Query<(&Star, &mut Transform), Without<NebulaPlane>>,
    mut nebula_q: Query<&mut Transform, (With<NebulaPlane>, Without<Star>)>,
    bounds: Res<ViewBounds>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (star, mut tf) in star_q.iter_mut() {
        let speed = STAR_LAYER_SPEEDS[star.layer] * SCROLL_SPEED * STAR_SLOW_FACTOR;
        tf.translation.y -= speed * dt;
        let limit = bounds.half_height * 1.5;
        if tf.translation.y < -limit {
            tf.translation.y += limit * 2.0;
            tf.translation.x = rand::random::<f32>() * bounds.half_width * 3.0
                - bounds.half_width * 1.5;
        }
    }
    let nebula_speed = SCROLL_SPEED * NEBULA_SCROLL_SPEED_MULT * STAR_SLOW_FACTOR * dt;
    for mut tf in nebula_q.iter_mut() {
        tf.translation.y -= nebula_speed;
        if tf.translation.y < -NEBULA_SIZE {
            tf.translation.y += NEBULA_SIZE * 2.0;
        }
    }
}

/// Hide HUD elements in NewEarthMode (set alpha to 0).
pub fn hide_hud(
    mut dist_q: Query<&mut TextColor, With<DistanceText>>,
    mut time_q: Query<&mut TextColor, (With<TimeText>, Without<DistanceText>, Without<CrystalText>)>,
    mut crys_q: Query<&mut TextColor, (With<CrystalText>, Without<DistanceText>, Without<TimeText>)>,
) {
    for mut tc in dist_q.iter_mut() { tc.0 = Color::srgba(0.0, 0.0, 0.0, 0.0); }
    for mut tc in time_q.iter_mut() { tc.0 = Color::srgba(0.0, 0.0, 0.0, 0.0); }
    for mut tc in crys_q.iter_mut() { tc.0 = Color::srgba(0.0, 0.0, 0.0, 0.0); }
}

/// Override the intro fade to use a slower, more gentle reveal.
pub fn gentle_intro_fade(
    timer: Res<NewEarthTimer>,
    mut fade_q: Query<&mut BackgroundColor, With<FadeOverlay>>,
) {
    if timer.0 > 3.0 { return; }
    let alpha = 1.0 - (timer.0 / 3.0).clamp(0.0, 1.0);
    for mut bg in fade_q.iter_mut() {
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    }
}
