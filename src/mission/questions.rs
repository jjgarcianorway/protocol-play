// SPDX-License-Identifier: GPL-3.0-or-later

//! Anna's interactive philosophical questions — UI and systems.
//! Question data lives in question_data.rs.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::question_data::{QUESTIONS, QuestionDef};
use crate::save_state::{save_game_state, GameState};

/// Find the next unanswered question the player has unlocked.
pub fn next_pending_question(gs: &GameState) -> Option<&'static QuestionDef> {
    for q in QUESTIONS {
        if gs.bot_level >= q.required_bot_level {
            let answered = gs.decisions.iter().any(|d| {
                d.starts_with(&format!("q{}_", q.id))
            });
            if !answered {
                return Some(q);
            }
        }
    }
    None
}

// === UI Components ===

/// Marker for the question modal overlay.
#[derive(Component)]
pub struct QuestionModal;

/// Marker for question text.
#[derive(Component)]
pub struct QuestionText;

/// Marker for an option button, stores the decision key.
#[derive(Component)]
pub struct QuestionOptionBtn {
    pub decision_key: String,
    pub question_id: u32,
}

/// Marker for Anna's reaction text after answering.
#[derive(Component)]
pub struct ReactionText;

/// Marker for the reaction overlay (shown after answering).
#[derive(Component)]
pub struct ReactionOverlay {
    pub timer: f32,
}

/// Resource tracking whether we should show a question this visit.
#[derive(Resource)]
pub struct QuestionState {
    /// Whether we've already shown/checked for a question this session.
    pub checked: bool,
    /// Timer before showing the question (brief delay after returning).
    pub delay_timer: f32,
    /// Whether a question is currently displayed.
    pub showing: bool,
}

impl Default for QuestionState {
    fn default() -> Self {
        Self { checked: false, delay_timer: 3.0, showing: false }
    }
}

// === Question modal constants ===
const MODAL_BG_ALPHA: f32 = 0.88;
const QUESTION_FONT: f32 = 20.0;
const OPTION_FONT: f32 = 16.0;
const REACTION_FONT: f32 = 17.0;
const REACTION_HOLD: f32 = 3.0;
const MODAL_MAX_WIDTH: f32 = 600.0;
const OPTION_PAD: f32 = 14.0;
const OPTION_CORNER: f32 = 8.0;
const OPTION_BG: (f32, f32, f32, f32) = (0.12, 0.14, 0.20, 0.9);
const OPTION_HOVER_BG: (f32, f32, f32, f32) = (0.20, 0.24, 0.34, 0.95);
const OPTION_BORDER: (f32, f32, f32, f32) = (0.30, 0.35, 0.50, 0.6);
const OPTION_HOVER_BORDER: (f32, f32, f32, f32) = (0.50, 0.60, 0.85, 0.9);

/// Spawn the question modal overlay.
pub fn spawn_question_modal(
    commands: &mut Commands, font: &Handle<Font>, q: &QuestionDef,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.03, 0.06, MODAL_BG_ALPHA)),
        GlobalZIndex(30),
        QuestionModal,
    )).with_children(|overlay| {
        overlay.spawn(Node {
            max_width: Val::Px(MODAL_MAX_WIDTH),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            padding: UiRect::all(Val::Px(32.0)),
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
                Text::new(q.question),
                TextFont { font: font.clone(), font_size: QUESTION_FONT, ..default() },
                TextColor(Color::srgb(0.92, 0.90, 0.85)),
                QuestionText,
            ));
            for opt in q.options {
                spawn_option_btn(col, font, opt.label, opt.decision_key, q.id);
            }
        });
    });
}

fn spawn_option_btn(
    parent: &mut ChildSpawnerCommands, font: &Handle<Font>,
    label: &str, key: &str, qid: u32,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(OPTION_PAD)),
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(OPTION_CORNER)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(
            OPTION_BG.0, OPTION_BG.1, OPTION_BG.2, OPTION_BG.3,
        )),
        BorderColor::all(Color::srgba(
            OPTION_BORDER.0, OPTION_BORDER.1, OPTION_BORDER.2, OPTION_BORDER.3,
        )),
        QuestionOptionBtn { decision_key: key.to_string(), question_id: qid },
    )).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: OPTION_FONT, ..default() },
        TextColor(Color::srgb(0.85, 0.88, 0.95)),
    ));
}

/// System: check if we should show a question after returning from a game.
pub fn check_pending_question(
    time: Res<Time>,
    mut qs: ResMut<QuestionState>,
    gs: Res<GameState>,
    running: Res<RunningGame>,
    mut commands: Commands,
    font: Res<MissionFont>,
    modal_q: Query<Entity, With<QuestionModal>>,
) {
    if running.0.is_some() { return; }
    if qs.showing || qs.checked { return; }
    if !modal_q.is_empty() { return; }

    qs.delay_timer -= time.delta_secs();
    if qs.delay_timer > 0.0 { return; }

    qs.checked = true;
    if let Some(q) = next_pending_question(&gs) {
        spawn_question_modal(&mut commands, &font.0, q);
        qs.showing = true;
    }
}

/// System: hover effects on option buttons.
pub fn question_option_hover(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<QuestionOptionBtn>),
    >,
) {
    for (interaction, mut bg, mut border) in query.iter_mut() {
        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(
                    OPTION_HOVER_BG.0, OPTION_HOVER_BG.1,
                    OPTION_HOVER_BG.2, OPTION_HOVER_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    OPTION_HOVER_BORDER.0, OPTION_HOVER_BORDER.1,
                    OPTION_HOVER_BORDER.2, OPTION_HOVER_BORDER.3,
                ));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(
                    OPTION_BG.0, OPTION_BG.1, OPTION_BG.2, OPTION_BG.3,
                ));
                *border = BorderColor::all(Color::srgba(
                    OPTION_BORDER.0, OPTION_BORDER.1,
                    OPTION_BORDER.2, OPTION_BORDER.3,
                ));
            }
        }
    }
}

/// System: handle clicking an option button.
pub fn question_option_click(
    query: Query<(&Interaction, &QuestionOptionBtn), Changed<Interaction>>,
    mut gs: ResMut<GameState>,
    mut commands: Commands,
    modal_q: Query<Entity, With<QuestionModal>>,
    font: Res<MissionFont>,
    mut qs: ResMut<QuestionState>,
) {
    for (interaction, opt) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        gs.decisions.push(opt.decision_key.clone());
        save_game_state(&gs);

        let reaction = QUESTIONS.iter()
            .find(|q| q.id == opt.question_id)
            .and_then(|q| q.options.iter()
                .find(|o| o.decision_key == opt.decision_key))
            .map(|o| o.anna_reaction)
            .unwrap_or("...");

        for entity in modal_q.iter() {
            commands.entity(entity).despawn();
        }
        spawn_reaction_overlay(&mut commands, &font.0, reaction);
        qs.showing = true;
    }
}

/// Spawn a brief reaction overlay after answering.
fn spawn_reaction_overlay(
    commands: &mut Commands, font: &Handle<Font>, reaction: &str,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.03, 0.06, MODAL_BG_ALPHA)),
        GlobalZIndex(30),
        ReactionOverlay { timer: REACTION_HOLD },
    )).with_children(|overlay| {
        overlay.spawn(Node {
            max_width: Val::Px(MODAL_MAX_WIDTH),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            padding: UiRect::all(Val::Px(32.0)),
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
                Text::new(reaction),
                TextFont { font: font.clone(), font_size: REACTION_FONT, ..default() },
                TextColor(Color::srgb(0.90, 0.85, 0.75)),
                ReactionText,
            ));
        });
    });
}

/// System: auto-dismiss reaction overlay after timer.
pub fn update_reaction_overlay(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ReactionOverlay)>,
    mut qs: ResMut<QuestionState>,
) {
    for (entity, mut reaction) in query.iter_mut() {
        reaction.timer -= time.delta_secs();
        if reaction.timer <= 0.0 {
            commands.entity(entity).despawn();
            qs.showing = false;
        }
    }
}

/// Reset question check state when returning from a game.
pub fn reset_question_check(qs: &mut QuestionState) {
    qs.checked = false;
    qs.delay_timer = 3.0;
    qs.showing = false;
}
