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

            // Image container (populated dynamically for nodes with images)
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                DialogImageContainer,
            ));

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

/// Detect Anna's emotional mood from narrator glow descriptions.
/// Ordered by specificity — specific phrases checked before broad keywords.
fn detect_anna_mood(text: &str) -> Option<(f32, f32, f32)> {
    let t = text.to_lowercase();
    if !t.contains("glow") && !t.contains("dims") && !t.contains("flicker") { return None; }
    // Complete absence
    if t.contains("completely dark") || t.contains("glow is gone") { return Some(ANNA_MOOD_DIM); }
    // Specific named colors
    if t.contains("clinical white") || t.contains("hard, clinical") { return Some(ANNA_MOOD_CLINICAL); }
    if t.contains("steel grey") || t.contains("steel gray") || t.contains("heavy grey") { return Some(ANNA_MOOD_GREY); }
    if t.contains("warning red") || t.contains("flickers red") { return Some(ANNA_MOOD_RED); }
    if t.contains("deep green") || t.contains("gold and green") { return Some(ANNA_MOOD_GREEN); }
    if t.contains("cold") && t.contains("glow") { return Some(ANNA_MOOD_COLD); }
    if t.contains("lavender") || t.contains("violet") { return Some(ANNA_MOOD_VULNERABLE); }
    // Dimming
    if t.contains("glow dims") || t.contains("almost nothing") || t.contains("barely visible")
        || t.contains("dims to") || t.contains("dims her glow")
        || t.contains("near-darkness") || t.contains("single point") { return Some(ANNA_MOOD_DIM); }
    // Conflicted / unstable
    if t.contains("conflicted") || t.contains("fractured") || t.contains("strobing")
        || t.contains("unresolved") || t.contains("arrhythmic") || t.contains("stutters") { return Some(ANNA_MOOD_CONFLICTED); }
    // Warm / joyful gold
    if t.contains("warm gold") || t.contains("warm amber") || t.contains("soft gold")
        || t.contains("sunrise") || t.contains("glow warms") || t.contains("warmest blue") { return Some(ANNA_MOOD_JOY); }
    if t.contains("amber") { return Some(ANNA_MOOD_WARM); }
    // Brightening
    if t.contains("glow brightens") || t.contains("burns bright")
        || t.contains("glow fills") || t.contains("glow blooms") { return Some(ANNA_MOOD_BRIGHT); }
    // Calming
    if t.contains("glow softens") || t.contains("glow steadies") || t.contains("calm") { return Some(ANNA_MOOD_CALM); }
    // Flickering / shifting (general unease)
    if t.contains("glow flickers") || t.contains("glow flares") || t.contains("glow pulses")
        || t.contains("glow shifts") || t.contains("glow contracts")
        || t.contains("glow tightens") || t.contains("glow hardens") { return Some(ANNA_MOOD_CONFLICTED); }
    None
}

/// Update speaker display for current node.
/// `node_text` is the current dialog text — used to detect glow mood changes.
pub fn update_speaker_display(
    speaker: Speaker,
    node_text: &str,
    speaker_q: &mut Query<&mut TextColor, (With<DialogSpeakerText>, Without<DialogBodyText>, Without<DialogSkipHint>)>,
    speaker_text_q: &mut Query<&mut Text, (With<DialogSpeakerText>, Without<DialogBodyText>, Without<DialogSkipHint>)>,
    circle_q: &mut Query<(&mut BackgroundColor, &mut BoxShadow), With<DialogAnnaCircle>>,
    glow_mood: &mut ResMut<AnnaGlowMood>,
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

    // Detect mood from narrator glow descriptions
    if speaker == Speaker::Narrator {
        if let Some(mood_color) = detect_anna_mood(node_text) {
            glow_mood.target = mood_color;
        }
    } else if speaker == Speaker::Anna {
        // When Anna speaks, reset to calm blue (unless narrator set a mood)
        glow_mood.target = ANNA_MOOD_CALM;
    }

    // Update circle visibility/color — use mood for Anna, speaker color for System
    let circle_color = match speaker {
        Speaker::Anna => Some(glow_mood.current),
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
pub fn clear_choice_buttons(commands: &mut Commands, btn_q: &Query<Entity, With<DialogChoiceBtn>>) {
    for entity in btn_q.iter() { commands.entity(entity).despawn(); }
}

fn choice_color(hover: bool) -> (Color, Color) {
    let (bg, bd) = if hover { (DIALOG_CHOICE_HOVER, DIALOG_CHOICE_HOVER_BORDER) }
                   else { (DIALOG_CHOICE_BG, DIALOG_CHOICE_BORDER) };
    (Color::srgba(bg.0, bg.1, bg.2, bg.3), Color::srgba(bd.0, bd.1, bd.2, bd.3))
}

/// System: hover effects on dialog choice buttons.
pub fn dialog_choice_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<DialogChoiceBtn>)>,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        let hover = matches!(interaction, Interaction::Hovered | Interaction::Pressed);
        let (bc, bdc) = choice_color(hover);
        *bg = BackgroundColor(bc);
        *border = BorderColor::all(bdc);
    }
}

/// System: animate dialog panel glow based on current speaker.
pub fn animate_dialog_glow(
    time: Res<Time>, state: Res<DialogState>,
    mut query: Query<&mut BoxShadow, With<DialogPanelGlow>>,
) {
    let active = match &state.active_scene { Some(a) => a, None => return };
    let node = match active.scene.nodes.get(active.node_index) { Some(n) => n, None => return };
    let (r, g, b) = match node.speaker {
        Speaker::Anna => DIALOG_ANNA_COLOR, Speaker::System => DIALOG_SYSTEM_COLOR,
        Speaker::Narrator => DIALOG_NARRATOR_COLOR, Speaker::Player => DIALOG_PLAYER_COLOR,
    };
    let pulse = 0.10 + 0.06 * (time.elapsed_secs() * 1.5).sin();
    for mut shadow in query.iter_mut() {
        *shadow = BoxShadow::new(Color::srgba(r, g, b, pulse), Val::ZERO, Val::ZERO,
            Val::Px(DIALOG_GLOW_SPREAD), Val::Px(DIALOG_GLOW_BLUR));
    }
}

/// System: animate Anna's circle with smooth mood color transitions + pulse.
pub fn animate_dialog_circle(
    time: Res<Time>, state: Res<DialogState>,
    mut glow_mood: ResMut<AnnaGlowMood>,
    mut query: Query<(&mut BackgroundColor, &mut BoxShadow), With<DialogAnnaCircle>>,
) {
    let active = match &state.active_scene { Some(a) => a, None => return };
    let node = match active.scene.nodes.get(active.node_index) { Some(n) => n, None => return };
    // Smoothly lerp current color toward target
    let speed = ANNA_GLOW_LERP_SPEED * time.delta_secs();
    glow_mood.current.0 += (glow_mood.target.0 - glow_mood.current.0) * speed;
    glow_mood.current.1 += (glow_mood.target.1 - glow_mood.current.1) * speed;
    glow_mood.current.2 += (glow_mood.target.2 - glow_mood.current.2) * speed;
    // Only animate the circle when Anna or Narrator is speaking
    if node.speaker != Speaker::Anna && node.speaker != Speaker::Narrator { return; }
    let t = time.elapsed_secs();
    let brightness = 0.85 + 0.15 * (t * 1.2).sin();
    let (r, g, b) = glow_mood.current;
    let (cr, cg, cb) = (r * brightness, g * brightness, b * brightness);
    for (mut bg, mut shadow) in query.iter_mut() {
        *bg = BackgroundColor(Color::srgb(cr, cg, cb));
        *shadow = BoxShadow::new(Color::srgba(cr, cg, cb, 0.5 * brightness),
            Val::ZERO, Val::ZERO, Val::Px(5.0), Val::Px(12.0));
    }
}

/// Update the skip hint text based on dialog state.
pub fn update_skip_hint(
    state: &DialogState,
    hint_q: &mut Query<(&mut Text, &mut TextColor), (With<DialogSkipHint>, Without<DialogBodyText>, Without<DialogSpeakerText>)>,
) {
    let active = match &state.active_scene { Some(a) => a, None => return };
    for (mut text, mut color) in hint_q.iter_mut() {
        let (t, a) = if active.reaction_text.is_some() || active.choices_visible { ("", 0.0) }
            else if !active.text_complete { ("Click to skip", 0.5) }
            else { ("Click to continue", 0.6) };
        *text = Text::new(t);
        *color = TextColor(Color::srgba(0.5, 0.55, 0.65, a));
    }
}

