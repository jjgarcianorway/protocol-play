// SPDX-License-Identifier: GPL-3.0-or-later
//! Exit confirmation modal — "Return to Mission Control? Progress will be lost."

use bevy::prelude::*;
use super::types::{GameScene, SceneFade};

/// Marker for the exit confirmation overlay.
#[derive(Component)]
pub struct ExitConfirmOverlay;

/// Marker for Yes/No buttons.
#[derive(Component)]
pub enum ExitConfirmBtn { Yes, No }

/// Resource tracking whether the confirm dialog is showing.
#[derive(Resource, Default)]
pub struct ExitConfirmOpen(pub bool);

const OVERLAY_BG: Color = Color::srgba(0.02, 0.03, 0.06, 0.85);
const PANEL_BG: Color = Color::srgba(0.08, 0.09, 0.14, 0.95);
const BTN_BG: Color = Color::srgb(0.12, 0.14, 0.22);
const BTN_HOVER: Color = Color::srgb(0.20, 0.24, 0.35);
const TEXT_COLOR: Color = Color::srgb(0.88, 0.88, 0.92);
#[allow(dead_code)]
const WARN_COLOR: Color = Color::srgb(0.95, 0.7, 0.3);

/// System: ESC opens the exit confirmation (if not already open).
/// Works in any GameScene that isn't Dashboard.
pub fn esc_open_confirm(
    keys: Res<ButtonInput<KeyCode>>,
    scene: Option<Res<State<GameScene>>>,
    mut confirm: ResMut<ExitConfirmOpen>,
    overlay_q: Query<Entity, With<ExitConfirmOverlay>>,
) {
    if !keys.just_pressed(KeyCode::Escape) { return; }
    let Some(s) = scene else { return };
    if *s.get() == GameScene::Dashboard { return; }
    if confirm.0 {
        // ESC while confirm is open = close it (cancel)
        confirm.0 = false;
        return;
    }
    if overlay_q.is_empty() {
        confirm.0 = true;
    }
}

/// System: spawn/despawn the confirmation overlay based on ExitConfirmOpen.
pub fn manage_confirm_overlay(
    confirm: Res<ExitConfirmOpen>,
    mut commands: Commands,
    overlay_q: Query<Entity, With<ExitConfirmOverlay>>,
    font: Option<Res<super::types::MissionFont>>,
) {
    if confirm.0 && overlay_q.is_empty() {
        let f = match font {
            Some(ref f) => f.0.clone(),
            None => return,
        };
        spawn_confirm(&mut commands, &f);
    } else if !confirm.0 {
        for e in overlay_q.iter() { commands.entity(e).despawn(); }
    }
}

fn spawn_confirm(commands: &mut Commands, font: &Handle<Font>) {
    commands.spawn((
        ExitConfirmOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(OVERLAY_BG),
        GlobalZIndex(200),
    )).with_children(|overlay| {
        overlay.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(32.0)),
                row_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                max_width: Val::Px(420.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        )).with_children(|panel| {
            panel.spawn((
                Text::new("Return to Mission Control?"),
                TextFont { font: font.clone(), font_size: 20.0, ..default() },
                TextColor(TEXT_COLOR),
            ));
            panel.spawn((
                Text::new("Your progress has been saved."),
                TextFont { font: font.clone(), font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.5, 0.8, 0.6)),
            ));
            panel.spawn(Node { height: Val::Px(8.0), ..default() });
            // Buttons row
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            }).with_children(|row| {
                // Yes
                row.spawn((
                    Button, ExitConfirmBtn::Yes,
                    Node {
                        padding: UiRect::axes(Val::Px(28.0), Val::Px(10.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(BTN_BG),
                )).with_child((
                    Text::new("Yes, return"),
                    TextFont { font: font.clone(), font_size: 16.0, ..default() },
                    TextColor(TEXT_COLOR),
                ));
                // No
                row.spawn((
                    Button, ExitConfirmBtn::No,
                    Node {
                        padding: UiRect::axes(Val::Px(28.0), Val::Px(10.0)),
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(BTN_BG),
                )).with_child((
                    Text::new("Continue playing"),
                    TextFont { font: font.clone(), font_size: 16.0, ..default() },
                    TextColor(TEXT_COLOR),
                ));
            });
        });
    });
}

/// System: handle button clicks on the confirm dialog.
pub fn confirm_btn_click(
    query: Query<(&Interaction, &ExitConfirmBtn), Changed<Interaction>>,
    mut confirm: ResMut<ExitConfirmOpen>,
    mut next_scene: ResMut<NextState<GameScene>>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        match btn {
            ExitConfirmBtn::Yes => {
                confirm.0 = false;
                next_scene.set(GameScene::Dashboard);
            }
            ExitConfirmBtn::No => {
                confirm.0 = false;
            }
        }
    }
}

/// System: hover effect on confirm buttons.
pub fn confirm_btn_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), (With<ExitConfirmBtn>, Changed<Interaction>)>,
) {
    for (interaction, mut bg) in query.iter_mut() {
        bg.0 = if *interaction == Interaction::Hovered { BTN_HOVER } else { BTN_BG };
    }
}
