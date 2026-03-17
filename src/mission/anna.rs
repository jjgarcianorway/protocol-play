// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Spawn Anna's communication panel.
pub fn spawn_anna_panel(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            left: Val::Px(SECTION_PAD),
            right: Val::Px(SECTION_PAD),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(14.0),
            padding: UiRect::all(Val::Px(ANNA_PANEL_PAD)),
            border_radius: BorderRadius::all(Val::Px(ANNA_PANEL_CORNER)),
            ..default()
        },
        BackgroundColor(Color::srgba(
            ANNA_PANEL_BG.0, ANNA_PANEL_BG.1, ANNA_PANEL_BG.2, ANNA_PANEL_BG.3,
        )),
    )).with_children(|panel| {
        // Anna's "portrait" — glowing circle
        panel.spawn((
            Node {
                width: Val::Px(ANNA_CIRCLE_SIZE),
                height: Val::Px(ANNA_CIRCLE_SIZE),
                border_radius: BorderRadius::all(Val::Px(ANNA_CIRCLE_SIZE / 2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(
                ANNA_CIRCLE_COLOR.0, ANNA_CIRCLE_COLOR.1, ANNA_CIRCLE_COLOR.2,
            )),
            BoxShadow::new(
                Color::srgba(ANNA_CIRCLE_COLOR.0, ANNA_CIRCLE_COLOR.1, ANNA_CIRCLE_COLOR.2, 0.5),
                Val::ZERO, Val::ZERO,
                Val::Px(ANNA_GLOW_SPREAD), Val::Px(ANNA_GLOW_BLUR),
            ),
        )).with_children(|circle| {
            circle.spawn((
                Text::new("A"),
                TextFont { font: font.clone(), font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Name + message column
        panel.spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            flex_grow: 1.0,
            ..default()
        }).with_children(|col| {
            col.spawn((
                Text::new("ANNA"),
                TextFont { font: font.clone(), font_size: ANNA_NAME_FONT, ..default() },
                TextColor(Color::srgb(
                    ANNA_NAME_COLOR.0, ANNA_NAME_COLOR.1, ANNA_NAME_COLOR.2,
                )),
            ));
            col.spawn((
                Text::new(""),
                TextFont { font: font.clone(), font_size: ANNA_MSG_FONT, ..default() },
                TextColor(Color::srgba(
                    ANNA_MSG_COLOR.0, ANNA_MSG_COLOR.1, ANNA_MSG_COLOR.2, 0.0,
                )),
                AnnaMessageText,
            ));
        });
    });
}

/// Pick an appropriate message based on ship status.
fn pick_message(ship: &ShipStatus) -> &'static str {
    let (lowest_idx, lowest_val) = ship.lowest_resource();

    if lowest_val < 25.0 {
        return match lowest_idx {
            0 => "Power levels critical. We need to process crystals immediately.",
            1 => "Life support is failing. Prioritize crystal processing.",
            2 => "Cryo systems are degrading. The crew is at risk.",
            3 => "Shields are nearly depleted. A gathering run is urgent.",
            4 => "Repair systems offline. 3 subsystems need attention.",
            _ => "Systems require immediate attention.",
        };
    }

    if lowest_val < 50.0 {
        return match lowest_idx {
            0 => "Power reserves are low. Consider processing some crystals.",
            1 => "Life support needs attention soon.",
            2 => "Cryo efficiency is dropping. Monitor closely.",
            3 => "Shield reserves are low. Consider a gathering run.",
            4 => "Repair systems need attention. Some subsystems offline.",
            _ => "Some systems are below optimal levels.",
        };
    }

    if ship.crystals == 0 {
        return "No crystals in reserve. A gathering run would help.";
    }

    if ship.crystals > 20 {
        return "We have crystals to process. The converter is ready.";
    }

    // Default rotating messages based on day
    let msgs: &[&str] = &[
        "Welcome back. Systems are nominal.",
        "All systems within acceptable parameters.",
        "The crew sleeps soundly. Journey continues.",
        "Steady progress. We are on course.",
    ];
    msgs[(ship.day as usize / 10) % msgs.len()]
}

/// System: cycle Anna's messages with fade in/out.
pub fn update_anna_messages(
    time: Res<Time>,
    ship: Res<ShipStatus>,
    mut state: ResMut<AnnaState>,
    mut query: Query<(&mut Text, &mut TextColor), With<AnnaMessageText>>,
) {
    let dt = time.delta_secs();
    state.timer -= dt;

    if state.fading_out {
        state.fade_alpha = (state.fade_alpha - ANNA_FADE_SPEED * dt).max(0.0);
        if state.fade_alpha <= 0.0 {
            // Pick new message and start fading in
            state.current_msg = pick_message(&ship).to_string();
            state.fading_out = false;
            state.timer = ANNA_MSG_HOLD;
        }
    } else if state.timer <= 0.0 {
        // Start fading out
        state.fading_out = true;
    } else {
        // Fading in or holding
        state.fade_alpha = (state.fade_alpha + ANNA_FADE_SPEED * dt).min(1.0);
    }

    for (mut text, mut color) in query.iter_mut() {
        if text.0 != state.current_msg {
            *text = Text::new(state.current_msg.clone());
        }
        *color = TextColor(Color::srgba(
            ANNA_MSG_COLOR.0, ANNA_MSG_COLOR.1, ANNA_MSG_COLOR.2, state.fade_alpha,
        ));
    }
}
