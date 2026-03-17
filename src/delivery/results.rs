// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Rating string from efficiency percentage.
fn efficiency_rating(eff: f32) -> &'static str {
    if eff >= 95.0 { "S - Perfect" }
    else if eff >= 85.0 { "A - Excellent" }
    else if eff >= 70.0 { "B - Great" }
    else if eff >= 50.0 { "C - Good" }
    else if eff >= 30.0 { "D - Fair" }
    else { "E - Wasteful" }
}

/// Spawn the results screen overlay.
pub fn spawn_results_screen(
    mut commands: Commands,
    font: Res<DeliveryFont>,
    state: Res<DeliveryState>,
) {
    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    let eff = state.efficiency();
    let rating = efficiency_rating(eff);
    let delivered: u32 = state.score.iter().sum();

    // Save delivery results to cross-game state
    {
        let mut gs = crate::save_state::load_game_state();
        let efficiency_factor = eff / 100.0;
        gs.power = (gs.power + state.score[0] as f32 * efficiency_factor * 0.5).clamp(0.0, 100.0);
        gs.life_support = (gs.life_support + state.score[1] as f32 * efficiency_factor * 0.5).clamp(0.0, 100.0);
        gs.cryo = (gs.cryo + state.score[2] as f32 * efficiency_factor * 0.5).clamp(0.0, 100.0);
        gs.shields = (gs.shields + state.score[3] as f32 * efficiency_factor * 0.5).clamp(0.0, 100.0);
        gs.repair = (gs.repair + state.score[4] as f32 * efficiency_factor * 0.5).clamp(0.0, 100.0);
        crate::save_state::save_game_state(&gs);
    }

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
                row_gap: Val::Px(14.0),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                RESULTS_BG_D.0, RESULTS_BG_D.1, RESULTS_BG_D.2, RESULTS_BG_D.3,
            )),
        )).with_children(|panel| {
            panel.spawn((
                Text::new("Delivery Complete"),
                tf(RESULTS_TITLE_FONT_D),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.95)),
            ));

            let lines = vec![
                format!("Pods delivered: {} / {}", delivered, state.total_pods),
                format!("Wasted (wrong slot): {}", state.wasted),
                format!("Missed (no route): {}", state.missed),
                format!("Best streak: {}", state.best_streak),
                format!("Efficiency: {:.0}%  ({})", eff, rating),
            ];
            for line in &lines {
                panel.spawn((
                    Text::new(line.clone()),
                    tf(RESULTS_FONT_D),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ));
            }

            // Per-resource breakdown
            panel.spawn((
                Text::new("Resources delivered:"),
                tf(RESULTS_FONT_D),
                TextColor(Color::srgba(1.0, 1.0, 0.7, 0.9)),
            ));
            for i in 0..5 {
                let (cr, cg, cb) = POD_COLORS[i];
                panel.spawn((
                    Text::new(format!(
                        "  {} {} -- {}",
                        RESOURCE_ICONS[i], RESOURCE_NAMES[i], state.score[i],
                    )),
                    tf(RESULTS_FONT_D),
                    TextColor(Color::srgb(cr, cg, cb)),
                ));
            }

            // Return button
            panel.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                    margin: UiRect::top(Val::Px(12.0)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(BTN_BG_D.0, BTN_BG_D.1, BTN_BG_D.2)),
                ReturnButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play Again"),
                    tf(RESULTS_BTN_FONT_D),
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

/// Handle return button interaction — reset the game.
pub fn return_button_interaction(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ReturnButton>),
    >,
    mut next_state: ResMut<NextState<DeliveryPhase>>,
    mut state: ResMut<DeliveryState>,
    results_q: Query<Entity, With<ResultsScreen>>,
    root_q: Query<Entity, With<DeliveryRoot>>,
    pod_q: Query<Entity, With<Pod>>,
    trail_q: Query<Entity, With<PodTrail>>,
    popup_q: Query<Entity, With<StreakPopup>>,
    star_q: Query<Entity, With<StarDotD>>,
    mut commands: Commands,
) {
    for (interaction, mut bg) in interaction_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *state = DeliveryState::default();

                for entity in results_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in root_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in pod_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in trail_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in popup_q.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in star_q.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(DeliveryPhase::Playing);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_HOVER_D.0, BTN_HOVER_D.1, BTN_HOVER_D.2,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_BG_D.0, BTN_BG_D.1, BTN_BG_D.2,
                ));
            }
        }
    }
}
