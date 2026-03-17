// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod ship;
mod asteroids;
mod collision;
mod crystals;
mod damage;
mod difficulty;
mod background;
mod hud;
mod game_over;
mod stats;
mod pause;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use constants::*;
use types::*;
use game_over::*;

pub fn build_app(app: &mut App) {
    let best = stats::load_best();
    app.insert_resource(ClearColor(Color::srgb(CLEAR_COLOR_G.0, CLEAR_COLOR_G.1, CLEAR_COLOR_G.2)))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(AMBIENT_COLOR_G.0, AMBIENT_COLOR_G.1, AMBIENT_COLOR_G.2),
            brightness: AMBIENT_BRIGHTNESS_G,
            ..default()
        })
        .insert_resource(ShipState::default())
        .insert_resource(ScreenShake::default())
        .insert_resource(ViewBounds::default())
        .insert_resource(AsteroidSpawnTimer::default())
        .insert_resource(CrystalSpawnTimer::default())
        .insert_resource(Difficulty::default())
        .insert_resource(FadeTimer::default())
        .insert_resource(game_over::IntroFade::default())
        .insert_resource(HitFlash::default())
        .insert_resource(Paused::default())
        .insert_resource(best)
        .init_state::<GatheringState>()
        .add_systems(Startup, setup_gathering)
        .add_systems(Update, (
            ship::move_ship,
            asteroids::spawn_asteroids,
            asteroids::move_asteroids,
            asteroids::asteroid_asteroid_collisions.after(asteroids::move_asteroids),
            asteroids::move_sparks,
            collision::check_collisions.after(asteroids::move_asteroids),
            collision::tick_hit_cooldowns,
            crystals::spawn_crystals,
            crystals::move_crystals,
            crystals::absorb_crystals.after(crystals::move_crystals),
            crystals::move_particles,
            crystals::update_floating_texts,
            damage::update_shield_regen,
            damage::update_screen_shake.after(collision::check_collisions),
            damage::update_bars,
            damage::update_hit_flash.after(collision::check_collisions),
        ))
        .add_systems(Update, (
            difficulty::update_difficulty,
            background::scroll_stars,
            hud::update_hud,
            hud::update_game_time,
            check_game_over.after(collision::check_collisions),
            update_fade,
            try_again_interaction,
            game_over::try_again_hover,
            game_over::update_intro_fade,
            spawn_game_over_screen.after(check_game_over).after(try_again_interaction),
            ship::restore_cursor.run_if(|s: Res<State<GatheringState>>| *s.get() == GatheringState::GameOver),
            ship::update_shield_bubble,
            ship::spawn_engine_particles,
            ship::move_engine_particles,
            pause::toggle_pause,
            pause::resume_button_interaction,
            pause::resume_button_hover,
        ));
}

fn setup_gathering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Bloom { intensity: 0.20, low_frequency_boost: 0.4,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 0.9, ..default() },
        Transform::from_xyz(0.0, 0.0, CAMERA_Z).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: CAMERA_FOV.to_radians(), ..default()
        }),
    ));

    commands.spawn((
        DirectionalLight { illuminance: DIR_LIGHT_BRIGHTNESS, shadows_enabled: false, ..default() },
        Transform::default().looking_to(Vec3::new(DIR_LIGHT_DIR[0], DIR_LIGHT_DIR[1], DIR_LIGHT_DIR[2]), Vec3::Y),
    ));

    let bounds = ViewBounds::default();

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(GatheringFont(font.clone()));

    ship::spawn_ship(&mut commands, &asset_server, &mut meshes, &mut materials);

    let assets = asteroids::create_asteroid_assets(&mut meshes, &mut materials);
    commands.insert_resource(assets);

    background::spawn_stars(&mut commands, &mut meshes, &mut materials, &bounds);
    background::spawn_nebula_planes(&mut commands, &mut meshes, &mut materials, &mut images, &bounds);

    damage::spawn_bars(&mut commands, font.clone());
    hud::spawn_hud(&mut commands, font.clone());
    spawn_fade_overlay(&mut commands);

    // Vignette overlay (same as Bot Game)
    let vignette = background::create_vignette(&mut images);
    commands.spawn((Node { position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), ..default() }, ImageNode::new(vignette)));

    // Version label
    commands.spawn(Node { position_type: PositionType::Absolute, right: Val::Px(6.0),
        bottom: Val::Px(4.0), ..default() })
        .with_child((Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
            TextFont { font, font_size: 11.0, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35))));
}
