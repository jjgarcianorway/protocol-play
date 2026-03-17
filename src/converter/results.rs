// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Calculate overall efficiency rating.
pub fn calc_efficiency(stats: &ConversionStats, tanks: &ResourceTanks) -> f32 {
    let total_resource: f32 = tanks.levels.iter().sum();
    if stats.total_converted == 0 { return 0.0; }
    // Efficiency = actual resources / (converted crystals * 1.0x baseline)
    (total_resource / stats.total_converted as f32 * 100.0).clamp(0.0, 300.0)
}

/// Rating string from efficiency percentage.
pub fn efficiency_rating(eff: f32) -> &'static str {
    if eff >= 250.0 { "S - Perfect" }
    else if eff >= 200.0 { "A - Excellent" }
    else if eff >= 150.0 { "B - Great" }
    else if eff >= 100.0 { "C - Good" }
    else if eff >= 70.0 { "D - Fair" }
    else { "E - Wasteful" }
}

/// Spawn results screen overlay.
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

    // Save results to cross-game state
    save_converter_results(&stats, &tanks);

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
                row_gap: Val::Px(16.0),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                RESULTS_BG.0, RESULTS_BG.1, RESULTS_BG.2, RESULTS_BG.3,
            )),
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Processing Complete"),
                tf(RESULTS_TITLE_FONT),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.95)),
            ));

            // Stats
            let lines = vec![
                format!("Crystals processed: {}", stats.total_converted),
                format!("Chains triggered: {}", stats.chains_triggered),
                format!("Best chain: {}", stats.best_chain),
                format!("Cascades: {}", stats.cascades),
                format!("Efficiency: {:.0}%  ({})", eff, rating),
            ];
            for line in &lines {
                panel.spawn((
                    Text::new(line.clone()),
                    tf(RESULTS_FONT),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ));
            }

            // Resource summary
            panel.spawn((
                Text::new("Resources produced:"),
                tf(RESULTS_FONT),
                TextColor(Color::srgba(1.0, 1.0, 0.7, 0.9)),
            ));
            for i in 0..5 {
                let (cr, cg, cb) = CRYSTAL_COLORS[i];
                panel.spawn((
                    Text::new(format!("  {} {} — {:.1}",
                        RESOURCE_ICONS[i], RESOURCE_NAMES[i], tanks.levels[i])),
                    tf(RESULTS_FONT),
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
                BackgroundColor(Color::srgb(BTN_BG_C.0, BTN_BG_C.1, BTN_BG_C.2)),
                ReturnButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play Again"),
                    tf(RESULTS_BTN_FONT),
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

/// Save converter results to cross-game state.
pub fn save_converter_results(
    stats: &ConversionStats,
    tanks: &ResourceTanks,
) {
    if stats.total_converted == 0 { return; }
    let mut gs = crate::save_state::load_game_state();
    // Add resource levels from tanks (each tank value maps to resource %)
    gs.power = (gs.power + tanks.levels[0]).clamp(0.0, 100.0);
    gs.life_support = (gs.life_support + tanks.levels[1]).clamp(0.0, 100.0);
    gs.cryo = (gs.cryo + tanks.levels[2]).clamp(0.0, 100.0);
    gs.shields = (gs.shields + tanks.levels[3]).clamp(0.0, 100.0);
    gs.repair = (gs.repair + tanks.levels[4]).clamp(0.0, 100.0);
    // Subtract consumed crystals
    let used = stats.total_converted;
    let per_color = used / 5;
    gs.crystals_red = gs.crystals_red.saturating_sub(per_color);
    gs.crystals_green = gs.crystals_green.saturating_sub(per_color);
    gs.crystals_blue = gs.crystals_blue.saturating_sub(per_color);
    gs.crystals_yellow = gs.crystals_yellow.saturating_sub(per_color);
    gs.crystals_purple = gs.crystals_purple.saturating_sub(per_color);
    crate::save_state::save_game_state(&gs);
}

/// Handle return button interaction — reset the game.
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
                // Reset everything
                *grid_state = GridState::default();
                *pile = CrystalPile::default();
                *tanks = ResourceTanks::default();
                *stats = ConversionStats::default();
                grid_state.phase = GridPhase::Refilling;
                grid_state.phase_timer = 0.1;

                // Despawn results screen
                for entity in results_q.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(ConverterPhase::Processing);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(BTN_HOVER_C.0, BTN_HOVER_C.1, BTN_HOVER_C.2));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(BTN_BG_C.0, BTN_BG_C.1, BTN_BG_C.2));
            }
        }
    }
}
