// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::effects;

/// Spawn the main game UI: deposit slots, HUD, stars.
pub fn spawn_delivery_ui(
    mut commands: Commands,
    font: Res<DeliveryFont>,
    mut state: ResMut<DeliveryState>,
) {
    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    state.game_started = true;

    // Star background
    effects::spawn_star_background(&mut commands);

    // Root container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        DeliveryRoot,
    ));

    spawn_hud(&mut commands, &tf);
    spawn_deposit_slots(&mut commands, &tf);

    // Intro text (fades out)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        IntroOverlay { timer: 2.0 },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
    )).with_children(|parent| {
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        }).with_children(|col| {
            col.spawn((
                Text::new("Distributing resources..."),
                tf(TITLE_FONT_D),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.95)),
            ));
            col.spawn((
                Text::new(format!("{} pods to deliver", state.total_pods)),
                tf(HUD_FONT),
                TextColor(Color::srgba(1.0, 1.0, 0.7, 0.8)),
            ));
            col.spawn((
                Text::new("Click the matching colored slot!"),
                tf(SLOT_LABEL_FONT),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
            ));
        });
    });
}

fn spawn_hud(commands: &mut Commands, tf: &dyn Fn(f32) -> TextFont) {
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(HUD_TOP_MARGIN),
        left: Val::Px(HUD_SIDE_MARGIN),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        ..default()
    }).with_children(|col| {
        col.spawn((
            Text::new("Delivered: 0"),
            tf(HUD_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.85)),
            ScoreText,
        ));
        col.spawn((
            Text::new("Remaining: 100"),
            tf(HUD_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
            PodsRemainingText,
        ));
        col.spawn((
            Text::new("Speed: --"),
            tf(SPEED_METER_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
            SpeedMeterText,
        ));
    });

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(HUD_TOP_MARGIN),
        right: Val::Px(HUD_SIDE_MARGIN),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::End,
        row_gap: Val::Px(6.0),
        ..default()
    }).with_children(|col| {
        col.spawn((
            Text::new(""),
            tf(STREAK_FONT),
            TextColor(Color::srgba(1.0, 0.9, 0.2, 0.0)),
            StreakText,
        ));
        col.spawn((
            Text::new(""),
            tf(STREAK_FONT),
            TextColor(Color::srgba(1.0, 0.85, 0.2, 0.0)),
            MultiplierText,
        ));
    });
}

fn spawn_deposit_slots(commands: &mut Commands, tf: &dyn Fn(f32) -> TextFont) {
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(SLOT_BOTTOM_MARGIN),
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        column_gap: Val::Px(SLOT_GAP),
        ..default()
    }).with_children(|row| {
        for i in 0..SLOT_COUNT {
            let (r, g, b) = POD_COLORS[i];
            row.spawn((
                Button,
                Node {
                    width: Val::Px(SLOT_WIDTH),
                    height: Val::Px(SLOT_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(SLOT_BORDER)),
                    border_radius: BorderRadius::all(Val::Px(SLOT_CORNER)),
                    row_gap: Val::Px(2.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(r, g, b, 0.15)),
                BorderColor::all(Color::srgba(
                    UNSELECTED_BORDER_COLOR.0,
                    UNSELECTED_BORDER_COLOR.1,
                    UNSELECTED_BORDER_COLOR.2,
                    UNSELECTED_BORDER_COLOR.3,
                )),
                BoxShadow::new(
                    Color::srgba(r, g, b, 0.0),
                    Val::ZERO, Val::ZERO,
                    Val::Px(SLOT_MATCH_GLOW_SPREAD), Val::Px(SLOT_MATCH_GLOW_BLUR),
                ),
                DepositSlot(i),
            )).with_children(|slot| {
                slot.spawn((
                    Text::new(RESOURCE_ICONS[i]),
                    tf(SLOT_ICON_FONT),
                    TextColor(Color::srgb(r, g, b)),
                ));
                slot.spawn((
                    Text::new(RESOURCE_NAMES[i]),
                    tf(SLOT_LABEL_FONT),
                    TextColor(Color::srgba(r, g, b, 0.7)),
                ));
            });
        }
    });
}

/// Sync HUD text with game state.
pub fn sync_hud(
    state: Res<DeliveryState>,
    time: Res<Time>,
    mut score_q: Query<&mut Text, (With<ScoreText>, Without<PodsRemainingText>,
        Without<StreakText>, Without<MultiplierText>, Without<SpeedMeterText>)>,
    mut remaining_q: Query<&mut Text, (With<PodsRemainingText>, Without<ScoreText>,
        Without<StreakText>, Without<MultiplierText>, Without<SpeedMeterText>)>,
    mut streak_q: Query<(&mut Text, &mut TextColor), (With<StreakText>,
        Without<ScoreText>, Without<PodsRemainingText>,
        Without<MultiplierText>, Without<SpeedMeterText>)>,
    mut mult_q: Query<(&mut Text, &mut TextColor), (With<MultiplierText>,
        Without<ScoreText>, Without<PodsRemainingText>,
        Without<StreakText>, Without<SpeedMeterText>)>,
    mut speed_q: Query<&mut Text, (With<SpeedMeterText>, Without<ScoreText>,
        Without<PodsRemainingText>, Without<StreakText>, Without<MultiplierText>)>,
) {
    let delivered: u32 = state.score.iter().sum();
    for mut text in score_q.iter_mut() {
        *text = Text::new(format!("Delivered: {}", delivered));
    }
    let remaining = state.total_pods.saturating_sub(state.pods_resolved);
    for mut text in remaining_q.iter_mut() {
        *text = Text::new(format!("Remaining: {}", remaining));
    }
    for (mut text, mut color) in streak_q.iter_mut() {
        if state.streak >= STREAK_TIER_1 {
            *text = Text::new(format!("streak: {}", state.streak));
            *color = TextColor(Color::srgba(1.0, 0.9, 0.2, 0.95));
        } else if state.streak > 0 {
            *text = Text::new(format!("streak: {}", state.streak));
            *color = TextColor(Color::srgba(1.0, 1.0, 1.0, 0.4));
        } else {
            *text = Text::new("");
            *color = TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0));
        }
    }
    let t = time.elapsed_secs();
    for (mut text, mut color) in mult_q.iter_mut() {
        let mult = state.streak_mult();
        if mult > 1.0 {
            *text = Text::new(format!("x{:.1}", mult));
            let pulse = ((t * 3.0).sin() * 0.15 + 0.85).clamp(0.7, 1.0);
            *color = TextColor(Color::srgba(1.0, 0.85, 0.2, pulse));
        } else {
            *text = Text::new("");
            *color = TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0));
        }
    }
    for mut text in speed_q.iter_mut() {
        let speed_pct = (state.difficulty * 100.0).clamp(0.0, 100.0);
        *text = Text::new(format!("Speed: {:.0}%", speed_pct));
    }
}

/// Highlight hovered/selected deposit slots + glow matching pods.
pub fn highlight_slots(
    state: Res<DeliveryState>,
    time: Res<Time>,
    pod_q: Query<&Pod>,
    mut slot_q: Query<(&DepositSlot, &Interaction, &mut BorderColor, &mut BoxShadow)>,
) {
    let t = time.elapsed_secs();
    let pulse = ((t * SLOT_MATCH_PULSE_SPEED).sin() * 0.3 + 0.7).clamp(0.4, 1.0);

    let mut falling_colors = [false; 5];
    for pod in pod_q.iter() {
        if pod.routed.is_none() {
            falling_colors[pod.color.index()] = true;
        }
    }

    for (slot, interaction, mut border, mut shadow) in slot_q.iter_mut() {
        let selected = state.selected_slot == Some(slot.0);
        let hovered = *interaction == Interaction::Hovered;
        let matching = falling_colors[slot.0];

        if selected || hovered {
            *border = BorderColor::all(Color::srgba(
                SELECTED_BORDER_COLOR.0, SELECTED_BORDER_COLOR.1,
                SELECTED_BORDER_COLOR.2, if selected { 1.0 } else { 0.6 },
            ));
        } else {
            *border = BorderColor::all(Color::srgba(
                UNSELECTED_BORDER_COLOR.0, UNSELECTED_BORDER_COLOR.1,
                UNSELECTED_BORDER_COLOR.2, UNSELECTED_BORDER_COLOR.3,
            ));
        }

        let (r, g, b) = POD_COLORS[slot.0];
        if matching {
            *shadow = BoxShadow::new(
                Color::srgba(r, g, b, 0.3 * pulse),
                Val::ZERO, Val::ZERO,
                Val::Px(SLOT_MATCH_GLOW_SPREAD), Val::Px(SLOT_MATCH_GLOW_BLUR),
            );
        } else {
            *shadow = BoxShadow::new(
                Color::srgba(r, g, b, 0.0),
                Val::ZERO, Val::ZERO,
                Val::Px(0.0), Val::Px(0.0),
            );
        }
    }
}

/// Fade out the intro overlay.
pub fn fade_intro(
    time: Res<Time>,
    mut intro_q: Query<(Entity, &mut IntroOverlay, &mut BackgroundColor)>,
    mut commands: Commands,
) {
    for (entity, mut intro, mut bg) in intro_q.iter_mut() {
        intro.timer -= time.delta_secs();
        if intro.timer <= 0.0 {
            commands.entity(entity).despawn();
        } else if intro.timer < 0.5 {
            let alpha = intro.timer / 0.5 * 0.5;
            *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, alpha));
        }
    }
}

/// Intro overlay component.
#[derive(Component)]
pub struct IntroOverlay {
    pub timer: f32,
}
