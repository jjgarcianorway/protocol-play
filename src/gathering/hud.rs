// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

pub fn spawn_hud(commands: &mut Commands, font: Handle<Font>) {
    let label_color = Color::srgba(HUD_LABEL_COLOR.0, HUD_LABEL_COLOR.1, HUD_LABEL_COLOR.2, HUD_LABEL_COLOR.3);
    let value_color = Color::srgba(HUD_VALUE_COLOR.0, HUD_VALUE_COLOR.1, HUD_VALUE_COLOR.2, HUD_VALUE_COLOR.3);
    let tf = TextFont { font, font_size: HUD_FONT, ..default() };

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(HUD_MARGIN_PX),
        right: Val::Px(HUD_MARGIN_PX + BAR_MARGIN_PX + BAR_WIDTH_PX * 2.0 + BAR_GAP_PX + 20.0),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(HUD_GAP_PX),
        align_items: AlignItems::FlexEnd,
        ..default()
    }).with_children(|parent| {
        // Distance
        parent.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(6.0), ..default() })
            .with_children(|row| {
                row.spawn((Text::new("DIST"), tf.clone(), TextColor(label_color)));
                row.spawn((Text::new("0 AU"), tf.clone(), TextColor(value_color), DistanceText));
            });
        // Time
        parent.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(6.0), ..default() })
            .with_children(|row| {
                row.spawn((Text::new("TIME"), tf.clone(), TextColor(label_color)));
                row.spawn((Text::new("0d"), tf.clone(), TextColor(value_color), TimeText));
            });
    });
}

pub fn update_hud(
    state: Res<ShipState>,
    mut dist_q: Query<&mut Text, (With<DistanceText>, Without<TimeText>)>,
    mut time_q: Query<&mut Text, (With<TimeText>, Without<DistanceText>)>,
) {
    let mut dist_text = dist_q.single_mut();
    let au = state.distance * 0.01;
    if au < 1.0 {
        **dist_text = format!("{:.2} AU", au);
    } else {
        **dist_text = format!("{:.1} AU", au);
    }
    let mut time_text = time_q.single_mut();
    let days = (state.elapsed_time * 0.1) as u32;
    **time_text = format!("{}d", days);
}

pub fn update_game_time(mut state: ResMut<ShipState>, time: Res<Time>) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    state.elapsed_time += dt;
    state.distance += SCROLL_SPEED * dt;
}
