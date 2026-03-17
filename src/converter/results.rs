// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Calculate overall efficiency rating.
pub fn calc_efficiency(stats: &ConversionStats, tanks: &ResourceTanks) -> f32 {
    let total_resource: f32 = tanks.levels.iter().sum();
    if stats.total_converted == 0 { return 0.0; }
    (total_resource / stats.total_converted as f32 * 100.0).clamp(0.0, 300.0)
}

/// Rating string from efficiency percentage.
pub fn efficiency_rating(eff: f32) -> &'static str {
    if eff >= 250.0 { "S" }
    else if eff >= 200.0 { "A" }
    else if eff >= 150.0 { "B" }
    else if eff >= 100.0 { "C" }
    else { "D" }
}

/// Spawn results screen overlay — clean card, same style as Gathering.
pub fn spawn_results_screen(
    mut commands: Commands,
    font: Res<ConverterFont>,
    stats: Res<ConversionStats>,
    tanks: Res<ResourceTanks>,
) {
    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };
    let eff = calc_efficiency(&stats, &tanks);
    let rating = efficiency_rating(eff);

    save_converter_results(&stats, &tanks);

    let label_color = Color::srgba(0.7, 0.7, 0.75, 0.9);
    let value_color = Color::srgba(0.6, 0.9, 0.7, 1.0);

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
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
                RESULTS_BG.0, RESULTS_BG.1, RESULTS_BG.2, RESULTS_BG.3,
            )),
        )).with_children(|card| {
            // Title
            card.spawn((
                Text::new("Processing Complete"),
                tf(RESULTS_TITLE_FONT),
                TextColor(value_color),
            ));
            card.spawn(Node { height: Val::Px(8.0), ..default() });

            // Stats rows
            stat_row(card, "Crystals processed", &format!("{}", stats.total_converted),
                     &tf(RESULTS_FONT), label_color, value_color);
            stat_row(card, "Chains triggered", &format!("{}", stats.chains_triggered),
                     &tf(RESULTS_FONT), label_color, value_color);
            stat_row(card, "Best chain", &format!("{}", stats.best_chain),
                     &tf(RESULTS_FONT), label_color, value_color);
            stat_row(card, "Cascades", &format!("{}", stats.cascades),
                     &tf(RESULTS_FONT), label_color, value_color);
            stat_row(card, "Efficiency", &format!("{:.0}%", eff),
                     &tf(RESULTS_FONT), label_color, value_color);
            stat_row(card, "Rating", rating,
                     &tf(RESULTS_FONT), label_color,
                     rating_color(rating));

            card.spawn(Node { height: Val::Px(4.0), ..default() });

            // Resource breakdown
            card.spawn((
                Text::new("Resources produced"),
                tf(RESULTS_FONT),
                TextColor(Color::srgba(1.0, 1.0, 0.7, 0.8)),
            ));
            for i in 0..5 {
                let (cr, cg, cb) = CRYSTAL_COLORS[i];
                card.spawn((
                    Text::new(format!("  {} {} -- {:.1}",
                        RESOURCE_ICONS[i], RESOURCE_NAMES[i], tanks.levels[i])),
                    tf(RESULTS_FONT - 2.0),
                    TextColor(Color::srgb(cr, cg, cb)),
                ));
            }

            card.spawn(Node { height: Val::Px(8.0), ..default() });

            // Continue button
            card.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(BTN_BG_C.0, BTN_BG_C.1, BTN_BG_C.2)),
                ReturnButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Continue"),
                    tf(RESULTS_BTN_FONT),
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

fn stat_row(
    parent: &mut ChildSpawnerCommands, label: &str, value: &str,
    font: &TextFont, label_color: Color, value_color: Color,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Px(280.0),
        ..default()
    }).with_children(|row| {
        row.spawn((Text::new(label), font.clone(), TextColor(label_color)));
        row.spawn((Text::new(value), font.clone(), TextColor(value_color)));
    });
}

fn rating_color(rating: &str) -> Color {
    match rating {
        "S" => Color::srgb(1.0, 0.85, 0.2),
        "A" => Color::srgb(0.3, 1.0, 0.4),
        "B" => Color::srgb(0.5, 0.8, 1.0),
        "C" => Color::srgb(0.7, 0.7, 0.7),
        _ => Color::srgb(0.6, 0.4, 0.4),
    }
}

/// Save converter results to cross-game state.
pub fn save_converter_results(
    stats: &ConversionStats,
    tanks: &ResourceTanks,
) {
    if stats.total_converted == 0 { return; }
    let mut gs = crate::save_state::load_game_state();
    gs.power = (gs.power + tanks.levels[0]).clamp(0.0, 100.0);
    gs.life_support = (gs.life_support + tanks.levels[1]).clamp(0.0, 100.0);
    gs.cryo = (gs.cryo + tanks.levels[2]).clamp(0.0, 100.0);
    gs.shields = (gs.shields + tanks.levels[3]).clamp(0.0, 100.0);
    gs.repair = (gs.repair + tanks.levels[4]).clamp(0.0, 100.0);
    let used = stats.total_converted;
    let per_color = used / 5;
    gs.crystals_red = gs.crystals_red.saturating_sub(per_color);
    gs.crystals_green = gs.crystals_green.saturating_sub(per_color);
    gs.crystals_blue = gs.crystals_blue.saturating_sub(per_color);
    gs.crystals_yellow = gs.crystals_yellow.saturating_sub(per_color);
    gs.crystals_purple = gs.crystals_purple.saturating_sub(per_color);
    crate::save_state::save_game_state(&gs);
}

/// Handle return button interaction.
pub fn return_button_interaction(
    mut interaction_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ReturnButton>),
    >,
    mut next_state: ResMut<NextState<ConverterPhase>>,
    mut grid_state: ResMut<GridState>,
    mut pile: ResMut<CrystalPile>,
    mut tanks: ResMut<ResourceTanks>,
    mut stats: ResMut<ConversionStats>,
    results_q: Query<Entity, With<ResultsScreen>>,
    mut commands: Commands,
) {
    for (interaction, mut bg) in interaction_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                let gs = crate::save_state::load_game_state();
                let crystal_count = gs.total_crystals();
                let pile_size = if crystal_count > 0 {
                    crystal_count.max(MIN_PILE_SIZE)
                } else {
                    INITIAL_PILE_SIZE
                };
                *grid_state = GridState::default();
                *pile = CrystalPile { total: pile_size, remaining: pile_size };
                *tanks = ResourceTanks::default();
                *stats = ConversionStats::default();
                grid_state.phase = GridPhase::Refilling;
                grid_state.phase_timer = REFILL_DELAY;
                for entity in results_q.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(ConverterPhase::Processing);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_HOVER_C.0, BTN_HOVER_C.1, BTN_HOVER_C.2,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(
                    BTN_BG_C.0, BTN_BG_C.1, BTN_BG_C.2,
                ));
            }
        }
    }
}
