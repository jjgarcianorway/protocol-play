// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod ship;
mod asteroids;
mod collision;
mod damage;
mod background;
mod hud;
mod game_over;

use bevy::prelude::*;
use bevy::core_pipeline::bloom::Bloom;
use constants::*;
use types::*;
use game_over::*;

pub fn build_app(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(CLEAR_COLOR_G.0, CLEAR_COLOR_G.1, CLEAR_COLOR_G.2)))
        .insert_resource(AmbientLight {
            color: Color::srgb(AMBIENT_COLOR_G.0, AMBIENT_COLOR_G.1, AMBIENT_COLOR_G.2),
            brightness: AMBIENT_BRIGHTNESS_G,
        })
        .insert_resource(ShipState::default())
        .insert_resource(ScreenShake::default())
        .insert_resource(ViewBounds::default())
        .insert_resource(AsteroidSpawnTimer::default())
        .insert_resource(FadeTimer::default())
        .init_state::<GatheringState>()
        .add_systems(Startup, setup_gathering)
        .add_systems(Update, (
            ship::move_ship,
            asteroids::spawn_asteroids,
            asteroids::move_asteroids,
            collision::check_collisions.after(asteroids::move_asteroids),
            collision::tick_hit_cooldowns,
            damage::update_shield_regen,
            damage::update_screen_shake.after(collision::check_collisions),
            damage::update_bars,
            background::scroll_stars,
            hud::update_hud,
            hud::update_game_time,
            check_game_over.after(collision::check_collisions),
            update_fade,
            spawn_game_over_screen.after(check_game_over),
            try_again_interaction,
        ));
}

fn setup_gathering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fonts: ResMut<Assets<Font>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Camera { hdr: true, ..default() },
        Bloom { intensity: 0.15, low_frequency_boost: 0.5,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 1.0, ..default() },
        Transform::from_xyz(0.0, 0.0, CAMERA_Z).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: CAMERA_FOV.to_radians(), ..default()
        }),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight { illuminance: DIR_LIGHT_BRIGHTNESS, shadows_enabled: false, ..default() },
        Transform::default().looking_to(Vec3::new(DIR_LIGHT_DIR[0], DIR_LIGHT_DIR[1], DIR_LIGHT_DIR[2]), Vec3::Y),
    ));

    // View bounds from camera
    let bounds = ViewBounds::default();

    // Font
    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(GatheringFont(font.clone()));

    // Ship
    ship::spawn_ship(&mut commands, &mut meshes, &mut materials);

    // Asteroid assets
    let assets = asteroids::create_asteroid_assets(&mut meshes, &mut materials);
    commands.insert_resource(assets);

    // Background stars
    background::spawn_stars(&mut commands, &mut meshes, &mut materials, &bounds);

    // UI
    damage::spawn_bars(&mut commands, font.clone());
    hud::spawn_hud(&mut commands, font);
    spawn_fade_overlay(&mut commands);
}
