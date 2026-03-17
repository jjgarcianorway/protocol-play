// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

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

/// System: handle game card click.
pub fn card_click_interaction(
    query: Query<(&Interaction, &GameCard), Changed<Interaction>>,
) {
    for (interaction, card) in query.iter() {
        if *interaction == Interaction::Pressed {
            let name = match card {
                GameCard::BotGame => "Bot Game (Repair Systems)",
                GameCard::Gathering => "The Gathering (Gather Resources)",
                GameCard::Converter => "The Converter (Process Crystals)",
                GameCard::Delivery => "The Delivery (Distribute Resources)",
            };
            info!("Would launch: {}", name);
        }
    }
}
