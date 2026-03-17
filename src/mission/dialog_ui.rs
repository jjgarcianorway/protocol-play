// SPDX-License-Identifier: GPL-3.0-or-later

//! Dialog UI rendering — spawning the overlay, typewriter effect, choices.

use bevy::prelude::*;
use super::dialog_types::*;
use super::types::MissionFont;

/// Spawn the full-screen dialog overlay with speaker, text area, and choice container.
pub fn spawn_dialog_overlay(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            DIALOG_OVERLAY_BG.0, DIALOG_OVERLAY_BG.1,
            DIALOG_OVERLAY_BG.2, DIALOG_OVERLAY_BG.3,
        )),
        GlobalZIndex(40),
        DialogOverlay,
        Button,
    )).with_children(|overlay| {
        // Central panel with glow
        overlay.spawn((
            Node {
                max_width: Val::Px(DIALOG_MAX_WIDTH),
                width: Val::Percent(85.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(DIALOG_PADDING)),
                row_gap: Val::Px(20.0),
                border_radius: BorderRadius::all(Val::Px(DIALOG_PANEL_CORNER)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                DIALOG_PANEL_BG.0, DIALOG_PANEL_BG.1,
                DIALOG_PANEL_BG.2, DIALOG_PANEL_BG.3,
            )),
            BoxShadow::new(
                Color::srgba(
                    DIALOG_GLOW_COLOR.0, DIALOG_GLOW_COLOR.1,
                    DIALOG_GLOW_COLOR.2, DIALOG_GLOW_COLOR.3,
                ),
                Val::ZERO, Val::ZERO,
                Val::Px(DIALOG_GLOW_SPREAD), Val::Px(DIALOG_GLOW_BLUR),
            ),
            DialogPanelGlow,
        )).with_children(|panel| {
            // Top row: portrait + speaker name
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(14.0),
                ..default()
            }).with_children(|row| {
                // Anna's circle portrait
                row.spawn((
                    Node {
                        width: Val::Px(DIALOG_CIRCLE_SIZE),
                        height: Val::Px(DIALOG_CIRCLE_SIZE),
                        border_radius: BorderRadius::all(Val::Px(DIALOG_CIRCLE_SIZE / 2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(
                        DIALOG_ANNA_COLOR.0, DIALOG_ANNA_COLOR.1, DIALOG_ANNA_COLOR.2,
                    )),
                    BoxShadow::new(
                        Color::srgba(
                            DIALOG_ANNA_COLOR.0, DIALOG_ANNA_COLOR.1,
                            DIALOG_ANNA_COLOR.2, 0.5,
                        ),
                        Val::ZERO, Val::ZERO,
                        Val::Px(5.0), Val::Px(12.0),
                    ),
                    DialogAnnaCircle,
                )).with_children(|circle| {
                    circle.spawn((
                        Text::new("A"),
                        TextFont { font: font.clone(), font_size: 24.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });

                // Speaker name
                row.spawn((
                    Text::new("ANNA"),
                    TextFont { font: font.clone(), font_size: DIALOG_SPEAKER_FONT, ..default() },
                    TextColor(Color::srgb(
                        DIALOG_ANNA_COLOR.0, DIALOG_ANNA_COLOR.1, DIALOG_ANNA_COLOR.2,
                    )),
                    DialogSpeakerText,
                ));
            });

            // Dialog body text
            panel.spawn((
                Text::new(""),
                TextFont { font: font.clone(), font_size: DIALOG_BODY_FONT, ..default() },
                TextColor(Color::srgb(
                    DIALOG_BODY_COLOR.0, DIALOG_BODY_COLOR.1, DIALOG_BODY_COLOR.2,
                )),
                DialogBodyText,
            ));

            // Choice container (starts empty, choices spawned dynamically)
            panel.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                DialogChoiceContainer,
            ));

            // Skip hint
            panel.spawn((
                Text::new(""),
                TextFont { font: font.clone(), font_size: DIALOG_HINT_FONT, ..default() },
                TextColor(Color::srgba(0.5, 0.55, 0.65, 0.0)),
                DialogSkipHint,
            ));
        });
    });
}

/// Despawn the dialog overlay and all children.
pub fn despawn_dialog_overlay(commands: &mut Commands, query: &Query<Entity, With<DialogOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Update speaker display for current node.
pub fn update_speaker_display(
    speaker: Speaker,
    speaker_q: &mut Query<&mut TextColor, (With<DialogSpeakerText>, Without<DialogBodyText>)>,
    speaker_text_q: &mut Query<&mut Text, (With<DialogSpeakerText>, Without<DialogBodyText>)>,
    circle_q: &mut Query<(&mut BackgroundColor, &mut BoxShadow), With<DialogAnnaCircle>>,
) {
    let (label, color) = match speaker {
        Speaker::Anna => ("ANNA", DIALOG_ANNA_COLOR),
        Speaker::System => ("[SYSTEM]", DIALOG_SYSTEM_COLOR),
        Speaker::Narrator => ("", DIALOG_NARRATOR_COLOR),
        Speaker::Player => ("You", DIALOG_PLAYER_COLOR),
    };

    for mut tc in speaker_q.iter_mut() {
        *tc = TextColor(Color::srgb(color.0, color.1, color.2));
    }
    for mut text in speaker_text_q.iter_mut() {
        *text = Text::new(label);
    }

    // Update circle visibility/color
    let circle_color = match speaker {
        Speaker::Anna => Some(DIALOG_ANNA_COLOR),
        Speaker::System => Some(DIALOG_SYSTEM_COLOR),
        _ => None,
    };
    for (mut bg, mut shadow) in circle_q.iter_mut() {
        if let Some(c) = circle_color {
            *bg = BackgroundColor(Color::srgb(c.0, c.1, c.2));
            *shadow = BoxShadow::new(
                Color::srgba(c.0, c.1, c.2, 0.5),
                Val::ZERO, Val::ZERO, Val::Px(5.0), Val::Px(12.0),
            );
        } else {
            *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
            *shadow = BoxShadow::new(
                Color::srgba(0.0, 0.0, 0.0, 0.0),
                Val::ZERO, Val::ZERO, Val::Px(0.0), Val::Px(0.0),
            );
        }
    }
}

/// Spawn choice buttons into the choice container.
pub fn spawn_choice_buttons(
    commands: &mut Commands,
    container_q: &Query<Entity, With<DialogChoiceContainer>>,
    choices: &[DialogChoice],
    font: &Handle<Font>,
) {
    for container in container_q.iter() {
        commands.entity(container).with_children(|parent| {
            for (i, choice) in choices.iter().enumerate() {
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(DIALOG_CHOICE_PAD)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(DIALOG_CHOICE_CORNER)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(
                        DIALOG_CHOICE_BG.0, DIALOG_CHOICE_BG.1,
                        DIALOG_CHOICE_BG.2, DIALOG_CHOICE_BG.3,
                    )),
                    BorderColor::all(Color::srgba(
                        DIALOG_CHOICE_BORDER.0, DIALOG_CHOICE_BORDER.1,
                        DIALOG_CHOICE_BORDER.2, DIALOG_CHOICE_BORDER.3,
                    )),
                    DialogChoiceBtn(i),
                )).with_child((
                    Text::new(choice.text),
                    TextFont { font: font.clone(), font_size: DIALOG_CHOICE_FONT, ..default() },
                    TextColor(Color::srgb(0.85, 0.88, 0.95)),
                ));
            }
        });
    }
}

/// Clear choice buttons from the container.
pub fn clear_choice_buttons(
    commands: &mut Commands,
    btn_q: &Query<Entity, With<DialogChoiceBtn>>,
) {
    for entity in btn_q.iter() {
        commands.entity(entity).despawn();
    }
}

/// System: hover effects on dialog choice buttons.
pub fn dialog_choice_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<DialogChoiceBtn>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(
                    DIALOG_CHOICE_HOVER.0, DIALOG_CHOICE_HOVER.1,
                    DIALOG_CHOICE_HOVER.2, DIALOG_CHOICE_HOVER.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    DIALOG_CHOICE_HOVER_BORDER.0, DIALOG_CHOICE_HOVER_BORDER.1,
                    DIALOG_CHOICE_HOVER_BORDER.2, DIALOG_CHOICE_HOVER_BORDER.3,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(
                    DIALOG_CHOICE_BG.0, DIALOG_CHOICE_BG.1,
                    DIALOG_CHOICE_BG.2, DIALOG_CHOICE_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    DIALOG_CHOICE_BORDER.0, DIALOG_CHOICE_BORDER.1,
                    DIALOG_CHOICE_BORDER.2, DIALOG_CHOICE_BORDER.3,
                ));
            }
        }
    }
}

/// System: animate dialog panel glow based on current speaker.
pub fn animate_dialog_glow(
    time: Res<Time>,
    state: Res<DialogState>,
    mut query: Query<&mut BoxShadow, With<DialogPanelGlow>>,
) {
    let active = match &state.active_scene {
        Some(a) => a,
        None => return,
    };
    let node = match active.scene.nodes.get(active.node_index) {
        Some(n) => n,
        None => return,
    };

    let t = time.elapsed_secs();
    let (r, g, b) = match node.speaker {
        Speaker::Anna => DIALOG_ANNA_COLOR,
        Speaker::System => DIALOG_SYSTEM_COLOR,
        Speaker::Narrator => DIALOG_NARRATOR_COLOR,
        Speaker::Player => DIALOG_PLAYER_COLOR,
    };

    let pulse = 0.10 + 0.06 * (t * 1.5).sin();
    for mut shadow in query.iter_mut() {
        *shadow = BoxShadow::new(
            Color::srgba(r, g, b, pulse),
            Val::ZERO, Val::ZERO,
            Val::Px(DIALOG_GLOW_SPREAD), Val::Px(DIALOG_GLOW_BLUR),
        );
    }
}

/// System: animate Anna's circle in dialog during emotional moments.
pub fn animate_dialog_circle(
    time: Res<Time>,
    state: Res<DialogState>,
    mut query: Query<(&mut BackgroundColor, &mut BoxShadow), With<DialogAnnaCircle>>,
) {
    let active = match &state.active_scene {
        Some(a) => a,
        None => return,
    };
    let node = match active.scene.nodes.get(active.node_index) {
        Some(n) => n,
        None => return,
    };
    if node.speaker != Speaker::Anna { return; }

    let t = time.elapsed_secs();
    let brightness = 0.8 + 0.2 * (t * 1.2).sin();
    let (r, g, b) = DIALOG_ANNA_COLOR;
    let cr = r * brightness;
    let cg = g * brightness;
    let cb = b * brightness;

    for (mut bg, mut shadow) in query.iter_mut() {
        *bg = BackgroundColor(Color::srgb(cr, cg, cb));
        *shadow = BoxShadow::new(
            Color::srgba(cr, cg, cb, 0.5 * brightness),
            Val::ZERO, Val::ZERO, Val::Px(5.0), Val::Px(12.0),
        );
    }
}

/// Update the skip hint text based on dialog state.
pub fn update_skip_hint(
    state: &DialogState,
    hint_q: &mut Query<(&mut Text, &mut TextColor), With<DialogSkipHint>>,
) {
    let active = match &state.active_scene {
        Some(a) => a,
        None => return,
    };

    for (mut text, mut color) in hint_q.iter_mut() {
        if active.reaction_text.is_some() {
            *text = Text::new("");
            *color = TextColor(Color::srgba(0.5, 0.55, 0.65, 0.0));
        } else if !active.text_complete {
            *text = Text::new("Click to skip");
            *color = TextColor(Color::srgba(0.5, 0.55, 0.65, 0.5));
        } else if active.choices_visible {
            *text = Text::new("");
            *color = TextColor(Color::srgba(0.5, 0.55, 0.65, 0.0));
        } else {
            *text = Text::new("Click to continue");
            *color = TextColor(Color::srgba(0.5, 0.55, 0.65, 0.6));
        }
    }
}
