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
        hud_row(parent, "CRYS", "0K", &tf, label_color, value_color, Some(CrystalText));
        hud_row(parent, "DIST", "0 AU", &tf, label_color, value_color, Some(DistanceText));
        hud_row(parent, "TIME", "0d", &tf, label_color, value_color, Some(TimeText));
    });
}

fn hud_row<C: Component>(parent: &mut ChildBuilder, label: &str, initial: &str, tf: &TextFont, label_color: Color, value_color: Color, marker: Option<C>) {
    parent.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(6.0), ..default() })
        .with_children(|row| {
            row.spawn((Text::new(label), tf.clone(), TextColor(label_color)));
            let mut cmd = row.spawn((Text::new(initial), tf.clone(), TextColor(value_color)));
            if let Some(m) = marker { cmd.insert(m); }
        });
}

pub fn update_hud(
    state: Res<ShipState>,
    mut dist_q: Query<&mut Text, (With<DistanceText>, Without<TimeText>, Without<CrystalText>)>,
    mut time_q: Query<&mut Text, (With<TimeText>, Without<DistanceText>, Without<CrystalText>)>,
    mut crystal_q: Query<&mut Text, (With<CrystalText>, Without<DistanceText>, Without<TimeText>)>,
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

    let mut crys_text = crystal_q.single_mut();
    if state.crystals >= 1_000_000 {
        **crys_text = format!("{:.1}M", state.crystals as f64 / 1_000_000.0);
    } else if state.crystals >= 1_000 {
        **crys_text = format!("{}K", state.crystals / 1_000);
    } else {
        **crys_text = format!("{}", state.crystals);
    }
}

pub fn update_game_time(mut state: ResMut<ShipState>, time: Res<Time>) {
    if !state.alive { return; }
    let dt = time.delta_secs();
    state.elapsed_time += dt;
    state.distance += SCROLL_SPEED * dt;
}
