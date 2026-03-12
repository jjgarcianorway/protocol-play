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
) {
    if !fade.triggered { return; }
    fade.timer = (fade.timer + time.delta_secs()).min(FADE_DURATION + 0.1);
    let alpha = (fade.timer / FADE_DURATION).clamp(0.0, 1.0);
    let mut bg = fade_q.single_mut();
    bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
}

#[derive(Component)]
pub struct FadeOverlay;

pub fn spawn_fade_overlay(commands: &mut Commands) {
    commands.spawn((
        FadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(10),
    ));
}

pub fn spawn_game_over_screen(
    mut commands: Commands,
    state: Res<ShipState>,
    gathering_state: Res<State<GatheringState>>,
    existing: Query<Entity, With<GameOverScreen>>,
    font: Res<GatheringFont>,
) {
    if *gathering_state.get() != GatheringState::GameOver { return; }
    if !existing.is_empty() { return; }

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
            ..default()
        }, BackgroundColor(Color::srgb(STATS_CARD_BG.0, STATS_CARD_BG.1, STATS_CARD_BG.2)),
           BorderRadius::all(Val::Px(12.0)),
        )).with_children(|card| {
            card.spawn((Text::new("Ship Stopped for Repairs"), title_font.clone(),
                TextColor(value_color)));
            card.spawn(Node { height: Val::Px(8.0), ..default() });
            stat_row(card, "Distance", &format!("{:.1} AU", au), &stat_font, label_color, value_color);
            stat_row(card, "Time", &format!("{} days", days), &stat_font, label_color, value_color);
            stat_row(card, "Hits taken", &format!("{}", state.hits_taken), &stat_font, label_color, value_color);
            card.spawn(Node { height: Val::Px(8.0), ..default() });
            card.spawn((
                Button, TryAgainButton,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                BorderRadius::all(Val::Px(6.0)),
            )).with_child((Text::new("Try Again"), stat_font.clone(), TextColor(Color::WHITE)));
        });
    });
}

fn stat_row(parent: &mut ChildBuilder, label: &str, value: &str, font: &TextFont, label_color: Color, value_color: Color) {
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

pub fn try_again_interaction(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<TryAgainButton>)>,
    mut next_state: ResMut<NextState<GatheringState>>,
    mut state: ResMut<ShipState>,
    mut shake: ResMut<ScreenShake>,
    mut fade: ResMut<FadeTimer>,
    game_over_q: Query<Entity, With<GameOverScreen>>,
    fade_q: Query<Entity, With<FadeOverlay>>,
    asteroid_q: Query<Entity, With<Asteroid>>,
    mut commands: Commands,
) {
    for interaction in interaction_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        *state = ShipState::default();
        *shake = ScreenShake::default();
        *fade = FadeTimer::default();
        next_state.set(GatheringState::Running);
        for entity in game_over_q.iter() { commands.entity(entity).despawn_recursive(); }
        for entity in fade_q.iter() {
            if let Some(mut bg) = commands.get_entity(entity) {
                bg.insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)));
            }
        }
        for entity in asteroid_q.iter() { commands.entity(entity).despawn(); }
    }
}

