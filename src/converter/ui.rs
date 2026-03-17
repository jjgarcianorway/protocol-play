// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::grid;

/// Spawn the full converter UI: grid, resource tanks, crystal pile, title.
pub fn spawn_converter_ui(
    mut commands: Commands,
    font: Res<ConverterFont>,
    mut grid_state: ResMut<GridState>,
    mut pile: ResMut<CrystalPile>,
) {
    // Fill grid initially
    grid::fill_grid(&mut grid_state, &mut pile);

    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    // Root container
    commands.spawn((
        Node {
            width: Val::Percent(100.0), height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(40.0),
            ..default()
        },
        ConverterRoot,
    )).with_children(|root: &mut ChildSpawnerCommands| {
        // Left: Crystal pile
        spawn_pile_panel(root, &tf);

        // Center: Grid
        spawn_grid(root, &grid_state, &tf);

        // Right: Resource tanks
        spawn_tanks(root, &tf);
    });
}

fn spawn_pile_panel(parent: &mut ChildSpawnerCommands, tf: &dyn Fn(f32) -> TextFont) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|col: &mut ChildSpawnerCommands| {
        col.spawn((
            Text::new("Crystals"),
            tf(PILE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        ));
        // Pile bar background
        col.spawn((
            Node {
                width: Val::Px(PILE_BAR_WIDTH),
                height: Val::Px(PILE_BAR_HEIGHT),
                border_radius: BorderRadius::all(Val::Px(TANK_CORNER)),
                flex_direction: FlexDirection::ColumnReverse,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(TANK_BG.0, TANK_BG.1, TANK_BG.2, TANK_BG.3)),
        )).with_children(|bar: &mut ChildSpawnerCommands| {
            bar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(TANK_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.6, 0.6, 0.7, 0.5)),
                PileFill,
            ));
        });
        col.spawn((
            Text::new(format!("{}", INITIAL_PILE_SIZE)),
            tf(PILE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
            PileCountText,
        ));
    });
}

fn spawn_grid(
    parent: &mut ChildSpawnerCommands, grid_state: &GridState, tf: &dyn Fn(f32) -> TextFont,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|col: &mut ChildSpawnerCommands| {
        col.spawn((
            Text::new("The Converter"),
            tf(TITLE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
        ));
        col.spawn((
            Text::new(""),
            tf(CHAIN_SIZE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 0.5, 0.0)),
            ChainSizeLabel,
        ));
        col.spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(CELL_GAP),
            padding: UiRect::all(Val::Px(8.0)),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        }).with_children(|gc: &mut ChildSpawnerCommands| {
            for row in 0..grid_state.height {
                gc.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(CELL_GAP),
                    ..default()
                }).with_children(|rn: &mut ChildSpawnerCommands| {
                    for c in 0..grid_state.width {
                        let bg = cell_bg_color(grid_state.cells[row][c]);
                        rn.spawn((
                            Button,
                            Node {
                                width: Val::Px(CELL_SIZE),
                                height: Val::Px(CELL_SIZE),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(CELL_BORDER)),
                                border_radius: BorderRadius::all(Val::Px(CELL_CORNER)),
                                ..default()
                            },
                            BackgroundColor(bg),
                            BorderColor::all(Color::NONE),
                            GridCell { row, col: c },
                        ));
                    }
                });
            }
        });
    });
}

fn spawn_tanks(parent: &mut ChildSpawnerCommands, tf: &dyn Fn(f32) -> TextFont) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(TANK_GAP),
        align_items: AlignItems::FlexEnd,
        ..default()
    }).with_children(|tanks: &mut ChildSpawnerCommands| {
        for i in 0..5 {
            let (cr, cg, cb) = CRYSTAL_COLORS[i];
            tanks.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            }).with_children(|tc: &mut ChildSpawnerCommands| {
                tc.spawn((
                    Text::new(RESOURCE_ICONS[i]),
                    tf(TANK_LABEL_FONT),
                    TextColor(Color::srgb(cr, cg, cb)),
                    TankLabel(i),
                ));
                tc.spawn((
                    Node {
                        width: Val::Px(TANK_WIDTH),
                        height: Val::Px(TANK_HEIGHT),
                        border_radius: BorderRadius::all(Val::Px(TANK_CORNER)),
                        flex_direction: FlexDirection::ColumnReverse,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(
                        TANK_BG.0, TANK_BG.1, TANK_BG.2, TANK_BG.3,
                    )),
                )).with_children(|bar: &mut ChildSpawnerCommands| {
                    bar.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(0.0),
                            border_radius: BorderRadius::all(Val::Px(TANK_CORNER)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(cr, cg, cb, 0.7)),
                        TankFill(i),
                    ));
                });
                tc.spawn((
                    Text::new("0%"),
                    tf(TANK_PCT_FONT),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                    TankLabel(i + 100),
                ));
            });
        }
    });
}

/// Compute background color for a cell.
pub fn cell_bg_color(crystal: Option<CrystalColor>) -> Color {
    match crystal {
        Some(c) => {
            let (r, g, b) = c.rgb();
            Color::srgba(r, g, b, 0.85)
        }
        None => Color::srgba(CELL_EMPTY_COLOR.0, CELL_EMPTY_COLOR.1,
                             CELL_EMPTY_COLOR.2, CELL_EMPTY_COLOR.3),
    }
}

/// Update cell visuals to match grid state.
pub fn sync_grid_visuals(
    grid_state: Res<GridState>,
    mut query: Query<(&GridCell, &mut BackgroundColor)>,
) {
    for (cell, mut bg) in query.iter_mut() {
        let crystal = grid_state.cells[cell.row][cell.col];
        *bg = BackgroundColor(cell_bg_color(crystal));
    }
}

/// Update tank fill visuals.
pub fn sync_tank_visuals(
    tanks: Res<ResourceTanks>,
    mut fill_q: Query<(&TankFill, &mut Node), Without<TankLabel>>,
    mut label_q: Query<(&TankLabel, &mut Text)>,
) {
    for (tank_fill, mut node) in fill_q.iter_mut() {
        let pct = (tanks.levels[tank_fill.0] / RESOURCE_MAX * 100.0).clamp(0.0, 100.0);
        node.height = Val::Percent(pct);
    }
    for (tank_label, mut text) in label_q.iter_mut() {
        if tank_label.0 >= 100 {
            let idx = tank_label.0 - 100;
            let pct = (tanks.levels[idx] / RESOURCE_MAX * 100.0).clamp(0.0, 100.0);
            *text = Text::new(format!("{:.0}%", pct));
        }
    }
}

/// Update pile display.
pub fn sync_pile_visuals(
    pile: Res<CrystalPile>,
    mut text_q: Query<&mut Text, With<PileCountText>>,
    mut fill_q: Query<&mut Node, With<PileFill>>,
) {
    for mut text in text_q.iter_mut() {
        *text = Text::new(format!("{}", pile.remaining));
    }
    for mut node in fill_q.iter_mut() {
        let pct = if pile.total > 0 {
            pile.remaining as f32 / pile.total as f32 * 100.0
        } else { 0.0 };
        node.height = Val::Percent(pct);
    }
}
