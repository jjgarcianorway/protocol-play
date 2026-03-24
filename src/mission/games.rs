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
        spawn_card(panel, font, GameCard::Orben, ship);
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
        GameCard::Gathering => {
            let available = ship.bot_level > 0;
            let status = if available {
                format!("Shields at {}%", ship.shields as u32)
            } else {
                "Repair systems first".to_string()
            };
            ("Gather Resources", status, available && ship.shields < 50.0, available)
        }
        GameCard::Converter => (
            "Process Crystals",
            format!("{} crystals available", ship.crystals),
            ship.crystals > 0,
            ship.crystals > 0,
        ),
        GameCard::Delivery => {
            let available = ship.crystals > 0;
            let status = if available {
                format!("{}% average systems", avg_resources(ship) as u32)
            } else {
                "Gather resources first".to_string()
            };
            ("Distribute Resources", status, false, available)
        }
        GameCard::Orben => (
            "Play Orben",
            "Stay human. Play cards.".to_string(),
            false,
            true,
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
    gs: Res<GameState>,
) {
    for (interaction, card, mut bg, mut border) in query.iter_mut() {
        let is_blocked = gs.pending_game.as_deref()
            .map_or(false, |p| p != card.save_name());
        if is_blocked { continue; }

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
        GameCard::Orben => "protocol-play-orben",
    }
}

/// Get the directory where the current executable lives.
fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

/// System: handle game card click.
/// In full mode: transition to GameScene sub-state (except BotPuzzle).
/// In standalone mode: launch child process.
pub fn card_click_interaction(
    query: Query<(&Interaction, &GameCard), Changed<Interaction>>,
    mut running: ResMut<RunningGame>,
    mut anna: ResMut<AnnaState>,
    mut next_scene: Option<ResMut<NextState<GameScene>>>,
    gs: Res<GameState>,
) {
    // Don't launch if a game is already running
    if running.0.is_some() {
        return;
    }

    for (interaction, card) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }

        // Block other games if one is pending
        if let Some(ref pending) = gs.pending_game {
            if pending.as_str() != card.save_name() { continue; }
        }

        // If GameScene sub-state is available (full mode), use scene transitions
        if let Some(ref mut scene_state) = next_scene {
            let scene = match card {
                GameCard::BotGame => GameScene::BotPuzzle,
                GameCard::Gathering => GameScene::Gathering,
                GameCard::Converter => GameScene::Converter,
                GameCard::Delivery => GameScene::Delivery,
                GameCard::Orben => GameScene::Orben,
            };
            info!("Transitioning to {:?}", scene);
            scene_state.set(scene);
            return;
        }

        // Standalone mode: launch child process
        launch_child_process(card, &mut running, &mut anna);
    }
}

/// System: update card visuals when a game is pending.
/// Dims blocked cards, highlights the pending one.
pub fn update_pending_card_visuals(
    gs: Res<GameState>,
    mut card_q: Query<(&GameCard, &mut BackgroundColor, &mut BorderColor)>,
    mut status_q: Query<(&CardStatusText, &mut Text, &mut TextColor)>,
) {
    if !gs.is_changed() { return; }
    let pending = gs.pending_game.as_deref();

    for (card, mut bg, mut border) in card_q.iter_mut() {
        let is_pending = pending.map_or(false, |p| p == card.save_name());
        let is_blocked = pending.is_some() && !is_pending;
        if is_pending {
            *border = BorderColor::all(Color::srgba(0.35, 0.75, 1.0, 0.85));
            *bg = BackgroundColor(Color::srgba(CARD_BG.0, CARD_BG.1, CARD_BG.2, CARD_BG.3));
        } else if is_blocked {
            *border = BorderColor::all(Color::srgba(
                CARD_BORDER_COLOR.0, CARD_BORDER_COLOR.1, CARD_BORDER_COLOR.2, 0.2,
            ));
            *bg = BackgroundColor(Color::srgba(CARD_BG.0, CARD_BG.1, CARD_BG.2, 0.2));
        } else {
            *border = BorderColor::all(Color::srgba(
                CARD_BORDER_COLOR.0, CARD_BORDER_COLOR.1, CARD_BORDER_COLOR.2, CARD_BORDER_COLOR.3,
            ));
            *bg = BackgroundColor(Color::srgba(CARD_BG.0, CARD_BG.1, CARD_BG.2, CARD_BG.3));
        }
    }

    for (CardStatusText(card), mut text, mut color) in status_q.iter_mut() {
        let is_pending = pending.map_or(false, |p| p == card.save_name());
        if is_pending {
            **text = "IN PROGRESS — Resume".to_string();
            *color = TextColor(Color::srgba(0.4, 0.9, 0.6, 0.9));
        }
    }
}

/// Launch a child process for a game card (standalone mode / bot puzzle).
fn launch_child_process(
    card: &GameCard,
    running: &mut ResMut<RunningGame>,
    anna: &mut ResMut<AnnaState>,
) {
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

/// System: poll the running child game process.
/// When it exits, reload GameState and update ShipStatus.
pub fn poll_running_game(
    mut running: ResMut<RunningGame>,
    mut ship: ResMut<ShipStatus>,
    mut gs: ResMut<GameState>,
    mut qs: ResMut<super::questions::QuestionState>,
    mut ds: ResMut<super::dialog_types::DialogState>,
) {
    let child = match running.0.as_mut() {
        Some(c) => c,
        None => return,
    };

    match child.try_wait() {
        Ok(Some(status)) => {
            info!("Child game exited with: {}", status);
            running.0 = None;

            // Reset question and dialog state so we check for pending content
            super::questions::reset_question_check(&mut qs);
            super::dialog_system::reset_dialog_check(&mut ds);

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
