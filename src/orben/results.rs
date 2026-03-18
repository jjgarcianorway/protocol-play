// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::rules;

/// Spawn the results screen overlay.
pub fn spawn_results_screen(
    mut commands: Commands,
    font: Res<OrbenFont>,
    state: Res<OrbGameState>,
) {
    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    let (player_wins, npc_wins) = rules::determine_winner(
        state.player_captured_orbs,
        state.npc_captured_orbs,
        state.player_treasure,
        state.npc_treasure,
    );

    let result_text = if player_wins {
        "You Win!"
    } else if npc_wins {
        "NPC Wins"
    } else {
        "It's a Tie"
    };

    let result_color = if player_wins {
        Color::srgba(0.3, 1.0, 0.4, 1.0)
    } else if npc_wins {
        Color::srgba(1.0, 0.4, 0.3, 1.0)
    } else {
        Color::srgba(0.7, 0.7, 0.7, 1.0)
    };

    let player_total = state.player_captured_orbs + state.player_treasure;
    let npc_total = state.npc_captured_orbs + state.npc_treasure;

    let label_color = Color::srgba(0.7, 0.7, 0.75, 0.9);
    let value_color = Color::srgba(0.6, 0.9, 0.7, 1.0);

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ResultsScreen,
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(12.0),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                RESULTS_BG_O.0, RESULTS_BG_O.1,
                RESULTS_BG_O.2, RESULTS_BG_O.3,
            )),
        )).with_children(|card| {
            // Result
            card.spawn((
                Text::new(result_text),
                tf(RESULTS_TITLE_FONT_O),
                TextColor(result_color),
            ));
            card.spawn(Node { height: Val::Px(8.0), ..default() });

            // Player stats
            stat_row(card, "Your orbs", &format!("{}", state.player_captured_orbs),
                     &tf(RESULTS_FONT_O), label_color, value_color);
            stat_row(card, "Your bonus", &format!("{}", state.player_treasure),
                     &tf(RESULTS_FONT_O), label_color, value_color);
            stat_row(card, "Your total", &format!("{}", player_total),
                     &tf(RESULTS_FONT_O), label_color,
                     Color::srgba(0.9, 0.85, 0.4, 1.0));

            card.spawn(Node { height: Val::Px(4.0), ..default() });

            stat_row(card, "NPC orbs", &format!("{}", state.npc_captured_orbs),
                     &tf(RESULTS_FONT_O), label_color, value_color);
            stat_row(card, "NPC bonus", &format!("{}", state.npc_treasure),
                     &tf(RESULTS_FONT_O), label_color, value_color);
            stat_row(card, "NPC total", &format!("{}", npc_total),
                     &tf(RESULTS_FONT_O), label_color,
                     Color::srgba(1.0, 0.5, 0.3, 1.0));

            card.spawn(Node { height: Val::Px(4.0), ..default() });

            stat_row(card, "Rounds", &format!("{}", state.round_number),
                     &tf(RESULTS_FONT_O), label_color, value_color);

            if player_total >= WIN_THRESHOLD {
                card.spawn((
                    Text::new("Guaranteed victory (21+)!"),
                    tf(SMALL_FONT_O),
                    TextColor(Color::srgba(1.0, 0.85, 0.2, 0.8)),
                ));
            }

            card.spawn(Node { height: Val::Px(8.0), ..default() });

            // Play Again button
            card.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(
                    BTN_BG_O.0, BTN_BG_O.1, BTN_BG_O.2,
                )),
                PlayAgainButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play Again"),
                    tf(RESULTS_BTN_FONT_O),
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

fn stat_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value: &str,
    font: &TextFont,
    label_color: Color,
    value_color: Color,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Px(260.0),
        ..default()
    }).with_children(|row| {
        row.spawn((Text::new(label), font.clone(), TextColor(label_color)));
        row.spawn((Text::new(value), font.clone(), TextColor(value_color)));
    });
}

/// Handle play again button interaction.
pub fn play_again_interaction(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayAgainButton>),
    >,
    mut next_state: ResMut<NextState<OrbenPhase>>,
    mut state: ResMut<OrbGameState>,
    results_q: Query<Entity, With<ResultsScreen>>,
    mut commands: Commands,
) {
    for (interaction, mut bg) in interaction_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Reset game
                *state = OrbGameState::default();
                super::deck::deal_initial(&mut state);
                state.turn_phase = TurnPhase::RondaCheck;
                state.phase_timer = 0.1;
                state.player_turn = rand::random();
                // Remove results screen
                for entity in results_q.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(OrbenPhase::Playing);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_HOVER_O.0, BTN_HOVER_O.1, BTN_HOVER_O.2,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_BG_O.0, BTN_BG_O.1, BTN_BG_O.2,
                ));
            }
        }
    }
}
