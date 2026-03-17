// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use std::path::PathBuf;
use super::constants::*;
use super::types::*;
use crate::save_state::{load_game_state, GameState};

/// Spawn the game selection panel (right side).
pub fn spawn_game_cards(parent: &mut ChildSpawnerCommands, font: &Handle<Font>, ship: &ShipStatus) {
    parent.spawn(Node {
        width: Val::Percent(GAMES_WIDTH_PCT),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(SECTION_PAD)),
        row_gap: Val::Px(SECTION_GAP),
        align_items: AlignItems::Center,
        ..default()
    }).with_children(|panel| {
        // Section header
        panel.spawn((
            Text::new("OPERATIONS"),
            TextFont { font: font.clone(), font_size: SECTION_TITLE_FONT, ..default() },
            TextColor(Color::srgb(
                SECTION_TITLE_COLOR.0, SECTION_TITLE_COLOR.1, SECTION_TITLE_COLOR.2,
            )),
        ));

        // Game cards
        spawn_card(panel, font, GameCard::BotGame, ship);
        spawn_card(panel, font, GameCard::Gathering, ship);
        spawn_card(panel, font, GameCard::Converter, ship);
        spawn_card(panel, font, GameCard::Delivery, ship);
    });
}

fn card_info(card: GameCard, ship: &ShipStatus) -> (&'static str, String, bool, bool) {
    match card {
        GameCard::BotGame => (
            "Repair Systems",
            format!("Level {}/149", ship.bot_level),
            ship.repair < 50.0,
            true,
        ),
        GameCard::Gathering => (
            "Gather Resources",
            format!("Shields at {}%", ship.shields as u32),
            ship.shields < 50.0,
            true,
        ),
        GameCard::Converter => (
            "Process Crystals",
            format!("{} crystals available", ship.crystals),
            ship.crystals > 0,
            ship.crystals > 0,
        ),
        GameCard::Delivery => (
            "Distribute Resources",
            format!("{}% average systems", avg_resources(ship) as u32),
            false,
            false, // not available yet
        ),
    }
}

fn avg_resources(ship: &ShipStatus) -> f32 {
    (ship.power + ship.life_support + ship.cryo + ship.shields + ship.repair) / 5.0
}

fn spawn_card(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    card: GameCard,
    ship: &ShipStatus,
) {
    let (title, status, recommended, available) = card_info(card, ship);
    let alpha = if available { 1.0 } else { CARD_DISABLED_ALPHA };

    parent.spawn((
        Button,
        Node {
            width: Val::Px(CARD_WIDTH),
            min_height: Val::Px(CARD_HEIGHT),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(CARD_PAD)),
            row_gap: Val::Px(4.0),
            border: UiRect::all(Val::Px(CARD_BORDER)),
            border_radius: BorderRadius::all(Val::Px(CARD_CORNER)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            CARD_BG.0, CARD_BG.1, CARD_BG.2, CARD_BG.3 * alpha,
        )),
        BorderColor::all(Color::srgba(
            CARD_BORDER_COLOR.0, CARD_BORDER_COLOR.1, CARD_BORDER_COLOR.2, CARD_BORDER_COLOR.3,
        )),
        card,
    )).with_children(|c| {
        // Title row
        c.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            ..default()
        }).with_children(|row| {
            row.spawn((
                Text::new(title),
                TextFont { font: font.clone(), font_size: CARD_TITLE_FONT, ..default() },
                TextColor(Color::srgba(
                    CARD_TITLE_COLOR.0, CARD_TITLE_COLOR.1, CARD_TITLE_COLOR.2, alpha,
                )),
            ));
            if recommended && available {
                row.spawn((
                    Text::new("RECOMMENDED"),
                    TextFont { font: font.clone(), font_size: 10.0, ..default() },
                    TextColor(Color::srgb(
                        CARD_RECOMMENDED_COLOR.0, CARD_RECOMMENDED_COLOR.1, CARD_RECOMMENDED_COLOR.2,
                    )),
                    CardRecommended(card),
                ));
            }
        });

        // Status text
        c.spawn((
            Text::new(status),
            TextFont { font: font.clone(), font_size: CARD_STATUS_FONT, ..default() },
            TextColor(Color::srgba(
                CARD_STATUS_COLOR.0, CARD_STATUS_COLOR.1, CARD_STATUS_COLOR.2, alpha,
            )),
            CardStatusText(card),
        ));

        // Unavailable label
        if !available {
            c.spawn((
                Text::new("Not available"),
                TextFont { font: font.clone(), font_size: 11.0, ..default() },
                TextColor(Color::srgba(0.5, 0.4, 0.4, 0.7)),
            ));
        }
    });
}

/// System: handle game card hover effects.
pub fn card_hover_interaction(
    mut query: Query<(&Interaction, &GameCard, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (interaction, _card, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(
                    CARD_HOVER_BG.0, CARD_HOVER_BG.1, CARD_HOVER_BG.2, CARD_HOVER_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    CARD_BORDER_HOVER.0, CARD_BORDER_HOVER.1,
                    CARD_BORDER_HOVER.2, CARD_BORDER_HOVER.3,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(
                    CARD_BG.0, CARD_BG.1, CARD_BG.2, CARD_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    CARD_BORDER_COLOR.0, CARD_BORDER_COLOR.1,
                    CARD_BORDER_COLOR.2, CARD_BORDER_COLOR.3,
                ));
            }
        }
    }
}

/// Binary name for each game card.
fn binary_name(card: &GameCard) -> &'static str {
    match card {
        GameCard::BotGame => "protocol-play-player",
        GameCard::Gathering => "protocol-play-gathering",
        GameCard::Converter => "protocol-play-converter",
        GameCard::Delivery => "protocol-play-delivery",
    }
}

/// Get the directory where the current executable lives.
fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

/// System: handle game card click — launch child game process.
pub fn card_click_interaction(
    query: Query<(&Interaction, &GameCard), Changed<Interaction>>,
    mut running: ResMut<RunningGame>,
    mut anna: ResMut<AnnaState>,
) {
    // Don't launch if a game is already running
    if running.0.is_some() {
        return;
    }

    for (interaction, card) in query.iter() {
        if *interaction == Interaction::Pressed {
            let bin = binary_name(card);
            let dir = match exe_dir() {
                Some(d) => d,
                None => {
                    anna.queue.push((
                        "That system isn't available right now.".to_string(),
                        false,
                    ));
                    return;
                }
            };
            let path = dir.join(bin);

            // Check if binary exists
            if !path.exists() {
                anna.queue.push((
                    "That system isn't available right now.".to_string(),
                    false,
                ));
                info!("Binary not found: {}", path.display());
                return;
            }

            info!("Launching: {}", path.display());
            match std::process::Command::new(&path)
                .current_dir(&dir)
                .spawn()
            {
                Ok(child) => {
                    running.0 = Some(child);
                }
                Err(e) => {
                    anna.queue.push((
                        "That system isn't available right now.".to_string(),
                        false,
                    ));
                    warn!("Failed to launch {}: {}", bin, e);
                }
            }
        }
    }
}

/// System: poll the running child game process.
/// When it exits, reload GameState and update ShipStatus.
pub fn poll_running_game(
    mut running: ResMut<RunningGame>,
    mut ship: ResMut<ShipStatus>,
    mut gs: ResMut<GameState>,
    mut qs: ResMut<super::questions::QuestionState>,
) {
    let child = match running.0.as_mut() {
        Some(c) => c,
        None => return,
    };

    match child.try_wait() {
        Ok(Some(status)) => {
            info!("Child game exited with: {}", status);
            running.0 = None;

            // Reset question state so we check for pending questions
            super::questions::reset_question_check(&mut qs);

            // Reload GameState from disk (child may have updated it)
            let fresh = load_game_state();
            ship.power = fresh.power;
            ship.life_support = fresh.life_support;
            ship.cryo = fresh.cryo;
            ship.shields = fresh.shields;
            ship.repair = fresh.repair;
            ship.crystals = fresh.total_crystals();
            ship.crew_count = fresh.crew_count;
            ship.day = fresh.day;
            ship.distance_au = fresh.distance_au;
            ship.bot_level = fresh.bot_level;
            *gs = fresh;
        }
        Ok(None) => {
            // Still running
        }
        Err(e) => {
            warn!("Error polling child process: {}", e);
            running.0 = None;
        }
    }
}

/// System: show/hide the "Game in progress..." overlay.
pub fn manage_game_overlay(
    running: Res<RunningGame>,
    mut commands: Commands,
    overlay_q: Query<Entity, With<GameRunningOverlay>>,
    font_res: Res<MissionFont>,
) {
    let game_running = running.0.is_some();
    let overlay_exists = !overlay_q.is_empty();

    if game_running && !overlay_exists {
        // Spawn overlay
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.03, 0.06, 0.85)),
            GameRunningOverlay,
            GlobalZIndex(10),
        )).with_children(|overlay| {
            overlay.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            }).with_children(|col| {
                col.spawn((
                    Text::new("Game in progress..."),
                    TextFont {
                        font: font_res.0.clone(),
                        font_size: 28.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.7, 0.8, 0.95, 0.9)),
                ));
                col.spawn((
                    Text::new("Mission Control will resume when the game exits."),
                    TextFont {
                        font: font_res.0.clone(),
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.5, 0.55, 0.65, 0.7)),
                ));
            });
        });
    } else if !game_running && overlay_exists {
        // Remove overlay
        for entity in overlay_q.iter() {
            commands.entity(entity).despawn();
        }
    }
}
