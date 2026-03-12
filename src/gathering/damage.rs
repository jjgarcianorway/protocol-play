// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

pub fn update_shield_regen(mut state: ResMut<ShipState>, time: Res<Time>) {
    if !state.alive { return; }
    if state.shield < SHIELD_MAX {
        state.shield = (state.shield + SHIELD_REGEN_RATE * time.delta_secs()).min(SHIELD_MAX);
    }
}

pub fn update_screen_shake(
    mut shake: ResMut<ScreenShake>,
    mut camera_q: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    if shake.intensity > 0.01 {
        let mut rng = || (rand::random::<f32>() - 0.5) * 2.0;
        shake.offset = Vec3::new(rng() * shake.intensity, rng() * shake.intensity, 0.0);
        shake.intensity *= (-SCREEN_SHAKE_DECAY * dt).exp();
    } else {
        shake.offset = Vec3::ZERO;
        shake.intensity = 0.0;
    }

    let mut cam_tf = camera_q.single_mut();
    cam_tf.translation.x = shake.offset.x;
    cam_tf.translation.y = shake.offset.y;
}

pub fn spawn_bars(commands: &mut Commands, font: Handle<Font>) {
    // Right-side vertical bars container
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(BAR_MARGIN_PX),
        top: Val::Px(BAR_TOP_PX),
        bottom: Val::Px(BAR_BOTTOM_PX),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(BAR_GAP_PX),
        align_items: AlignItems::Stretch,
        ..default()
    }).with_children(|parent| {
        // Shield bar
        spawn_bar(parent, true, &font);
        // Life bar
        spawn_bar(parent, false, &font);
    });
}

fn spawn_bar(parent: &mut ChildBuilder, is_shield: bool, font: &Handle<Font>) {
    let label = if is_shield { "S" } else { "L" };
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        width: Val::Px(BAR_WIDTH_PX),
        ..default()
    }).with_children(|col| {
        col.spawn((
            Text::new(label),
            TextFont { font: font.clone(), font_size: HUD_FONT, ..default() },
            TextColor(Color::srgba(HUD_LABEL_COLOR.0, HUD_LABEL_COLOR.1, HUD_LABEL_COLOR.2, HUD_LABEL_COLOR.3)),
        ));
        // Bar background
        col.spawn((Node {
            width: Val::Px(BAR_WIDTH_PX),
            flex_grow: 1.0,
            border: UiRect::all(Val::Px(BAR_STROKE_PX)),
            margin: UiRect::top(Val::Px(4.0)),
            ..default()
        }, BackgroundColor(Color::srgba(BAR_BG_COLOR.0, BAR_BG_COLOR.1, BAR_BG_COLOR.2, BAR_BG_COLOR.3)),
           BorderColor(Color::srgba(BAR_STROKE_COLOR.0, BAR_STROKE_COLOR.1, BAR_STROKE_COLOR.2, BAR_STROKE_COLOR.3)),
        )).with_children(|bar| {
            // Fill (anchored to bottom)
            let full_color = if is_shield { SHIELD_FULL_COLOR } else { LIFE_FULL_COLOR };
            bar.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_self: AlignSelf::FlexEnd,
                ..default()
            }).with_children(|fill_parent| {
                let component = if is_shield {
                    fill_parent.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        BackgroundColor(Color::srgb(full_color.0, full_color.1, full_color.2)),
                        ShieldBarFill,
                    )).id()
                } else {
                    fill_parent.spawn((
                        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        BackgroundColor(Color::srgb(full_color.0, full_color.1, full_color.2)),
                        LifeBarFill,
                    )).id()
                };
                let _ = component;
            });
        });
    });
}

pub fn update_bars(
    state: Res<ShipState>,
    mut shield_q: Query<(&mut Node, &mut BackgroundColor), (With<ShieldBarFill>, Without<LifeBarFill>)>,
    mut life_q: Query<(&mut Node, &mut BackgroundColor), (With<LifeBarFill>, Without<ShieldBarFill>)>,
) {
    let shield_pct = (state.shield / SHIELD_MAX).clamp(0.0, 1.0);
    let (mut s_node, mut s_bg) = shield_q.single_mut();
    s_node.height = Val::Percent(shield_pct * 100.0);
    let t = shield_pct;
    s_bg.0 = Color::srgb(
        SHIELD_LOW_COLOR.0 + (SHIELD_FULL_COLOR.0 - SHIELD_LOW_COLOR.0) * t,
        SHIELD_LOW_COLOR.1 + (SHIELD_FULL_COLOR.1 - SHIELD_LOW_COLOR.1) * t,
        SHIELD_LOW_COLOR.2 + (SHIELD_FULL_COLOR.2 - SHIELD_LOW_COLOR.2) * t,
    );

    let life_pct = (state.life / LIFE_MAX).clamp(0.0, 1.0);
    let (mut l_node, mut l_bg) = life_q.single_mut();
    l_node.height = Val::Percent(life_pct * 100.0);
    let t = life_pct;
    l_bg.0 = Color::srgb(
        LIFE_LOW_COLOR.0 + (LIFE_FULL_COLOR.0 - LIFE_LOW_COLOR.0) * t,
        LIFE_LOW_COLOR.1 + (LIFE_FULL_COLOR.1 - LIFE_LOW_COLOR.1) * t,
        LIFE_LOW_COLOR.2 + (LIFE_FULL_COLOR.2 - LIFE_LOW_COLOR.2) * t,
    );
}
