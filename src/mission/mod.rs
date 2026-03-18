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
mod dialog_scenes_consequences;
mod dialog_scenes_secrets;
mod dialog_scenes_crossref;
mod dialog_scenes_crossref2;
mod dialog_scenes_orben;
mod dialog_scenes_characters;
mod dialog_scenes_consequences2;
mod dialog_scenes_viktor_arc;
mod dialog_scenes_viktor_arc2;
mod dialog_scenes_collapse_climate;
mod dialog_scenes_collapse_resource;
mod dialog_scenes_collapse_pandemic;
mod dialog_scenes_collapse_nuclear;
mod dialog_scenes_collapse_ai;
mod dialog_scenes_collapse_political;
mod dialog_scenes_collapse_economic;
mod dialog_scenes_elena;
mod dialog_scenes_elena2;
mod dialog_scenes_youssef;
mod dialog_scenes_amira_arc;
mod dialog_scenes_amira_arc2;
mod dialog_scenes_meilin_arc;
mod dialog_scenes_kwame_arc;
mod dialog_scenes_kwame_arc2;
mod dialog_scenes_connections;
mod dialog_scenes_connections2;
mod dialog_scenes_ngplus;
mod dialog_scenes_ngplus2;
mod dialog_scenes_bright;
mod dialog_scenes_bright2;
mod dialog_scenes_bright3;
mod dialog_scenes_climax;
mod dialog_scenes_climax2;
mod dialog_scenes_colony;
mod dialog_scenes_colony2;
mod dialog_scenes_resolution;
mod dialog_scenes_daily;
mod dialog_scenes_earth_memory;
mod dialog_scenes_crew_daily;
mod dialog_scenes_anna_daily;
mod dialog_scenes_philosophy3;
mod dialog_scenes_secrets2;
mod dialog_scenes_arrival;
mod endings_extended;
pub mod world_seed;
pub mod crew_stories;
pub mod settings;
mod settings_systems;
mod settings_seed;
mod main_menu;
mod main_menu_ui;
mod loading_screen;
pub mod stats_screen;
mod stats_ui;
mod decision_tree;
mod decision_tree_ui;
pub mod credits;
mod credits_content;
mod credits_systems;
pub mod profiles;
mod profiles_ui;
mod profiles_ui_systems;
use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use constants::*;
use types::*;
use crate::save_state::{load_game_state, save_game_state, GameState};
#[derive(Resource)]
#[allow(dead_code)]
pub struct CachedWorldState {
    pub world: world_seed::WorldState,
    pub crew: Vec<crew_stories::CrewMember>,
}
pub fn build_app(app: &mut App) {
    // Initialize profile system (migrates legacy saves if needed)
    let profile_mgr = profiles::init_profiles();

    let mut gs = load_game_state();
    let world = world_seed::generate_world(gs.world_seed);
    let crew = crew_stories::generate_crew(gs.world_seed);
    if gs.day <= 1 { gs.crew_count = world.aurora_crew; save_game_state(&gs); }
    let cached = CachedWorldState { world, crew };

    let collapse_flag = match cached.world.earth_collapse.primary_cause {
        world_seed::CollapseCause::ClimateCollapse => "collapse_climate",
        world_seed::CollapseCause::ResourceWars => "collapse_resource",
        world_seed::CollapseCause::PandemicCascade => "collapse_pandemic",
        world_seed::CollapseCause::NuclearExchange => "collapse_nuclear",
        world_seed::CollapseCause::AIUprising => "collapse_ai",
        world_seed::CollapseCause::PoliticalCollapse => "collapse_political",
        world_seed::CollapseCause::EconomicMeltdown => "collapse_economic",
    };
    if !gs.decisions.contains(&collapse_flag.to_string()) {
        gs.decisions.push(collapse_flag.to_string());
        let sev = cached.world.earth_collapse.severity;
        let sev_flag = if sev <= 2 { "severity_mild" } else if sev >= 4 { "severity_extreme" } else { "severity_moderate" };
        if !gs.decisions.contains(&sev_flag.to_string()) { gs.decisions.push(sev_flag.to_string()); }
        save_game_state(&gs);
    }

    let ship = ShipStatus {
        power: gs.power, life_support: gs.life_support,
        cryo: gs.cryo, shields: gs.shields, repair: gs.repair,
        crystals: gs.total_crystals(), crew_count: gs.crew_count,
        day: gs.day, distance_au: gs.distance_au, bot_level: gs.bot_level,
    };
    app.insert_resource(ClearColor(Color::srgb(CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2)))
    .init_state::<AppPhase>()
    .insert_resource(ship).insert_resource(gs)
    .insert_resource(BarDisplayValues::default()).insert_resource(AnnaState::default())
    .insert_resource(DrainTimer::default()).insert_resource(RunningGame::default())
    .insert_resource(questions::QuestionState::default())
    .insert_resource(dialog_types::DialogState::default())
    .insert_resource(dialog_types::AnnaGlowMood::default()).insert_resource(cached)
    .insert_resource(settings::SettingsOpen::default())
    .insert_resource(settings::ActiveSettingsTab::default())
    .insert_resource(settings_seed::SeedInputState::default())
    .insert_resource(main_menu::MenuTransition::default())
    .insert_resource(main_menu::MenuTimer::default()).insert_resource(profile_mgr)
    .add_systems(Startup, setup_shared)
    // === Profile selection screen ===
    .add_systems(OnEnter(AppPhase::ProfileSelect), profiles_ui::enter_profile_select)
    .add_systems(Update, (
        profiles_ui_systems::animate_profile_fade_in,
        profiles_ui_systems::profile_slot_hover,
        profiles_ui_systems::profile_slot_click,
        profiles_ui_systems::profile_fade_out,
        profiles_ui_systems::profile_delete_hover,
        profiles_ui_systems::profile_delete_click,
        profiles_ui_systems::profile_confirm_click,
        profiles_ui_systems::profile_confirm_hover,
        profiles_ui_systems::profile_name_click,
        profiles_ui_systems::profile_rename_keyboard,
        twinkle_stars,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::ProfileSelect)))
    .add_systems(OnExit(AppPhase::ProfileSelect),
        profiles_ui_systems::cleanup_profile_select)
    // === Main menu ===
    .add_systems(OnEnter(AppPhase::MainMenu), enter_main_menu)
    .add_systems(Update, (
        main_menu::tick_menu_timer,
        main_menu::animate_menu_fade_in,
        main_menu::menu_button_hover,
        main_menu::menu_button_click,
        main_menu::confirm_button_click,
        main_menu::confirm_button_hover,
        main_menu::menu_fade_out,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::MainMenu)))
    .add_systems(Update, twinkle_stars.run_if(in_state(AppPhase::MainMenu)))
    .add_systems(Update, (
        stats_screen::stats_dismiss, stats_screen::animate_stats_glow,
        decision_tree::decision_tree_dismiss, decision_tree::animate_decision_tree_glow,
        decision_tree::parallax_system, decision_tree::decision_node_hover,
    ).run_if(in_state(AppPhase::MainMenu)))
    .add_systems(OnEnter(AppPhase::Loading), loading_screen::enter_loading)
    .add_systems(Update, (
        loading_screen::generate_world_during_loading,
        loading_screen::tick_loading_progress,
        twinkle_stars,
        main_menu::drift_menu_stars,
    ).run_if(in_state(AppPhase::Loading)))
    .add_systems(OnExit(AppPhase::Loading), loading_screen::cleanup_loading)
    .add_systems(OnEnter(AppPhase::Playing), enter_playing)
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
    ).run_if(in_state(AppPhase::Playing)))
    .add_systems(Update, (
        questions::check_pending_question,
        questions::question_option_hover,
        questions::question_option_click,
        questions::update_reaction_overlay,
        final_voyage_click,
    ).run_if(in_state(AppPhase::Playing)))
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
    ).run_if(in_state(AppPhase::Playing)))
    .add_systems(Update, (
        endings_anim::animate_ending_screen,
        endings_anim::animate_ending_stats,
        endings_anim::animate_ending_glow,
    ).run_if(in_state(AppPhase::Playing)
        .and(resource_exists::<endings::EndingState>)))
    .add_systems(Update, (
        endings_anim::new_journey_hover,
        endings_anim::new_journey_click,
    ).run_if(in_state(AppPhase::Playing)
        .and(resource_exists::<endings::EndingState>)))
    // Settings systems run in both states
    .add_systems(Update, (
        settings_systems::toggle_settings,
        settings_systems::animate_settings_fade,
        settings_systems::dismiss_on_bg_click,
        settings_systems::tab_click,
        settings_systems::tab_hover,
        settings_systems::language_click,
        settings_systems::lang_btn_hover,
        settings_systems::reset_click,
        settings_systems::confirm_reset_click,
        settings_systems::reset_btn_hover,
        settings_seed::seed_input_click,
        settings_seed::seed_input_deactivate,
        settings_seed::seed_keyboard_input,
        settings_seed::seed_apply_click,
        settings_seed::seed_apply_hover,
    ))
    .add_systems(Update, (
        credits_systems::update_credits,
        credits_systems::credits_keyboard,
        credits_systems::cleanup_credits,
    ).run_if(in_state(AppPhase::MainMenu)
        .and(resource_exists::<credits::CreditsState>)));
}
fn setup_shared(
    mut commands: Commands, mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Bloom { intensity: BLOOM_INTENSITY_M, low_frequency_boost: BLOOM_LF_BOOST_M,
            low_frequency_boost_curvature: 0.7, high_pass_frequency: 1.0, ..default() },
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.5, 0.55, 0.7), brightness: 50.0, ..default()
    });
    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(MissionFont(font.clone()));
    main_menu::spawn_menu_stars(&mut commands, &mut meshes, &mut materials);
    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node { position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        ImageNode::new(vignette),
    ));
}

fn enter_main_menu(
    mut commands: Commands,
    font: Res<MissionFont>,
    mut gs: ResMut<GameState>,
    mut ship: ResMut<ShipStatus>,
    mut cached: ResMut<CachedWorldState>,
    mut menu_timer: ResMut<main_menu::MenuTimer>,
    mut menu_transition: ResMut<main_menu::MenuTransition>,
) {
    *gs = load_game_state();
    cached.world = world_seed::generate_world(gs.world_seed);
    cached.crew = crew_stories::generate_crew(gs.world_seed);
    *ship = ShipStatus {
        power: gs.power, life_support: gs.life_support,
        cryo: gs.cryo, shields: gs.shields, repair: gs.repair,
        crystals: gs.total_crystals(), crew_count: gs.crew_count,
        day: gs.day, distance_au: gs.distance_au, bot_level: gs.bot_level,
    };
    menu_timer.0 = 0.0;
    *menu_transition = main_menu::MenuTransition::default();
    main_menu_ui::spawn_menu_ui(&mut commands, &font.0);
}

fn enter_playing(mut commands: Commands, font: Res<MissionFont>) {
    let ship = load_game_state();
    let ship_status = ShipStatus {
        power: ship.power, life_support: ship.life_support,
        cryo: ship.cryo, shields: ship.shields, repair: ship.repair,
        crystals: ship.total_crystals(), crew_count: ship.crew_count,
        day: ship.day, distance_au: ship.distance_au, bot_level: ship.bot_level,
    };
    let font = &font.0;
    commands.spawn(Node { width: Val::Percent(100.0), height: Val::Percent(100.0),
        flex_direction: FlexDirection::Row, ..default()
    }).with_children(|root| {
        dashboard::spawn_dashboard(root, font);
        games::spawn_game_cards(root, font, &ship_status);
    });

    commands.spawn(Node {
        position_type: PositionType::Absolute, width: Val::Percent(100.0),
        height: Val::Percent(100.0), ..default()
    }).with_children(|overlay| { anna::spawn_anna_panel(overlay, font); });

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

    commands.spawn(Node {
        position_type: PositionType::Absolute, right: Val::Px(6.0),
        bottom: Val::Px(4.0), ..default()
    }).with_child((
        Text::new(format!("Mission Control · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font: font.clone(), font_size: VERSION_FONT_M, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

fn twinkle_stars(time: Res<Time>, mut query: Query<(&StarTwinkle, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (star, mut tf) in query.iter_mut() {
        tf.scale = Vec3::splat(0.7 + 0.3 * (t * star.speed + star.phase).sin());
    }
}

fn final_voyage_click(
    query: Query<&Interaction, (Changed<Interaction>, With<endings::FinalVoyageBtn>)>,
    mut commands: Commands, gs: Res<GameState>,
    font: Res<MissionFont>, ending_q: Query<Entity, With<endings::EndingScreen>>,
) {
    if !ending_q.is_empty() { return; }
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed { endings::spawn_ending_screen(&mut commands, &font.0, &gs); }
    }
}

fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
    let (size, center, max_d) = (256u32, 128.0_f32, 128.0 * 1.2);
    let mut data = vec![0u8; (size * size * 4) as usize];
    for y in 0..size { for x in 0..size {
        let d = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
        let a = ((d / max_d).clamp(0.0, 1.0).powi(2) * 0.7 * 255.0) as u8;
        data[((y * size + x) * 4 + 3) as usize] = a;
    }}
    images.add(Image::new(Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default()))
}
