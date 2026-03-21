// SPDX-License-Identifier: GPL-3.0-or-later
//! Integrated mode: registers Gathering systems under GameScene::Gathering.

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use super::constants::*;
use super::types::*;
use super::{ship, asteroids, collision, crystals, damage, difficulty, background};
use super::{hud, game_over, pause, effects, warnings, new_earth, anna, stats};
use crate::mission::types::GameScene;

/// Run condition: GameScene is Gathering AND NewEarthMode is NOT present.
fn gathering_no_new_earth(
    scene: Option<Res<State<GameScene>>>,
    mode: Option<Res<new_earth::NewEarthMode>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Gathering) && mode.is_none()
}

/// Run condition: GameScene is Gathering AND NewEarthMode IS present.
fn gathering_new_earth(
    scene: Option<Res<State<GameScene>>>,
    mode: Option<Res<new_earth::NewEarthMode>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Gathering) && mode.is_some()
}

/// Register all Gathering systems for integrated mode.
pub fn register_integrated_systems(app: &mut App) {
    app.init_state::<GatheringState>();
    app
    .add_systems(OnEnter(GameScene::Gathering), (
        enter_gathering,
        new_earth::check_new_earth_mode.after(enter_gathering),
        anna::setup_gathering_anna.after(enter_gathering),
    ))
    .add_systems(OnExit(GameScene::Gathering), exit_gathering)
    // Normal gameplay (no NewEarth)
    .add_systems(Update, (
        ship::move_ship,
        asteroids::spawn_asteroids,
        asteroids::move_asteroids,
        asteroids::asteroid_asteroid_collisions.after(asteroids::move_asteroids),
        effects::move_sparks,
        collision::check_collisions.after(asteroids::move_asteroids),
        collision::check_near_misses.after(collision::check_collisions),
        collision::tick_hit_cooldowns,
        collision::tick_near_miss_cooldowns,
        crystals::spawn_crystals,
        crystals::move_crystals,
        crystals::absorb_crystals.after(crystals::move_crystals),
        crystals::move_particles,
        crystals::update_floating_texts,
        damage::update_shield_regen,
        damage::update_screen_shake.after(collision::check_collisions),
        damage::update_bars,
        damage::update_hit_flash.after(collision::check_collisions),
        damage::update_near_miss_flash.after(collision::check_near_misses),
    ).run_if(gathering_no_new_earth))
    .add_systems(Update, (
        difficulty::update_difficulty,
        difficulty::update_background_color.after(difficulty::update_difficulty),
        background::scroll_stars,
        hud::update_hud,
        hud::update_game_time,
        game_over::check_game_over.after(collision::check_collisions),
        game_over::update_fade,
        game_over::try_again_interaction,
        game_over::try_again_hover,
        game_over::update_intro_fade,
        game_over::try_again_cleanup.after(game_over::try_again_interaction),
        game_over::spawn_game_over_screen
            .after(game_over::check_game_over)
            .after(game_over::try_again_interaction),
    ).run_if(gathering_no_new_earth))
    .add_systems(Update, (
        ship::restore_cursor.run_if(
            |s: Option<Res<State<GatheringState>>>|
                s.is_some_and(|s| *s.get() == GatheringState::GameOver)
        ),
        ship::update_shield_bubble,
        ship::spawn_engine_particles,
        ship::move_engine_particles,
        pause::toggle_pause,
        pause::resume_button_interaction,
        pause::resume_button_hover,
    ).run_if(gathering_no_new_earth))
    .add_systems(Update, (
        game_over::update_game_over_fade,
        pause::update_pause_fade,
    ).run_if(gathering_no_new_earth))
    .add_systems(Update, (
        crate::anna_comments::tick_anna_comments,
        anna::gathering_anna_reactive,
    ).run_if(gathering_no_new_earth))
    .add_systems(Update, (
        warnings::update_warning_indicators,
        ship::update_magnet_ring,
        collision::update_damage_direction,
        ship::spawn_damage_particles,
        ship::move_damage_particles,
        effects::spawn_asteroid_trails,
        effects::move_trail_particles,
    ).run_if(gathering_no_new_earth))
    // New Earth cutscene
    .add_systems(Update, (
        new_earth::tick_new_earth_timer,
        new_earth::spawn_new_earth,
        new_earth::grow_earth,
        new_earth::new_earth_sequence,
        new_earth::update_white_fade,
        new_earth::update_new_earth_text,
        new_earth::auto_fly_ship,
        new_earth::scroll_stars_slow,
        new_earth::hide_hud,
        new_earth::gentle_intro_fade,
        ship::spawn_engine_particles,
        ship::move_engine_particles,
    ).run_if(gathering_new_earth));
}

/// Enter Gathering scene: insert resources, spawn camera + game entities.
fn enter_gathering(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
) {
    // Hide Mission Control camera
    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }

    // Set Gathering visual settings
    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_G.0, CLEAR_COLOR_G.1, CLEAR_COLOR_G.2,
    ));
    *ambient = GlobalAmbientLight {
        color: Color::srgb(
            AMBIENT_COLOR_G.0, AMBIENT_COLOR_G.1, AMBIENT_COLOR_G.2,
        ),
        brightness: AMBIENT_BRIGHTNESS_G,
        ..default()
    };

    // Insert game resources
    let best = stats::load_best();
    commands.insert_resource(ShipState::default());
    commands.insert_resource(ScreenShake::default());
    commands.insert_resource(ViewBounds::default());
    commands.insert_resource(AsteroidSpawnTimer::default());
    commands.insert_resource(CrystalSpawnTimer::default());
    commands.insert_resource(Difficulty::default());
    commands.insert_resource(game_over::FadeTimer::default());
    commands.insert_resource(game_over::IntroFade::default());
    commands.insert_resource(HitFlash::default());
    commands.insert_resource(NearMissFlash::default());
    commands.insert_resource(Paused::default());
    commands.insert_resource(CrystalChain::default());
    commands.insert_resource(game_over::TryAgainTriggered::default());
    commands.insert_resource(best);
    commands.insert_resource(crate::anna_comments::AnnaComments::default());
    commands.insert_resource(anna::AnnaReactiveFlags::default());

    // Spawn Gathering camera
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: 0.20, low_frequency_boost: 0.4,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 0.9,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, CAMERA_Z).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            fov: CAMERA_FOV.to_radians(), ..default()
        }),
        GatheringEntity,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: DIR_LIGHT_BRIGHTNESS, shadows_enabled: false, ..default()
        },
        Transform::default().looking_to(
            Vec3::new(DIR_LIGHT_DIR[0], DIR_LIGHT_DIR[1], DIR_LIGHT_DIR[2]),
            Vec3::Y,
        ),
        GatheringEntity,
    ));

    let bounds = ViewBounds::default();

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(GatheringFont(font.clone()));

    ship::spawn_ship(&mut commands, &asset_server, &mut meshes, &mut materials);
    let assets = asteroids::create_asteroid_assets(&mut meshes, &mut materials);
    commands.insert_resource(assets);
    background::spawn_stars(&mut commands, &mut meshes, &mut materials, &bounds);
    background::spawn_nebula_planes(
        &mut commands, &mut meshes, &mut materials, &mut images, &bounds,
    );
    damage::spawn_bars(&mut commands, font.clone());
    hud::spawn_hud(&mut commands, font.clone());
    game_over::spawn_fade_overlay(&mut commands);

    let vignette = background::create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
        GatheringEntity,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0), bottom: Val::Px(4.0), ..default()
        },
        GatheringEntity,
    )).with_child((
        Text::new(format!("The Gathering · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: 11.0, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

/// Exit Gathering: despawn all entities, remove resources, restore MC.
fn exit_gathering(
    mut commands: Commands,
    q1: Query<Entity, Or<(
        With<GatheringEntity>, With<Ship>, With<Asteroid>,
        With<Star>, With<NebulaPlane>, With<ShieldBubble>,
        With<GameOverScreen>, With<PauseScreen>, With<MagnetRing>,
        With<HitFlashOverlay>,
    )>>,
    q2: Query<Entity, Or<(
        With<FloatingText>, With<Spark>,
        With<EngineParticle>, With<CrystalCloud>, With<CrystalParticle>,
        With<WarningIndicator>, With<DamageDirectionIndicator>,
        With<DamageSmoke>, With<DamageSpark>, With<AsteroidTrailParticle>,
    )>>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
    mut clear_color: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    for entity in q1.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q2.iter() {
        commands.entity(entity).despawn();
    }

    // Remove game resources
    commands.remove_resource::<ShipState>();
    commands.remove_resource::<ScreenShake>();
    commands.remove_resource::<AsteroidSpawnTimer>();
    commands.remove_resource::<CrystalSpawnTimer>();
    commands.remove_resource::<Difficulty>();
    commands.remove_resource::<game_over::FadeTimer>();
    commands.remove_resource::<game_over::IntroFade>();
    commands.remove_resource::<HitFlash>();
    commands.remove_resource::<NearMissFlash>();
    commands.remove_resource::<Paused>();
    commands.remove_resource::<CrystalChain>();
    commands.remove_resource::<game_over::TryAgainTriggered>();
    commands.remove_resource::<BestStats>();
    commands.remove_resource::<GatheringFont>();
    commands.remove_resource::<GatheringAssets>();
    commands.remove_resource::<ViewBounds>();
    commands.remove_resource::<crate::anna_comments::AnnaComments>();
    commands.remove_resource::<anna::AnnaReactiveFlags>();

    // Restore Mission Control camera
    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }

    // Restore Mission Control visual settings
    use crate::mission::constants::*;
    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2,
    ));
    *ambient = GlobalAmbientLight {
        color: Color::srgb(0.5, 0.55, 0.7), brightness: 50.0, ..default()
    };
}

