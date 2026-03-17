// SPDX-License-Identifier: GPL-3.0-or-later

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::grid;

/// Spawn the full converter UI: grid, resource tanks, crystal pile, title, stars.
pub fn spawn_converter_ui(
    mut commands: Commands,
    font: Res<ConverterFont>,
    mut grid_state: ResMut<GridState>,
    mut pile: ResMut<CrystalPile>,
) {
    grid::fill_grid(&mut grid_state, &mut pile);

    let f = &font.0;
    let tf = |size: f32| TextFont { font: f.clone(), font_size: size, ..default() };

    spawn_star_background(&mut commands);

    // Root container — centered row layout
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
        spawn_pile_panel(root, &tf);
        spawn_grid_panel(root, &grid_state, &tf);
        spawn_tanks_panel(root, &tf);
    });
}

fn spawn_star_background(commands: &mut Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..STAR_COUNT {
        let x = rng.gen_range(1.0..99.0);
        let y = rng.gen_range(1.0..99.0);
        let size = rng.gen_range(STAR_MIN_SIZE..STAR_MAX_SIZE);
        let alpha = rng.gen_range(STAR_MIN_ALPHA..STAR_MAX_ALPHA);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(x),
                top: Val::Percent(y),
                width: Val::Px(size),
                height: Val::Px(size),
                border_radius: BorderRadius::all(Val::Px(size / 2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.7, 0.75, 1.0, alpha)),
            StarDot,
        ));
    }
}

/// Crystal pile — simple text counter on the left.
fn spawn_pile_panel(parent: &mut ChildSpawnerCommands, tf: &dyn Fn(f32) -> TextFont) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(4.0),
        width: Val::Px(80.0),
        ..default()
    }).with_children(|col: &mut ChildSpawnerCommands| {
        col.spawn((
            Text::new("Crystals"),
            tf(PILE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        ));
        col.spawn((
            Text::new(format!("{}", INITIAL_PILE_SIZE)),
            tf(PILE_FONT + 4.0),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            PileCountText,
        ));
        col.spawn((
            Text::new("remaining"),
            tf(PILE_FONT - 2.0),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
        ));
    });
}

/// Grid panel — title, chain info, and the grid itself.
fn spawn_grid_panel(
    parent: &mut ChildSpawnerCommands, grid_state: &GridState,
    tf: &dyn Fn(f32) -> TextFont,
) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|col: &mut ChildSpawnerCommands| {
        // Title
        col.spawn((
            Text::new("The Converter"),
            tf(TITLE_FONT),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.9)),
        ));
        // Chain size label (shown on hover)
        col.spawn(Node {
            min_height: Val::Px(28.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|info: &mut ChildSpawnerCommands| {
            info.spawn((
                Text::new(""),
                tf(CHAIN_SIZE_FONT),
                TextColor(Color::srgba(1.0, 1.0, 0.5, 0.0)),
                ChainSizeLabel,
            ));
        });
        // Grid with subtle dark panel + thin border
        col.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(CELL_GAP),
                padding: UiRect::all(Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(
                GRID_BG_COLOR.0, GRID_BG_COLOR.1, GRID_BG_COLOR.2, GRID_BG_ALPHA,
            )),
            BorderColor::all(Color::srgba(
                GRID_PANEL_BORDER.0, GRID_PANEL_BORDER.1,
                GRID_PANEL_BORDER.2, GRID_PANEL_BORDER.3,
            )),
        )).with_children(|gc: &mut ChildSpawnerCommands| {
            for row in 0..grid_state.height {
                gc.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(CELL_GAP),
                    ..default()
                }).with_children(|rn: &mut ChildSpawnerCommands| {
                    for c in 0..grid_state.width {
                        let (bg, border) = cell_colors(grid_state.cells[row][c]);
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
                            BorderColor::all(border),
                            GridCell { row, col: c },
                        ));
                    }
                });
            }
        });
    });
}

/// Resource tanks — 5 clean vertical bars on the right.
fn spawn_tanks_panel(parent: &mut ChildSpawnerCommands, tf: &dyn Fn(f32) -> TextFont) {
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
                // Resource icon
                tc.spawn((
                    Text::new(RESOURCE_ICONS[i]),
                    tf(TANK_LABEL_FONT),
                    TextColor(Color::srgb(cr, cg, cb)),
                ));
                // Tank bar container
                tc.spawn((
                    Node {
                        width: Val::Px(TANK_WIDTH),
                        height: Val::Px(TANK_HEIGHT),
                        border_radius: BorderRadius::all(Val::Px(TANK_CORNER)),
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(
                        TANK_BG.0, TANK_BG.1, TANK_BG.2, TANK_BG.3,
                    )),
                    BorderColor::all(Color::srgba(0.20, 0.21, 0.28, 0.5)),
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
                // Percentage text
                tc.spawn((
                    Text::new("0%"),
                    tf(TANK_PCT_FONT),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                    TankLabel(i),
                ));
                // Resource name
                tc.spawn((
                    Text::new(RESOURCE_NAMES[i]),
                    tf(TANK_PCT_FONT - 1.0),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
                ));
            });
        }
    });
}

/// Compute background + border color for a cell.
pub fn cell_colors(crystal: Option<CrystalColor>) -> (Color, Color) {
    match crystal {
        Some(c) => {
            let (r, g, b) = c.rgb();
            // Solid color with subtle lighter top edge effect via border
            let bg = Color::srgba(r, g, b, 0.85);
            let border = Color::srgba(
                (r + 0.12).min(1.0), (g + 0.12).min(1.0), (b + 0.12).min(1.0), 0.35,
            );
            (bg, border)
        }
        None => (
            Color::srgba(CELL_EMPTY_COLOR.0, CELL_EMPTY_COLOR.1,
                         CELL_EMPTY_COLOR.2, CELL_EMPTY_COLOR.3),
            Color::srgba(CELL_EMPTY_BORDER.0, CELL_EMPTY_BORDER.1,
                         CELL_EMPTY_BORDER.2, CELL_EMPTY_BORDER.3),
        ),
    }
}

/// Update cell visuals to match grid state, with clean yellow highlight.
pub fn sync_grid_visuals(
    grid_state: Res<GridState>,
    hovered: Res<HoveredGroup>,
    mut query: Query<(&GridCell, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (cell, mut bg, mut border) in query.iter_mut() {
        let crystal = grid_state.cells[cell.row][cell.col];
        let (cell_bg, cell_border) = cell_colors(crystal);
        *bg = BackgroundColor(cell_bg);

        let is_hovered = hovered.cells.iter().any(|&(r, c)| r == cell.row && c == cell.col);
        if is_hovered {
            let (hr, hg, hb, ha) = HIGHLIGHT_BORDER_COLOR;
            *border = BorderColor::all(Color::srgba(hr, hg, hb, ha));
        } else {
            *border = BorderColor::all(cell_border);
        }
    }
}

/// Update tank fill visuals (no shimmer — clean solid fill).
pub fn sync_tank_visuals(
    tanks: Res<ResourceTanks>,
    mut fill_q: Query<(&TankFill, &mut Node, &mut BackgroundColor), Without<TankLabel>>,
    mut label_q: Query<(&TankLabel, &mut Text)>,
) {
    for (tank_fill, mut node, mut bg) in fill_q.iter_mut() {
        let pct = (tanks.levels[tank_fill.0] / RESOURCE_MAX * 100.0).clamp(0.0, 100.0);
        node.height = Val::Percent(pct);
        let (cr, cg, cb) = CRYSTAL_COLORS[tank_fill.0];
        *bg = BackgroundColor(Color::srgba(cr, cg, cb, 0.7));
    }
    for (tank_label, mut text) in label_q.iter_mut() {
        let pct = (tanks.levels[tank_label.0] / RESOURCE_MAX * 100.0).clamp(0.0, 100.0);
        *text = Text::new(format!("{:.0}%", pct));
    }
}

/// Detect tank level changes and spawn flash + floating text.
pub fn detect_tank_changes(
    mut tanks: ResMut<ResourceTanks>,
    mut commands: Commands,
    font: Res<ConverterFont>,
) {
    for i in 0..5 {
        let delta = tanks.levels[i] - tanks.prev_levels[i];
        if delta.abs() > 0.01 {
            // Spawn brief flash marker
            commands.spawn((
                TankFlash { index: i, timer: TANK_FLASH_DURATION },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(0.0),
                    height: Val::Px(0.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
            // Spawn floating "+N" text
            if delta > 0.1 {
                super::effects::spawn_tank_float(&mut commands, &font.0, i, delta);
            }
        }
        tanks.prev_levels[i] = tanks.levels[i];
    }
}

/// Animate tank flash markers (despawn when done).
pub fn animate_tank_flashes(
    time: Res<Time>,
    mut flash_q: Query<(Entity, &mut TankFlash)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut flash) in flash_q.iter_mut() {
        flash.timer -= dt;
        if flash.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Update pile display — simple text counter.
pub fn sync_pile_visuals(
    pile: Res<CrystalPile>,
    mut text_q: Query<&mut Text, With<PileCountText>>,
) {
    for mut text in text_q.iter_mut() {
        *text = Text::new(format!("{}", pile.remaining));
    }
}
