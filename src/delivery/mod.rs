// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod pods;
mod ui;
mod effects;
mod results;
pub mod anna;
#[cfg(feature = "full")]
pub mod integrated;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use constants::*;
use types::*;

pub fn build_app(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(
        CLEAR_COLOR_D.0, CLEAR_COLOR_D.1, CLEAR_COLOR_D.2,
    )))
    .insert_resource(DeliveryState::default())
    .insert_resource(crate::anna_comments::AnnaComments::default())
    .init_state::<DeliveryPhase>()
    .add_systems(Startup, (setup_delivery, anna::setup_delivery_anna.after(setup_delivery)))
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
    ).run_if(in_state(DeliveryPhase::Playing)))
    .add_systems(OnEnter(DeliveryPhase::Results), results::spawn_results_screen)
    .add_systems(Update,
        results::return_button_interaction.run_if(in_state(DeliveryPhase::Results)),
    );
}

fn setup_delivery(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<DeliveryState>,
) {
    // Load GameState and set pod count from resource levels
    let gs = crate::save_state::load_game_state();
    let resource_sum = (gs.power + gs.life_support + gs.cryo + gs.shields + gs.repair) as u32;
    let pod_count = if resource_sum > 0 {
        resource_sum.clamp(MIN_PODS, MAX_PODS)
    } else {
        TOTAL_PODS
    };
    state.total_pods = pod_count;

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
    ));

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());

    // Insert font resource immediately for use by other systems
    commands.insert_resource(DeliveryFont(font.clone()));

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

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(6.0),
        bottom: Val::Px(4.0),
        ..default()
    }).with_child((
        Text::new(format!("The Delivery · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font: font.clone(), font_size: 11.0, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));

    // Spawn the game UI directly (font handle available as local)
    ui::spawn_delivery_ui_with_font(&mut commands, &font, &mut state);
}

pub(crate) fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
    let size = 256u32;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;
    for y in 0..size {
        for x in 0..size {
            let dx = (x as f32 - center) / center;
            let dy = (y as f32 - center) / center;
            let dist = (dx * dx + dy * dy).sqrt().clamp(0.0, 1.0);
            let alpha = if dist < 0.5 { 0.0 }
                else { ((dist - 0.5) * 2.0).powi(2) * 0.6 };
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 0;
            data[idx + 1] = 0;
            data[idx + 2] = 0;
            data[idx + 3] = (alpha * 255.0) as u8;
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb,
        default(),
    ))
}
