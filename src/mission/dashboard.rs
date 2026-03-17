// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;

/// Spawn the ship status dashboard (left/center panel).
pub fn spawn_dashboard(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent.spawn(Node {
        width: Val::Percent(DASHBOARD_WIDTH_PCT),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(SECTION_PAD)),
        row_gap: Val::Px(SECTION_GAP),
        ..default()
    }).with_children(|dash| {
        // Ship name
        dash.spawn((
            Text::new(SHIP_NAME),
            TextFont { font: font.clone(), font_size: SHIP_NAME_FONT, ..default() },
            TextColor(Color::srgb(SHIP_NAME_COLOR.0, SHIP_NAME_COLOR.1, SHIP_NAME_COLOR.2)),
        ));

        // Section: SYSTEMS
        spawn_section_header(dash, "SYSTEMS", font);

        // Resource bars
        for i in 0..5 {
            spawn_resource_bar(dash, i, font);
        }

        // Section: STATUS
        spawn_section_header(dash, "STATUS", font);

        // Crystal reserves
        dash.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(8.0), ..default() })
            .with_children(|row| {
                row.spawn((
                    Text::new("Crystal reserves: "),
                    TextFont { font: font.clone(), font_size: INFO_FONT, ..default() },
                    TextColor(Color::srgb(INFO_COLOR.0, INFO_COLOR.1, INFO_COLOR.2)),
                ));
                row.spawn((
                    Text::new("47"),
                    TextFont { font: font.clone(), font_size: INFO_FONT, ..default() },
                    TextColor(Color::srgb(INFO_HIGHLIGHT_COLOR.0, INFO_HIGHLIGHT_COLOR.1, INFO_HIGHLIGHT_COLOR.2)),
                    CrystalText,
                ));
            });

        // Crew status
        dash.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(8.0), ..default() })
            .with_children(|row| {
                row.spawn((
                    Text::new(""),
                    TextFont { font: font.clone(), font_size: INFO_FONT, ..default() },
                    TextColor(Color::srgb(INFO_HIGHLIGHT_COLOR.0, INFO_HIGHLIGHT_COLOR.1, INFO_HIGHLIGHT_COLOR.2)),
                    CrewText,
                ));
            });

        // Journey progress
        dash.spawn(Node { flex_direction: FlexDirection::Row, column_gap: Val::Px(8.0), ..default() })
            .with_children(|row| {
                row.spawn((
                    Text::new(""),
                    TextFont { font: font.clone(), font_size: INFO_FONT, ..default() },
                    TextColor(Color::srgb(INFO_COLOR.0, INFO_COLOR.1, INFO_COLOR.2)),
                    JourneyText,
                ));
            });
    });
}

fn spawn_section_header(parent: &mut ChildSpawnerCommands, label: &str, font: &Handle<Font>) {
    parent.spawn(Node {
        margin: UiRect::top(Val::Px(4.0)),
        ..default()
    }).with_child((
        Text::new(label),
        TextFont { font: font.clone(), font_size: SECTION_TITLE_FONT, ..default() },
        TextColor(Color::srgb(
            SECTION_TITLE_COLOR.0, SECTION_TITLE_COLOR.1, SECTION_TITLE_COLOR.2,
        )),
    ));
}

fn spawn_resource_bar(parent: &mut ChildSpawnerCommands, index: usize, font: &Handle<Font>) {
    let color = RES_COLORS[index];
    let name = RES_NAMES[index];
    let icon = RES_ICONS[index];

    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(2.0),
        ..default()
    }).with_children(|col| {
        // Label row: icon + name ... percentage
        col.spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Px(BAR_WIDTH),
            ..default()
        }).with_children(|row| {
            row.spawn((
                Text::new(format!("{} {}", icon, name)),
                TextFont { font: font.clone(), font_size: BAR_LABEL_FONT, ..default() },
                TextColor(Color::srgb(color.0, color.1, color.2)),
            ));
            row.spawn((
                Text::new("0%"),
                TextFont { font: font.clone(), font_size: BAR_PCT_FONT, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                ResourcePctText(index),
            ));
        });

        // Bar background
        col.spawn((
            Node {
                width: Val::Px(BAR_WIDTH),
                height: Val::Px(BAR_HEIGHT),
                border_radius: BorderRadius::all(Val::Px(BAR_CORNER)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(
                BAR_BG_COLOR.0, BAR_BG_COLOR.1, BAR_BG_COLOR.2, BAR_BG_COLOR.3,
            )),
        )).with_children(|bar_bg| {
            // Bar fill
            bar_bg.spawn((
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(BAR_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgb(color.0, color.1, color.2)),
                ResourceBarFill(index),
            ));
        });
    });
}

/// System: smoothly animate resource bars toward target values.
pub fn animate_resource_bars(
    time: Res<Time>,
    ship: Res<ShipStatus>,
    mut display: ResMut<BarDisplayValues>,
    mut fill_q: Query<(&mut Node, &ResourceBarFill)>,
    mut pct_q: Query<(&mut Text, &ResourcePctText)>,
) {
    let dt = time.delta_secs();
    let targets = [ship.power, ship.life_support, ship.cryo, ship.shields, ship.repair];

    for i in 0..5 {
        let diff = targets[i] - display.values[i];
        if diff.abs() < 0.1 {
            display.values[i] = targets[i];
        } else {
            display.values[i] += diff * BAR_LERP_SPEED * dt;
        }
    }

    for (mut node, fill) in fill_q.iter_mut() {
        node.width = Val::Percent(display.values[fill.0].clamp(0.0, 100.0));
    }

    for (mut text, pct) in pct_q.iter_mut() {
        *text = Text::new(format!("{}%", display.values[pct.0] as u32));
    }
}

/// System: update crew and journey text.
pub fn update_status_texts(
    ship: Res<ShipStatus>,
    mut crew_q: Query<&mut Text, (With<CrewText>, Without<JourneyText>, Without<CrystalText>)>,
    mut journey_q: Query<&mut Text, (With<JourneyText>, Without<CrewText>, Without<CrystalText>)>,
    mut crystal_q: Query<&mut Text, (With<CrystalText>, Without<CrewText>, Without<JourneyText>)>,
) {
    for mut text in crew_q.iter_mut() {
        *text = Text::new(format!("{} crew in cryo", fmt_num(ship.crew_count)));
    }
    for mut text in journey_q.iter_mut() {
        *text = Text::new(format!("Day {} — {:.1} AU traveled", ship.day, ship.distance_au));
    }
    for mut text in crystal_q.iter_mut() {
        *text = Text::new(format!("{}", ship.crystals));
    }
}

/// Format number with thousands separator.
fn fmt_num(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}
