// SPDX-License-Identifier: GPL-3.0-or-later

//! Cinematic loading/progress screen shown during world generation.

use bevy::prelude::*;
use super::constants::*;
use super::types::*;
use super::world_seed;
use super::crew_stories;
use super::CachedWorldState;
use crate::save_state::save_game_state;

/// Loading screen progress state.
#[derive(Resource)]
pub struct LoadingState {
    pub progress: f32,
    pub step: usize,
    pub step_timer: f32,
    pub step_text_len: usize,
    pub complete: bool,
    pub hold_timer: f32,
    pub world_generated: bool,
    pub fade_out_alpha: f32,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self { progress: 0.0, step: 0, step_timer: 0.0, step_text_len: 0,
               complete: false, hold_timer: 0.0, world_generated: false,
               fade_out_alpha: 0.0 }
    }
}

const STEP_THRESHOLDS: [f32; 8] = [0.0, 0.10, 0.25, 0.40, 0.55, 0.70, 0.85, 0.95];
const DETAIL_COLLAPSE_AT: f32 = 0.20;
const DETAIL_SEVERITY_AT: f32 = 0.25;
const DETAIL_CREW_AT: f32 = 0.35;
const DETAIL_SEED_AT: f32 = 0.50;

#[derive(Component)] pub struct LoadingRoot;
#[derive(Component)] pub struct LoadingBarFill;
#[derive(Component)] pub struct LoadingStepText;
#[derive(Component)] pub struct LoadingDetailText(pub usize);
#[derive(Component)] pub struct LoadingFadeOverlay;

fn step_text(step: usize, crew_count: u32) -> String {
    match step {
        0 => "Seeding the universe...".to_string(),
        1 => "Mapping Earth's final years...".to_string(),
        2 => format!("Selecting the crew... {} souls", crew_count),
        3 => "Charting the Aurora's course...".to_string(),
        4 => "Calibrating repair systems... 149 configurations".to_string(),
        5 => "Initializing Anna...".to_string(),
        6 => "Recording Earth's last broadcast...".to_string(),
        7 => "Your world is ready.".to_string(),
        _ => String::new(),
    }
}

fn severity_description(severity: u32) -> &'static str {
    match severity {
        1 => "Regional crisis",
        2 => "Continental destabilization",
        3 => "Global cascade failure",
        4 => "Near-total civilizational collapse",
        _ => "Complete civilizational extinction",
    }
}

/// Spawn loading screen UI on entering Loading state.
pub fn enter_loading(
    mut commands: Commands,
    font: Res<MissionFont>,
    menu_root_q: Query<Entity, With<super::main_menu::MainMenuRoot>>,
    menu_version_q: Query<Entity, With<super::main_menu::MenuVersionLabel>>,
    menu_fadeout_q: Query<Entity, With<super::main_menu::MenuFadeOut>>,
    confirm_q: Query<Entity, With<super::main_menu::ConfirmDialog>>,
    submenu_q: Query<Entity, With<super::main_menu::NewGameSubMenu>>,
) {
    // Despawn menu UI
    for entity in menu_root_q.iter() { commands.entity(entity).despawn(); }
    for entity in menu_version_q.iter() { commands.entity(entity).despawn(); }
    for entity in menu_fadeout_q.iter() { commands.entity(entity).despawn(); }
    for entity in confirm_q.iter() { commands.entity(entity).despawn(); }
    for entity in submenu_q.iter() { commands.entity(entity).despawn(); }

    let font = &font.0;

    // Root container — centered column
    commands.spawn((
        LoadingRoot,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            ..default()
        },
    )).with_children(|root| {
        // Step text (typewriter)
        root.spawn((
            LoadingStepText,
            Text::new(""),
            TextFont { font: font.clone(), font_size: LOADING_STEP_FONT, ..default() },
            TextColor(Color::srgba(
                LOADING_STEP_COLOR.0, LOADING_STEP_COLOR.1,
                LOADING_STEP_COLOR.2, LOADING_STEP_COLOR.3,
            )),
            Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
        ));

        // Progress bar container
        root.spawn(Node {
            width: Val::Px(LOADING_BAR_WIDTH),
            height: Val::Px(LOADING_BAR_HEIGHT),
            border_radius: BorderRadius::all(Val::Px(LOADING_BAR_CORNER)),
            overflow: Overflow::clip(),
            ..default()
        }).with_children(|bar_bg| {
            // Background track
            bar_bg.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(LOADING_BAR_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgba(
                    LOADING_BAR_BG.0, LOADING_BAR_BG.1,
                    LOADING_BAR_BG.2, LOADING_BAR_BG.3,
                )),
            ));
            // Fill bar
            bar_bg.spawn((
                LoadingBarFill,
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(LOADING_BAR_CORNER)),
                    ..default()
                },
                BackgroundColor(Color::srgb(
                    LOADING_BAR_FILL.0, LOADING_BAR_FILL.1, LOADING_BAR_FILL.2,
                )),
                BoxShadow::new(
                    Color::srgba(
                        LOADING_BAR_GLOW_COLOR.0, LOADING_BAR_GLOW_COLOR.1,
                        LOADING_BAR_GLOW_COLOR.2, LOADING_BAR_GLOW_COLOR.3,
                    ),
                    Val::ZERO, Val::ZERO,
                    Val::Px(LOADING_BAR_GLOW_BLUR),
                    Val::Px(LOADING_BAR_GLOW_SPREAD),
                ),
            ));
        });

        // World details container
        root.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        }).with_children(|details| {
            for i in 0..4 {
                details.spawn((
                    LoadingDetailText(i),
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: LOADING_DETAIL_FONT,
                        ..default()
                    },
                    TextColor(Color::srgba(
                        LOADING_DETAIL_COLOR.0, LOADING_DETAIL_COLOR.1,
                        LOADING_DETAIL_COLOR.2, 0.0,
                    )),
                ));
            }
        });
    });

    // Fade-out overlay (used when transitioning to Playing)
    commands.spawn((
        LoadingFadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
    ));

    commands.insert_resource(LoadingState::default());
}

/// Regenerate world data during loading (runs once).
pub fn generate_world_during_loading(
    mut loading: ResMut<LoadingState>,
    mut cached: ResMut<CachedWorldState>,
    mut gs: ResMut<crate::save_state::GameState>,
) {
    if loading.world_generated || loading.progress < 0.05 { return; }
    loading.world_generated = true;

    // Regenerate world + crew from current seed
    let world = world_seed::generate_world(gs.world_seed);
    let crew = crew_stories::generate_crew(gs.world_seed);
    if gs.day <= 1 { gs.crew_count = world.aurora_crew; save_game_state(&gs); }

    // Update collapse flag
    let collapse_flag = match world.earth_collapse.primary_cause {
        world_seed::CollapseCause::ClimateCollapse => "collapse_climate",
        world_seed::CollapseCause::ResourceWars => "collapse_resource",
        world_seed::CollapseCause::PandemicCascade => "collapse_pandemic",
        world_seed::CollapseCause::NuclearExchange => "collapse_nuclear",
        world_seed::CollapseCause::AIUprising => "collapse_ai",
        world_seed::CollapseCause::PoliticalCollapse => "collapse_political",
        world_seed::CollapseCause::EconomicMeltdown => "collapse_economic",
    };
    if !gs.decisions.contains(&collapse_flag.to_string()) {
        gs.decisions.push(collapse_flag.to_string());
        save_game_state(&gs);
    }

    cached.world = world;
    cached.crew = crew;
}

/// Advance progress bar and update step text with typewriter effect.
pub fn tick_loading_progress(
    time: Res<Time>,
    mut loading: ResMut<LoadingState>,
    cached: Res<CachedWorldState>,
    gs: Res<crate::save_state::GameState>,
    mut bar_q: Query<&mut Node, With<LoadingBarFill>>,
    mut step_q: Query<(&mut Text, &mut TextColor), With<LoadingStepText>>,
    mut detail_q: Query<(&LoadingDetailText, &mut Text, &mut TextColor),
        Without<LoadingStepText>>,
    mut fade_q: Query<&mut BackgroundColor, With<LoadingFadeOverlay>>,
    mut next_state: ResMut<NextState<AppPhase>>,
) {
    let dt = time.delta_secs();

    if loading.complete {
        // Hold phase, then fade to Playing
        loading.hold_timer += dt;
        if loading.hold_timer >= LOADING_HOLD_DURATION * 0.5 {
            loading.fade_out_alpha = ((loading.hold_timer - LOADING_HOLD_DURATION * 0.5)
                / (LOADING_HOLD_DURATION * 0.5)).clamp(0.0, 1.0);
            for mut bg in fade_q.iter_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, loading.fade_out_alpha);
            }
        }
        if loading.hold_timer >= LOADING_HOLD_DURATION {
            next_state.set(AppPhase::Playing);
        }
        // Pulse "Your world is ready." while holding
        pulse_ready_text(&time, &mut step_q);
        return;
    }

    // Advance progress
    loading.progress = (loading.progress + dt / LOADING_DURATION).min(1.0);

    // Update bar width
    for mut node in bar_q.iter_mut() {
        node.width = Val::Percent(loading.progress * 100.0);
    }

    // Determine current step
    let new_step = STEP_THRESHOLDS.iter()
        .rposition(|&t| loading.progress >= t)
        .unwrap_or(0);

    if new_step != loading.step {
        loading.step = new_step;
        loading.step_timer = 0.0;
        loading.step_text_len = 0;
    }

    // Typewriter effect
    loading.step_timer += dt;
    let full_text = step_text(loading.step, cached.world.aurora_crew);
    let target_len = (loading.step_timer * LOADING_TYPEWRITER_SPEED) as usize;
    let clamped = target_len.min(full_text.len());

    if clamped != loading.step_text_len {
        loading.step_text_len = clamped;
        for (mut text, _) in step_q.iter_mut() {
            let display: String = full_text.chars().take(clamped).collect();
            **text = display;
        }
    }

    // Update step text color (brighter for final step)
    if loading.step == 7 {
        for (_, mut color) in step_q.iter_mut() {
            color.0 = Color::srgba(
                LOADING_READY_COLOR.0, LOADING_READY_COLOR.1,
                LOADING_READY_COLOR.2, 1.0,
            );
        }
    }

    // World details fade-in
    update_details(&loading, &cached, &gs, dt, &mut detail_q);

    // Check completion
    if loading.progress >= 1.0 {
        loading.complete = true;
    }
}

fn update_details(
    loading: &LoadingState,
    cached: &CachedWorldState,
    gs: &crate::save_state::GameState,
    dt: f32,
    detail_q: &mut Query<(&LoadingDetailText, &mut Text, &mut TextColor),
        Without<LoadingStepText>>,
) {
    let details_data: [(f32, String); 4] = [
        (DETAIL_COLLAPSE_AT, format!(
            "Earth collapsed: {}", cached.world.earth_collapse.primary_cause.name()
        )),
        (DETAIL_SEVERITY_AT, format!(
            "Severity: {}", severity_description(cached.world.earth_collapse.severity)
        )),
        (DETAIL_CREW_AT, format!(
            "Crew: {} passengers", cached.world.aurora_crew
        )),
        (DETAIL_SEED_AT, format!("Seed: {:016X}", gs.world_seed)),
    ];

    for (detail_marker, mut text, mut color) in detail_q.iter_mut() {
        let idx = detail_marker.0;
        if idx >= 4 { continue; }
        let (threshold, ref content) = details_data[idx];

        if loading.progress >= threshold {
            if text.as_str().is_empty() {
                **text = content.clone();
            }
            let c = color.0.to_srgba();
            let new_alpha = (c.alpha + dt * LOADING_DETAIL_FADE_SPEED).min(1.0);
            color.0 = Color::srgba(
                LOADING_DETAIL_COLOR.0, LOADING_DETAIL_COLOR.1,
                LOADING_DETAIL_COLOR.2, new_alpha,
            );
        }
    }
}

fn pulse_ready_text(
    time: &Res<Time>,
    step_q: &mut Query<(&mut Text, &mut TextColor), With<LoadingStepText>>,
) {
    let t = time.elapsed_secs();
    let pulse = 0.8 + 0.2 * (t * LOADING_READY_PULSE_SPEED).sin();
    for (_, mut color) in step_q.iter_mut() {
        color.0 = Color::srgba(
            LOADING_READY_COLOR.0, LOADING_READY_COLOR.1,
            LOADING_READY_COLOR.2, pulse,
        );
    }
}

/// Cleanup loading screen when entering Playing state.
pub fn cleanup_loading(
    mut commands: Commands,
    root_q: Query<Entity, With<LoadingRoot>>,
    fade_q: Query<Entity, With<LoadingFadeOverlay>>,
) {
    for entity in root_q.iter() { commands.entity(entity).despawn(); }
    for entity in fade_q.iter() { commands.entity(entity).despawn(); }
    commands.remove_resource::<LoadingState>();
}
