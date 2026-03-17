// SPDX-License-Identifier: GPL-3.0-or-later

mod constants;
mod types;
mod grid;
mod ui;
mod effects;
mod results;

use bevy::prelude::*;
use bevy::post_process::bloom::Bloom;
use bevy::render::render_resource::*;
use constants::*;
use types::*;

pub fn build_app(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(
        CLEAR_COLOR_C.0, CLEAR_COLOR_C.1, CLEAR_COLOR_C.2,
    )))
    .insert_resource(GridState::default())
    .insert_resource(CrystalPile::default())
    .insert_resource(ResourceTanks::default())
    .insert_resource(ConversionStats::default())
    .insert_resource(HoveredGroup::default())
    .init_state::<ConverterPhase>()
    .add_systems(Startup, (setup_converter, ui::spawn_converter_ui.after(setup_converter)))
    .add_systems(Update, (
        handle_hover,
        handle_click.after(handle_hover),
        process_grid_phases.after(handle_click),
        ui::sync_grid_visuals.after(process_grid_phases),
        ui::sync_tank_visuals,
        ui::sync_pile_visuals,
        ui::detect_tank_changes.after(ui::sync_tank_visuals),
        ui::animate_tank_flashes,
        effects::update_burst_particles,
        effects::animate_cascade_text,
        effects::animate_stars,
        update_chain_label.after(handle_hover),
        check_round_complete.after(process_grid_phases),
    ).run_if(in_state(ConverterPhase::Processing)))
    .add_systems(OnEnter(ConverterPhase::Results), results::spawn_results_screen)
    .add_systems(Update,
        results::return_button_interaction.run_if(in_state(ConverterPhase::Results)),
    );
}

fn setup_converter(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        Camera3d::default(),
        Bloom {
            intensity: BLOOM_INTENSITY_C,
            low_frequency_boost: BLOOM_LF_BOOST_C,
            low_frequency_boost_curvature: 0.7,
            high_pass_frequency: 1.0,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let font_bytes = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf").to_vec();
    let font = fonts.add(Font::try_from_bytes(font_bytes).unwrap());
    commands.insert_resource(ConverterFont(font.clone()));

    let vignette = create_vignette(&mut images);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(vignette),
    ));

    commands.spawn(Node {
        position_type: PositionType::Absolute,
        right: Val::Px(6.0),
        bottom: Val::Px(4.0),
        ..default()
    }).with_child((
        Text::new(format!("v{}", env!("CARGO_PKG_VERSION"))),
        TextFont { font, font_size: 11.0, ..default() },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
    ));
}

fn create_vignette(images: &mut Assets<Image>) -> Handle<Image> {
    let size = 256u32;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let center = size as f32 / 2.0;
    for y in 0..size {
        for x in 0..size {
            let dx = (x as f32 - center) / center;
            let dy = (y as f32 - center) / center;
            let dist = (dx * dx + dy * dy).sqrt().clamp(0.0, 1.0);
            let alpha = if dist < 0.5 { 0.0 }
                else { ((dist - 0.5) * 2.0).powi(2) * 0.6 };
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 0;
            data[idx + 1] = 0;
            data[idx + 2] = 0;
            data[idx + 3] = (alpha * 255.0) as u8;
        }
    }
    images.add(Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, default(),
    ))
}

fn handle_hover(
    grid_state: Res<GridState>,
    mut hovered: ResMut<HoveredGroup>,
    interaction_q: Query<(&Interaction, &GridCell)>,
) {
    if grid_state.phase != GridPhase::Idle {
        hovered.cells.clear();
        hovered.color = None;
        return;
    }

    let mut found = None;
    for (interaction, cell) in interaction_q.iter() {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            if grid_state.cells[cell.row][cell.col].is_some() {
                found = Some((cell.row, cell.col));
                break;
            }
        }
    }

    match found {
        Some((row, col)) => {
            let group = grid::flood_fill(&grid_state, row, col);
            hovered.color = grid_state.cells[row][col];
            hovered.cells = group;
        }
        None => {
            hovered.cells.clear();
            hovered.color = None;
        }
    }
}

fn handle_click(
    mut grid_state: ResMut<GridState>,
    mut stats: ResMut<ConversionStats>,
    hovered: Res<HoveredGroup>,
    interaction_q: Query<(&Interaction, &GridCell), Changed<Interaction>>,
    mut commands: Commands,
) {
    if grid_state.phase != GridPhase::Idle { return; }

    let mut clicked = false;
    for (interaction, _cell) in interaction_q.iter() {
        if *interaction == Interaction::Pressed {
            clicked = true;
            break;
        }
    }

    if !clicked || hovered.cells.is_empty() { return; }

    let color = match hovered.color {
        Some(c) => c,
        None => return,
    };

    let chain_size = hovered.cells.len() as u32;
    let mult = efficiency_mult(chain_size);

    grid::remove_cells(&mut grid_state, &hovered.cells);

    stats.total_converted += chain_size as u64;
    stats.chains_triggered += 1;
    if chain_size > stats.best_chain {
        stats.best_chain = chain_size;
    }

    let tank_target = Vec2::new(900.0, 300.0);
    for &(row, col) in &hovered.cells {
        let cell_pos = Vec2::new(
            GRID_LEFT_MARGIN + col as f32 * (CELL_SIZE + CELL_GAP) + CELL_SIZE / 2.0,
            120.0 + row as f32 * (CELL_SIZE + CELL_GAP) + CELL_SIZE / 2.0,
        );
        effects::spawn_burst_particles(
            &mut commands, cell_pos, tank_target, color,
            (mult * 2.0) as u32 + 1,
        );
    }

    grid_state.phase = GridPhase::Gravity;
    grid_state.phase_timer = GRAVITY_DELAY;
}

fn process_grid_phases(
    time: Res<Time>,
    mut grid_state: ResMut<GridState>,
    mut pile: ResMut<CrystalPile>,
    mut stats: ResMut<ConversionStats>,
    mut commands: Commands,
    font: Res<ConverterFont>,
) {
    let dt = time.delta_secs();
    grid_state.phase_timer -= dt;
    if grid_state.phase_timer > 0.0 { return; }

    match grid_state.phase {
        GridPhase::Idle => {}
        GridPhase::Bursting => {
            grid_state.phase = GridPhase::Gravity;
            grid_state.phase_timer = GRAVITY_DELAY;
        }
        GridPhase::Gravity => {
            grid::apply_gravity(&mut grid_state);
            grid_state.phase = GridPhase::CascadeCheck;
            grid_state.phase_timer = CASCADE_DELAY;
        }
        GridPhase::CascadeCheck => {
            let groups = grid::find_all_groups(&grid_state);
            if let Some((color, group)) = groups.into_iter().next() {
                let chain_size = group.len() as u32;
                grid::remove_cells(&mut grid_state, &group);
                stats.cascades += 1;
                stats.total_converted += chain_size as u64;
                if chain_size > stats.best_chain {
                    stats.best_chain = chain_size;
                }

                // Cascade feedback text
                effects::spawn_cascade_text(&mut commands, &font.0, stats.cascades);

                let tank_target = Vec2::new(900.0, 300.0);
                let mult = efficiency_mult(chain_size);
                for &(row, col) in &group {
                    let cell_pos = Vec2::new(
                        GRID_LEFT_MARGIN + col as f32 * (CELL_SIZE + CELL_GAP) + CELL_SIZE / 2.0,
                        120.0 + row as f32 * (CELL_SIZE + CELL_GAP) + CELL_SIZE / 2.0,
                    );
                    effects::spawn_burst_particles(
                        &mut commands, cell_pos, tank_target, color,
                        (mult * 2.0) as u32 + 1,
                    );
                }

                grid_state.phase = GridPhase::Gravity;
                grid_state.phase_timer = GRAVITY_DELAY;
            } else {
                if pile.remaining > 0 {
                    grid_state.phase = GridPhase::Refilling;
                    grid_state.phase_timer = 0.1;
                } else {
                    grid_state.phase = GridPhase::Idle;
                }
            }
        }
        GridPhase::Refilling => {
            grid::refill_from_pile(&mut grid_state, &mut pile);
            grid_state.phase = GridPhase::Idle;
        }
    }
}

fn update_chain_label(
    hovered: Res<HoveredGroup>,
    mut size_q: Query<
        (&mut Text, &mut TextColor),
        (With<ChainSizeLabel>, Without<ChainMultLabel>),
    >,
    mut mult_q: Query<
        (&mut Text, &mut TextColor),
        (With<ChainMultLabel>, Without<ChainSizeLabel>),
    >,
) {
    for (mut text, mut color) in size_q.iter_mut() {
        if hovered.cells.is_empty() {
            *color = TextColor(Color::srgba(1.0, 1.0, 0.5, 0.0));
        } else {
            let size = hovered.cells.len();
            *text = Text::new(format!("Chain: {}", size));
            *color = TextColor(Color::srgba(1.0, 1.0, 0.5, 0.9));
        }
    }
    for (mut text, mut color) in mult_q.iter_mut() {
        if hovered.cells.is_empty() {
            *color = TextColor(Color::srgba(1.0, 1.0, 0.5, 0.0));
        } else {
            let size = hovered.cells.len();
            let mult = efficiency_mult(size as u32);
            *text = Text::new(format!("x{:.1}", mult));
            let c = if mult >= 1.5 {
                Color::srgba(0.3, 1.0, 0.4, 0.85)
            } else if mult >= 1.0 {
                Color::srgba(1.0, 1.0, 0.5, 0.85)
            } else {
                Color::srgba(1.0, 0.4, 0.3, 0.7)
            };
            *color = TextColor(c);
        }
    }
}

fn check_round_complete(
    grid_state: Res<GridState>,
    pile: Res<CrystalPile>,
    mut next_state: ResMut<NextState<ConverterPhase>>,
) {
    if grid_state.phase != GridPhase::Idle { return; }
    if pile.remaining == 0 && !grid::grid_has_crystals(&grid_state) {
        next_state.set(ConverterPhase::Results);
    }
}
