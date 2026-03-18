// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod dashboard;
mod games;
mod anna;
mod anna_messages;
mod story;
mod resources;
mod question_data;
mod questions;
mod endings;
mod endings_anim;
mod dialog_types;
mod dialog_ui;
mod dialog_system;
mod dialog_scenes;
mod dialog_scenes_act1;
mod dialog_scenes_act2;
mod dialog_scenes_act3;
mod dialog_scenes_act4;
mod dialog_scenes_crew;
mod dialog_scenes_crew_ng;
mod dialog_scenes_philosophy;
mod dialog_scenes_philosophy2;
mod dialog_scenes_hidden;
mod dialog_scenes_earth;
mod dialog_scenes_earth2;
mod dialog_scenes_anna_personal;
mod endings_extended;
pub mod world_seed;
pub mod crew_stories;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use rand::Rng;
use constants::*;
use types::*;
use crate::save_state::{load_game_state, GameState};

/// Cached world state — generated once on startup from the seed.
#[derive(Resource)]
#[allow(dead_code)]
pub struct CachedWorldState {
    pub world: world_seed::WorldState,
    pub crew: Vec<crew_stories::CrewMember>,
}

pub fn build_app(app: &mut App) {
    let gs = load_game_state();

    // Generate world from seed (deterministic, cached for the session)
    let world = world_seed::generate_world(gs.world_seed);
    let crew = crew_stories::generate_crew(gs.world_seed);
    let cached = CachedWorldState { world, crew };

    let ship = ShipStatus {
        power: gs.power,
        life_support: gs.life_support,
        cryo: gs.cryo,
        shields: gs.shields,
        repair: gs.repair,
        crystals: gs.total_crystals(),
        crew_count: gs.crew_count,
        day: gs.day,
        distance_au: gs.distance_au,
        bot_level: gs.bot_level,
    };
    app.insert_resource(ClearColor(Color::srgb(
        CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2,
    )))
    .insert_resource(ship)
    .insert_resource(gs)
    .insert_resource(BarDisplayValues::default())
    .insert_resource(AnnaState::default())
    .insert_resource(DrainTimer::default())
    .insert_resource(RunningGame::default())
    .insert_resource(questions::QuestionState::default())
    .insert_resource(dialog_types::DialogState::default())
    .insert_resource(cached)
    .add_systems(Startup, setup_mission)
    .add_systems(Update, (
        dashboard::animate_resource_bars,
        dashboard::update_status_texts,
        games::card_hover_interaction,
        games::card_click_interaction,
        games::poll_running_game,
        games::manage_game_overlay,
        resources::drain_resources,
        anna::update_anna_messages,
        anna::anna_click_dismiss,
        anna::update_anna_glow,
        twinkle_stars,
    ))
    .add_systems(Update, (
        questions::check_pending_question,
        questions::question_option_hover,
        questions::question_option_click,
        questions::update_reaction_overlay,
        final_voyage_click,
    ))
    .add_systems(Update, (
        dialog_system::check_dialog_triggers,
        dialog_system::start_next_dialog,
        dialog_system::update_typewriter,
        dialog_system::dialog_click_advance,
        dialog_system::dialog_choice_click,
        dialog_system::spawn_choices_when_ready,
        dialog_ui::dialog_choice_hover,
        dialog_ui::animate_dialog_glow,
        dialog_ui::animate_dialog_circle,
    ))
    .add_systems(Update, (
        endings_anim::animate_ending_screen,
        endings_anim::animate_ending_stats,
        endings_anim::animate_ending_glow,
    ).run_if(resource_exists::<endings::EndingState>))
    .add_systems(Update, (
        endings_anim::new_journey_hover,
        endings_anim::new_journey_click,
    ).run_if(resource_exists::<endings::EndingState>));
}

fn setup_mission(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: BLOOM_INTENSITY_M,
            low_frequency_boost: BLOOM_LF_BOOST_M,
            low_frequency_boost_curvature: 0.7,
            high_pass_frequency: 1.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light (dim, space-like)
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.5, 0.55, 0.7),
        brightness: 50.0,
        ..default()
    });

    // Font
    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(MissionFont(font.clone()));

    // Stars (subtle background)
    spawn_stars(&mut commands, &mut meshes, &mut materials);

    // Vignette
    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
    ));

    // Main UI layout
    let ship = load_game_state();
    let ship_status = ShipStatus {
        power: ship.power, life_support: ship.life_support,
        cryo: ship.cryo, shields: ship.shields, repair: ship.repair,
        crystals: ship.total_crystals(), crew_count: ship.crew_count,
        day: ship.day, distance_au: ship.distance_au, bot_level: ship.bot_level,
    };
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        ..default()
    }).with_children(|root| {
        dashboard::spawn_dashboard(root, &font);
        games::spawn_game_cards(root, &font, &ship_status);
    });

    // Anna's panel (absolute positioned at bottom)
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }).with_children(|overlay| {
        anna::spawn_anna_panel(overlay, &font);
    });

    // Final Voyage button (only if bot_level >= 149)
    if ship_status.bot_level >= 149 {
        commands.spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(100.0),
                left: Val::Percent(50.0),
                padding: UiRect::axes(Val::Px(28.0), Val::Px(14.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.9)),
            BorderColor::all(Color::srgba(0.8, 0.7, 0.4, 0.7)),
            BoxShadow::new(
                Color::srgba(0.9, 0.8, 0.4, 0.3),
                Val::ZERO, Val::ZERO,
                Val::Px(5.0), Val::Px(15.0),
            ),
            endings::FinalVoyageBtn,
        )).with_child((
            Text::new("Final Voyage"),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.95, 0.85, 0.5)),
        ));
    }

    // Version label
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(6.0),
        bottom: Val::Px(4.0),
        ..default()
    }).with_child((
        Text::new(format!("Mission Control · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: VERSION_FONT_M, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

fn spawn_stars(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_STARS {
        let size = rng.gen_range(STAR_MIN_SIZE..STAR_MAX_SIZE);
        let brightness = rng.gen_range(0.4..1.0_f32);
        let mesh = meshes.add(Sphere::new(size));
        let emissive = bevy::color::LinearRgba::new(
            brightness, brightness, brightness * 1.1, 1.0,
        ) * 2.5;
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(brightness, brightness, brightness * 1.1),
            emissive,
            unlit: true,
            ..default()
        });
        let x = rng.gen_range(-STAR_SPREAD_X..STAR_SPREAD_X);
        let y = rng.gen_range(-STAR_SPREAD_Y..STAR_SPREAD_Y);
        let z = STAR_DEPTH + rng.gen_range(-5.0..5.0_f32);
        commands.spawn((
            StarTwinkle {
                phase: rng.gen_range(0.0..std::f32::consts::TAU),
                speed: rng.gen_range(0.5..STAR_TWINKLE_SPEED),
            },
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(x, y, z),
        ));
    }
}

fn twinkle_stars(
    time: Res<Time>,
    mut query: Query<(&StarTwinkle, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    for (star, mut transform) in query.iter_mut() {
        let scale = 0.7 + 0.3 * (t * star.speed + star.phase).sin();
        transform.scale = Vec3::splat(scale);
    }
}

/// System: handle Final Voyage button click — trigger ending sequence.
fn final_voyage_click(
    query: Query<&Interaction, (Changed<Interaction>, With<endings::FinalVoyageBtn>)>,
    mut commands: Commands,
    gs: Res<GameState>,
    font: Res<MissionFont>,
    ending_q: Query<Entity, With<endings::EndingScreen>>,
) {
    // Don't trigger if ending already showing
    if !ending_q.is_empty() { return; }

    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            endings::spawn_ending_screen(&mut commands, &font.0, &gs);
        }
    }
}

fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
    let size = 256u32;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;
    let max_dist = center * 1.2;
    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();
            let t = (dist / max_dist).clamp(0.0, 1.0);
            let alpha = (t * t * 0.7 * 255.0) as u8;
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 0;
            data[idx + 1] = 0;
            data[idx + 2] = 0;
            data[idx + 3] = alpha;
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}
