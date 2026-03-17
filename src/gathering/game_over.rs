// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

#[derive(Resource)]
pub struct FadeTimer {
    pub timer: f32,
    pub triggered: bool,
}

impl Default for FadeTimer {
    fn default() -> Self { Self { timer: 0.0, triggered: false } }
}

pub fn check_game_over(
    state: Res<ShipState>,
    gathering_state: Res<State<GatheringState>>,
    mut next_state: ResMut<NextState<GatheringState>>,
    mut fade: ResMut<FadeTimer>,
) {
    if *gathering_state.get() == GatheringState::Running && !state.alive && !fade.triggered {
        fade.triggered = true;
        fade.timer = 0.0;
    }
    if fade.triggered && fade.timer >= FADE_DURATION && *gathering_state.get() == GatheringState::Running {
        next_state.set(GatheringState::GameOver);
    }
}

pub fn update_fade(
    mut fade: ResMut<FadeTimer>,
    time: Res<Time>,
    mut fade_q: Query<&mut BackgroundColor, With<FadeOverlay>>,
) -> Result {
    if !fade.triggered { return Ok(()); }
    fade.timer = (fade.timer + time.delta_secs()).min(FADE_DURATION + 0.1);
    let alpha = (fade.timer / FADE_DURATION).clamp(0.0, 1.0);
    let mut bg = fade_q.single_mut()?;
    bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    Ok(())
}

#[derive(Component)]
pub struct FadeOverlay;

pub fn spawn_fade_overlay(commands: &mut Commands) {
    // Start fully black — fades in during the first second
    commands.spawn((
        FadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        ZIndex(10),
    ));
}

/// Resource for tracking the intro fade-in
#[derive(Resource)]
pub struct IntroFade(pub f32);

impl Default for IntroFade { fn default() -> Self { Self(0.0) } }

pub fn update_intro_fade(
    mut intro: ResMut<IntroFade>,
    time: Res<Time>,
    mut fade_q: Query<&mut BackgroundColor, With<FadeOverlay>>,
    fade: Res<FadeTimer>,
) {
    if fade.triggered { return; } // death fade takes over
    if intro.0 >= 1.0 { return; }
    intro.0 = (intro.0 + time.delta_secs() / FADE_DURATION).min(1.0);
    let alpha = 1.0 - intro.0;
    for mut bg in fade_q.iter_mut() {
        bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
    }
}

pub fn spawn_game_over_screen(
    mut commands: Commands,
    state: Res<ShipState>,
    gathering_state: Res<State<GatheringState>>,
    existing: Query<Entity, With<GameOverScreen>>,
    font: Res<GatheringFont>,
    fade: Res<FadeTimer>,
    mut best: ResMut<BestStats>,
) {
    if *gathering_state.get() != GatheringState::GameOver { return; }
    if !existing.is_empty() { return; }
    if !fade.triggered { return; }
    let new_record = super::stats::save_session(&state, &mut best);

    let title_font = TextFont { font: font.0.clone(), font_size: STATS_TITLE_FONT, ..default() };
    let stat_font = TextFont { font: font.0.clone(), font_size: STATS_FONT, ..default() };
    let value_color = Color::srgb(STATS_SUCCESS_COLOR.0, STATS_SUCCESS_COLOR.1, STATS_SUCCESS_COLOR.2);
    let label_color = Color::srgba(0.7, 0.7, 0.75, 0.9);

    let au = state.distance * 0.01;
    let days = (state.elapsed_time * 0.1) as u32;

    commands.spawn((
        GameOverScreen,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ZIndex(20),
    )).with_children(|parent| {
        parent.spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(STATS_CARD_PAD)),
            row_gap: Val::Px(STATS_CARD_GAP),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        }, BackgroundColor(Color::srgb(STATS_CARD_BG.0, STATS_CARD_BG.1, STATS_CARD_BG.2)),
        )).with_children(|card| {
            card.spawn((Text::new("Ship Stopped for Repairs"), title_font.clone(),
                TextColor(value_color)));
            card.spawn(Node { height: Val::Px(8.0), ..default() });
            stat_row(card, "Distance", &format!("{:.1} AU", au), &stat_font, label_color, value_color);
            stat_row(card, "Time", &format!("{} days", days), &stat_font, label_color, value_color);
            let crys = if state.crystals >= 1_000 { format!("{}K", state.crystals / 1_000) } else { format!("{}", state.crystals) };
            stat_row(card, "Crystals", &crys, &stat_font, label_color, value_color);
            stat_row(card, "Hits taken", &format!("{}", state.hits_taken), &stat_font, label_color, value_color);
            if new_record {
                card.spawn(Node { height: Val::Px(4.0), ..default() });
                card.spawn((
                    Text::new("New Record!"),
                    TextFont { font: font.0.clone(), font_size: 24.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                ));
            }
            card.spawn(Node { height: Val::Px(8.0), ..default() });
            card.spawn((
                Button, TryAgainButton,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
            )).with_child((Text::new("Try Again"), stat_font.clone(), TextColor(Color::WHITE)));
        });
    });
}

fn stat_row(parent: &mut ChildSpawnerCommands, label: &str, value: &str, font: &TextFont, label_color: Color, value_color: Color) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Px(240.0),
        ..default()
    }).with_children(|row| {
        row.spawn((Text::new(label), font.clone(), TextColor(label_color)));
        row.spawn((Text::new(value), font.clone(), TextColor(value_color)));
    });
}

pub fn try_again_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<TryAgainButton>)>,
) {
    for (interaction, mut bg) in query.iter_mut() {
        bg.0 = match interaction {
            Interaction::Pressed => Color::srgb(0.15, 0.4, 0.6),
            Interaction::Hovered => Color::srgb(0.3, 0.3, 0.45),
            Interaction::None => Color::srgb(0.2, 0.2, 0.3),
        };
    }
}

pub fn try_again_interaction(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<TryAgainButton>)>,
    mut next_state: ResMut<NextState<GatheringState>>,
    mut state: ResMut<ShipState>,
    mut shake: ResMut<ScreenShake>,
    mut fade: ResMut<FadeTimer>,
    game_over_q: Query<Entity, With<GameOverScreen>>,
    mut fade_bg_q: Query<&mut BackgroundColor, With<FadeOverlay>>,
    asteroid_q: Query<Entity, With<Asteroid>>,
    crystal_q: Query<Entity, With<CrystalCloud>>,
    particle_q: Query<Entity, With<CrystalParticle>>,
    spark_q: Query<Entity, With<Spark>>,
    engine_q: Query<Entity, With<EngineParticle>>,
    float_q: Query<Entity, With<FloatingText>>,
    mut difficulty: ResMut<Difficulty>,
    mut hit_flash: ResMut<HitFlash>,
    mut commands: Commands,
) {
    for interaction in interaction_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        *state = ShipState::default();
        *shake = ScreenShake::default();
        *fade = FadeTimer::default();
        *difficulty = Difficulty::default();
        *hit_flash = HitFlash::default();
        next_state.set(GatheringState::Running);
        for entity in game_over_q.iter() { commands.entity(entity).despawn(); }
        for mut bg in fade_bg_q.iter_mut() {
            bg.0 = Color::srgba(0.0, 0.0, 0.0, 0.0);
        }
        for entity in asteroid_q.iter() { commands.entity(entity).despawn(); }
        for entity in crystal_q.iter() { commands.entity(entity).despawn(); }
        for entity in particle_q.iter() { commands.entity(entity).despawn(); }
        for entity in spark_q.iter() { commands.entity(entity).despawn(); }
        for entity in engine_q.iter() { commands.entity(entity).despawn(); }
        for entity in float_q.iter() { commands.entity(entity).despawn(); }
    }
}
