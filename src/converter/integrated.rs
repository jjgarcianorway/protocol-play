// SPDX-License-Identifier: GPL-3.0-or-later
//! Integrated mode: registers Converter systems under GameScene::Converter.

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use super::constants::*;
use super::types::*;
use super::{grid, ui, effects, results, anna};
use super::{handle_hover, handle_click, process_grid_phases};
use super::{update_chain_label, check_round_complete, create_vignette};
use crate::mission::types::GameScene;

/// Run condition: GameScene is Converter AND ConverterPhase is Processing.
fn converter_processing(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<ConverterPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Converter)
        && phase.is_some_and(|p| *p.get() == ConverterPhase::Processing)
}

/// Run condition: GameScene is Converter AND ConverterPhase is Results.
fn converter_results(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<ConverterPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Converter)
        && phase.is_some_and(|p| *p.get() == ConverterPhase::Results)
}

/// Register all Converter systems for integrated mode.
pub fn register_integrated_systems(app: &mut App) {
    app.init_state::<ConverterPhase>()
    .add_systems(OnEnter(GameScene::Converter), (
        enter_converter,
        ui::spawn_converter_ui.after(enter_converter),
        anna::setup_converter_anna.after(enter_converter),
    ))
    .add_systems(OnExit(GameScene::Converter), exit_converter)
    .add_systems(Update, (
        handle_hover,
        handle_click.after(handle_hover),
        process_grid_phases.after(handle_click),
        ui::sync_grid_visuals.after(process_grid_phases),
        ui::sync_tank_visuals,
        ui::sync_pile_visuals,
        ui::detect_tank_changes.after(ui::sync_tank_visuals),
        ui::animate_tank_flashes,
        effects::update_pop_particles,
        effects::animate_tank_floats,
        effects::animate_stars,
        update_chain_label.after(handle_hover),
        check_round_complete.after(process_grid_phases),
        crate::anna_comments::tick_anna_comments,
        anna::converter_anna_reactive,
    ).run_if(converter_processing))
    .add_systems(OnEnter(ConverterPhase::Results), results::spawn_results_screen)
    .add_systems(Update, (
        results::return_button_interaction,
        esc_return_to_dashboard,
    ).run_if(converter_results))
    .add_systems(Update,
        esc_return_to_dashboard.run_if(converter_processing),
    );
}

/// Enter Converter scene.
fn enter_converter(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut clear_color: ResMut<ClearColor>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
) {
    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }

    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_C.0, CLEAR_COLOR_C.1, CLEAR_COLOR_C.2,
    ));

    // Insert game resources
    let gs = crate::save_state::load_game_state();
    let crystal_count = gs.total_crystals();
    let pile_size = if crystal_count > 0 {
        crystal_count.max(MIN_PILE_SIZE)
    } else {
        INITIAL_PILE_SIZE
    };
    let mut pile = CrystalPile::default();
    pile.total = pile_size;
    pile.remaining = pile_size;

    commands.insert_resource(GridState::default());
    commands.insert_resource(pile);
    commands.insert_resource(ResourceTanks::default());
    commands.insert_resource(ConversionStats::default());
    commands.insert_resource(HoveredGroup::default());
    commands.insert_resource(crate::anna_comments::AnnaComments::default());

    // Camera
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: BLOOM_INTENSITY_C,
            low_frequency_boost: BLOOM_LF_BOOST_C,
            low_frequency_boost_curvature: 0.7,
            high_pass_frequency: 1.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ConverterEntity,
    ));

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(ConverterFont(font.clone()));

    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
        ConverterEntity,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0), bottom: Val::Px(4.0), ..default()
        },
        ConverterEntity,
    )).with_child((
        Text::new(format!("The Converter · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: 11.0, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

/// Exit Converter scene.
fn exit_converter(
    mut commands: Commands,
    all_q: Query<Entity, Or<(
        With<ConverterEntity>, With<ConverterRoot>, With<GridCell>,
        With<ResultsScreen>, With<PopParticle>, With<TankFloatText>,
        With<StarDot>,
    )>>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
    mut clear_color: ResMut<ClearColor>,
) {
    for entity in all_q.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<GridState>();
    commands.remove_resource::<CrystalPile>();
    commands.remove_resource::<ResourceTanks>();
    commands.remove_resource::<ConversionStats>();
    commands.remove_resource::<HoveredGroup>();
    commands.remove_resource::<ConverterFont>();
    commands.remove_resource::<crate::anna_comments::AnnaComments>();

    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    use crate::mission::constants::*;
    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2,
    ));
}

/// ESC returns to dashboard.
fn esc_return_to_dashboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_scene: ResMut<NextState<GameScene>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_scene.set(GameScene::Dashboard);
    }
}
