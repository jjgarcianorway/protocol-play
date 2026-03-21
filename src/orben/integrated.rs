// SPDX-License-Identifier: GPL-3.0-or-later
//! Integrated mode: registers Orben systems under GameScene::Orben.

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use super::constants::*;
use super::types::*;
use super::{gameplay, ui, results, deck};
use super::{sync_ui, animate_mesa_limpia, animate_se_cayo_bar, animate_stars};
use super::{check_game_over, create_vignette};
use crate::mission::types::GameScene;

/// Run condition: GameScene is Orben AND OrbenPhase is Playing.
fn orben_playing(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<OrbenPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Orben)
        && phase.is_some_and(|p| *p.get() == OrbenPhase::Playing)
}

/// Run condition: GameScene is Orben AND OrbenPhase is Results.
fn orben_results(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<OrbenPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Orben)
        && phase.is_some_and(|p| *p.get() == OrbenPhase::Results)
}

/// Register all Orben systems for integrated mode.
pub fn register_integrated_systems(app: &mut App) {
    app.init_state::<OrbenPhase>()
    .add_systems(OnEnter(GameScene::Orben), (
        enter_orben,
        ui::spawn_game_ui.after(enter_orben),
    ))
    .add_systems(OnExit(GameScene::Orben), exit_orben)
    .add_systems(Update, (
        gameplay::handle_orb_click,
        gameplay::process_turn_phases,
        sync_ui,
        animate_mesa_limpia,
        animate_se_cayo_bar,
        animate_stars,
        check_game_over,
    ).run_if(orben_playing))
    .add_systems(OnEnter(OrbenPhase::Results), results::spawn_results_screen)
    .add_systems(Update, (
        results::play_again_interaction,
        esc_return_to_dashboard,
    ).run_if(orben_results))
    .add_systems(Update,
        esc_return_to_dashboard.run_if(orben_playing),
    );
}

/// Enter Orben scene.
fn enter_orben(
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
        CLEAR_COLOR_O.0, CLEAR_COLOR_O.1, CLEAR_COLOR_O.2,
    ));

    // Initialize game state
    let mut state = OrbGameState::default();
    deck::deal_initial(&mut state);
    state.turn_phase = TurnPhase::RondaCheck;
    state.phase_timer = 0.1;
    state.player_turn = rand::random();
    commands.insert_resource(state);

    // Camera
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: BLOOM_INTENSITY_O,
            low_frequency_boost: BLOOM_LF_BOOST_O,
            low_frequency_boost_curvature: 0.7,
            high_pass_frequency: 1.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbenEntity,
    ));

    // Font
    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(OrbenFont(font.clone()));

    // Vignette
    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
        OrbenEntity,
    ));

    // Version label
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0), bottom: Val::Px(4.0), ..default()
        },
        OrbenEntity,
    )).with_child((
        Text::new(format!("Orben · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: VERSION_FONT_O, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

/// Exit Orben scene.
fn exit_orben(
    mut commands: Commands,
    all_q: Query<Entity, Or<(
        With<OrbenEntity>, With<ResultsScreen>, With<OrbNode>,
        With<StarDot>, With<MesaLimpiaFlash>, With<SeCayoTimer>,
        With<RondaGlow>,
    )>>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
    mut clear_color: ResMut<ClearColor>,
) {
    for entity in all_q.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<OrbGameState>();
    commands.remove_resource::<OrbenFont>();

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
