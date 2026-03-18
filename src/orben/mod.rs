// SPDX-License-Identifier: GPL-3.0-or-later

#[allow(dead_code)]
mod constants;
#[allow(dead_code)]
mod types;
#[allow(dead_code)]
mod deck;
mod rules;
mod ui;
mod gameplay;
mod npc;
mod results;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use constants::*;
use types::*;

pub fn build_app(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(
        CLEAR_COLOR_O.0, CLEAR_COLOR_O.1, CLEAR_COLOR_O.2,
    )))
    .insert_resource(OrbGameState::default())
    .init_state::<OrbenPhase>()
    .add_systems(Startup, (
        setup_orben,
        ui::spawn_game_ui.after(setup_orben),
    ))
    .add_systems(Update, (
        gameplay::handle_orb_click,
        gameplay::process_turn_phases,
        sync_ui,
        animate_mesa_limpia,
        animate_se_cayo_bar,
        animate_stars,
        check_game_over,
    ).run_if(in_state(OrbenPhase::Playing)))
    .add_systems(OnEnter(OrbenPhase::Results), results::spawn_results_screen)
    .add_systems(Update,
        results::play_again_interaction.run_if(in_state(OrbenPhase::Results)),
    );
}

fn setup_orben(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<OrbGameState>,
) {
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
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
    ));

    // Version label
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(6.0),
        bottom: Val::Px(4.0),
        ..default()
    }).with_child((
        Text::new(format!("Orben · v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: VERSION_FONT_O, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));

    // Initialize deck and deal
    deck::deal_initial(&mut state);
    state.turn_phase = TurnPhase::RondaCheck;
    state.phase_timer = 0.1;
    state.player_turn = rand::random();
}

fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
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
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

/// Sync UI visuals with game state every frame.
fn sync_ui(
    mut commands: Commands,
    state: Res<OrbGameState>,
    font: Res<OrbenFont>,
    player_area_q: Query<Entity, With<PlayerHandArea>>,
    npc_area_q: Query<Entity, With<NpcHandArea>>,
    table_area_q: Query<Entity, With<TableArea>>,
    mut status_q: Query<(&mut Text, &mut TextColor), (With<StatusText>, Without<RondaText>)>,
    mut ronda_q: Query<(&mut Text, &mut TextColor), (With<RondaText>, Without<StatusText>)>,
    mut player_treasure_q: Query<&mut Text, (
        With<PlayerTreasureText>,
        Without<StatusText>, Without<RondaText>,
        Without<NpcTreasureText>, Without<DeckCountText>,
        Without<PlayerCapturedText>, Without<NpcCapturedText>,
    )>,
    mut npc_treasure_q: Query<&mut Text, (
        With<NpcTreasureText>,
        Without<StatusText>, Without<RondaText>,
        Without<PlayerTreasureText>, Without<DeckCountText>,
        Without<PlayerCapturedText>, Without<NpcCapturedText>,
    )>,
    mut deck_q: Query<&mut Text, (
        With<DeckCountText>,
        Without<StatusText>, Without<RondaText>,
        Without<PlayerTreasureText>, Without<NpcTreasureText>,
        Without<PlayerCapturedText>, Without<NpcCapturedText>,
    )>,
    mut player_cap_q: Query<&mut Text, (
        With<PlayerCapturedText>,
        Without<StatusText>, Without<RondaText>,
        Without<PlayerTreasureText>, Without<NpcTreasureText>,
        Without<DeckCountText>, Without<NpcCapturedText>,
    )>,
    mut npc_cap_q: Query<&mut Text, (
        With<NpcCapturedText>,
        Without<StatusText>, Without<RondaText>,
        Without<PlayerTreasureText>, Without<NpcTreasureText>,
        Without<DeckCountText>, Without<PlayerCapturedText>,
    )>,
) {
    if !state.is_changed() {
        return;
    }

    // Rebuild orb nodes
    let Ok(player_area) = player_area_q.single() else { return };
    let Ok(npc_area) = npc_area_q.single() else { return };
    let Ok(table_area) = table_area_q.single() else { return };

    ui::rebuild_orbs(
        &mut commands, &state, &font.0,
        player_area, npc_area, table_area,
    );

    // Status text
    for (mut text, mut color) in status_q.iter_mut() {
        *text = Text::new(&state.status_message);
        let c = if state.turn_phase == TurnPhase::SeCayoWindow {
            Color::srgba(1.0, 0.4, 0.3, 1.0) // Urgent red
        } else {
            Color::srgba(1.0, 0.9, 0.5, 0.9)
        };
        *color = TextColor(c);
    }

    // Ronda text
    for (mut text, mut color) in ronda_q.iter_mut() {
        if let Some(ref msg) = state.ronda_message {
            *text = Text::new(msg);
            *color = TextColor(Color::srgba(1.0, 0.85, 0.2, 0.9));
        } else {
            *text = Text::new("");
            *color = TextColor(Color::srgba(1.0, 0.85, 0.2, 0.0));
        }
    }

    // Treasures
    for mut text in player_treasure_q.iter_mut() {
        *text = Text::new(format!("{}", state.player_treasure));
    }
    for mut text in npc_treasure_q.iter_mut() {
        *text = Text::new(format!("{}", state.npc_treasure));
    }

    // Deck count
    for mut text in deck_q.iter_mut() {
        *text = Text::new(format!("Deck: {}", state.deck.len()));
    }

    // Captured orbs
    for mut text in player_cap_q.iter_mut() {
        *text = Text::new(format!("({} orbs)", state.player_captured_orbs));
    }
    for mut text in npc_cap_q.iter_mut() {
        *text = Text::new(format!("({} orbs)", state.npc_captured_orbs));
    }
}

/// Animate mesa limpia golden flash.
fn animate_mesa_limpia(
    state: Res<OrbGameState>,
    mut flash_q: Query<&mut BackgroundColor, With<MesaLimpiaFlash>>,
) {
    for mut bg in flash_q.iter_mut() {
        if state.mesa_limpia_flash > 0.0 {
            let alpha = (state.mesa_limpia_flash / MESA_LIMPIA_DURATION) * MESA_LIMPIA_COLOR.3;
            *bg = BackgroundColor(Color::srgba(
                MESA_LIMPIA_COLOR.0, MESA_LIMPIA_COLOR.1,
                MESA_LIMPIA_COLOR.2, alpha,
            ));
        } else {
            *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
        }
    }
}

/// Animate se cayo timer bar.
fn animate_se_cayo_bar(
    state: Res<OrbGameState>,
    mut timer_q: Query<&mut Visibility, With<SeCayoTimer>>,
    mut bar_q: Query<(&mut Node, &mut BackgroundColor), With<SeCayoTimerBar>>,
    time: Res<Time>,
) {
    let show = state.se_cayo_timer.is_some();
    for mut vis in timer_q.iter_mut() {
        *vis = if show { Visibility::Visible } else { Visibility::Hidden };
    }

    if let Some(remaining) = state.se_cayo_timer {
        let pct = (remaining / SE_CAYO_DURATION * 100.0).clamp(0.0, 100.0);
        let pulse = (time.elapsed_secs() * SE_CAYO_PULSE_SPEED).sin() * 0.15 + 0.85;
        for (mut node, mut bg) in bar_q.iter_mut() {
            node.width = Val::Percent(pct);
            let urgency = 1.0 - (remaining / SE_CAYO_DURATION);
            *bg = BackgroundColor(Color::srgba(
                0.8 + urgency * 0.2,
                0.4 - urgency * 0.3,
                0.3 - urgency * 0.2,
                pulse,
            ));
        }
    }
}

/// Animate star dots with subtle twinkle.
fn animate_stars(
    time: Res<Time>,
    mut query: Query<&mut BackgroundColor, With<StarDot>>,
) {
    let t = time.elapsed_secs();
    let mut i = 0u32;
    for mut bg in query.iter_mut() {
        let phase = i as f32 * 2.7;
        let twinkle = ((t * 1.5 + phase).sin() * 0.5 + 0.5).clamp(0.2, 1.0);
        let base_alpha = (i as f32 * 0.37).sin().abs() * 0.25 + 0.1;
        *bg = BackgroundColor(Color::srgba(0.7, 0.75, 1.0, base_alpha * twinkle));
        i += 1;
    }
}

/// Check if game should transition to results.
fn check_game_over(
    state: Res<OrbGameState>,
    mut next_state: ResMut<NextState<OrbenPhase>>,
) {
    if state.turn_phase == TurnPhase::GameOver {
        next_state.set(OrbenPhase::Results);
    }
}
