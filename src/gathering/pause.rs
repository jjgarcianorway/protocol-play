// SPDX-License-Identifier: GPL-3.0-or-later
//! Pause overlay for The Gathering — ESC to pause/resume.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

#[derive(Component)]
pub struct ResumeButton;

/// Tracks the pause overlay fade-in progress (0.0 = invisible, 1.0 = fully visible).
#[derive(Component)]
pub struct PauseFadeIn(pub f32);

pub fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut paused: ResMut<Paused>,
    state: Res<ShipState>,
    gathering_state: Res<State<GatheringState>>,
    mut commands: Commands,
    pause_screen_q: Query<Entity, With<PauseScreen>>,
    font: Res<GatheringFont>,
) {
    if *gathering_state.get() != GatheringState::Running { return; }
    if !state.alive { return; }
    if !keys.just_pressed(KeyCode::Escape) { return; }

    paused.0 = !paused.0;

    if paused.0 {
        spawn_pause_overlay(&mut commands, &font.0);
    } else {
        for entity in pause_screen_q.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn resume_button_interaction(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut paused: ResMut<Paused>,
    pause_screen_q: Query<Entity, With<PauseScreen>>,
    mut commands: Commands,
) {
    for interaction in interaction_q.iter() {
        if *interaction != Interaction::Pressed { continue; }
        paused.0 = false;
        for entity in pause_screen_q.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn resume_button_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
) {
    for (interaction, mut bg) in query.iter_mut() {
        bg.0 = match interaction {
            Interaction::Pressed => Color::srgb(0.15, 0.4, 0.6),
            Interaction::Hovered => Color::srgb(0.3, 0.3, 0.45),
            Interaction::None => Color::srgb(0.2, 0.2, 0.3),
        };
    }
}

/// Smoothly fade in the pause overlay instead of popping it in.
pub fn update_pause_fade(
    mut query: Query<(&mut PauseFadeIn, &mut BackgroundColor), With<PauseScreen>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut fade, mut bg) in query.iter_mut() {
        if fade.0 >= 1.0 { continue; }
        fade.0 = (fade.0 + dt / PAUSE_FADE_IN_SECS).min(1.0);
        let alpha = fade.0 * PAUSE_OVERLAY_ALPHA;
        bg.0 = Color::srgba(0.0, 0.0, 0.05, alpha);
    }
}

fn spawn_pause_overlay(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        PauseScreen,
        PauseFadeIn(0.0),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.05, 0.0)),
        ZIndex(25),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("PAUSED"),
            TextFont { font: font.clone(), font_size: PAUSE_FONT, ..default() },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
        ));
        parent.spawn((
            Button, ResumeButton,
            Node {
                padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
        )).with_child((
            Text::new("Resume"),
            TextFont { font: font.clone(), font_size: STATS_FONT, ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("Press ESC to resume"),
            TextFont { font: font.clone(), font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.6, 0.6, 0.65, 0.7)),
        ));
    });
}
