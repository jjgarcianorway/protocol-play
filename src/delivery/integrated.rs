// SPDX-License-Identifier: GPL-3.0-or-later
//! Integrated mode: registers Delivery systems under GameScene::Delivery.

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use super::constants::*;
use super::types::*;
use super::{pods, ui, effects, results, anna, create_vignette};
use crate::mission::types::GameScene;

/// Run condition: GameScene is Delivery AND DeliveryPhase is Playing.
fn delivery_playing(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<DeliveryPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Delivery)
        && phase.is_some_and(|p| *p.get() == DeliveryPhase::Playing)
}

/// Run condition: GameScene is Delivery AND DeliveryPhase is Results.
fn delivery_results(
    scene: Option<Res<State<GameScene>>>,
    phase: Option<Res<State<DeliveryPhase>>>,
) -> bool {
    scene.is_some_and(|s| *s.get() == GameScene::Delivery)
        && phase.is_some_and(|p| *p.get() == DeliveryPhase::Results)
}

/// Register all Delivery systems for integrated mode.
pub fn register_integrated_systems(app: &mut App) {
    app.init_state::<DeliveryPhase>();
    app
    .add_systems(OnEnter(GameScene::Delivery), (
        enter_delivery,
        ui::respawn_delivery_ui.after(enter_delivery),
        anna::setup_delivery_anna.after(enter_delivery),
    ))
    .add_systems(OnExit(GameScene::Delivery), exit_delivery)
    .add_systems(OnEnter(DeliveryPhase::Playing), ui::respawn_delivery_ui)
    .add_systems(Update, (
        pods::spawn_pods,
        pods::move_pods,
        pods::route_pod_to_slot,
        pods::resolve_pods.after(pods::move_pods),
        pods::update_slot_flashes,
        pods::check_game_complete.after(pods::resolve_pods),
        ui::sync_hud,
        ui::highlight_slots,
        ui::fade_intro,
        effects::animate_stars,
        effects::animate_streak_popups,
        crate::anna_comments::tick_anna_comments,
        anna::delivery_anna_reactive,
    ).run_if(delivery_playing))
    .add_systems(OnEnter(DeliveryPhase::Results), results::spawn_results_screen)
    .add_systems(Update, (
        results::return_button_interaction,
    ).run_if(delivery_results));
}

/// Enter Delivery scene.
fn enter_delivery(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut clear_color: ResMut<ClearColor>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
    root_ui_q: Query<Entity, (With<Node>, Without<bevy::prelude::ChildOf>)>,
) {
    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
    for entity in root_ui_q.iter() { commands.entity(entity).insert(Visibility::Hidden); }

    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_D.0, CLEAR_COLOR_D.1, CLEAR_COLOR_D.2,
    ));

    // Initialize state from save
    let gs = crate::save_state::load_game_state();
    let resource_sum =
        (gs.power + gs.life_support + gs.cryo + gs.shields + gs.repair) as u32;
    let pod_count = if resource_sum > 0 {
        resource_sum.clamp(MIN_PODS, MAX_PODS)
    } else {
        TOTAL_PODS
    };
    let mut state = DeliveryState::default();
    state.total_pods = pod_count;
    commands.insert_resource(crate::anna_comments::AnnaComments::default());

    // Camera
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: BLOOM_INTENSITY_D,
            low_frequency_boost: BLOOM_LF_BOOST_D,
            low_frequency_boost_curvature: 0.7,
            high_pass_frequency: 1.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        DeliveryEntity,
    ));

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(DeliveryFont(font.clone()));

    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
        DeliveryEntity,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(6.0), bottom: Val::Px(4.0), ..default()
        },
        DeliveryEntity,
    )).with_child((
        Text::new(format!("The Delivery · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font: font.clone(), font_size: 11.0, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));

    commands.insert_resource(state);
}

/// Exit Delivery scene.
fn exit_delivery(
    mut commands: Commands,
    all_q: Query<Entity, Or<(
        With<DeliveryEntity>, With<DeliveryRoot>, With<Pod>,
        With<DepositSlot>, With<ResultsScreen>, With<PodVisual>,
        With<PodTrail>, With<StarDotD>, With<StreakPopup>,
    )>>,
    camera_q: Query<Entity, With<crate::mission::types::MissionCamera>>,
    root_ui_q: Query<Entity, (With<Node>, Without<bevy::prelude::ChildOf>)>,
    mut clear_color: ResMut<ClearColor>,
) {
    for entity in all_q.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<DeliveryState>();
    commands.remove_resource::<DeliveryFont>();
    commands.remove_resource::<crate::anna_comments::AnnaComments>();

    for entity in camera_q.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    for entity in root_ui_q.iter() {
        commands.entity(entity).insert(Visibility::Visible);
    }
    use crate::mission::constants::*;
    *clear_color = ClearColor(Color::srgb(
        CLEAR_COLOR_M.0, CLEAR_COLOR_M.1, CLEAR_COLOR_M.2,
    ));
}
