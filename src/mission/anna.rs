// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::anna_messages;
use crate::save_state::GameState;

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
        Button,
        AnnaPanelArea,
    )).with_children(|panel| {
        // Anna's "portrait" -- glowing circle
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
                Color::srgba(
                    ANNA_CIRCLE_COLOR.0, ANNA_CIRCLE_COLOR.1,
                    ANNA_CIRCLE_COLOR.2, 0.5,
                ),
                Val::ZERO, Val::ZERO,
                Val::Px(ANNA_GLOW_SPREAD), Val::Px(ANNA_GLOW_BLUR),
            ),
            AnnaCircle,
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

/// System: handle click-to-dismiss on the Anna panel.
pub fn anna_click_dismiss(
    query: Query<&Interaction, (Changed<Interaction>, With<AnnaPanelArea>)>,
    mut state: ResMut<AnnaState>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed && state.fade_alpha > 0.5 {
            state.dismissed = true;
        }
    }
}

/// System: cycle Anna's messages with fade in/out, priority queue, and story tracking.
pub fn update_anna_messages(
    time: Res<Time>,
    ship: Res<ShipStatus>,
    mut gs: ResMut<GameState>,
    mut state: ResMut<AnnaState>,
    mut query: Query<(&mut Text, &mut TextColor), With<AnnaMessageText>>,
) {
    let dt = time.delta_secs();

    // Initialize: pick first message
    if !state.initialized {
        let msg = anna_messages::pick_best_message(
            &ship, &gs, state.last_personality_idx,
        );
        apply_message(&mut state, &msg.text, msg.is_story, &ship);
        mark_story_seen(&msg.text, &mut gs);
        state.initialized = true;
    }

    // Handle dismiss
    if state.dismissed {
        state.fading_out = true;
        state.dismissed = false;
    }

    if state.fading_out {
        state.fade_alpha = (state.fade_alpha - ANNA_FADE_SPEED * dt).max(0.0);
        if state.fade_alpha <= 0.0 {
            pick_next_message(&mut state, &ship, &mut gs);
        }
    } else if state.timer <= 0.0 {
        state.fading_out = true;
    } else {
        state.fade_alpha = (state.fade_alpha + ANNA_FADE_SPEED * dt).min(1.0);
        state.timer -= dt;
    }

    // Apply to UI
    let msg_color = if state.is_story_msg {
        ANNA_STORY_MSG_COLOR
    } else {
        ANNA_MSG_COLOR
    };
    for (mut text, mut color) in query.iter_mut() {
        if text.0 != state.current_msg {
            *text = Text::new(state.current_msg.clone());
        }
        *color = TextColor(Color::srgba(
            msg_color.0, msg_color.1, msg_color.2, state.fade_alpha,
        ));
    }
}

/// System: animate Anna's glowing circle based on mood.
pub fn update_anna_glow(
    time: Res<Time>,
    state: Res<AnnaState>,
    mut query: Query<(&mut BackgroundColor, &mut BoxShadow), With<AnnaCircle>>,
) {
    let t = time.elapsed_secs();

    for (mut bg, mut shadow) in query.iter_mut() {
        let (base_r, base_g, base_b, pulse_speed) = match state.mood {
            AnnaMood::Normal => (
                ANNA_CIRCLE_COLOR.0, ANNA_CIRCLE_COLOR.1,
                ANNA_CIRCLE_COLOR.2, ANNA_NORMAL_PULSE_SPEED,
            ),
            AnnaMood::Warning => (
                ANNA_WARNING_COLOR.0, ANNA_WARNING_COLOR.1,
                ANNA_WARNING_COLOR.2, ANNA_WARNING_PULSE_SPEED,
            ),
            AnnaMood::Story => (
                ANNA_STORY_COLOR.0, ANNA_STORY_COLOR.1,
                ANNA_STORY_COLOR.2, ANNA_NORMAL_PULSE_SPEED,
            ),
            AnnaMood::Glitching => (
                ANNA_GLITCH_COLOR.0, ANNA_GLITCH_COLOR.1,
                ANNA_GLITCH_COLOR.2, ANNA_GLITCH_SPEED,
            ),
        };

        let brightness = match state.mood {
            AnnaMood::Glitching => {
                let flicker = (t * pulse_speed).sin()
                    * (t * pulse_speed * 2.3).cos();
                0.5 + 0.5 * flicker.abs()
            }
            AnnaMood::Story => {
                0.85 + 0.15 * (t * pulse_speed).sin()
            }
            AnnaMood::Warning => {
                0.6 + 0.4 * (t * pulse_speed).sin().abs()
            }
            AnnaMood::Normal => {
                0.8 + 0.2 * (t * pulse_speed).sin()
            }
        };

        let r = base_r * brightness;
        let g = base_g * brightness;
        let b = base_b * brightness;

        *bg = BackgroundColor(Color::srgb(r, g, b));
        *shadow = BoxShadow::new(
            Color::srgba(r, g, b, 0.5 * brightness),
            Val::ZERO, Val::ZERO,
            Val::Px(ANNA_GLOW_SPREAD), Val::Px(ANNA_GLOW_BLUR),
        );
    }
}

/// Pick the next message, pulling from queue or generating a new one.
fn pick_next_message(
    state: &mut ResMut<AnnaState>,
    ship: &ShipStatus,
    gs: &mut ResMut<GameState>,
) {
    if let Some((text, is_story)) = state.queue.pop() {
        apply_message(state, &text, is_story, ship);
        return;
    }

    let msg = anna_messages::pick_best_message(
        ship, gs, state.last_personality_idx,
    );
    apply_message(state, &msg.text, msg.is_story, ship);
    mark_story_seen(&msg.text, gs);

    if !msg.is_story {
        if let Some(idx) = anna_messages::personality_index(&msg.text) {
            state.last_personality_idx = Some(idx);
        }
    }
}

/// Apply a message to the state and set mood + timing.
fn apply_message(
    state: &mut AnnaState,
    text: &str,
    is_story: bool,
    ship: &ShipStatus,
) {
    state.current_msg = text.to_string();
    state.is_story_msg = is_story;
    state.fading_out = false;
    state.timer = if is_story { ANNA_STORY_HOLD } else { ANNA_MSG_HOLD };

    if is_story {
        state.mood = AnnaMood::Story;
    } else if ship.power < 20.0 || ship.repair < 15.0 {
        state.mood = if ship.power < 10.0 {
            AnnaMood::Glitching
        } else {
            AnnaMood::Warning
        };
    } else if ship.life_support < 30.0 || ship.cryo < 30.0
        || ship.shields < 25.0
    {
        state.mood = AnnaMood::Warning;
    } else {
        state.mood = AnnaMood::Normal;
    }
}

/// If the message matches a story chapter, mark it as seen and save.
fn mark_story_seen(text: &str, gs: &mut GameState) {
    use super::story::STORY_CHAPTERS;
    for ch in STORY_CHAPTERS {
        if ch.message == text && !gs.story_seen.contains(&ch.id) {
            gs.story_seen.push(ch.id);
            gs.story_chapter = gs.story_chapter.max(ch.id);
            crate::save_state::save_game_state(gs);
            break;
        }
    }
}
